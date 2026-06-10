use crate::mna::spice_parser::spice_parser;
use std::fs;
use std::path::Path;

#[test]
fn test_spice_parser() {

    let spice_dir = Path::new("src/tests/spice");
    let output_dir = std::env::temp_dir().join("mna_output");

    fs::create_dir_all(&output_dir).unwrap();

    let node_map = spice_parser(spice_dir, &output_dir, "ota").unwrap();

    println!("{node_map}");

    let output_file = output_dir.join("ota.cir");
    let output_content = fs::read_to_string(&output_file).unwrap();

    println!();
    println!("Generated parser output");
    println!("=======================");
    println!("file: {}", output_file.display());
    println!();
    println!("{output_content}");

    let _ = std::fs::remove_dir_all(&output_dir);
}
