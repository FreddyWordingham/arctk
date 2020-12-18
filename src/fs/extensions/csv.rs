//! Commer-Separated-Variable file handling.

use crate::{data::Table, err::Error, fs::File};
use std::{
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

impl<T: FromStr> File for Table<T> {
    #[inline]
    fn load(path: &Path) -> Result<Self, Error> {
        let mut lines: Vec<_> = BufReader::new(std::fs::File::open(path)?)
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
