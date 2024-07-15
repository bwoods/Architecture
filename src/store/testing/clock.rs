use std::time::Duration;

/// When testing, it is important to have control over the (simulated) passage of time.
///
/// …
pub trait TestClock {
    fn advance(&self, duration: Duration);
}
