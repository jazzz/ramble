use ramble::{load_ramble_file, CodeGenerator, TargetC};
use std::{path::PathBuf, process::Command};
use tempfile::tempdir;

#[test]
fn run_cpp_tests() {
    let data =
        load_ramble_file(PathBuf::from("tests/ramble.yaml")).expect("cannot load ramble.yaml");
    let dest = tempdir().expect("unable to create tempdir");
    let code_gen = CodeGenerator::new(dest.path());
    let _ = code_gen
        .to_code::<TargetC>(&data)
        .expect("bad code generation");

    let test_results = Command::new("bash")
        .env("RAMBLE_GENERATED_DIRECTORY", dest.path())
        .env("RUST_BACKTRACE", "full")
        .arg("-c")
        .arg(" ./tests/cpp/run_tests.sh")
        .output()
        .expect("failed to execute process");

    assert!(
        test_results.status.success(),
        "{}",
        std::str::from_utf8(test_results.stdout.as_slice()).expect("cannot convert to utf8")
    );
}
