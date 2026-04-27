use std::{
    collections::{BTreeMap, HashMap},
    env, fs,
    io::{self, Cursor, Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    process,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use pipeline_core::RepoPaths;
use pipeline_io::{load_config_from_env_map, ResolvedConfig};
use pipeline_mineru::{convert_lesson, ConvertError, ConvertOptions};
use serde_json::{json, Value};
use zip::{write::SimpleFileOptions, CompressionMethod, ZipWriter};

static NEXT_TEMP_REPO_ID: AtomicU64 = AtomicU64::new(0);
const NO_REQUEST_OBSERVATION_WINDOW: Duration = Duration::from_millis(250);

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
        let root = env::temp_dir().join(format!("pipeline-mineru-tests-{unique_id}"));

        fs::create_dir_all(root.join(".git")).expect("temp repo should create .git directory");
        write_file(&root.join(".ci/agent/AGENTS.md"), "test harness\n");
        write_file(&root.join("docs/progress.json"), "{}\n");
        write_file(&root.join("research/README.md"), "test harness\n");

        Self { root }
    }

    fn paths(&self) -> RepoPaths {
        RepoPaths::from_root(self.root.clone()).expect("temp repo root should resolve")
    }

    fn lesson(&self, lesson_id: &str) -> pipeline_core::LessonPaths {
        self.paths()
            .resolve_lesson(lesson_id)
            .expect("lesson should resolve")
    }

    fn write_bytes(&self, relative: impl AsRef<Path>, contents: &[u8]) {
        write_bytes(&self.root.join(relative), contents);
    }

    fn root(&self) -> &Path {
        &self.root
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[derive(Clone, Debug)]
struct CapturedRequest {
    method: String,
    path: String,
    authorization: Option<String>,
    body: Vec<u8>,
}

struct StubResponse {
    status: u16,
    content_type: &'static str,
    body: Vec<u8>,
}

impl StubResponse {
    fn json(value: Value) -> Self {
        Self {
            status: 200,
            content_type: "application/json",
            body: serde_json::to_vec(&value).expect("json response should serialize"),
        }
    }

    fn empty(status: u16) -> Self {
        Self {
            status,
            content_type: "text/plain",
            body: Vec::new(),
        }
    }

    fn bytes(content_type: &'static str, body: Vec<u8>) -> Self {
        Self {
            status: 200,
            content_type,
            body,
        }
    }
}

struct StubServer {
    base_url: String,
    requests: Arc<Mutex<Vec<CapturedRequest>>>,
    handle: thread::JoinHandle<()>,
}

impl StubServer {
    fn start<F>(build_responses: F) -> Self
    where
        F: FnOnce(&str) -> Vec<StubResponse> + Send + 'static,
    {
        let listener =
            TcpListener::bind("127.0.0.1:0").expect("stub server should bind a local port");
        listener
            .set_nonblocking(true)
            .expect("stub server should allow polling");
        let addr = listener
            .local_addr()
            .expect("stub server should report its bound address");
        let base_url = format!("http://{addr}");
        let responses = build_responses(&base_url);
        let requests = Arc::new(Mutex::new(Vec::new()));
        let requests_for_thread = Arc::clone(&requests);

        let handle = thread::spawn(move || {
            if responses.is_empty() {
                let deadline = Instant::now() + NO_REQUEST_OBSERVATION_WINDOW;
                while Instant::now() < deadline {
                    match listener.accept() {
                        Ok((mut stream, _peer)) => {
                            let request = read_request(&mut stream)
                                .expect("stub server should read unexpected request");
                            requests_for_thread
                                .lock()
                                .expect("requests mutex should not be poisoned")
                                .push(request);
                            write_response(&mut stream, &StubResponse::empty(500))
                                .expect("stub server should reply to unexpected request");
                        }
                        Err(source) if source.kind() == io::ErrorKind::WouldBlock => {
                            thread::sleep(Duration::from_millis(10));
                        }
                        Err(source) => panic!("stub server accept failed: {source}"),
                    }
                }

                return;
            }

            let deadline = Instant::now() + Duration::from_secs(5);
            let mut served = 0usize;

            while served < responses.len() && Instant::now() < deadline {
                match listener.accept() {
                    Ok((mut stream, _peer)) => {
                        let request =
                            read_request(&mut stream).expect("stub server should read request");
                        requests_for_thread
                            .lock()
                            .expect("requests mutex should not be poisoned")
                            .push(request);
                        write_response(&mut stream, &responses[served])
                            .expect("stub server should write response");
                        served += 1;
                    }
                    Err(source) if source.kind() == io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(source) => panic!("stub server accept failed: {source}"),
                }
            }

            assert_eq!(
                served,
                responses.len(),
                "stub server expected {} requests but handled {served}",
                responses.len()
            );
        });

        Self {
            base_url,
            requests,
            handle,
        }
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn finish(self) -> Vec<CapturedRequest> {
        self.handle
            .join()
            .expect("stub server should complete all expected requests");
        match Arc::try_unwrap(self.requests) {
            Ok(requests) => requests
                .into_inner()
                .expect("requests mutex should not be poisoned"),
            Err(shared) => shared
                .lock()
                .expect("requests mutex should not be poisoned")
                .clone(),
        }
    }
}

#[test]
fn convert_lesson_matches_the_stubbed_mineru_contract() {
    let repo = TempRepo::new();
    let raw_pdf_bytes = b"%PDF-1.7\nmineru test pdf\n".to_vec();
    repo.write_bytes("research/pipeline/0-raw/L2.pdf", &raw_pdf_bytes);

    let stale_staging_dir = repo
        .root()
        .join("research/pipeline/1-plain/.L2.convert-staging.stale");
    fs::create_dir_all(&stale_staging_dir).expect("stale staging directory should be created");

    let zip_bytes = zip_bytes(&[
        ZipEntry::File {
            path: "nested/00-intro.md",
            contents: b"intro text\n",
        },
        ZipEntry::File {
            path: "nested/full.md",
            contents: b"full markdown\n",
        },
        ZipEntry::File {
            path: "images/figure.txt",
            contents: b"diagram bytes",
        },
    ]);
    let full_zip_url_path = "/downloads/result.zip";
    let server = StubServer::start(move |base_url| {
        let full_zip_url = format!("{base_url}{full_zip_url_path}");
        vec![
            StubResponse::json(json!({
                "code": 0,
                "data": {
                    "batch_id": "batch-123",
                    "file_urls": [format!("{base_url}/upload/presigned")]
                }
            })),
            StubResponse::empty(200),
            StubResponse::json(json!({
                "code": 0,
                "data": { "extract_result": [] }
            })),
            StubResponse::json(json!({
                "code": 0,
                "data": {
                    "extract_result": [{
                        "data_id": "L2",
                        "file_name": "L2.pdf",
                        "state": "done",
                        "err_msg": "",
                        "full_zip_url": full_zip_url
                    }]
                }
            })),
            StubResponse::bytes("application/zip", zip_bytes),
        ]
    });
    let server_base_url = server.base_url().to_owned();
    let config = load_test_config(&repo, &server_base_url, &[]);
    let lesson = repo.lesson("L2");

    let success = convert_lesson(&lesson, &config, &ConvertOptions::new())
        .expect("stubbed convert should succeed");
    let requests = server.finish();

    assert_eq!(success.batch_id(), "batch-123");
    assert_eq!(success.output_dir(), lesson.plain_output_dir());
    assert_eq!(
        success.archive_path(),
        lesson.plain_output_dir().join("result.zip")
    );
    assert_eq!(
        success.artifacts_dir(),
        lesson.plain_output_dir().join("artifacts")
    );
    assert_eq!(
        success.plain_text_path(),
        lesson.plain_output_dir().join("plain.txt")
    );
    assert_eq!(success.plain_text_bytes(), b"full markdown\n".len() as u64);
    assert!(
        stale_staging_dir.is_dir(),
        "stale staging dir should be left alone"
    );

    assert_eq!(
        read_dir_names(lesson.plain_output_dir().as_path()),
        vec!["artifacts", "job.json", "plain.txt", "result.zip"]
    );
    assert_eq!(
        fs::read(lesson.plain_text_path()).expect("plain text should be readable"),
        b"full markdown\n"
    );
    assert_eq!(
        fs::read(lesson.plain_output_dir().join("artifacts/nested/full.md"))
            .expect("full.md should be extracted"),
        b"full markdown\n"
    );
    assert_eq!(
        fs::read(
            lesson
                .plain_output_dir()
                .join("artifacts/images/figure.txt")
        )
        .expect("figure should be extracted"),
        b"diagram bytes"
    );

    let job_raw = fs::read_to_string(lesson.plain_output_dir().join("job.json"))
        .expect("job.json should be readable");
    assert!(
        job_raw.contains("\n  \"batch_id\": \"batch-123\""),
        "job.json should stay pretty-printed with two-space indentation: {job_raw}"
    );
    assert!(
        !job_raw.contains("test-token"),
        "job.json should not leak secrets"
    );

    let job = read_json(lesson.plain_output_dir().join("job.json"));
    assert_eq!(
        job["input_file"],
        lesson.raw_pdf_path().display().to_string()
    );
    assert_eq!(
        job["output_dir"],
        lesson.plain_output_dir().display().to_string()
    );
    assert_eq!(job["batch_id"], "batch-123");
    assert_eq!(job["model_version"], "vlm");
    assert_eq!(job["language"], "ch");
    assert_eq!(job["enable_formula"], true);
    assert_eq!(job["enable_table"], true);
    assert_eq!(job["is_ocr"], true);
    assert_eq!(
        job["result"],
        json!({
            "data_id": "L2",
            "file_name": "L2.pdf",
            "state": "done",
            "err_msg": "",
            "full_zip_url": format!("{server_base_url}{full_zip_url_path}"),
        })
    );

    assert_eq!(requests.len(), 5);

    assert_eq!(requests[0].method, "POST");
    assert_eq!(requests[0].path, "/api/v4/file-urls/batch");
    assert_eq!(
        requests[0].authorization.as_deref(),
        Some("Bearer test-token")
    );
    let create_payload: Value =
        serde_json::from_slice(&requests[0].body).expect("create payload should be valid JSON");
    assert_eq!(create_payload["enable_formula"], true);
    assert_eq!(create_payload["language"], "ch");
    assert_eq!(create_payload["enable_table"], true);
    assert_eq!(create_payload["model_version"], "vlm");
    let files = create_payload["files"]
        .as_array()
        .expect("create payload should include a files array");
    assert_eq!(files.len(), 1);
    assert_eq!(files[0]["name"], "L2.pdf");
    assert_eq!(files[0]["is_ocr"], true);
    assert_eq!(files[0]["data_id"], "L2");

    assert_eq!(requests[1].method, "PUT");
    assert_eq!(requests[1].path, "/upload/presigned");
    assert_eq!(requests[1].authorization, None);
    assert_eq!(requests[1].body, raw_pdf_bytes);

    for request in &requests[2..4] {
        assert_eq!(request.method, "GET");
        assert_eq!(request.path, "/api/v4/extract-results/batch/batch-123");
        assert_eq!(request.authorization.as_deref(), Some("Bearer test-token"));
        assert!(
            request.body.is_empty(),
            "poll requests should not carry a request body"
        );
    }

    assert_eq!(requests[4].method, "GET");
    assert_eq!(requests[4].path, full_zip_url_path);
    assert_eq!(requests[4].authorization, None);
}

#[test]
fn convert_lesson_uses_the_first_markdown_when_full_md_is_missing() {
    let repo = TempRepo::new();
    repo.write_bytes(
        "research/pipeline/0-raw/L3.pdf",
        b"%PDF-1.7\nfallback pdf\n",
    );

    let zip_bytes = zip_bytes(&[
        ZipEntry::File {
            path: "z-last.md",
            contents: b"last markdown\n",
        },
        ZipEntry::File {
            path: "alpha/first.md",
            contents: b"first markdown\n",
        },
        ZipEntry::File {
            path: "notes.txt",
            contents: b"not markdown",
        },
    ]);
    let server = StubServer::start(move |base_url| {
        vec![
            StubResponse::json(json!({
                "code": 0,
                "data": {
                    "batch_id": "batch-fallback",
                    "file_urls": [format!("{base_url}/upload/presigned")]
                }
            })),
            StubResponse::empty(200),
            StubResponse::json(json!({
                "code": 0,
                "data": {
                    "extract_result": [{
                        "data_id": "L3",
                        "file_name": "L3.pdf",
                        "state": "done",
                        "err_msg": "",
                        "full_zip_url": format!("{base_url}/downloads/result.zip")
                    }]
                }
            })),
            StubResponse::bytes("application/zip", zip_bytes),
        ]
    });
    let config = load_test_config(&repo, server.base_url(), &[]);
    let lesson = repo.lesson("L3");

    let success = convert_lesson(&lesson, &config, &ConvertOptions::new())
        .expect("stubbed convert should succeed");
    let _requests = server.finish();

    assert_eq!(success.batch_id(), "batch-fallback");
    assert_eq!(
        fs::read(lesson.plain_text_path()).expect("plain text should be readable"),
        b"first markdown\n"
    );
    assert_eq!(success.plain_text_bytes(), b"first markdown\n".len() as u64);
}

#[test]
fn convert_lesson_writes_empty_plain_text_when_markdown_is_absent_and_allowed() {
    let repo = TempRepo::new();
    repo.write_bytes(
        "research/pipeline/0-raw/L6.pdf",
        b"%PDF-1.7\nno markdown allowed pdf\n",
    );

    let zip_bytes = zip_bytes(&[ZipEntry::File {
        path: "assets/figure.txt",
        contents: b"binary-ish asset",
    }]);
    let server = StubServer::start(move |base_url| {
        vec![
            StubResponse::json(json!({
                "code": 0,
                "data": {
                    "batch_id": "batch-no-markdown-ok",
                    "file_urls": [format!("{base_url}/upload/presigned")]
                }
            })),
            StubResponse::empty(200),
            StubResponse::json(json!({
                "code": 0,
                "data": {
                    "extract_result": [{
                        "data_id": "L6",
                        "file_name": "L6.pdf",
                        "state": "done",
                        "err_msg": "",
                        "full_zip_url": format!("{base_url}/downloads/result.zip")
                    }]
                }
            })),
            StubResponse::bytes("application/zip", zip_bytes),
        ]
    });
    let config = load_test_config(
        &repo,
        server.base_url(),
        &[("MINERU_FAIL_ON_EMPTY_TEXT", "false")],
    );
    let lesson = repo.lesson("L6");

    let success = convert_lesson(&lesson, &config, &ConvertOptions::new())
        .expect("convert should succeed when empty plain text is allowed");
    let _requests = server.finish();

    assert_eq!(success.batch_id(), "batch-no-markdown-ok");
    assert_eq!(success.plain_text_bytes(), 0);
    assert_eq!(
        fs::read(lesson.plain_text_path()).expect("plain text should be readable"),
        Vec::<u8>::new()
    );
    assert_eq!(
        fs::read_to_string(lesson.plain_text_path()).expect("plain text should be valid utf-8"),
        ""
    );
    assert_eq!(
        read_dir_names(lesson.plain_output_dir().as_path()),
        vec!["artifacts", "job.json", "plain.txt", "result.zip"]
    );
}

#[test]
fn convert_lesson_rejects_empty_plain_text_when_markdown_is_absent_and_forbidden() {
    let repo = TempRepo::new();
    repo.write_bytes(
        "research/pipeline/0-raw/L7.pdf",
        b"%PDF-1.7\nno markdown forbidden pdf\n",
    );

    let zip_bytes = zip_bytes(&[ZipEntry::File {
        path: "assets/figure.txt",
        contents: b"binary-ish asset",
    }]);
    let server = StubServer::start(move |base_url| {
        vec![
            StubResponse::json(json!({
                "code": 0,
                "data": {
                    "batch_id": "batch-no-markdown-fail",
                    "file_urls": [format!("{base_url}/upload/presigned")]
                }
            })),
            StubResponse::empty(200),
            StubResponse::json(json!({
                "code": 0,
                "data": {
                    "extract_result": [{
                        "data_id": "L7",
                        "file_name": "L7.pdf",
                        "state": "done",
                        "err_msg": "",
                        "full_zip_url": format!("{base_url}/downloads/result.zip")
                    }]
                }
            })),
            StubResponse::bytes("application/zip", zip_bytes),
        ]
    });
    let config = load_test_config(
        &repo,
        server.base_url(),
        &[("MINERU_FAIL_ON_EMPTY_TEXT", "true")],
    );
    let lesson = repo.lesson("L7");

    let error = convert_lesson(&lesson, &config, &ConvertOptions::new())
        .expect_err("convert should fail when empty plain text is forbidden");
    let _requests = server.finish();

    match error {
        ConvertError::EmptyPlainText { .. } => {}
        other => panic!("expected EmptyPlainText, got {other:?}"),
    }

    assert!(
        !lesson.plain_output_dir().exists(),
        "final output directory should stay absent after empty-text rejection"
    );
    assert!(
        list_staging_dirs(&lesson).is_empty(),
        "current-run staging directories should be cleaned up after failure"
    );
}

#[test]
fn convert_lesson_uses_the_files_array_when_file_urls_are_missing() {
    let repo = TempRepo::new();
    repo.write_bytes(
        "research/pipeline/0-raw/L5.pdf",
        b"%PDF-1.7\nfallback upload url pdf\n",
    );

    let zip_bytes = zip_bytes(&[ZipEntry::File {
        path: "full.md",
        contents: b"fallback upload url\n",
    }]);
    let server = StubServer::start(move |base_url| {
        vec![
            StubResponse::json(json!({
                "code": 0,
                "data": {
                    "batch_id": "batch-files-fallback",
                    "files": [format!("{base_url}/upload/from-files-array")]
                }
            })),
            StubResponse::empty(200),
            StubResponse::json(json!({
                "code": 0,
                "data": {
                    "extract_result": [{
                        "data_id": "L5",
                        "file_name": "L5.pdf",
                        "state": "done",
                        "err_msg": "",
                        "full_zip_url": format!("{base_url}/downloads/result.zip")
                    }]
                }
            })),
            StubResponse::bytes("application/zip", zip_bytes),
        ]
    });
    let config = load_test_config(&repo, server.base_url(), &[]);
    let lesson = repo.lesson("L5");

    let success = convert_lesson(&lesson, &config, &ConvertOptions::new())
        .expect("stubbed convert should succeed");
    let requests = server.finish();

    assert_eq!(success.batch_id(), "batch-files-fallback");
    assert_eq!(
        fs::read(lesson.plain_text_path()).expect("plain text should be readable"),
        b"fallback upload url\n"
    );
    assert_eq!(requests.len(), 4);
    assert_eq!(requests[1].method, "PUT");
    assert_eq!(requests[1].path, "/upload/from-files-array");
    assert_eq!(requests[1].authorization, None);
}

#[test]
fn convert_lesson_fails_before_network_when_output_dir_already_exists() {
    let repo = TempRepo::new();
    repo.write_bytes(
        "research/pipeline/0-raw/L8.pdf",
        b"%PDF-1.7\nexisting output pdf\n",
    );
    repo.write_bytes("research/pipeline/1-plain/L8/plain.txt", b"already here\n");

    let server = StubServer::start(|_| Vec::new());
    let config = load_test_config(&repo, server.base_url(), &[]);
    let lesson = repo.lesson("L8");

    let error = convert_lesson(&lesson, &config, &ConvertOptions::new())
        .expect_err("existing output directory should fail before any network work");
    let requests = server.finish();

    match error {
        ConvertError::OutputAlreadyExists { path } => {
            assert_eq!(path, lesson.plain_output_dir());
        }
        other => panic!("expected OutputAlreadyExists, got {other:?}"),
    }

    assert!(
        requests.is_empty(),
        "existing-output preflight gate should prevent all HTTP requests"
    );
    assert_eq!(
        fs::read(lesson.plain_text_path()).expect("seeded plain text should be preserved"),
        b"already here\n"
    );
}

#[test]
fn convert_lesson_rejects_normalized_escape_entries_without_leaving_output_behind() {
    let repo = TempRepo::new();
    repo.write_bytes(
        "research/pipeline/0-raw/L4.pdf",
        b"%PDF-1.7\nunsafe zip pdf\n",
    );

    let zip_bytes = zip_bytes(&[ZipEntry::File {
        path: "nested/../../escape.md",
        contents: b"escape\n",
    }]);
    let server = StubServer::start(move |base_url| {
        vec![
            StubResponse::json(json!({
                "code": 0,
                "data": {
                    "batch_id": "batch-unsafe",
                    "file_urls": [format!("{base_url}/upload/presigned")]
                }
            })),
            StubResponse::empty(200),
            StubResponse::json(json!({
                "code": 0,
                "data": {
                    "extract_result": [{
                        "data_id": "L4",
                        "file_name": "L4.pdf",
                        "state": "done",
                        "err_msg": "",
                        "full_zip_url": format!("{base_url}/downloads/result.zip")
                    }]
                }
            })),
            StubResponse::bytes("application/zip", zip_bytes),
        ]
    });
    let config = load_test_config(&repo, server.base_url(), &[]);
    let lesson = repo.lesson("L4");

    let error = convert_lesson(&lesson, &config, &ConvertOptions::new())
        .expect_err("unsafe zip entries should be rejected");
    let _requests = server.finish();

    match error {
        ConvertError::ZipEntryInvalid { entry, reason, .. } => {
            assert_eq!(entry, "nested/../../escape.md");
            assert_eq!(reason, "path traversal is not allowed");
        }
        other => panic!("expected ZipEntryInvalid, got {other:?}"),
    }

    assert!(
        !lesson.plain_output_dir().exists(),
        "final output directory should stay absent after a staged failure"
    );
    assert!(
        list_staging_dirs(&lesson).is_empty(),
        "current-run staging directories should be cleaned up after failure"
    );
}

enum ZipEntry<'a> {
    File { path: &'a str, contents: &'a [u8] },
}

fn load_test_config(repo: &TempRepo, base_url: &str, extra: &[(&str, &str)]) -> ResolvedConfig {
    let mut env = BTreeMap::from([
        ("MINERU_API_TOKEN".to_owned(), "test-token".to_owned()),
        ("MINERU_API_BASE_URL".to_owned(), base_url.to_owned()),
        ("MINERU_REQUEST_TIMEOUT_SECONDS".to_owned(), "1".to_owned()),
        ("MINERU_UPLOAD_TIMEOUT_SECONDS".to_owned(), "1".to_owned()),
        ("MINERU_RESULT_TIMEOUT_SECONDS".to_owned(), "1".to_owned()),
        ("MINERU_POLL_INTERVAL_SECONDS".to_owned(), "0".to_owned()),
        ("MINERU_DOWNLOAD_TIMEOUT_SECONDS".to_owned(), "1".to_owned()),
    ]);
    for (key, value) in extra {
        env.insert((*key).to_owned(), (*value).to_owned());
    }

    load_config_from_env_map(&repo.paths(), &env).expect("test config should load")
}

fn zip_bytes(entries: &[ZipEntry<'_>]) -> Vec<u8> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o644);

    for entry in entries {
        match entry {
            ZipEntry::File { path, contents } => {
                writer
                    .start_file(*path, options)
                    .expect("zip file entry should start");
                writer
                    .write_all(contents)
                    .expect("zip file entry should be written");
            }
        }
    }

    writer
        .finish()
        .expect("zip archive should finish")
        .into_inner()
}

fn list_staging_dirs(lesson: &pipeline_core::LessonPaths) -> Vec<String> {
    let output_dir = lesson.plain_output_dir();
    let Some(parent) = output_dir.parent() else {
        return Vec::new();
    };
    let prefix = format!(".{}.convert-staging.", lesson.lesson_id());
    let mut names = Vec::new();

    let Ok(entries) = fs::read_dir(parent) else {
        return names;
    };

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with(&prefix) {
            names.push(name.into_owned());
        }
    }

    names.sort();
    names
}

fn read_dir_names(path: &Path) -> Vec<String> {
    let mut names = fs::read_dir(path)
        .expect("directory should be readable")
        .map(|entry| {
            entry
                .expect("directory entry should be readable")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    names.sort();
    names
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(
        &fs::read(path.as_ref()).expect("json file should be readable from disk"),
    )
    .expect("json file should parse")
}

fn write_file(path: &Path, contents: &str) {
    write_bytes(path, contents.as_bytes());
}

fn write_bytes(path: &Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directories should be created");
    }
    fs::write(path, contents).expect("file should be written");
}

fn read_request(stream: &mut TcpStream) -> io::Result<CapturedRequest> {
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;

    let mut buffer = Vec::new();
    let mut chunk = [0; 4096];
    let header_end = loop {
        let read = stream.read(&mut chunk)?;
        if read == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "request ended before headers completed",
            ));
        }
        buffer.extend_from_slice(&chunk[..read]);
        if let Some(position) = find_header_end(&buffer) {
            break position;
        }
    };

    let header_bytes = &buffer[..header_end];
    let header_text = String::from_utf8_lossy(header_bytes);
    let mut lines = header_text.split("\r\n");
    let request_line = lines.next().unwrap_or_default();
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next().unwrap_or_default().to_owned();
    let path = request_parts.next().unwrap_or_default().to_owned();

    let mut headers = HashMap::new();
    for line in lines {
        if let Some((name, value)) = line.split_once(':') {
            headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_owned());
        }
    }

    let content_length = headers
        .get("content-length")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    let body_start = header_end + 4;
    while buffer.len() < body_start + content_length {
        let read = stream.read(&mut chunk)?;
        if read == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "request ended before body completed",
            ));
        }
        buffer.extend_from_slice(&chunk[..read]);
    }

    Ok(CapturedRequest {
        method,
        path,
        authorization: headers.get("authorization").cloned(),
        body: buffer[body_start..body_start + content_length].to_vec(),
    })
}

fn write_response(stream: &mut TcpStream, response: &StubResponse) -> io::Result<()> {
    let reason = match response.status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "OK",
    };
    let headers = format!(
        concat!(
            "HTTP/1.1 {} {}\r\n",
            "Content-Length: {}\r\n",
            "Content-Type: {}\r\n",
            "Connection: close\r\n",
            "\r\n"
        ),
        response.status,
        reason,
        response.body.len(),
        response.content_type
    );

    stream.write_all(headers.as_bytes())?;
    stream.write_all(&response.body)?;
    stream.flush()?;
    Ok(())
}

fn find_header_end(buffer: &[u8]) -> Option<usize> {
    buffer.windows(4).position(|window| window == b"\r\n\r\n")
}
