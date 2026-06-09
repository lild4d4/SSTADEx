use std::fs;
use std::Path;
use std::process::Command;

use crate::macromodel::Macromodel;
use crate::spice::converter::spice2ptxt

#[derive(Debug)]
pub enum MnaError {
    Io(std::io::Error),
    SpiceConversion(String),
    SymMna(String),
    MissingNode(String),
}

impl From<std::io::Error> for MnaError {
    fn from(error: std::io::Error) -> Self {
        MnaError::Io(error)
    }
}

pub fn mna(
    xschem_rcfile: &Path,
    xscehm_dir: &Path,
    spice_dir: &Path,
    output_dir: &Path,
    macromodel: &mut Macromodel,
    ) -> Result<MnaResult, MnaError> {
    println!("Running MNA...");

    let design_name = &macromodel.name;

    let nodes = spice2ptxt(spice_dir, output_dir, design_name).map_err(MnaError::SpiceConversion)?;
    let input_path = output_dir.join(format!("{design_name}.txt"));
    let content = fs::read_to_string(input_path)?;

    let symmna_output = smna(&content).map_err(MnaError::SymMna)?;

    macromodel.a = Some(symmna_output.a.clone());
    macromodel.x = Some(symmna_output.x.clone());
    macromodel.z = Some(symmna_output.z.clone());
    macromodel.nodes = Some(nodes.clone());

    Ok(MnaResult {
        report: symmna_output.report,
        df: symmna_output.df,
        df2: symna_output.df2,
        a: symmna_output.a,
        x: symmna_output.x,
        z: symmna_output.z,
        nodes,
    })
   }
