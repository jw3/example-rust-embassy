#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{debug, info};
use embassy_executor;
use embassy_executor::Spawner;
use embassy_nrf::bind_interrupts;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::saadc::{ChannelConfig, Config, Resistor, Saadc};
use embassy_sync::channel::Channel;
use embassy_sync::pubsub::PubSubBehavior;
use embassy_time::{Duration, Timer};

use example_rust_embassy::{OutputChannel, InputBus, OutputM, T, InputM};

static INPUTS: InputBus =  InputBus::new();
static OUTPUTS: OutputChannel = Channel::new();

bind_interrupts!(struct Irqs {
    SAADC => embassy_nrf::saadc::InterruptHandler;
});

#[embassy_executor::task]
async fn s() {
    loop {
        match OUTPUTS.recv().await {
            OutputM::T => info!("timeout"),
            OutputM::M => info!("------ m -------"),
            OutputM::H => info!("++++++ h +++++++"),
        }
    }
}

fn h(v: &i16) -> bool {
    *v > 4000
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut p = embassy_nrf::init(Default::default());
    info!("Starting!");

    let mut t0 = T::new("d2", INPUTS.subscriber().unwrap(), &OUTPUTS, Output::new(p.P1_01, Level::High, OutputDrive::Standard));
    let mut t1 = T::new("d3", INPUTS.subscriber().unwrap(), &OUTPUTS, Output::new(p.P1_02, Level::High, OutputDrive::Standard));
    let mut t2 = T::new("d4", INPUTS.subscriber().unwrap(), &OUTPUTS, Output::new(p.P1_08, Level::High, OutputDrive::Standard));
    let mut t3 = T::new("d5", INPUTS.subscriber().unwrap(), &OUTPUTS, Output::new(p.P1_10, Level::High, OutputDrive::Standard));

    let config = Config::default();
    let mut pins = [
        ChannelConfig::single_ended(&mut p.P0_28),
        ChannelConfig::single_ended(&mut p.P0_29),
        ChannelConfig::single_ended(&mut p.P0_30),
        ChannelConfig::single_ended(&mut p.P0_31)];
    for p in &mut pins {
        p.resistor = Resistor::PULLDOWN;
    }

    let mut saadc = Saadc::new(p.SAADC, Irqs, config, pins);

    spawner.spawn(s()).unwrap();

    let chk = async {
        loop {
            let mut buf = [0; 4];
            saadc.sample(&mut buf).await;
            for (i, b) in buf.iter().enumerate() {
                if h(b) {
                    debug!("h {} -- {}", i, b);
                    INPUTS.publish_immediate(InputM::H(i));
                }
            }
        }
    };

    let script = async {
        loop {
            t0.activate_for(3).await;
            t1.activate_for(3).await;
            t2.activate_for(3).await;
            t3.activate_for(3).await;

            //

            futures::join!(async { t0.activate_for(3).await }, async { t3.activate_for(3).await });

            //

            futures::join!(async { t1.activate_for(3).await }, async { t2.activate_for(3).await });

            //

            let x = 3;
            for i in 0..x {
                t0.activate_for(x - i).await;
                t1.activate_for(x - i).await;
                t2.activate_for(x - i).await;
                t3.activate_for(x - i).await;
            }

            //

            futures::join!(async { t0.activate_for(3).await }, async { t1.activate_for(1).await }, async { t2.activate_for(1).await }, async { t3.activate_for(3).await });
            futures::join!(async { t0.activate_for(4).await }, async { t1.activate_for(3).await }, async { t2.activate_for(2).await }, async { t3.activate_for(1).await });
            futures::join!(async { t0.activate_for(1).await }, async { t1.activate_for(2).await }, async { t2.activate_for(3).await }, async { t3.activate_for(4).await });

            //

            futures::join!(async { t0.activate_after(1).await }, async { t1.activate_after(2).await }, async { t2.activate_after(3).await }, async { t3.activate_after(4).await });
            futures::join!(async { t0.deactivate_after(1).await }, async { t1.deactivate_after(2).await }, async { t2.deactivate_after(3).await }, async { t3.deactivate_after(4).await });

            //

            debug!("Restarting after 5 seconds");
            Timer::after(Duration::from_secs(5)).await;
        }
    };

    futures::join!(chk, script);
}
