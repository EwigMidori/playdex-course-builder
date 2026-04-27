use std::path::PathBuf;

use pipeline_core::{LessonPathError, RepoPaths};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate directory should live under the repository root")
        .to_path_buf()
}

#[test]
fn resolves_valid_lessons_to_deterministic_paths() {
    let root = repo_root();
    let repo = RepoPaths::from_root(root.clone()).expect("repository root should resolve");

    let lesson = repo
        .resolve_lesson("L2")
        .expect("valid lesson should resolve");

    assert_eq!(lesson.lesson_id(), "L2");
    assert_eq!(
        lesson.raw_pdf_relative_string(),
        "research/pipeline/0-raw/L2.pdf"
    );
    assert_eq!(
        lesson.raw_pdf_path(),
        root.join("research/pipeline/0-raw/L2.pdf")
    );
    assert_eq!(
        lesson
            .checked_raw_pdf_path()
            .expect("sample PDF should exist"),
        root.join("research/pipeline/0-raw/L2.pdf")
    );
}

#[test]
fn rejects_invalid_lesson_ids() {
    let repo = RepoPaths::from_root(repo_root()).expect("repository root should resolve");

    for (lesson_id, reason) in [
        ("", "lesson ID must not be empty"),
        (".", "lesson ID must not be '.' or '..'"),
        ("..", "lesson ID must not be '.' or '..'"),
        ("../L2", "lesson ID must not contain path separators"),
        ("a/b", "lesson ID must not contain path separators"),
        (r"a\b", "lesson ID must not contain path separators"),
    ] {
        match repo
            .resolve_lesson(lesson_id)
            .expect_err("lesson should be rejected")
        {
            LessonPathError::InvalidLessonId {
                lesson_id: actual,
                reason: actual_reason,
            } => {
                assert_eq!(actual, lesson_id);
                assert_eq!(actual_reason, reason);
            }
            other => panic!("expected InvalidLessonId, got {other:?}"),
        }
    }
}

#[test]
fn requires_the_repository_root_for_resolution() {
    let non_root = repo_root().join("research");

    match RepoPaths::from_root(non_root.clone()).expect_err("non-root directory should fail") {
        LessonPathError::NotRepositoryRoot { cwd } => assert_eq!(cwd, non_root),
        other => panic!("expected NotRepositoryRoot, got {other:?}"),
    }
}

#[test]
fn reports_missing_raw_pdfs_with_repo_relative_paths() {
    let repo = RepoPaths::from_root(repo_root()).expect("repository root should resolve");
    let lesson = repo
        .resolve_lesson("ZZZ_TEST_MISSING_DO_NOT_CREATE")
        .expect("syntactically valid lesson should resolve");

    match lesson
        .checked_raw_pdf_path()
        .expect_err("missing PDFs should return a repository-owned error")
    {
        LessonPathError::MissingRawPdf {
            lesson_id,
            raw_pdf_relative,
        } => {
            assert_eq!(lesson_id, "ZZZ_TEST_MISSING_DO_NOT_CREATE");
            assert_eq!(
                raw_pdf_relative,
                PathBuf::from("research/pipeline/0-raw/ZZZ_TEST_MISSING_DO_NOT_CREATE.pdf")
            );
        }
        other => panic!("expected MissingRawPdf, got {other:?}"),
    }
}
