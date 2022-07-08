//! Delay functions (blocking)

use crate::{ClockTick, Instant};

/// Sleep for a number of ticks
pub fn delay_ticks<C: ClockTick>(ticks: u64) {
    let start = Instant::<C>::now().to_ticks();
    while Instant::<C>::now().to_ticks() < start + ticks {}
}

/// Sleep for a number of microseconds
pub fn delay_us<C: ClockTick>(micros: u64) {
    let start = Instant::<C>::now().to_micros();
    while Instant::<C>::now().to_micros() < start + micros {}
}

/// Sleep for a number of milliseconds
pub fn delay_ms<C: ClockTick>(millis: u64) {
    let start = Instant::<C>::now().to_millis();
    while Instant::<C>::now().to_millis() < start + millis {}
}

/// Sleep for a number of seconds
pub fn delay_secs<C: ClockTick>(secs: u64) {
    let start = Instant::<C>::now().to_secs();
    while Instant::<C>::now().to_secs() < start + secs {}
}
