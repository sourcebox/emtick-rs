//! Duration type representing a span of time.
//!
//! Based on <https://github.com/embassy-rs/embassy>

use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::{conv, ClockTick};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration<C>
where
    C: ClockTick,
{
    pub(crate) ticks: u64,
    pub(crate) clock: PhantomData<C>,
}

impl<C> Copy for Duration<C> where C: ClockTick {}

impl<C> Clone for Duration<C>
where
    C: ClockTick,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<C> Duration<C>
where
    C: ClockTick,
{
    /// Create a duration from a ticks count.
    pub fn from_ticks(ticks: u64) -> Self {
        Self {
            ticks,
            clock: PhantomData,
        }
    }

    /// Create a duration from a microseconds count.
    pub fn from_micros(micros: u64) -> Self {
        Self {
            ticks: conv::micros_to_ticks(micros, C::ticks_per_second()),
            clock: PhantomData,
        }
    }

    /// Create a duration from a milliseconds count.
    pub fn from_millis(millis: u64) -> Self {
        Self {
            ticks: conv::millis_to_ticks(millis, C::ticks_per_second()),
            clock: PhantomData,
        }
    }

    /// Create a duration from a seconds count.
    pub fn from_secs(secs: u64) -> Self {
        Self {
            ticks: conv::secs_to_ticks(secs, C::ticks_per_second()),
            clock: PhantomData,
        }
    }

    /// Return tick count.
    pub fn to_ticks(&self) -> u64 {
        self.ticks
    }

    /// Return duration as microseconds.
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

    /// Return duration as milliseconds.
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

    /// Return duration as seconds.
    pub fn to_secs(&self) -> u64 {
        self.ticks / C::ticks_per_second()
    }

    /// Add durations, return a new duration or None in case of overflow.
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.ticks.checked_add(rhs.ticks).map(|ticks| Self {
            ticks,
            clock: PhantomData,
        })
    }

    /// Subtract durations, return a new duration or None in case of overflow.
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.ticks.checked_sub(rhs.ticks).map(|ticks| Self {
            ticks,
            clock: PhantomData,
        })
    }

    /// Multiply durations by a scalar, return a new duration or None in case of overflow.
    pub fn checked_mul(self, rhs: u32) -> Option<Self> {
        self.ticks.checked_mul(rhs as _).map(|ticks| Self {
            ticks,
            clock: PhantomData,
        })
    }

    /// Divide duration by a scalar, return a new duration or None in case of overflow.
    pub fn checked_div(self, rhs: u32) -> Option<Self> {
        self.ticks.checked_div(rhs as _).map(|ticks| Self {
            ticks,
            clock: PhantomData,
        })
    }
}

impl<C> Add for Duration<C>
where
    C: ClockTick,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.checked_add(rhs)
            .expect("Overflow when adding durations.")
    }
}

impl<C> AddAssign for Duration<C>
where
    C: ClockTick,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<C> Sub for Duration<C>
where
    C: ClockTick,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.checked_sub(rhs)
            .expect("Overflow when subtracting durations.")
    }
}

impl<C> SubAssign for Duration<C>
where
    C: ClockTick,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<C> Mul<u32> for Duration<C>
where
    C: ClockTick,
{
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        self.checked_mul(rhs)
            .expect("Overflow when multiplying duration by scalar.")
    }
}

impl<C> Mul<Duration<C>> for u32
where
    C: ClockTick,
{
    type Output = Duration<C>;

    fn mul(self, rhs: Duration<C>) -> Duration<C> {
        rhs * self
    }
}

impl<C> MulAssign<u32> for Duration<C>
where
    C: ClockTick,
{
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl<C> Div<u32> for Duration<C>
where
    C: ClockTick,
{
    type Output = Self;

    fn div(self, rhs: u32) -> Self {
        self.checked_div(rhs)
            .expect("Divide by zero error when dividing duration by scalar.")
    }
}

impl<C> DivAssign<u32> for Duration<C>
where
    C: ClockTick,
{
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

impl<C> core::fmt::Display for Duration<C>
where
    C: ClockTick,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{} ticks", self.ticks)
    }
}
