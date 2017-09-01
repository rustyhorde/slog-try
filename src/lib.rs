//! Convenience macros for logging with optional slog `Logger`s.
//!
//! # Example
//!
//! ```
//! # #[macro_use] extern crate slog;
//! # #[macro_use] extern crate slog_try;
//! #
//! # extern crate slog_term;
//! #
//! # use slog::{Drain, Logger};
//! #
//! # fn main() {
//! #[derive(Default)]
//! struct HasOptLogger {
//!     logger: Option<Logger>,
//! }
//!
//! let mut opt_logger: HasOptLogger = Default::default();
//! try_info!(opt_logger.logger, "You won't see me output.  The logger is None."; "opt" => "None");
//! try_info!(opt_logger.logger, #"imatag", "You won't see me output.  The logger is None."; "opt" => "None");
//!
//! // Setup a `Logger`
//! let plain = slog_term::PlainSyncDecorator::new(::std::io::stdout());
//! let logger = Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!("surname" => "Lava"));
//! opt_logger.logger = Some(logger);
//! try_info!(opt_logger.logger, "You will see me output!"; "opt" => "Some");
//! try_info!(opt_logger.logger, #"imatag", "You will see me output!"; "opt" => "Some");
//! # }
//! ```
#![deny(missing_docs)]
#[cfg(test)]
#[macro_use]
extern crate slog;
#[cfg(test)]
extern crate slog_term;

#[macro_export]
/// Log a trace level message if the given logger is `Some`, otherwise do nothing.
macro_rules! try_trace(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            trace!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            trace!(log, $($args)+);
        }
    };
);

#[macro_export]
/// Log a debug level message if the given logger is `Some`, otherwise do nothing.
macro_rules! try_debug(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            debug!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            debug!(log, $($args)+);
        }
    };
);

#[macro_export]
/// Log an info level message if the given logger is `Some`, otherwise do nothing.
macro_rules! try_info(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            info!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            info!(log, $($args)+);
        }
    };
);

#[macro_export]
/// Log a warn level message if the given logger is `Some`, otherwise do nothing.
macro_rules! try_warn(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            warn!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            warn!(log, $($args)+);
        }
    };
);

#[macro_export]
/// Log an error level message if the given logger is `Some`, otherwise do nothing.
macro_rules! try_error(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            error!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            error!(log, $($args)+);
        }
    };
);

#[macro_export]
/// Log a crit level message if the given logger is `Some`,, otherwise do nothing.
macro_rules! try_crit(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            crit!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            crit!(log, $($args)+);
        }
    };
);

#[cfg(test)]
mod tests {
    use slog::{Drain, Logger};
    use slog_term;

    #[test]
    // Note that theses aren't complete tests (output is not verified).  I have not figured
    // out how to capture the output in a moved buffer (maybe thread_local!).
    fn trace() {
        let plain = slog_term::PlainSyncDecorator::new(::std::io::stdout());
        let logger = Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        try_trace!(opt_logger, "SUCCESS: trace");
        try_trace!(opt_logger, #"tag", "SUCCESS: trace with tag");
        opt_logger = None;
        try_trace!(opt_logger, "FAILED: trace");
        try_trace!(opt_logger, #"tag", "FAILED: trace with tag");
    }

    #[test]
    fn debug() {
        let plain = slog_term::PlainSyncDecorator::new(::std::io::stdout());
        let logger = Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        try_debug!(opt_logger, "SUCCESS: debug");
        try_debug!(opt_logger, #"tag", "SUCCESS: debug with tag");
        opt_logger = None;
        try_debug!(opt_logger, "FAILED: debug");
        try_debug!(opt_logger, #"tag", "FAILED: debug with tag");
    }

    #[test]
    fn info() {
        let plain = slog_term::PlainSyncDecorator::new(::std::io::stdout());
        let logger = Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        try_info!(opt_logger, "SUCCESS: info");
        try_info!(opt_logger, #"tag", "SUCCESS: info with tag");
        opt_logger = None;
        try_info!(opt_logger, "FAILED: info");
        try_info!(opt_logger, #"tag", "FAILED: info with tag");
    }

    #[test]
    fn warn() {
        let plain = slog_term::PlainSyncDecorator::new(::std::io::stdout());
        let logger = Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        try_warn!(opt_logger, "SUCCESS: warn");
        try_warn!(opt_logger, #"tag", "SUCCESS: warn with tag");
        opt_logger = None;
        try_warn!(opt_logger, "FAILED: warn");
        try_warn!(opt_logger, #"tag", "FAILED: warn with tag");
    }

    #[test]
    fn error() {
        let plain = slog_term::PlainSyncDecorator::new(::std::io::stdout());
        let logger = Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        try_error!(opt_logger, "SUCCESS: error");
        try_error!(opt_logger, #"tag", "SUCCESS: error with tag");
        opt_logger = None;
        try_error!(opt_logger, "FAILED: error");
        try_error!(opt_logger, #"tag", "FAILED: error with tag");
    }

    #[test]
    fn crit() {
        let plain = slog_term::PlainSyncDecorator::new(::std::io::stdout());
        let logger = Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        try_crit!(opt_logger, "SUCCESS: crit");
        try_crit!(opt_logger, #"tag", "SUCCESS: crit with tag");
        opt_logger = None;
        try_crit!(opt_logger, "FAILED: crit");
        try_crit!(opt_logger, #"tag", "FAILED: crit with tag");
    }
}
