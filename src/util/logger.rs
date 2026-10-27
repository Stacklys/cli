use crate::constant::color::Colors;
use crate::constant::icon::Icons;
use crate::constant::style::Style;
use crate::util::get_time::get_time;

#[allow(dead_code)]
enum LogType {
  Info,
  Warning,
  Error,
  Success,
  Setup,
}

pub struct Logger;

impl Logger {
  fn log(&self, log_type: LogType, message: &str) {
    let (bg_color, label, icon) = match log_type {
      LogType::Info => (Colors::BG_BLUE, "INFO", Icons::INFO),
      LogType::Warning => (Colors::BG_YELLOW, "WARNING", Icons::WARNING),
      LogType::Error => (Colors::BG_RED, "ERROR", Icons::ERROR),
      LogType::Success => (Colors::BG_GREEN, "SUCCESS", Icons::CHECK),
      LogType::Setup => (Colors::BG_CYAN, "SETUP", Icons::ROCKET),
    };

    println!(
      "{} {}{} {}{} {} {} {}",
      get_time(),
      bg_color,
      Colors::WHITE,
      Style::BOLD,
      icon,
      label,
      Colors::RESET,
      message
    );
  }
  pub fn info(&self, message: &str) {
    self.log(LogType::Info, message);
  }

  pub fn error(&self, message: &str) {
    self.log(LogType::Error, message);
  }

  pub fn success(&self, message: &str) {
    self.log(LogType::Success, message);
  }

  pub fn warning(&self, message: &str) {
    self.log(LogType::Warning, message);
  }

  pub fn setup(&self, message: &str) {
    self.log(LogType::Setup, message);
  }
}
