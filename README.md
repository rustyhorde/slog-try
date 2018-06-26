# slog-try
Convenience macros for logging with an optional [slog](https://github.com/slog-rs/slog) Logger.

## Required dependencies
Add `slog` and `slog-try` as a dependency in your Cargo.toml file.

```toml
[dependencies]
slog = "^2"
slog-try = "^0.2"
```

## Project setup
Add the new dependencies as external crates into the `main.rs` or `lib.rs` file of your project:

```rust
#[macro_use]
extern crate slog;
#[macro_use]
extern crate slog_try;
```

## Usage example

Consider `HasOptLogger`, a data strcuture with an optionally attachable logger:

```rust
#[derive(Default)]
struct HasOptLogger {
    logger: Option<Logger>,
}
```

The macros contained in `slog-try` encapsulate the required boilerplate to use this logger without verifying whether the optional field actually contains a logger or not:

```rust
let mut opt_logger= HasOptLogger { logger: None };

// Try to log even if no logger exist
try_info!(opt_logger.logger, "You won't see me output.  The logger is None."; "opt" => "None");
try_info!(opt_logger.logger, #"imatag", "You won't see me output.  The logger is None."; "opt" => "None");

// Setup a `Logger`
let plain = slog_term::PlainSyncDecorator::new(::std::io::stdout());
let logger = Logger::root(slog_term::FullFormat::new(plain)
    .build()
    .fuse(), o!("surname" => "Lava"));

opt_logger.logger = Some(logger);

// Call again with the new attached logger
try_info!(opt_logger.logger, "You will see me output!"; "opt" => "Some");
try_info!(opt_logger.logger, #"imatag", "You will see me output!"; "opt" => "Some");
```
