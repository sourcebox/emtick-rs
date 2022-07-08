//! Functions for conversions between ticks and natural time units.

/// Convert microseconds to ticks.
pub fn micros_to_ticks(micros: u64, ticks_per_second: u64) -> u64 {
    if ticks_per_second == 1000000 {
        micros
    } else {
        micros
            .checked_mul(ticks_per_second)
            .expect("Overflow when converting from microseconds to ticks.")
            / 1000000
    }
}

/// Convert milliseconds to ticks.
pub fn millis_to_ticks(millis: u64, ticks_per_second: u64) -> u64 {
    if ticks_per_second == 1000 {
        millis
    } else {
        millis
            .checked_mul(ticks_per_second)
            .expect("Overflow when converting from milliseconds to ticks.")
            / 1000
    }
}

/// Convert seconds to ticks.
pub fn secs_to_ticks(secs: u64, ticks_per_second: u64) -> u64 {
    if ticks_per_second == 1 {
        secs
    } else {
        secs.checked_mul(ticks_per_second)
            .expect("Overflow when converting from seconds to ticks.")
    }
}

/// Convert ticks to microseconds.
pub fn ticks_to_micros(ticks: u64, ticks_per_second: u64) -> u64 {
    if ticks_per_second == 1000000 {
        ticks
    } else {
        ticks
            .checked_mul(1000000)
            .expect("Overflow when converting from ticks to microseconds.")
            / ticks_per_second
    }
}

/// Convert ticks to milliseconds.
pub fn ticks_to_millis(ticks: u64, ticks_per_second: u64) -> u64 {
    if ticks_per_second == 1000 {
        ticks
    } else {
        ticks
            .checked_mul(1000)
            .expect("Overflow when converting from ticks to milliseconds.")
            / ticks_per_second
    }
}

/// Convert ticks to seconds.
pub fn ticks_to_secs(ticks: u64, ticks_per_second: u64) -> u64 {
    if ticks_per_second == 1 {
        ticks
    } else {
        ticks / ticks_per_second
    }
}
