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
    fn load_data(path: &Path) -> Result<Self, Error> {
        let mut lines: Vec<_> = BufReader::new(File::open(path)?)
            .lines()
            .map(Result::unwrap)
            .filter(|line| !line.starts_with("//"))
            .collect();

        let mut rows = Vec::with_capacity(lines.len());
        let headings = lines
            .remove(0)
            .split(',')
            .map(|s| (*s).to_string())
            .collect();
        for mut line in lines {
            line.retain(|c| !c.is_whitespace());
            let row = line
                .split(',')
                .map(str::parse)
                .filter_map(Result::ok)
                .collect();
            rows.push(row);
        }

        Ok(Self::new(headings, rows))
    }
}
