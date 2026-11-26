use std::process::Command;

#[test]
fn program_prints_one_line_and_exits_successfully() {
    let result = Command::new(env!("CARGO_BIN_EXE_first-program"))
        .output()
        .expect("run the first-program executable");

    assert!(result.status.success());
    assert_eq!(result.stdout, b"Hello, Rust!\n");
    assert!(result.stderr.is_empty());
}
