//! Functions for conversions between ticks and natural time units.

/// Convert microseconds to ticks.
pub const fn micros_to_ticks(micros: u64, ticks_per_second: u64) -> u64 {
    let (nom, denom) = match ticks_per_second {
        1 => (1, 1000000),
        10 => (1, 100000),
        100 => (1, 10000),
        1000 => (1, 1000),
        10000 => (1, 100),
        100000 => (1, 10),
        1000000 => (1, 1),
        10000000 => (10, 1),
        100000000 => (100, 1),
        _ => (ticks_per_second, 1000000),
    };

    ((micros as u128) * nom as u128 / denom) as u64
}

/// Convert milliseconds to ticks.
pub const fn millis_to_ticks(millis: u64, ticks_per_second: u64) -> u64 {
    let (nom, denom) = match ticks_per_second {
        1 => (1, 1000),
        10 => (1, 100),
        100 => (1, 10),
        1000 => (1, 1),
        10000 => (10, 1),
        100000 => (100, 1),
        1000000 => (1000, 1),
        10000000 => (10000, 1),
        100000000 => (100000, 1),
        _ => (ticks_per_second, 1000),
    };

    ((millis as u128) * nom as u128 / denom) as u64
}

/// Convert seconds to ticks.
pub const fn secs_to_ticks(secs: u64, ticks_per_second: u64) -> u64 {
    secs * ticks_per_second
}

/// Convert ticks to microseconds.
pub const fn ticks_to_micros(ticks: u64, ticks_per_second: u64) -> u64 {
    let (nom, denom) = match ticks_per_second {
        1 => (1000000, 1),
        10 => (100000, 1),
        100 => (10000, 1),
        1000 => (1000, 1),
        10000 => (100, 1),
        100000 => (10, 1),
        1000000 => (1, 1),
        10000000 => (1, 10),
        100000000 => (1, 100),
        _ => (1000000, ticks_per_second),
    };

    ((ticks as u128) * nom / denom as u128) as u64
}

/// Convert ticks to milliseconds.
pub const fn ticks_to_millis(ticks: u64, ticks_per_second: u64) -> u64 {
    let (nom, denom) = match ticks_per_second {
        1 => (1000, 1),
        10 => (100, 1),
        100 => (10, 1),
        1000 => (1, 1),
        10000 => (1, 10),
        100000 => (1, 100),
        1000000 => (1, 1000),
        10000000 => (1, 10000),
        100000000 => (1, 100000),
        _ => (1000, ticks_per_second),
    };

    ((ticks as u128) * nom / denom as u128) as u64
}

/// Convert ticks to seconds.
pub const fn ticks_to_secs(ticks: u64, ticks_per_second: u64) -> u64 {
    ticks / ticks_per_second
}
