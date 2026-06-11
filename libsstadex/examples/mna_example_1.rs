use std::fs;
use std::path::Path;

use libsstadex::mna::mna::{mna, mna_solve};
use libsstadex::mna::pretty::{pretty_solutions, pretty_system};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spice_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/spice");
    let output_dir = std::env::temp_dir().join("mna_example_output");

    fs::create_dir_all(&output_dir)?;

    let mna_sys = mna(&spice_dir, &output_dir, "ota").unwrap();

    println!();
    println!("MNA report");
    println!("==========");
    println!("{}", mna_sys.report);

    println!();
    println!("{}", pretty_system(&mna_sys.a, &mna_sys.x, &mna_sys.z));

    let mna_sol = mna_solve(&mna_sys.a, &mna_sys.x, &mna_sys.z).unwrap();

    println!("{}", pretty_solutions(&mna_sol.solutions));

    let _ = fs::remove_dir_all(&output_dir);

    Ok(())
}
