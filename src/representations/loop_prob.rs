use super::affine_expr::AffineExpression;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoopProb {
    pub loop_nest: Vec<Loop>,
    pub body: Vec<Instruction>,
    pub conditionals: Vec<Conditionals>,
}

#[derive(Debug, Deserialize)]
pub struct Loop {
    pub iter_name: String,
    pub bounds: (i32, i32),
    pub step: i32,
}

#[derive(Debug, Deserialize)]
pub enum Instruction {
    DataAccess(DataAccess),
    Compute(Compute),
}

#[derive(Debug, Deserialize)]
pub struct DataAccess {
    pub name: String,
    pub duration: Option<i32>,
    pub access: AffineExpression,
}

#[derive(Debug, Deserialize)]
pub struct Compute {
    pub name: String,
    pub duration: i32,
}

#[derive(Debug, Deserialize)]
pub struct Conditionals {
    /// the loops that are executed conditionally
    pub skipped_loops: Vec<String>,
    pub cond_comp_loops: Vec<String>,
    pub prob: f64,
}

#[cfg(test)]
mod tests {
    use serde_yaml;
    use std::{fs, path::Path};

    use super::*;

    #[test]
    fn test_deserialize() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        // Construct the file path to loopprob.yaml
        let file_path = Path::new(manifest_dir).join("example/prob.yaml");
        println!("File path: {:?}", file_path);
        let yaml_str = fs::read_to_string(file_path).expect("Failed to read YAML file");
        let loop_prob: LoopProb =
            serde_yaml::from_str(&yaml_str).expect("Failed to deserialize YAML");
        println!("{:#?}", loop_prob);
    }
}
