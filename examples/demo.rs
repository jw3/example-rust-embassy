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

    let mut t0 = T::new("d2", Output::new(p.P1_01, Level::High, OutputDrive::Standard));
    let mut t1 = T::new("d3", Output::new(p.P1_02, Level::High, OutputDrive::Standard));
    let mut t2 = T::new("d4", Output::new(p.P1_08, Level::High, OutputDrive::Standard));
    let mut t3 = T::new("d5", Output::new(p.P1_10, Level::High, OutputDrive::Standard));

    loop {
        debug!("A");
        t0.activate_for(3).await;
        t1.activate_for(3).await;
        t2.activate_for(3).await;
        t3.activate_for(3).await;

        //

        debug!("B");
        futures::join!(async { t0.activate_for(3).await }, async { t3.activate_for(3).await });

        //

        debug!("C");
        futures::join!(async { t1.activate_for(3).await }, async { t2.activate_for(3).await });

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
        futures::join!(async { t0.activate_for(3).await }, async { t1.activate_for(1).await }, async { t2.activate_for(1).await }, async { t3.activate_for(3).await });
        futures::join!(async { t0.activate_for(4).await }, async { t1.activate_for(3).await }, async { t2.activate_for(2).await }, async { t3.activate_for(1).await });
        futures::join!(async { t0.activate_for(1).await }, async { t1.activate_for(2).await }, async { t2.activate_for(3).await }, async { t3.activate_for(4).await });

        //

        debug!("F");
        futures::join!(async { t0.activate_after(1).await }, async { t1.activate_after(2).await }, async { t2.activate_after(3).await }, async { t3.activate_after(4).await });
        futures::join!(async { t0.deactivate_after(1).await }, async { t1.deactivate_after(2).await }, async { t2.deactivate_after(3).await }, async { t3.deactivate_after(4).await });

        //

        debug!("Completed");
        Timer::after(Duration::from_secs(5)).await;
    }
}
