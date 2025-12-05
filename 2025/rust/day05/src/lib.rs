use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::Context;

pub mod solution;

pub fn read_input() -> anyhow::Result<Vec<String>> {
    let manifest = env::var("CARGO_MANIFEST_DIR")?;
    let input_path = format!("{}/day05.txt", manifest);
    let reader = BufReader::new(File::open(input_path).context("Input file was not found")?);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>()?;

    Ok(lines)
}
