// Copyright (c) 2021 slog-try developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Convenience macros for logging with optional slog `Logger`s.
//!
//! # Example
//!
//! ```
//! # use slog::{Discard, Drain, Logger, o};
//! # use slog_try::try_info;
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
//! # let logger = Logger::root(Discard, o!());
//! opt_logger.logger = Some(logger);
//! try_info!(opt_logger.logger, "You will see me output!"; "opt" => "Some");
//! try_info!(opt_logger.logger, #"imatag", "You will see me output!"; "opt" => "Some");
//! # }
//! ```
#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    arithmetic_overflow,
    array_into_iter,
    asm_sub_register,
    bare_trait_objects,
    bindings_with_variant_name,
    box_pointers,
    cenum_impl_drop_cast,
    clashing_extern_declarations,
    coherence_leak_check,
    conflicting_repr_hints,
    confusable_idents,
    const_err,
    const_evaluatable_unchecked,
    const_item_mutation,
    dead_code,
    deprecated,
    deprecated_in_future,
    disjoint_capture_drop_reorder,
    drop_bounds,
    elided_lifetimes_in_paths,
    ellipsis_inclusive_range_patterns,
    explicit_outlives_requirements,
    exported_private_dependencies,
    forbidden_lint_groups,
    function_item_references,
    illegal_floating_point_literal_pattern,
    ill_formed_attribute_input,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    incomplete_include,
    indirect_structural_match,
    ineffective_unstable_trait_impl,
    inline_no_sanitize,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    legacy_derive_helpers,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    missing_fragment_specifier,
    mixed_script_confusables,
    mutable_borrow_reservation_conflict,
    mutable_transmutes,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_fmt_panic,
    non_shorthand_field_patterns,
    non_snake_case,
    nontrivial_structural_match,
    non_upper_case_globals,
    noop_method_call,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_range_endpoints,
    path_statements,
    patterns_in_fns_without_body,
    pointer_structural_match,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolons,
    renamed_and_removed_lints,
    safe_packed_borrows,
    semicolon_in_expressions_from_macros,
    single_use_lifetimes,
    soft_unstable,
    stable_features,
    temporary_cstring_as_ptr,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unaligned_references,
    uncommon_codepoints,
    unconditional_panic,
    unconditional_recursion,
    uninhabited_static,
    unknown_crate_types,
    unknown_lints,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    // unsafe_op_in_unsafe_fn,
    unstable_features,
    unstable_name_collisions,
    unsupported_naked_functions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_braces,
    unused_comparisons,
    unused_crate_dependencies,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_results,
    unused_unsafe,
    unused_variables,
    useless_deprecated,
    variant_size_differences,
    warnings,
    where_clauses_object_safety,
    while_true
)]

#[cfg(test)]
mod drain;

pub use slog::{crit, debug, error, info, trace, warn};

#[macro_export]
/// Log a trace level message if the given logger is `Some`, otherwise do nothing.
macro_rules! try_trace(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::trace!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::trace!(log, $($args)+);
        }
    };
);

#[macro_export]
/// Log a debug level message if the given logger is `Some`, otherwise do nothing.
macro_rules! try_debug(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::debug!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::debug!(log, $($args)+);
        }
    };
);

#[macro_export]
/// Log an info level message if the given logger is `Some`, otherwise do nothing.
macro_rules! try_info(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::info!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::info!(log, $($args)+);
        }
    };
);

#[macro_export]
/// Log a warn level message if the given logger is `Some`, otherwise do nothing.
macro_rules! try_warn(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::warn!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::warn!(log, $($args)+);
        }
    };
);

#[macro_export]
/// Log an error level message if the given logger is `Some`, otherwise do nothing.
macro_rules! try_error(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::error!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::error!(log, $($args)+);
        }
    };
);

#[macro_export]
/// Log a crit level message if the given logger is `Some`,, otherwise do nothing.
macro_rules! try_crit(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::crit!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::crit!(log, $($args)+);
        }
    };
);

#[cfg(test)]
mod tests {
    use crate::drain::Buffer;
    use slog::{o, Logger};

    #[test]
    fn trace() {
        let buffer = Buffer::default();
        let logger = Logger::root(buffer.clone(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        try_trace!(opt_logger, "SUCCESS: trace");
        try_trace!(opt_logger, #"tag", "SUCCESS: trace with tag");
        opt_logger = None;
        try_trace!(opt_logger, "FAILED: trace");
        try_trace!(opt_logger, #"tag", "FAILED: trace with tag");
        assert_eq!(
            "SUCCESS: traceSUCCESS: trace with tag",
            format!("{}", buffer)
        );
    }

    #[test]
    fn debug() {
        let buffer = Buffer::default();
        let logger = Logger::root(buffer.clone(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        try_debug!(opt_logger, "SUCCESS: debug");
        try_debug!(opt_logger, #"tag", "SUCCESS: debug with tag");
        opt_logger = None;
        try_debug!(opt_logger, "FAILED: debug");
        try_debug!(opt_logger, #"tag", "FAILED: debug with tag");
        assert_eq!(
            "SUCCESS: debugSUCCESS: debug with tag",
            format!("{}", buffer)
        );
    }

    #[test]
    fn info() {
        let buffer = Buffer::default();
        let logger = Logger::root(buffer.clone(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        try_info!(opt_logger, "SUCCESS: info");
        try_info!(opt_logger, #"tag", "SUCCESS: info with tag");
        opt_logger = None;
        try_info!(opt_logger, "FAILED: info");
        try_info!(opt_logger, #"tag", "FAILED: info with tag");
        assert_eq!("SUCCESS: infoSUCCESS: info with tag", format!("{}", buffer));
    }

    #[test]
    fn warn() {
        let buffer = Buffer::default();
        let logger = Logger::root(buffer.clone(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        try_warn!(opt_logger, "SUCCESS: warn");
        try_warn!(opt_logger, #"tag", "SUCCESS: warn with tag");
        opt_logger = None;
        try_warn!(opt_logger, "FAILED: warn");
        try_warn!(opt_logger, #"tag", "FAILED: warn with tag");
        assert_eq!("SUCCESS: warnSUCCESS: warn with tag", format!("{}", buffer));
    }

    #[test]
    fn error() {
        let buffer = Buffer::default();
        let logger = Logger::root(buffer.clone(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        try_error!(opt_logger, "SUCCESS: error");
        try_error!(opt_logger, #"tag", "SUCCESS: error with tag");
        opt_logger = None;
        try_error!(opt_logger, "FAILED: error");
        try_error!(opt_logger, #"tag", "FAILED: error with tag");
        assert_eq!(
            "SUCCESS: errorSUCCESS: error with tag",
            format!("{}", buffer)
        );
    }

    #[test]
    fn crit() {
        let buffer = Buffer::default();
        let logger = Logger::root(buffer.clone(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        try_crit!(opt_logger, "SUCCESS: crit");
        try_crit!(opt_logger, #"tag", "SUCCESS: crit with tag");
        opt_logger = None;
        try_crit!(opt_logger, "FAILED: crit");
        try_crit!(opt_logger, #"tag", "FAILED: crit with tag");
        assert_eq!("SUCCESS: critSUCCESS: crit with tag", format!("{}", buffer));
    }
}
