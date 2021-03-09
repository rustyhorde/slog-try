# slog-try
Convenience macros for logging with an optional [slog](https://github.com/slog-rs/slog) Logger.

**NOTE** - See the bottom of this README for another method of using a Logger that doesn't require `Option<Logger>`

## Current Release
[![docs.rs](https://docs.rs/slog-try/badge.svg)](https://docs.rs/slog-try)
[![Crates.io](https://img.shields.io/crates/v/slog-try.svg)](https://crates.io/crates/slog-try)
[![Crates.io](https://img.shields.io/crates/l/slog-try.svg)](https://crates.io/crates/slog-try)
[![Crates.io](https://img.shields.io/crates/d/slog-try.svg)](https://crates.io/crates/slog-try)
[![codecov](https://codecov.io/gh/rustyhorde/slog-try/branch/master/graph/badge.svg?token=cBXro7o2UN)](https://codecov.io/gh/rustyhorde/slog-try)
![CI](https://github.com/rustyhorde/slog-try/actions/workflows/main.yml/badge.svg)

## Required dependencies
Add `slog-try` as a dependency in your Cargo.toml file.

```toml
[dependencies]
slog-try = "1"
```

## Project setup
Add use statements for the macros you wish to use:

```rust
use slog_try::try_info;
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
let mut opt_logger = HasOptLogger { logger: None };

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

## Using a Discard Logger
You can use slogs [`Discard`](https://docs.rs/slog/2.5.1/slog/struct.Discard.html) drain in lieu of the `Option` wrapped logger. Like the `try_*` macros, in case of a none present logger, the discarding logger is going to drop all incoming messages.

**How to initialize a discarding logger**
```rust
use slog::{Logger, Discard, o};

fn main() {
    let logger = Logger::root(Discard, o!());
    info!(logger, "nothing happens");
}
```
