mod devices;
mod utils;

use std::time::Duration;

use crate::devices::device::Device;
use crate::devices::traffic::data::TrafficLightData;

use tokio::{task, time};

#[tokio::main]
async fn main() {
    let device = Device::create_fake(devices::device::DeviceType::TrafficLight);
    device.save_config().await;
    let data = TrafficLightData::create_fake(&device);
    let output_format = serde_json::to_string_pretty(&data).unwrap();

    let forever = task::spawn(async {
        let mut interval = time::interval(Duration::from_millis(10));

        loop {
            interval.tick().await;
        }
    });

    forever.await;
}
