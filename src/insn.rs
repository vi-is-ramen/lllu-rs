#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod rdpid;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use rdpid::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod rdtscp;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use rdtscp::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod rdtsc;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use rdtsc::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod cpuid;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use cpuid::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod msr;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use msr::*;
