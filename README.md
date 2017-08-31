# slog-try
Convenience macros for logging with an optional [slog](https://github.com/slog-rs/slog) Logger.

# Usage
Add slog and slog-try as a dependency in your Cargo.toml file.

    [dependencies]
    slog = "2.0.9"
    slog-try = "0.1.0"

In `main.rs` or `lib.rs`

    #[macro_use]
    extern crate slog;
    #[macro_use]
    extern crate slog_try;

Then

    #[derive(Default)]
    struct HasOptLogger {
        logger: Option<Logger>,
    }

    let mut opt_logger: HasOptLogger = Default::default();
    try_info!(opt_logger.logger, "You won't see me output.  The logger is None."; "opt" => "None");
    try_info!(opt_logger.logger, #"imatag", "You won't see me output.  The logger is None."; "opt" => "None");

    // Setup a `Logger`
    let plain = slog_term::PlainSyncDecorator::new(::std::io::stdout());
    let logger = Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!("surname" => "Lava"));
    opt_logger.logger = Some(logger);
    try_info!(opt_logger.logger, "You will see me output!"; "opt" => "Some");
    try_info!(opt_logger.logger, #"imatag", "You will see me output!"; "opt" => "Some");