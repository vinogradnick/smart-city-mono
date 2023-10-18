use rand::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};

fn generate_random_version() -> String {
    let mut rng = rand::thread_rng();
    let major = rng.gen_range(0..10);
    let minor = rng.gen_range(0..10);
    let patch = rng.gen_range(0..10);

    format!("v{}.{}.{}", major, minor, patch)
}
fn generate_serial_number() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    const SERIAL_LENGTH: usize = 10;

    let mut rng = rand::thread_rng();
    let serial: String = (0..SERIAL_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    serial
}

pub enum DeviceType {
    TrafficLight,
    SmartCar,
    Notify,
}

#[derive(Serialize, Debug)]
pub struct Device {
    pub id: u32,              // Уникальный идентификатор устройства
    name: String,             // Имя устройства
    description: String,      // Описание устройства
    manufacturer: String,     // Производитель устройства
    serial_number: String,    // Серийный номер
    firmware_version: String, // Версия прошивки
}
impl Device {
    pub fn new() -> Self {
        Device {
            id: 1,
            name: "TrafficLight".to_string(),
            description: "".to_string(),
            manufacturer: "".to_string(),
            serial_number: "".to_string(),
            firmware_version: "".to_string(),
        }
    }
    pub fn create_fake(device_type: DeviceType) -> Self {
        let mut rng = rand::thread_rng();

        return match device_type {
            DeviceType::TrafficLight => Device {
                id: rng.gen(),
                name: String::from("TRL-009"),
                description: "".to_string(),
                manufacturer: "".to_string(),
                serial_number: generate_serial_number().to_string(),
                firmware_version: generate_random_version().to_string(),
            },
            _ => Device {
                id: rng.gen(),
                name: "".to_string(),
                description: "".to_string(),
                manufacturer: "".to_string(),
                serial_number: generate_serial_number().to_string(),
                firmware_version: generate_random_version().to_string(),
            },
        };
    }
    pub async fn save_config(&self) {
        let filename = format!("{}.yaml", self.name);
        let mut file = File::create(filename).await.unwrap();
        let yaml = serde_yaml::to_string(self).unwrap();
        file.write_all(yaml.as_bytes()).await.unwrap();
    }
}

impl ToString for Device {
    fn to_string(&self) -> String {
        return format!("{:?}", self);
    }
}
