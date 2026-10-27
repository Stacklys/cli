pub struct Colors;

impl Colors {
  pub const BLACK: &'static str = "\x1b[30m";
  pub const RED: &'static str = "\x1b[31m";
  pub const GREEN: &'static str = "\x1b[32m";
  pub const YELLOW: &'static str = "\x1b[33m";
  pub const BLUE: &'static str = "\x1b[34m";
  pub const PURPLE: &'static str = "\x1b[35m";
  pub const CYAN: &'static str = "\x1b[36m";
  pub const WHITE: &'static str = "\x1b[37m";
  pub const GRAY: &'static str = "\x1b[90m";
  pub const RESET: &'static str = "\x1b[0m";

  pub const BG_BLACK: &'static str = "\x1b[40m";
  pub const BG_BLUE: &'static str = "\x1b[44m";
  pub const BG_GREEN: &'static str = "\x1b[42m";
  pub const BG_AZURE: &'static str = "\x1b[104m";
  pub const BG_CYAN: &'static str = "\x1b[46m";
  pub const BG_YELLOW: &'static str = "\x1b[43m";
  pub const BG_RED: &'static str = "\x1b[41m";
  pub const BG_PURPLE: &'static str = "\x1b[45m";
  pub const BG_GRAY: &'static str = "\x1b[100m";
  pub const BG_WHITE: &'static str = "\x1b[47m";
}
