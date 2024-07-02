
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    OFF = 0,
    FATAL,
    ERROR,
    WARN,
    INFO,
    DEBUG,
    TRACE,
}

/* impl std::cmp::PartialOrd for LogLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (*self as usize).partial_cmp(&(*other as usize))
    }
}
impl std::cmp::Ord for LogLevel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
} */

static mut GLOBAL_LOGGER: Log = Log { logger: Logger { log_level: LogLevel::INFO } };

#[derive(Clone, Copy)]
pub struct Logger {
    log_level: LogLevel,
}

impl Logger {
    pub fn fatal() {
        println!()
    }
}

#[derive(Clone, Copy)]
pub struct Log {
    logger: Logger
}

/* impl Log {
    fn fatal<T>(s: T) where T: std::fmt::Display {
        // FatalLog!("{}", s)
    }
} */

impl Singleton for Log {
    fn instance() -> &'static Self {
        unsafe { &GLOBAL_LOGGER }
    }
    fn instance_mut() -> &'static mut Self {
        unsafe { &mut GLOBAL_LOGGER }
    }
}

macro_rules! MacroLog {
    ($level: ident, $($arg:tt)*) => {
        if crate::log_level() >= $level {
            println!($($arg)*);
        }
    };
}

macro_rules! FatalLog { ($($arg:tt)*) => { MacroLog!(FATAL, $($arg)*) }; }
macro_rules! ErrorLog { ($($arg:tt)*) => { MacroLog!(ERROR, $($arg)*) }; }
macro_rules! WarnLog { ($($arg:tt)*) => { MacroLog!(WARN, $($arg)*) }; }
macro_rules! InfoLog { ($($arg:tt)*) => { MacroLog!(INFO, $($arg)*) }; }
macro_rules! DebugLog { ($($arg:tt)*) => { MacroLog!(DEBUG, $($arg)*) }; }
macro_rules! TraceLog { ($($arg:tt)*) => { MacroLog!(TRACE, $($arg)*) }; }

pub(crate) use FatalLog;
pub(crate) use ErrorLog;
pub(crate) use WarnLog;
pub(crate) use InfoLog;
pub(crate) use DebugLog;
pub(crate) use TraceLog;

use super::property::Singleton;
