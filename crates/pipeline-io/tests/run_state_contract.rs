use std::{
    fs,
    path::{Path, PathBuf},
    process,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use pipeline_core::{RepoPaths, StageName};
use pipeline_io::{
    create_run_record, find_latest_resume_candidate, write_stage_failed, write_stage_passed,
    write_stage_pending, write_stage_running, StageMode,
};
use serde_json::{json, Value};

static NEXT_TEMP_REPO_ID: AtomicU64 = AtomicU64::new(0);

struct TempRepo {
    root: PathBuf,
}

impl TempRepo {
    fn new() -> Self {
        let unique_id = format!(
            "{}-{}-{}",
            process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after unix epoch")
                .as_nanos(),
            NEXT_TEMP_REPO_ID.fetch_add(1, Ordering::Relaxed)
        );
        let root = std::env::temp_dir().join(format!("pipeline-io-run-state-tests-{unique_id}"));

        fs::create_dir_all(root.join(".git")).expect("temp repo should create .git directory");
        write_file(&root.join(".ci/agent/AGENTS.md"), "test harness\n");
        write_file(&root.join("docs/progress.json"), "{}\n");
        write_file(&root.join("research/README.md"), "test harness\n");

        Self { root }
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn write(&self, relative: impl AsRef<Path>, contents: &str) {
        write_file(&self.root.join(relative), contents);
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directories should be created");
    }
    fs::write(path, contents).expect("file should be written");
}

fn lesson_paths(repo: &TempRepo, lesson_id: &str) -> pipeline_core::LessonPaths {
    RepoPaths::from_root(repo.root().to_path_buf())
        .expect("repository root should resolve")
        .resolve_lesson(lesson_id)
        .expect("lesson should resolve")
}

fn run_ids(lesson: &pipeline_core::LessonPaths) -> Vec<String> {
    let mut run_ids = fs::read_dir(lesson.run_root_dir())
        .expect("run root should exist")
        .map(|entry| {
            entry
                .expect("run dir entry should be readable")
                .file_name()
                .into_string()
                .expect("run dir name should be valid utf-8")
        })
        .collect::<Vec<_>>();
    run_ids.sort();
    run_ids
}

fn single_run_id(lesson: &pipeline_core::LessonPaths) -> String {
    let run_ids = run_ids(lesson);
    assert_eq!(
        run_ids.len(),
        1,
        "expected exactly one run, got {run_ids:?}"
    );
    run_ids.into_iter().next().expect("run id should exist")
}

fn read_text(path: &Path) -> String {
    fs::read_to_string(path).expect("file should be readable")
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&read_text(path)).expect("json should parse")
}

fn seed_run_metadata(
    lesson: &pipeline_core::LessonPaths,
    run_id: &str,
    run_json: &Value,
    stage_json: Option<&Value>,
) {
    write_file(
        &lesson.run_metadata_path(run_id),
        &(serde_json::to_string_pretty(run_json).expect("run json should serialize") + "\n"),
    );

    if let Some(stage_json) = stage_json {
        write_file(
            &lesson.stage_state_path(run_id, StageName::Convert),
            &(serde_json::to_string_pretty(stage_json).expect("stage json should serialize")
                + "\n"),
        );
    }
}

#[test]
fn create_pending_running_and_passed_preserve_the_frozen_schema_and_run_status_transitions() {
    let repo = TempRepo::new();
    let lesson = lesson_paths(&repo, "L2");

    let run = create_run_record(&lesson, "convert", true).expect("run record should be created");
    let run_id = single_run_id(&lesson);
    let run_json_path = lesson.run_metadata_path(&run_id);
    let stage_json_path = lesson.stage_state_path(&run_id, StageName::Convert);

    let created_text = read_text(&run_json_path);
    assert!(
        created_text.starts_with("{\n  \"schema_version\": 1,\n"),
        "run.json should stay pretty printed: {created_text}"
    );
    assert!(
        created_text.ends_with("}\n"),
        "run.json should end with a trailing newline: {created_text:?}"
    );

    let created_json = read_json(&run_json_path);
    let created_at = created_json["created_at_epoch_ms"]
        .as_u64()
        .expect("created_at should be u64");
    let updated_at = created_json["updated_at_epoch_ms"]
        .as_u64()
        .expect("updated_at should be u64");
    assert_eq!(created_json["schema_version"], json!(1));
    assert_eq!(created_json["run_id"], json!(run_id));
    assert_eq!(created_json["lesson_id"], json!("L2"));
    assert_eq!(created_json["command"], json!("convert"));
    assert_eq!(created_json["resume_requested"], json!(true));
    assert_eq!(created_json["resumed_from_run_id"], Value::Null);
    assert_eq!(created_json["status"], json!("running"));
    assert_eq!(created_at, updated_at);

    write_stage_pending(&run, StageName::Convert, StageMode::Executed)
        .expect("pending stage should be written");
    let pending_text = read_text(&stage_json_path);
    assert!(
        pending_text.starts_with("{\n  \"schema_version\": 1,\n"),
        "stage file should stay pretty printed: {pending_text}"
    );
    assert!(
        pending_text.ends_with("}\n"),
        "stage file should end with a trailing newline: {pending_text:?}"
    );
    let pending_json = read_json(&stage_json_path);
    let pending_run_json = read_json(&run_json_path);
    assert_eq!(pending_json["schema_version"], json!(1));
    assert_eq!(pending_json["run_id"], json!(run_id));
    assert_eq!(pending_json["lesson_id"], json!("L2"));
    assert_eq!(pending_json["stage"], json!("convert"));
    assert_eq!(pending_json["status"], json!("pending"));
    assert_eq!(pending_json["mode"], json!("executed"));
    assert_eq!(
        pending_json["output_dir"],
        json!("research/pipeline/1-plain/L2")
    );
    assert_eq!(pending_json["reused_from_run_id"], Value::Null);
    assert_eq!(pending_json["error"], Value::Null);
    assert_eq!(pending_run_json["status"], json!("running"));
    assert!(
        pending_run_json["updated_at_epoch_ms"]
            .as_u64()
            .expect("updated_at should be u64")
            >= created_at
    );

    write_stage_running(&run, StageName::Convert, StageMode::Executed)
        .expect("running stage should be written");
    let running_json = read_json(&stage_json_path);
    let running_run_json = read_json(&run_json_path);
    assert_eq!(running_json["status"], json!("running"));
    assert_eq!(running_json["mode"], json!("executed"));
    assert_eq!(running_json["error"], Value::Null);
    assert_eq!(running_run_json["status"], json!("running"));

    write_stage_passed(&run, StageName::Convert, StageMode::Executed, None)
        .expect("passed stage should be written");
    let passed_json = read_json(&stage_json_path);
    let passed_run_json = read_json(&run_json_path);
    assert_eq!(passed_json["status"], json!("passed"));
    assert_eq!(passed_json["mode"], json!("executed"));
    assert_eq!(passed_json["reused_from_run_id"], Value::Null);
    assert_eq!(passed_json["error"], Value::Null);
    assert_eq!(passed_run_json["status"], json!("passed"));
    assert_eq!(passed_run_json["resumed_from_run_id"], Value::Null);
}

#[test]
fn failed_stage_write_persists_the_error_and_flips_the_run_status() {
    let repo = TempRepo::new();
    let lesson = lesson_paths(&repo, "L2");
    let run = create_run_record(&lesson, "convert", false).expect("run record should be created");
    let run_id = single_run_id(&lesson);

    write_stage_pending(&run, StageName::Convert, StageMode::Executed)
        .expect("pending stage should be written");
    write_stage_failed(
        &run,
        StageName::Convert,
        StageMode::Executed,
        "  invalid integer for MINERU_RESULT_TIMEOUT_SECONDS: 'not-a-number'  ",
    )
    .expect("failed stage should be written");

    let stage_json = read_json(&lesson.stage_state_path(&run_id, StageName::Convert));
    let run_json = read_json(&lesson.run_metadata_path(&run_id));
    assert_eq!(stage_json["status"], json!("failed"));
    assert_eq!(stage_json["mode"], json!("executed"));
    assert_eq!(
        stage_json["error"],
        json!("invalid integer for MINERU_RESULT_TIMEOUT_SECONDS: 'not-a-number'")
    );
    assert_eq!(run_json["status"], json!("failed"));
    assert_eq!(run_json["resumed_from_run_id"], Value::Null);
}

#[test]
fn resume_scan_picks_the_newest_valid_passed_candidate_and_ignores_malformed_runs() {
    let repo = TempRepo::new();
    let lesson = lesson_paths(&repo, "L2");
    repo.write("research/pipeline/1-plain/L2/plain.txt", "plain text\n");

    seed_run_metadata(
        &lesson,
        "0000000000001-1",
        &json!({
            "schema_version": 1,
            "run_id": "0000000000001-1",
            "lesson_id": "L2",
            "command": "convert",
            "resume_requested": false,
            "resumed_from_run_id": null,
            "status": "passed",
            "created_at_epoch_ms": 1,
            "updated_at_epoch_ms": 2
        }),
        Some(&json!({
            "schema_version": 1,
            "run_id": "0000000000001-1",
            "lesson_id": "L2",
            "stage": "convert",
            "status": "passed",
            "mode": "executed",
            "output_dir": "research/pipeline/1-plain/L2",
            "reused_from_run_id": null,
            "error": null,
            "updated_at_epoch_ms": 2
        })),
    );

    write_file(&lesson.run_metadata_path("0000000000005-1"), "{not json\n");

    seed_run_metadata(
        &lesson,
        "0000000000004-1",
        &json!({
            "schema_version": 1,
            "run_id": "0000000000004-1",
            "lesson_id": "L2",
            "command": "convert",
            "resume_requested": false,
            "resumed_from_run_id": null,
            "status": "passed",
            "created_at_epoch_ms": 4,
            "updated_at_epoch_ms": 5
        }),
        Some(&json!({
            "schema_version": 1,
            "run_id": "0000000000004-1",
            "lesson_id": "L2",
            "stage": "convert",
            "status": "passed",
            "mode": "executed",
            "output_dir": "research/pipeline/1-plain/L2",
            "reused_from_run_id": null,
            "error": null,
            "updated_at_epoch_ms": 5
        })),
    );

    seed_run_metadata(
        &lesson,
        "0000000000003-1",
        &json!({
            "schema_version": 2,
            "run_id": "0000000000003-1",
            "lesson_id": "L2",
            "command": "convert",
            "resume_requested": false,
            "resumed_from_run_id": null,
            "status": "passed",
            "created_at_epoch_ms": 3,
            "updated_at_epoch_ms": 4
        }),
        Some(&json!({
            "schema_version": 1,
            "run_id": "0000000000003-1",
            "lesson_id": "L2",
            "stage": "convert",
            "status": "passed",
            "mode": "executed",
            "output_dir": "research/pipeline/1-plain/L2",
            "reused_from_run_id": null,
            "error": null,
            "updated_at_epoch_ms": 4
        })),
    );

    seed_run_metadata(
        &lesson,
        "0000000000006-1",
        &json!({
            "schema_version": 1,
            "run_id": "0000000000006-1",
            "lesson_id": "OTHER",
            "command": "convert",
            "resume_requested": false,
            "resumed_from_run_id": null,
            "status": "passed",
            "created_at_epoch_ms": 6,
            "updated_at_epoch_ms": 7
        }),
        Some(&json!({
            "schema_version": 1,
            "run_id": "0000000000006-1",
            "lesson_id": "L2",
            "stage": "convert",
            "status": "passed",
            "mode": "executed",
            "output_dir": "research/pipeline/1-plain/L2",
            "reused_from_run_id": null,
            "error": null,
            "updated_at_epoch_ms": 7
        })),
    );

    seed_run_metadata(
        &lesson,
        "0000000000007-1",
        &json!({
            "schema_version": 1,
            "run_id": "0000000000007-1",
            "lesson_id": "L2",
            "command": "convert",
            "resume_requested": false,
            "resumed_from_run_id": null,
            "status": "passed",
            "created_at_epoch_ms": 7,
            "updated_at_epoch_ms": 8
        }),
        None,
    );

    seed_run_metadata(
        &lesson,
        "0000000000008-1",
        &json!({
            "schema_version": 1,
            "run_id": "0000000000008-1",
            "lesson_id": "L2",
            "command": "convert",
            "resume_requested": false,
            "resumed_from_run_id": null,
            "status": "passed",
            "created_at_epoch_ms": 8,
            "updated_at_epoch_ms": 9
        }),
        Some(&json!({
            "schema_version": 1,
            "run_id": "0000000000008-1",
            "lesson_id": "L2",
            "stage": "textbook",
            "status": "passed",
            "mode": "executed",
            "output_dir": "research/pipeline/1-plain/L2",
            "reused_from_run_id": null,
            "error": null,
            "updated_at_epoch_ms": 9
        })),
    );

    seed_run_metadata(
        &lesson,
        "0000000000002-1",
        &json!({
            "schema_version": 1,
            "run_id": "0000000000002-1",
            "lesson_id": "L2",
            "command": "convert",
            "resume_requested": false,
            "resumed_from_run_id": null,
            "status": "passed",
            "created_at_epoch_ms": 2,
            "updated_at_epoch_ms": 3
        }),
        Some(&json!({
            "schema_version": 1,
            "run_id": "0000000000002-1",
            "lesson_id": "L2",
            "stage": "convert",
            "status": "passed",
            "mode": "executed",
            "output_dir": "research/pipeline/1-plain/OTHER",
            "reused_from_run_id": null,
            "error": null,
            "updated_at_epoch_ms": 3
        })),
    );

    write_file(&lesson.run_root_dir().join("not-a-run.txt"), "ignore me\n");

    let candidate = find_latest_resume_candidate(&lesson, StageName::Convert)
        .expect("resume scan should succeed")
        .expect("a valid candidate should be found");

    assert_eq!(candidate.run_id(), "0000000000004-1");
    assert_eq!(candidate.output_dir(), lesson.plain_output_dir());
}

#[test]
fn resume_scan_returns_none_when_the_final_output_directory_is_absent() {
    let repo = TempRepo::new();
    let lesson = lesson_paths(&repo, "L2");

    seed_run_metadata(
        &lesson,
        "0000000000001-1",
        &json!({
            "schema_version": 1,
            "run_id": "0000000000001-1",
            "lesson_id": "L2",
            "command": "convert",
            "resume_requested": false,
            "resumed_from_run_id": null,
            "status": "passed",
            "created_at_epoch_ms": 1,
            "updated_at_epoch_ms": 2
        }),
        Some(&json!({
            "schema_version": 1,
            "run_id": "0000000000001-1",
            "lesson_id": "L2",
            "stage": "convert",
            "status": "passed",
            "mode": "executed",
            "output_dir": "research/pipeline/1-plain/L2",
            "reused_from_run_id": null,
            "error": null,
            "updated_at_epoch_ms": 2
        })),
    );

    let candidate = find_latest_resume_candidate(&lesson, StageName::Convert)
        .expect("resume scan should succeed");
    assert!(
        candidate.is_none(),
        "missing output dir should block resume"
    );
}
