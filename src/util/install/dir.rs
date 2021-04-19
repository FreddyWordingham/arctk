//! Install directory information.

use crate::{err, util};
use std::{
    env::{current_dir, set_current_dir, var},
    fs::create_dir_all,
    path::{Path, PathBuf},
};

/// Get the arc installation directory path from the environment variable.
/// Environment variable must be set.
/// # Errors
/// if the environment variable `ARCTK_DIR` is not set.
#[inline]
pub fn root() -> Result<PathBuf, std::env::VarError> {
    Ok(Path::new(&var("ARCTK_DIR")?).to_path_buf())
}

/// Initialise the current working directory.
#[inline]
fn input_dir(dir: &Path) -> Result<PathBuf, std::io::Error> {
    set_current_dir(dir)?;
    current_dir()
}

/// Create an output directory.
#[inline]
fn output_dir(dir: &Path) -> Result<PathBuf, std::io::Error> {
    create_dir_all(dir)?;
    Ok(dir.to_path_buf())
}

/// Set and get the input and output directories.
/// Returned pair is (input, output).
/// # Errors
/// if the root installation directory can not be determined,
/// or if one of the input or output directories could not be created.
#[inline]
pub fn io_dirs(
    input: Option<PathBuf>,
    output: Option<PathBuf>,
) -> Result<(PathBuf, PathBuf), err::Error> {
    let exec_name = util::exec::name()?;

    let in_dir = if let Some(input) = input {
        input
    } else {
        root()?.join("input").join(&exec_name)
    };

    let out_dir = if let Some(output) = output {
        output
    } else {
        root()?.join("output").join(exec_name)
    };

    let in_dir = input_dir(&in_dir)?;
    let out_dir = output_dir(&out_dir)?;
    Ok((in_dir, out_dir))
}
