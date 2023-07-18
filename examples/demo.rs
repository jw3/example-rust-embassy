#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{debug, info};
use embassy_executor;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::{Duration, Timer};

use {defmt_rtt as _, panic_probe as _};
use example_rust_embassy::T;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    info!("Starting!");

    let mut t0 = T::new(Output::new(p.P1_01, Level::High, OutputDrive::Standard));
    let mut t1 = T::new(Output::new(p.P1_02, Level::High, OutputDrive::Standard));
    let mut t2 = T::new(Output::new(p.P1_08, Level::High, OutputDrive::Standard));
    let mut t3 = T::new(Output::new(p.P1_10, Level::High, OutputDrive::Standard));

    loop {
        debug!("A");
        t0.activate_for(3).await;
        t1.activate_for(3).await;
        t2.activate_for(3).await;
        t3.activate_for(3).await;

        //

        debug!("B");
        futures::join!(t0.activate_for(3), t3.activate_for(3));

        //

        debug!("C");
        futures::join!(t1.activate_for(3), t2.activate_for(3));

        //

        debug!("D");
        let x = 3;
        for i in 0..x {
            t0.activate_for(x - i).await;
            t1.activate_for(x - i).await;
            t2.activate_for(x - i).await;
            t3.activate_for(x - i).await;
        }

        //

        debug!("E");
        futures::join!(t0.activate_for(3), t1.activate_for(1), t2.activate_for(1), t3.activate_for(3));
        futures::join!(t0.activate_for(4), t1.activate_for(3), t2.activate_for(2), t3.activate_for(1));
        futures::join!(t0.activate_for(1), t1.activate_for(2), t2.activate_for(3), t3.activate_for(4));

        //

        debug!("F");
        futures::join!(t0.activate_after(1), t1.activate_after(2), t2.activate_after(3), t3.activate_after(4));
        futures::join!(t0.deactivate_after(1), t1.deactivate_after(2), t2.deactivate_after(3), t3.deactivate_after(4));

        //

        debug!("Completed");
        Timer::after(Duration::from_secs(5)).await;
    }
}
