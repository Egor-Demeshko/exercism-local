// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    pub s: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
    fn calculate_in_earth(d: &u64) -> f64 {
        *d as f64 / 60.0 / 60.0 / 24.0 / 365.25
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        Earth::calculate_in_earth(&d.s) / 0.2408467
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        Earth::calculate_in_earth(&d.s) / 0.61519726
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        Earth::calculate_in_earth(&d.s) / 1.0
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        Earth::calculate_in_earth(&d.s) / 1.8808158
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        Earth::calculate_in_earth(&d.s) / 11.862615
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        Earth::calculate_in_earth(&d.s) / 29.447498
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        Earth::calculate_in_earth(&d.s) / 84.016846
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        Earth::calculate_in_earth(&d.s) / 164.79132
    }
}

fn assert_in_delta(expected: f64, actual: f64) {
    let diff: f64 = (expected - actual).abs();
    let delta: f64 = 0.01;
    if diff > delta {
        panic!("Your result of {actual} should be within {delta} of the expected result {expected}")
    }
}
