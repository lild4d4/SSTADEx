use crate::mna::symmna::smna;
use std::fs;
use std::path::Path;

#[test]
fn test_symmna() {
    let netlist = Path::new("src/tests/netlist/ota.cir");
    let output_dir = std::env::temp_dir().join("symmna_output");

    let content = fs::read_to_string(netlist).unwrap();

    fs::create_dir_all(&output_dir).unwrap();

    let mna_map = smna(&content).unwrap();

    println!("{}", mna_map.report);
}
