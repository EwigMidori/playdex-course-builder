use std::{path::PathBuf, process::Command};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate directory should live under the repository root")
        .to_path_buf()
}

#[test]
fn run_prints_the_placeholder_line_from_repo_root() {
    let output = Command::new(env!("CARGO_BIN_EXE_coursegen"))
        .current_dir(repo_root())
        .args(["run", "L2"])
        .output()
        .expect("coursegen should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "placeholder: command=run stage=- lesson=L2 raw_pdf=research/pipeline/0-raw/L2.pdf\n"
    );
    assert!(
        output.stderr.is_empty(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn run_fails_clearly_outside_the_repository_root() {
    let cwd = repo_root().join("research");
    let output = Command::new(env!("CARGO_BIN_EXE_coursegen"))
        .current_dir(&cwd)
        .args(["run", "L2"])
        .output()
        .expect("coursegen should run");

    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        output.stdout.is_empty(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("repository root required: run this command from the repository root"),
        "stderr: {stderr}"
    );
    assert!(
        stderr.contains(&cwd.display().to_string()),
        "stderr: {stderr}"
    );
}
