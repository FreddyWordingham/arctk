//! Banner printing functions.

use colored::Colorize;

/// Index of the section.
static mut SECTION: i32 = 0;

/// Print a colourful title bar to the terminal.
#[inline]
pub fn title(term_width: usize, title: &str) {
    let title = title.to_uppercase();

    let (left_bar, right_bar) = if term_width < ((title.len() * 2) + 11) {
        (4, 4)
    } else {
        let left_bar = (term_width - (title.len() * 2) - 3) / 2;
        (left_bar, term_width - (title.len() * 2) - 3 - left_bar)
    };

    print!("{} ", "\u{2588}".repeat(left_bar));

    for (pos, ch) in title.chars().enumerate() {
        match pos % 6 {
            0 => print!(" {}", format!("{}", ch).bright_red().bold()),
            1 => print!(" {}", format!("{}", ch).bright_yellow().bold()),
            2 => print!(" {}", format!("{}", ch).bright_green().bold()),
            3 => print!(" {}", format!("{}", ch).bright_cyan().bold()),
            4 => print!(" {}", format!("{}", ch).bright_blue().bold()),
            5 => print!(" {}", format!("{}", ch).bright_magenta().bold()),
            _ => unreachable!(),
        }
    }

    println!("  {}", "\u{2588}".repeat(right_bar));
}

/// Print a section bar to the terminal.
#[inline]
pub fn section(term_width: usize, title: &str) {
    let title = title.to_uppercase();
    unsafe {
        SECTION += 1;
    }

    print!("====");
    print!(" {}", colour(&title).bold());

    let mut cur_len = 5 + title.len();
    if cur_len >= term_width {
        println!();
        return;
    }

    print!(" ");
    cur_len += 1;
    while cur_len < term_width {
        print!("=");
        cur_len += 1;
    }

    println!();
}

/// Print a sub-section message to the terminal.
#[inline]
pub fn sub_section(term_width: usize, title: &str) {
    println!(
        "---- {} {}",
        colour(title).bold(),
        "-".repeat(term_width - 6 - title.len())
    );
}

/// Print a sub-sub-section message to the terminal.
#[inline]
pub fn sub_sub_section(title: &str) {
    println!("---- {}", colour(title));
}

/// Colour a given message with the appropriate section colour.
#[inline]
#[must_use]
fn colour(string: &str) -> String {
    match unsafe { SECTION } % 6 {
        0 => format!("{}", string.bright_magenta()),
        1 => format!("{}", string.bright_red()),
        2 => format!("{}", string.bright_yellow()),
        3 => format!("{}", string.bright_green()),
        4 => format!("{}", string.bright_cyan()),
        5 => format!("{}", string.bright_blue()),
        _ => unreachable!(),
    }
}
