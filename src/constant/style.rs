pub struct Style;

impl Style {
  pub const RESET: &'static str = "\x1b[0m";
  pub const UNDERLINE: &'static str = "\x1b[4m";
  pub const BOLD: &'static str = "\x1b[1m";
  pub const DIM: &'static str = "\x1b[2m";
  pub const ITALIC: &'static str = "\x1b[3m";
  pub const INVERSE: &'static str = "\x1b[7m";
  pub const HIDDEN: &'static str = "\x1b[8m";
  pub const STRIKETHROUGH: &'static str = "\x1b[9m";
}
