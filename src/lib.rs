#![no_std]

#[cfg(not(manually_drop))]
mod manually_drop;

#[cfg(not(native_uninit))]
mod maybe_uninit;

#[cfg(not(native_uninit))]
pub use maybe_uninit::MaybeUninit;

#[cfg(native_uninit)]
pub use core::mem::MaybeUninit;
