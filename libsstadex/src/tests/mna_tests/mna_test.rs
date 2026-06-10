use crate::mna::mna::mna;
use std::fs;
use std::path::Path;

#[test]
fn test_mna() {
   let spice_dir = Path::new("src/tests/spice"); 
   let output_dir = std::env::temp_dir().join("mna_output");
   fs::create_dir_all(&output_dir).unwrap();

   let mna_results = mna(&spice_dir, &output_dir, "ota").unwrap();
   println!("{}", mna_results.report);
} 
