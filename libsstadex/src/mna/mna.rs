use std::fs;
use std::path::Path;

use crate::mna::spice_parser::{spice_parser, NodeMap, SpiceError};
use crate::mna::symmna::{smna, Matrix, Vector, SmnaError};

pub struct MnaResult {
      pub report: String,
      pub a: Matrix,
      pub x: Vector,
      pub z: Vector,
      pub nodes: NodeMap,
  }

#[derive(Debug)]
pub enum MnaError {
    Io(std::io::Error),
    SpiceConversion(SpiceError),
    SymMna(SmnaError),
    MissingNode(String),
}

impl From<std::io::Error> for MnaError {
    fn from(error: std::io::Error) -> Self {
        MnaError::Io(error)
    }
}

pub fn mna(
    spice_dir: &Path,
    output_dir: &Path,
    design_name: &str,
    ) -> Result<MnaResult, MnaError> {
    println!("Running MNA...");

    let nodes = spice_parser(spice_dir, output_dir, design_name).map_err(MnaError::SpiceConversion)?;
    let input_path = output_dir.join(format!("{design_name}.cir"));
    let content = fs::read_to_string(input_path)?;

    let symmna_output = smna(&content).map_err(MnaError::SymMna)?;

    Ok(MnaResult {
        report: symmna_output.report,
        a: symmna_output.a,
        x: symmna_output.x,
        z: symmna_output.z,
        nodes,
    })
   }
