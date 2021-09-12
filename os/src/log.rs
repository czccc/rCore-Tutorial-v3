#![allow(unused)]

static mut LOG_LEVEL: Level = Level::Off;

pub fn set_log_level(lvl: Level) {
    unsafe {
        LOG_LEVEL = lvl;
    }
}

enum Color {
    Red,
    Yellow,
    Blue,
    Green,
    Gray,
}

impl Color {
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

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum Level {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Off => "OFF  ",
            Level::Error => "ERROR",
            Level::Warn => "WARN ",
            Level::Info => "INFO ",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        }
    }
    pub fn color(&self) -> (&'static str, &'static str) {
        match self {
            Level::Error => Color::Red.ansi(),
            Level::Warn => Color::Yellow.ansi(),
            Level::Info => Color::Blue.ansi(),
            Level::Debug => Color::Green.ansi(),
            Level::Trace => Color::Gray.ansi(),
            Level::Off => ("", ""),
        }
    }
    pub fn is_enabled(&self) -> bool {
        unsafe { (*self as usize) <= (LOG_LEVEL as usize) }
    }
}

#[macro_export]
macro_rules! log {
    ($lvl: expr, $fmt: literal $(, $($arg: tt)+)?) => {
        let lvl = $lvl;
        if lvl.is_enabled() {
            let (color_s, color_e) = lvl.color();
            let this_file = file!();
            let current_line = line!();
            $crate::console::print(format_args!("{}", color_s));
            $crate::console::print(format_args!("[{} {}:{}] ", lvl.as_str(), this_file, current_line));
            $crate::console::print(format_args!($fmt $(, $($arg)+)?));
            $crate::console::print(format_args!("{}", color_e));
            $crate::console::print(format_args!("\n"));
        }
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!(crate::log::Level::Error, $fmt $(, $($arg)+)?);
    }
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!(crate::log::Level::Warn, $fmt $(, $($arg)+)?);
    }
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!(crate::log::Level::Info, $fmt $(, $($arg)+)?);
    }
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!(crate::log::Level::Debug, $fmt $(, $($arg)+)?);
    }
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!(crate::log::Level::Trace, $fmt $(, $($arg)+)?);
    }
}
