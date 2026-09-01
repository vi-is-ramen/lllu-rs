/// Generate a software interrupt.
/// This is a macro argument needs to be an immediate.
///
/// ***Unsafe* context required.**
#[macro_export]
macro_rules! int {
    ($x:expr) => {{
        core::arch::asm!("int ${vec}", vec = const ($x));
    }};
}
