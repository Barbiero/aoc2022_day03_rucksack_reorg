use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

/** from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html */
pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(BufReader::new(file).lines())
}
