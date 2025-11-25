
#[cfg(feature = "logging")]
pub fn init_logger() {
    ::log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
}

#[cfg(not(feature = "logging"))]
pub fn init_logger() {
}


// ---------- Log macros  ----------
#[cfg(feature = "logging")]
#[macro_export]
macro_rules! trace_log {
    ($($arg:tt)*) => {
        ::log::trace!($($arg)*)
    };
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! trace_log {
    ($($arg:tt)*) => {}
}

#[cfg(feature = "logging")]
#[macro_export]
macro_rules! error_log {
    ($($arg:tt)*) => {
        ::log::error!($($arg)*)
    };
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! error_log {
    ($($arg:tt)*) => {
        panic!($($arg)*)
    };
}
