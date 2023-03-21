//! Terminal I/O stream operations.

#[cfg(not(any(target_os = "wasi", target_os = "nto")))]
mod cf;
#[cfg(not(target_os = "wasi"))]
mod constants;
#[cfg(not(target_os = "wasi"))]
mod ioctl;
#[cfg(not(target_os = "wasi"))]
mod tc;
#[cfg(not(windows))]
mod tty;

#[cfg(not(any(target_os = "wasi", target_os = "nto")))]
pub use cf::*;
#[cfg(not(target_os = "wasi"))]
pub use constants::*;
#[cfg(not(target_os = "wasi"))]
pub use ioctl::*;
#[cfg(not(target_os = "wasi"))]
pub use tc::*;
#[cfg(not(windows))]
pub use tty::*;
