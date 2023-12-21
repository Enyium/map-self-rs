#![warn(clippy::pedantic)]

/// Functions to transform a type itself instead of something it contains with the usual closure approach.
pub trait MapSelf {
    fn map_self<O, T>(self, op: O) -> T
    where
        O: FnOnce(Self) -> T,
        Self: Sized,
    {
        #![must_use]
        //! Example:
        //!
        //! ```ignore
        //! let (hour, meridiem) = if use_12_hour_clock {
        //!     self.hour_meridiem() // Yields 12-hour clock time.
        //!         .map_self(|(hour, meridiem)| (hour, Some(meridiem)))
        //! } else {
        //!     (self.hour, None)
        //! };
        //! ```

        op(self)
    }

    fn map_self_or_keep<O>(self, op: O) -> Self
    where
        O: FnOnce(&Self) -> Option<Self>,
        Self: Sized,
    {
        #![must_use]
        //! Example:
        //!
        //! ```ignore
        //! // Initialize time to check for double-click.
        //! let mut last_click_time = Instant::now()
        //!     .map_self_or_keep(|now| now.checked_sub(Duration::from_secs(60)));
        //! ```

        op(&self).unwrap_or(self)
    }
}

impl<T> MapSelf for T {}

#[cfg(test)]
mod tests {
    use crate::MapSelf;

    #[test]
    fn map_self() {
        assert_eq!(
            (1, 2).map_self(|(first, second)| (first + 1, Some(second - 1))),
            (2, Some(1))
        );
        assert_eq!((1, 2).map_self(|(first, second)| first + second), 3);
    }

    #[test]
    fn map_self_or_keep() {
        let only_positive: usize = 10;
        assert_eq!(
            only_positive.map_self_or_keep(|val| val.checked_sub(100)),
            10
        );
        assert_eq!(only_positive.map_self_or_keep(|val| val.checked_sub(5)), 5);
    }
}
