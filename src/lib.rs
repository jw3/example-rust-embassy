#![no_std]


use defmt::debug;
use embassy_nrf::gpio::Output;
use embassy_time::{Duration, Timer};

pub struct T<'a, P: embassy_nrf::gpio::Pin> {
    id: &'a str,
    p: Output<'a, P>,
}

impl<'a, P: embassy_nrf::gpio::Pin> T<'a, P> {
    pub fn new(id: &'a str, p: Output<'a, P>) -> Self {
        Self { id, p }
    }

    pub fn activate(&mut self) {
        self.p.set_low();
    }

    pub fn deactivate(&mut self) {
        self.p.set_high();
    }

    pub async fn activate_for(&mut self, secs: u64) {
        debug!("activating {} for {}s", self.id, secs);
        self.activate();
        Timer::after(Duration::from_secs(secs)).await;
        self.deactivate();
        debug!("deactivated {}", self.id);
    }

    pub async fn activate_after(&mut self, secs: u64) {
        Timer::after(Duration::from_secs(secs)).await;
        self.activate();
    }

    pub async fn deactivate_after(&mut self, secs: u64) {
        Timer::after(Duration::from_secs(secs)).await;
        self.deactivate();
    }
}
