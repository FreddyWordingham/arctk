//! Datacube manipulation engine binary.

use arctk::util::banner::{section, title};

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "Wonder");

    section(term_width, "Finished");
}
