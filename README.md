# emtick

This `no_std` Rust crate provides `Instant` and `Duration` types for time-related calculations with a focus on embedded systems.

`emtick` is greatly inspired by the [embassy](https://github.com/embassy-rs/embassy) time module but uses generic clock sources to eleminate dependencies. It also allows the use of multiple optimized clock sources for performance reasons.

## Usage Example

Provide a clock source by implementing the `ClockTick` trait on a type.

```rust
use emtick::ClockTick;

pub const SYSTICK_FREQ: u32 = 1000;

pub struct Clock1k;

impl ClockTick for Clock1k {
    fn ticks() -> u64 {
        systick::ticks()
    }

    fn ticks_per_second() -> u64 {
        SYSTICK_FREQ as u64
    }
}
```

Create `Instant` and `Duration` types based on the clock source and use them for calculations.

```rust
use emtick::{delay::delay_until, Duration, Instant};

/// Create an instant with the current time.
let now = Instant::<Clock1k>::now();

/// Get the number of elapsed milliseconds since boot.
let millis = now.to_millis();

/// Create a duration of 5 seconds.
let duration = Duration::<Clock1k>::from_secs(5);

/// Calculate an instant in the future based on some duration.
let later = now + duration;

/// Sleep until an instant has elapsed.
delay_until(later);
```

## Performance Considerations

`emtick` uses `u64` ticks as base unit internally. While calculating time intervals via durations only use addition and subtraction instructions that are very fast on any platform, converting between ticks and natural time units (seconds, milliseconds, microseconds) have a performance impact. For that reason, it is recommened to minimize the amount of conversions.

In some cases conversion performance is optimized:

- When using a clock with 1kHz frequency (1ms pulse), all conversions between ticks and milliseconds are transparent.
- When using a clock with 1MHz frequency (1µs pulse), all conversions between ticks and microseconds are transparent.

Hint: if maximum performance is required for both milliseconds and microseconds conversions, use two separate optimized clock sources and create instants and durations with them appropriately.

## Preventing Overflows

There is a risk of integer overflow in intermediate calculations in some conversion functions. Whenever a conversion between ticks and natural time units is performed, this operation may panic on overflow. The most likely cause of an overflow is when convering an instant to an absolute microseconds value in a long-running application with a high tick count.

In order to prevent overflows, it is recommended to use internal calculations e.g. by adding/subtracting instants and durations rather than converting these values to natural units first and using these values to do so. This is also recommended for performance reasons.

## Alternatives

- [embedded-time](https://github.com/FluenTech/embedded-time/)
- [fugit](https://github.com/korken89/fugit)

## License

Published under the MIT license. Any contribution to this project must be provided under the same license conditions.

Author: Oliver Rockstedt <info@sourcebox.de>
