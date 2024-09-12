use std::time::Duration;
use rand::Rng;

pub trait Backoff {
    fn next_delay(&self, retries: u32) -> Duration;
}

pub struct ConstantBackoff {
    pub base: Duration,
    pub jitter: Duration,
}

impl Backoff for ConstantBackoff {
    fn next_delay(&self, _retries: u32) -> Duration {
        let jitter = rand::thread_rng().gen_range(Duration::ZERO..self.jitter);
        self.base + jitter
    }
}

pub struct ExponentialBackoff {
    pub base: Duration,
    pub multiplier: f64,
    pub jitter: Duration,
}

impl Backoff for ExponentialBackoff {
    fn next_delay(&self, retries: u32) -> Duration {
        let delay = self.base.as_secs_f64() * self.multiplier.powi(retries as i32);
        Duration::from_secs_f64(delay) + self.jitter
    }
}
