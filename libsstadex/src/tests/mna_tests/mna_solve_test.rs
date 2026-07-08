use crate::mna::mna::mna_solve;
use crate::mna::symmna::smna;
use std::process::Command;
use std::process::Stdio;

#[test]
fn test_mna_solve() {
    if !sympy_is_available() {
        eprintln!("skipping mna_solve test because python3 cannot import sympy");
        return;
    }

    let mna_map = smna("R1 1 0 1\nI1 1 0 1\n").unwrap();
    let solution = mna_solve(&mna_map.a, &mna_map.x, &mna_map.z).unwrap();

    println!("{:?}", solution.solutions);

    assert_eq!(
        solution.solutions.get("v1").map(String::as_str),
        Some("-1")
    );
}

fn sympy_is_available() -> bool {
    Command::new("python3")
        .arg("-c")
        .arg("import sympy")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}
