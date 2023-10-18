use crate::devices::device::Device;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct TrafficData {
    vehicles: VehiclesData,
    pedestrians: PedestriansData,
}

#[derive(Debug, Deserialize, Serialize)]
struct VehiclesData {
    count: u32,
    params: VehicleDataSpeed,
}
#[derive(Debug, Deserialize, Serialize)]
struct VehicleDataSpeed {
    speed_average: u32,
    speed_max: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct PedestriansData {
    crosswalk_1: u32,
    crosswalk_2: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct WeatherData {
    temperature: i32,
    humidity: u32,
    conditions: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AirQualityData {
    co2_level: u32,
    nox_level: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct SecurityData {
    speed_limit_violation: bool,
    obstacle_detected: bool,
    accident: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct EnergyConsumption {
    current_usage: f32,
    voltage: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrafficLightData {
    traffic_light_id: u32,
    current_state: String,
    remaining_time: u32,
    traffic_data: TrafficData,
    weather_data: WeatherData,
    air_quality_data: AirQualityData,
    security_data: SecurityData,
    energy_consumption: EnergyConsumption,
}

enum TrafficLightStatus {
    Green,
    Yellow,
    Red,
    FlashingYellow,
    Off,
}

fn get_random_element<T>(array: &Vec<T>) -> Option<&T> {
    if array.is_empty() {
        return None;
    }

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..array.len());

    Some(&array[index])
}
fn traffic_light_states() -> Vec<String> {
    return vec![
        "green".to_string(),
        "yellow".to_string(),
        "red".to_string(),
        "flashing_yellow".to_string(),
        "off".to_string(),
        "failed".to_string(),
    ];
}

impl TrafficLightData {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    pub fn create_fake(device: &Device) -> TrafficLightData {
        let states = traffic_light_states();
        let state = get_random_element(&states).unwrap();
        return TrafficLightData {
            traffic_light_id: device.id,
            current_state: state.clone(),
            remaining_time: 0,
            traffic_data: TrafficData {
                vehicles: VehiclesData {
                    count: 10,
                    params: VehicleDataSpeed {
                        speed_average: 10,
                        speed_max: 120,
                    },
                },
                pedestrians: PedestriansData {
                    crosswalk_1: 1,
                    crosswalk_2: 2,
                },
            },
            weather_data: WeatherData {
                temperature: 10,
                humidity: 20,
                conditions: "".to_string(),
            },

            security_data: SecurityData {
                speed_limit_violation: false,
                obstacle_detected: false,
                accident: false,
            },
            energy_consumption: EnergyConsumption {
                current_usage: 130.0,
                voltage: 220.0,
            },
            air_quality_data: AirQualityData {
                co2_level: 120,
                nox_level: 30,
            },
        };
    }
}
