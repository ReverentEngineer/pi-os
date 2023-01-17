#[cfg(any(target_os = "arm", target_arch = "aarch64"))]
pub mod arm;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(target_arch = "arm")]
mod aarch32;
#[cfg(target_arch = "arm")]
pub use aarch32::*;
