use chrono::Local;

mod colors;

pub struct Logger;
impl Logger {
    #[cfg(debug_assertions)]
    pub fn debug(message: &str) {
        Logger::log("debug", colors::BLACK, message);
    }

    pub fn info(message: &str) {
        Logger::log("info", colors::GREEN, message);
    }

    pub fn warn(message: &str) {
        Logger::log("warn", colors::YELLOW, message);
    }

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
macro_rules! debug {
    ($($arg:tt)*) => {
        {
            #[cfg(debug_assertions)]
            $crate::Logger::debug(format!($($arg)*).as_str())
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        {
            $crate::Logger::info(format!($($arg)*).as_str())
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        {
            $crate::Logger::warn(format!($($arg)*).as_str())
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        {
            $crate::Logger::error(format!($($arg)*).as_str())
        }
    };
}
