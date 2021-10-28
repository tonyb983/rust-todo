use std::{
    borrow::Cow,
    cell::{Ref, RefCell, RefMut},
    time::{Duration, Instant},
};

/// An **owning** timed value.
/// `TValue` - The type of value stored by this [TimedValue]
pub struct TimedValue<TValue> {
    value: RefCell<TValue>,
    expiration: Instant,
}

impl<TValue> TimedValue<TValue> {
    /// Creates a new [TimedValue] with the given value and expiriation time.
    /// `value` - The value to store
    /// `expiriation` - The time at which this value will expire
    pub fn new(value: TValue, expiriation: Instant) -> Self {
        Self {
            value: RefCell::new(value),
            expiration: expiriation,
        }
    }

    /// Returns the value stored in this [TimedValue].
    pub fn value(&self) -> Option<Ref<TValue>> {
        if self.expired() {
            None
        } else {
            self.value.try_borrow().ok()
        }
    }

    /// Returns the value stored in this [TimedValue] as mutable.
    pub fn value_mut(&mut self) -> Option<RefMut<TValue>> {
        if self.expired() {
            None
        } else {
            self.value.try_borrow_mut().ok()
        }
    }

    pub fn set_value(&mut self, value: TValue) {
        self.value = RefCell::new(value);
    }

    pub fn expiration(&self) -> Instant {
        self.expiration
    }

    pub fn set_expiration(&mut self, expiration: Instant) {
        self.expiration = expiration;
    }

    pub fn extend_expiration(&mut self, duration: Duration) {
        self.expiration = self.expiration + duration;
    }

    /// Returns `true` if this [TimedValue] has not yet expired.
    pub fn valid(&self) -> bool {
        !self.expired()
    }

    /// Returns `true` if this [TimedValue] has expired.
    pub fn expired(&self) -> bool {
        self.expiration <= Instant::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Tester {
        number: i32,
    }

    #[test]
    fn is_value_mut_actually_needed() {
        let mut tester = Tester { number: 0 };
        {
            let mut tester_ref: &mut Tester = &mut tester;
            tester_ref.number = 1;
        }
        assert_eq!(tester.number, 1);
        let mut timed_ref: TimedValue<&mut Tester> =
            TimedValue::new(&mut tester, Instant::now() + Duration::from_secs(10000));
        timed_ref.value_mut().unwrap().number = 2;
        assert_eq!(tester.number, 2);
    }
}
