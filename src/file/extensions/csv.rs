//! Commer-Separated-Variable file handling.

use crate::{data::Table, err::Error, file::Load};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

impl<T: FromStr> Load for Table<T> {
    #[inline]
    fn load(path: &Path) -> Result<Self, Error> {
        println!("loading: {}", path.display());

        let lines: Vec<_> = BufReader::new(File::open(path)?)
            .lines()
            .map(Result::unwrap)
            .filter(|line| !line.starts_with("//"))
            .collect();

        let mut rows = Vec::with_capacity(lines.len());
        for mut line in lines {
            line.retain(|c| !c.is_whitespace());
            let row = line
                .split(',')
                .map(str::parse)
                .filter_map(Result::ok)
                .collect();
            rows.push(row);
        }

        Ok(Self::new(rows))
    }
}
