use chrono::Local;

mod colors;

pub struct Logger;
impl Logger {
  pub fn error(message: &str) {
    Logger::log("error", colors::RED, message);
  }

  pub fn log(prefix: &str, color: &str, message: &str) {
      println!(
          "{}{}  {}{}{}:    {}",
          colors::GRAY,
          Local::now().format("%d/%m/%Y %H:%M"),
          color,
          prefix,
          colors::RESET,
          message
      );
  }
}

#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => {
    {
      $crate::Logger::error(format!($($arg)*).as_str())
    }
};
}