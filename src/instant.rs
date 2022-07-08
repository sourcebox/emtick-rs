//! Instant type representing a moment in time.
//!
//! Based on <https://github.com/embassy-rs/embassy>

use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{ClockTick, Duration};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant<C>
where
    C: ClockTick,
{
    ticks: u64,
    clock: PhantomData<C>,
}

impl<C> Copy for Instant<C> where C: ClockTick {}

impl<C> Clone for Instant<C>
where
    C: ClockTick,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<C> Instant<C>
where
    C: ClockTick,
{
    /// Return instant with current time.
    pub fn now() -> Self {
        Self {
            ticks: C::ticks(),
            clock: PhantomData,
        }
    }

    /// Create an instant from a ticks count since boot.
    pub fn from_ticks(ticks: u64) -> Self {
        Self {
            ticks,
            clock: PhantomData,
        }
    }

    /// Create an instant from a microseconds count since boot.
    pub fn from_micros(micros: u64) -> Self {
        let tps = C::ticks_per_second();
        let ticks = if tps == 1000000 {
            micros
        } else {
            micros
                .checked_mul(tps)
                .expect("Overflow when converting from microseconds.")
                / 1000000
        };
        Self {
            ticks,
            clock: PhantomData,
        }
    }

    /// Create an instant from a milliseconds count since boot.
    pub fn from_millis(millis: u64) -> Self {
        let tps = C::ticks_per_second();
        let ticks = if tps == 1000 {
            millis
        } else {
            millis
                .checked_mul(tps)
                .expect("Overflow when converting from milliseconds.")
                / 1000
        };
        Self {
            ticks,
            clock: PhantomData,
        }
    }

    /// Create an instant from a seconds count since boot.
    pub fn from_secs(secs: u64) -> Self {
        let tps = C::ticks_per_second();
        let ticks = if tps == 1 {
            secs
        } else {
            secs.checked_mul(tps)
                .expect("Overflow when converting from seconds.")
        };
        Self {
            ticks,
            clock: PhantomData,
        }
    }

    /// Return ticks count since boot.
    pub fn to_ticks(&self) -> u64 {
        self.ticks
    }

    /// Return microseconds count since boot.
    pub fn to_micros(&self) -> u64 {
        let tps = C::ticks_per_second();
        if tps == 1000000 {
            self.ticks
        } else {
            self.ticks
                .checked_mul(1000000)
                .expect("Overflow when converting to microseconds.")
                / tps
        }
    }

    /// Return milliseconds count since boot.
    pub fn to_millis(&self) -> u64 {
        let tps = C::ticks_per_second();
        if tps == 1000 {
            self.ticks
        } else {
            self.ticks
                .checked_mul(1000)
                .expect("Overflow when converting to milliseconds.")
                / tps
        }
    }

    /// Return seconds count since boot.
    pub fn to_secs(&self) -> u64 {
        self.ticks / C::ticks_per_second()
    }

    /// Return duration between current instant and an earlier one.
    /// Panics on overflow.
    pub fn duration_since(&self, earlier: Self) -> Duration<C> {
        Duration {
            ticks: self.ticks.checked_sub(earlier.ticks).unwrap(),
            clock: PhantomData,
        }
    }

    /// Duration elapsed since this instant.
    pub fn elapsed(&self) -> Duration<C> {
        Self::now() - *self
    }

    /// Adds one duration to self, returning a new `Instant` or None in the event of an overflow.
    pub fn checked_add(&self, duration: Duration<C>) -> Option<Self> {
        self.ticks.checked_add(duration.ticks).map(|ticks| Self {
            ticks,
            clock: PhantomData,
        })
    }

    /// Subtracts one Duration to self, returning a new `Instant` or None in the event of an overflow.
    pub fn checked_sub(&self, duration: Duration<C>) -> Option<Self> {
        self.ticks.checked_sub(duration.ticks).map(|ticks| Self {
            ticks,
            clock: PhantomData,
        })
    }
}

impl<C> Add<Duration<C>> for Instant<C>
where
    C: ClockTick,
{
    type Output = Self;

    fn add(self, other: Duration<C>) -> Self {
        self.checked_add(other)
            .expect("Overflow when adding duration to instant.")
    }
}

impl<C> AddAssign<Duration<C>> for Instant<C>
where
    C: ClockTick,
{
    fn add_assign(&mut self, other: Duration<C>) {
        *self = *self + other;
    }
}

impl<C> Sub<Duration<C>> for Instant<C>
where
    C: ClockTick,
{
    type Output = Self;

    fn sub(self, other: Duration<C>) -> Self {
        self.checked_sub(other)
            .expect("Overflow when subtracting duration from instant.")
    }
}

impl<C> SubAssign<Duration<C>> for Instant<C>
where
    C: ClockTick,
{
    fn sub_assign(&mut self, other: Duration<C>) {
        *self = *self - other;
    }
}

impl<C> Sub<Instant<C>> for Instant<C>
where
    C: ClockTick,
{
    type Output = Duration<C>;

    fn sub(self, other: Instant<C>) -> Duration<C> {
        self.duration_since(other)
    }
}

impl<C> core::fmt::Display for Instant<C>
where
    C: ClockTick,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{} ticks", self.ticks)
    }
}
