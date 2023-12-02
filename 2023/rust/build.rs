use build_const::ConstWriter;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Error, ErrorKind};

// Shamelessly stolen from https://github.com/ephemient/aoc2020/blob/main/rs/build.rs
fn main() -> io::Result<()> {
    let days = fs::read_dir(format!(
        "{}/..",
        env::var("CARGO_MANIFEST_DIR").map_err(|e| Error::new(ErrorKind::Other, e))?
    ))?
    .filter_map(|entry| Some(entry.ok()?.path()))
    .filter(|path| path.is_file())
    .filter_map(|path| {
        let name = path.file_name()?.to_str()?;
        if !name.starts_with("day") || !name.ends_with(".txt") {
            None
        } else {
            Some((
                name[3..name.len() - 4].parse::<u32>().ok()?,
                path.to_owned(),
            ))
        }
    });

    let mut consts = ConstWriter::for_build("aoc2023")?.finish_dependencies();
    for (day, path) in days {
        let lines = BufReader::new(File::open(path)?)
            .lines()
            .collect::<io::Result<Vec<String>>>()?;
        consts.add_value_raw(&format!("DAY{}", day), "&[&str]", &format!("&{:?}", lines));
    }
    consts.finish();
    Ok(())
}
