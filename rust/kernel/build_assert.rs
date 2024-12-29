// SPDX-License-Identifier: GPL-2.0

//! Build-time assert.

/// Fails the build if the code path calling `build_error!` can possibly be executed.
///
/// If the macro is executed in const context, `build_error!` will panic.
/// If the compiler or optimizer cannot guarantee that `build_error!` can never
/// be called, a build error will be triggered.
///
/// This macro ensures that certain checks are enforced during the compilation process,
/// and not at runtime, improving program safety by catching issues as early as possible.
///
/// # Examples
///
/// ```
/// # use kernel::build_error;
/// #[inline]
/// fn foo(a: usize) -> usize {
///     a.checked_add(1).unwrap_or_else(|| build_error!("overflow"))
/// }
///
/// assert_eq!(foo(usize::MAX - 1), usize::MAX); // OK.
/// // foo(usize::MAX); // Fails to compile.
/// ```
#[macro_export]
macro_rules! build_error {
    () => {{
        $crate::build_error("")
    }};
    ($msg:expr) => {{
        // Ensure that the error message is more informative
        $crate::build_error(concat!("[Build Error] ", $msg, " at line: ", stringify!($line)))
    }};
}

/// Asserts that a boolean expression is `true` at compile time.
///
/// If the condition is evaluated to `false` in const context, `build_assert!`
/// will panic. If the compiler or optimizer cannot guarantee the condition will
/// be evaluated to `true`, a build error will be triggered.
///
/// This macro helps you assert conditions that are crucial to the correctness of your program
/// before the code even runs, minimizing the potential for runtime errors.
///
/// [`static_assert!`] should be preferred to `build_assert!` whenever possible, as it provides 
/// better performance and clarity in simpler cases.
///
/// # Examples
///
/// These examples show that different types of assertions will trigger errors
/// at different stages of compilation. It is preferred to err as early as
/// possible, so [`static_assert!`] should be used whenever possible.
///
/// ```ignore
/// fn foo() {
///     static_assert!(1 > 1); // Compile-time error
///     build_assert!(1 > 1); // Build-time error
///     assert!(1 > 1); // Run-time error
/// }
/// ```
///
/// When the condition refers to generic parameters or parameters of an inline function,
/// [`static_assert!`] cannot be used. Use `build_assert!` in this scenario.
///
/// ```
/// fn foo<const N: usize>() {
///     // `static_assert!(N > 1);` is not allowed
///     build_assert!(N > 1); // Build-time check
///     assert!(N > 1); // Run-time check
/// }
///
/// #[inline]
/// fn bar(n: usize) {
///     // `static_assert!(n > 1);` is not allowed
///     build_assert!(n > 1); // Build-time check
///     assert!(n > 1); // Run-time check
/// }
/// ```
///
/// [`static_assert!`]: crate::static_assert!
#[macro_export]
macro_rules! build_assert {
    // Simplified condition, only condition provided
    ($cond:expr $(,)?) => {{
        if !$cond {
            $crate::build_error(concat!("assertion failed: ", stringify!($cond), " at line: ", stringify!($line)));
        }
    }};
    
    // Condition with a custom message
    ($cond:expr, $msg:expr) => {{
        if !$cond {
            $crate::build_error(concat!($msg, " at line: ", stringify!($line)));
        }
    }};
    
    // Condition with more context information (useful for debugging)
    ($cond:expr, $msg:expr, $context:expr) => {{
        if !$cond {
            $crate::build_error(concat!($msg, " at line: ", stringify!($line), " - Context: ", $context));
        }
    }};
    }
