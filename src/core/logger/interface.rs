use std::fmt;
use std::sync::Arc;

pub trait Logger: Send + Sync {
    fn info(&self, args: fmt::Arguments);
    fn debug(&self, args: fmt::Arguments);
    fn error(&self, args: fmt::Arguments);
    fn child(&self, name: &str) -> Arc<dyn Logger>;
    fn noop(&self) -> Arc<dyn Logger>;
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $($arg:tt)*) => {
        $logger.info(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $($arg:tt)*) => {
        $logger.debug(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $($arg:tt)*) => {
        $logger.error(format_args!($($arg)*));
    };
}
