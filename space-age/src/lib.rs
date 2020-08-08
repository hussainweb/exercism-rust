#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    fn seconds_in_year() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / Self::seconds_in_year()
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
    fn seconds_in_year() -> f64 {
        31557600.0 * 0.2408467
    }
}
impl Planet for Venus {
    fn seconds_in_year() -> f64 {
        31557600.0 * 0.61519726
    }
}
impl Planet for Earth {
    fn seconds_in_year() -> f64 {
        31557600 as f64
    }
}
impl Planet for Mars {
    fn seconds_in_year() -> f64 {
        31557600.0 * 1.8808158
    }
}
impl Planet for Jupiter {
    fn seconds_in_year() -> f64 {
        31557600.0 * 11.862615
    }
}
impl Planet for Saturn {
    fn seconds_in_year() -> f64 {
        31557600.0 * 29.447498
    }
}
impl Planet for Uranus {
    fn seconds_in_year() -> f64 {
        31557600.0 * 84.016846
    }
}
impl Planet for Neptune {
    fn seconds_in_year() -> f64 {
        31557600.0 * 164.79132
    }
}
