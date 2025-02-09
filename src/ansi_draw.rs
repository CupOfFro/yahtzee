pub struct Point {
    pub r: usize, // row
    pub c: usize, // column
}

pub fn set_bg_color(color: &str) {
    print!("{}", color);
}

pub const ANSI_CLEAR_SCREEN: &str = "\x1b[2J";
pub const ANSI_HOME: &str = "\x1b[H";
pub const ANSI_RESET_TEXT: &str = "\x1b[0m";

// Colors
// pub const ANSI_BLACK_TEXT: &str = "\x1b[30m";
// pub const ANSI_RED_TEXT: &str = "\x1b[31m";
// pub const ANSI_GREEN_TEXT: &str = "\x1b[32m";
// pub const ANSI_YELLOW_TEXT: &str = "\x1b[33m";
// pub const ANSI_BLUE_TEXT: &str = "\x1b[34m";
// pub const ANSI_MAGENTA_TEXT: &str = "\x1b[35m";
// pub const ANSI_CYAN_TEXT: &str = "\x1b[36m";
// pub const ANSI_WHITE_TEXT: &str = "\x1b[37m";

// Backgrounds
// pub const ANSI_BLACK_BG: &str = "\x1b[40m";
// pub const ANSI_RED_BG: &str = "\x1b[41m";
// pub const ANSI_GREEN_BG: &str = "\x1b[42m";
// pub const ANSI_YELLOW_BG: &str = "\x1b[43m";
// pub const ANSI_BLUE_BG: &str = "\x1b[44m";
// pub const ANSI_MAGENTA_BG: &str = "\x1b[45m";
// pub const ANSI_CYAN_BG: &str = "\x1b[46m";
pub const ANSI_WHITE_BG: &str = "\x1b[47m";
