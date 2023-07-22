#![no_std]

use defmt::debug;
use embassy_futures::select::{select, Either};
use embassy_nrf::gpio::Output;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::pubsub::{PubSubChannel, Subscriber, WaitResult};
use embassy_time::{Duration, Timer};

#[allow(unused)]
use {defmt_rtt as _, panic_probe as _};

#[derive(Debug, Clone)]
pub enum OutputM {
    T,
    M,
    H,
}

#[derive(Debug, Clone)]
pub enum InputM {
    H(usize),
}

/// inputs to the T
pub type InputBus = PubSubChannel<ThreadModeRawMutex, InputM, 4, 4, 1>;
pub type InputSub<'a> = Subscriber<'a, ThreadModeRawMutex, InputM, 4, 4, 1>;

/// outputs from the T
pub type OutputChannel = Channel<ThreadModeRawMutex, OutputM, 6>;

pub struct T<'a, P: embassy_nrf::gpio::Pin> {
    id: &'a str,
    p: Output<'a, P>,
    i: InputSub<'a>,
    o: &'a OutputChannel,
}

impl<'a, P: embassy_nrf::gpio::Pin> T<'a, P> {
    pub fn new(id: &'a str, i: InputSub<'a>, o: &'a OutputChannel, p: Output<'a, P>) -> Self {
        Self { id, p, i, o }
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
        let x = select(
            self.i.next_message(),
            Timer::after(Duration::from_secs(secs)),
        )
        .await;
        match x {
            Either::First(WaitResult::Message(m)) => match m {
                InputM::H(_) => {
                    self.o.send(OutputM::H).await;
                }
            },
            Either::Second(_timeout) => {
                self.o.send(OutputM::T).await;
            }
            _ => debug!("<><><> lag event <><><>"),
        }
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
