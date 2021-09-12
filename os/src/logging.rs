#![allow(unused)]

use core::{mem, str::FromStr};

use lazy_static::lazy_static;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

lazy_static! {
    static ref LOGGER: SimpleLogger = SimpleLogger::new();
}

// static LOGGER: SimpleLogger = SimpleLogger::default();

#[derive(Debug, Clone)]
pub struct SimpleLogger {
    level: LevelFilter,
    color: bool,
}

impl SimpleLogger {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_log_level(&mut self, level: &str) -> &mut Self {
        self.level = log::LevelFilter::from_str(level).unwrap();
        self
    }
    pub fn set_color(&mut self, color: bool) -> &mut Self {
        self.color = color;
        self
    }
    pub fn init(&mut self) -> Result<(), SetLoggerError> {
        unsafe {
            let tmp = &(*LOGGER) as *const SimpleLogger;
            let tmp = tmp as *mut SimpleLogger;
            (*tmp).clone_from(self);
        }
        // mem::swap(&mut (LOGGER), &mut self);
        log::set_logger(&(*LOGGER))?;
        log::set_max_level(self.level);
        Ok(())
    }
}

impl Default for SimpleLogger {
    fn default() -> Self {
        Self {
            level: LevelFilter::Off,
            color: false,
        }
    }
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let (color_s, color_e) = if self.color {
                Color::from_level(&record.level())
            } else {
                ("", "")
            };
            println!(
                "[{}{:<5} {}:{}] {}{}",
                color_s,
                record.level(),
                record.file().unwrap_or("*"),
                record.line().unwrap_or(0),
                record.args(),
                color_e
            );
        }
    }

    fn flush(&self) {}
}

enum Color {
    Red,
    Yellow,
    Blue,
    Green,
    Gray,
}

impl Color {
    fn from_level(level: &Level) -> (&'static str, &'static str) {
        match level {
            Level::Error => Color::Red.ansi(),
            Level::Warn => Color::Yellow.ansi(),
            Level::Info => Color::Blue.ansi(),
            Level::Debug => Color::Green.ansi(),
            Level::Trace => Color::Gray.ansi(),
        }
    }
    fn ansi(&self) -> (&'static str, &'static str) {
        match self {
            Color::Red => ("\x1b[31m", "\x1b[0m"),
            Color::Yellow => ("\x1b[93m", "\x1b[0m"),
            Color::Blue => ("\x1b[34m", "\x1b[0m"),
            Color::Green => ("\x1b[32m", "\x1b[0m"),
            Color::Gray => ("\x1b[90m", "\x1b[0m"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        logging::SimpleLogger::new()
            .set_log_level("Info")
            .set_color(true)
            .init()
            .unwrap();
    }
}
