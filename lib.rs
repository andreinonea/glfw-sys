#[allow(non_snake_case)]
mod ffi;

// Re-export bindings without warnings from C-style glfw namings.
pub use ffi::*;
