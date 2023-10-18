// Импорт необходимых пакетов
use heapless::Vec;
use serde::{Deserialize, Serialize};
use serde_json_core::{de, ser};

extern crate mio_httpc;
use mio_httpc::CallBuilder;

// Определение структуры данных
#[derive(Debug, Serialize, Deserialize)]
struct SensorData {
    temperature: f32,
    humidity: f32,
}

//https://lib.rs/crates/mio_httpc

fn main() {
    // Создание экземпляра структуры SensorData
    let sensor_data = SensorData {
        temperature: 25.5,
        humidity: 60.2,
    };

    // Сериализация структуры в байтовый массив
    let serializer: heapless::String<64> = ser::to_string(&sensor_data).unwrap();

    // Вывод сериализованных данных
    println!("Serialized: {:?}", serializer);

    // Десериализация байтового массива в структуру
    let deserializer = de::from_str::<SensorData>(&serializer).unwrap();

    println!("Deserialized: {:?}", deserializer);
    // Создание экземпляра Poll для мониторинга событий ввода/вывода

    let (response_meta, body) = CallBuilder::get()
        .timeout_ms(5000)
        .url("http://www.example.com")
        .unwrap()
        .exec()
        .unwrap();

    // With URL construction.
    // This way of building the URL is highly recommended as it will always result in correct
    // values by percent encoding any URL unsafe characters.
    // This calls: https://www.example.com/a/b?key1=val1
    let (response_meta, body) = CallBuilder::get()
        .timeout_ms(5000)
        .host("www.example.com")
        .path_segm("a")
        .path_segm("b")
        .query("key1", "val1")
        .exec()
        .unwrap();
}
