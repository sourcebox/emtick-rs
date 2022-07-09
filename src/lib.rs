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
    /// Return elapsed ticks since start.
    fn ticks() -> u64;

    /// Return the number of ticks per second.
    fn ticks_per_second() -> u64;
}
