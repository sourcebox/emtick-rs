//! Instant type and associated functions.
//!
//! Based on <https://github.com/embassy-rs/embassy>

use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{conv, ClockTick, Duration};

/// Instant type representing a moment in time.
#[derive(Debug)]
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

impl<C> PartialEq for Instant<C>
where
    C: ClockTick,
{
    fn eq(&self, other: &Self) -> bool {
        self.ticks == other.ticks
    }
}

impl<C> Eq for Instant<C> where C: ClockTick {}

impl<C> PartialOrd for Instant<C>
where
    C: ClockTick,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<C> Ord for Instant<C>
where
    C: ClockTick,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.ticks.cmp(&other.ticks)
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
    pub const fn from_ticks(ticks: u64) -> Self {
        Self {
            ticks,
            clock: PhantomData,
        }
    }

    /// Create an instant from a microseconds count since boot.
    pub const fn from_micros(micros: u64) -> Self {
        Self {
            ticks: conv::micros_to_ticks(micros, C::TICKS_PER_SECOND),
            clock: PhantomData,
        }
    }

    /// Create an instant from a milliseconds count since boot.
    pub const fn from_millis(millis: u64) -> Self {
        Self {
            ticks: conv::millis_to_ticks(millis, C::TICKS_PER_SECOND),
            clock: PhantomData,
        }
    }

    /// Create an instant from a seconds count since boot.
    pub const fn from_secs(secs: u64) -> Self {
        Self {
            ticks: conv::secs_to_ticks(secs, C::TICKS_PER_SECOND),
            clock: PhantomData,
        }
    }

    /// Return ticks count since boot.
    pub const fn to_ticks(&self) -> u64 {
        self.ticks
    }

    /// Return microseconds count since boot.
    pub const fn to_micros(&self) -> u64 {
        conv::ticks_to_micros(self.ticks, C::TICKS_PER_SECOND)
    }

    /// Return milliseconds count since boot.
    pub const fn to_millis(&self) -> u64 {
        conv::ticks_to_millis(self.ticks, C::TICKS_PER_SECOND)
    }

    /// Return seconds count since boot.
    pub const fn to_secs(&self) -> u64 {
        conv::ticks_to_secs(self.ticks, C::TICKS_PER_SECOND)
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
