#![doc = include_str!("../README.md")]
#![no_std]

pub mod conv;
pub mod delay;
pub mod duration;
pub mod instant;

pub use duration::Duration;
pub use instant::Instant;

/// Trait to be implemented by clock sources.
pub trait ClockTick {
    /// Number of ticks per second.
    const TICKS_PER_SECOND: u64;

    /// Return elapsed ticks since start.
    fn ticks() -> u64;
}
