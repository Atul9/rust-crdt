use num_bigint::BigUint;
use serde::{Serialize, Deserialize};

use crate::traits::{CvRDT, CmRDT, Causal};
use crate::vclock::{VClock, Actor, Dot};

/// `GCounter` is a grow-only witnessed counter.
///
/// # Examples
///
/// ```
/// use crdts::{GCounter, CmRDT};
///
/// let mut a = GCounter::new();
/// let mut b = GCounter::new();
///
/// a.apply(a.inc("A"));
/// b.apply(b.inc("B"));
///
/// assert_eq!(a.read(), b.read());
///
/// a.apply(a.inc("A"));
/// assert!(a.read() > b.read());
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct GCounter<A: Actor> {
    inner: VClock<A>,
}

impl<A: Actor> Default for GCounter<A> {
    fn default() -> Self {
        GCounter::new()
    }
}

impl<A: Actor> CmRDT for GCounter<A> {
    type Op = Dot<A>;

    fn apply(&mut self, op: Self::Op) {
        self.inner.apply(op)
    }
}

impl<A: Actor> CvRDT for GCounter<A> {
    fn merge(&mut self, other: Self) {
        self.inner.merge(other.inner);
    }
}

impl<A: Actor> Causal<A> for GCounter<A> {
    fn forget(&mut self, clock: &VClock<A>) {
        self.inner.forget(&clock);
    }
}

impl<A: Actor> GCounter<A> {
    /// Produce a new `GCounter`.
    pub fn new() -> Self {
        GCounter {
            inner: VClock::new(),
        }
    }

    /// Generate Op to increment the counter.
    pub fn inc(&self, actor: A) -> Dot<A> {
        self.inner.inc(actor)
    }

    /// Return the current sum of this counter.
    pub fn read(&self) -> BigUint {
        self.inner.iter()
            .map(|dot| dot.counter)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let mut a = GCounter::new();
        let mut b = GCounter::new();
        a.apply(a.inc("A"));
        b.apply(b.inc("B"));

        assert_eq!(a.read(), b.read());
        assert_ne!(a, b);

        a.apply(a.inc("A"));

        assert_eq!(a.read(), b.read() + BigUint::from(1u8));
    }
}
