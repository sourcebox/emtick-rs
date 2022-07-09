//! Delay functions (blocking).

use crate::{ClockTick, Duration, Instant};

/// Sleep for a duration.
pub fn delay<C: ClockTick>(duration: Duration<C>) {
    delay_until::<C>(Instant::<C>::now() + duration);
}

/// Sleep until an instant.
pub fn delay_until<C: ClockTick>(instant: Instant<C>) {
    while Instant::<C>::now() < instant {}
}

/// Sleep for a number of ticks.
pub fn delay_ticks<C: ClockTick>(ticks: u64) {
    delay::<C>(Duration::from_ticks(ticks));
}

/// Sleep for a number of microseconds.
pub fn delay_us<C: ClockTick>(micros: u64) {
    delay::<C>(Duration::from_micros(micros));
}

/// Sleep for a number of milliseconds.
pub fn delay_ms<C: ClockTick>(millis: u64) {
    delay::<C>(Duration::from_millis(millis));
}

/// Sleep for a number of seconds.
pub fn delay_secs<C: ClockTick>(secs: u64) {
    delay::<C>(Duration::from_secs(secs));
}
