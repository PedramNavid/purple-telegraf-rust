use std::env;
use std::thread;
use std::time;
use std::u8;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use telegraf::*;

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SensorData {
    #[serde(rename = "SensorId")]
    pub sensor_id: String,
    #[serde(rename = "DateTime")]
    pub date_time: String,
    #[serde(rename = "Geo")]
    pub geo: String,
    #[serde(rename = "Mem")]
    pub mem: i64,
    pub memfrag: i64,
    pub memfb: i64,
    pub memcs: i64,
    #[serde(rename = "Id")]
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    #[serde(rename = "Adc")]
    pub adc: f64,
    pub loggingrate: i64,
    pub place: String,
    pub version: String,
    pub uptime: i64,
    pub rssi: i64,
    pub period: i64,
    pub httpsuccess: i64,
    pub httpsends: i64,
    pub hardwareversion: String,
    pub hardwarediscovered: String,
    #[serde(rename = "current_temp_f")]
    pub current_temp_f: i32,
    #[serde(rename = "current_humidity")]
    pub current_humidity: f32,
    #[serde(rename = "current_dewpoint_f")]
    pub current_dewpoint_f: i32,
    pub pressure: f32,
    #[serde(rename = "p25aqic_b", default)]
    pub p25aqic_b: String,
    #[serde(rename = "pm2.5_aqi_b", default)]
    pub pm2_5_aqi_b: f32,
    #[serde(rename = "pm1_0_cf_1_b", default)]
    pub pm1_0_cf_1_b: f32,
    #[serde(rename = "p_0_3_um_b", default)]
    pub p_0_3_um_b: f32,
    #[serde(rename = "pm2_5_cf_1_b", default)]
    pub pm2_5_cf_1_b: f32,
    #[serde(rename = "p_0_5_um_b", default)]
    pub p_0_5_um_b: f32,
    #[serde(rename = "pm10_0_cf_1_b", default)]
    pub pm10_0_cf_1_b: f32,
    #[serde(rename = "p_1_0_um_b", default)]
    pub p_1_0_um_b: f32,
    #[serde(rename = "pm1_0_atm_b", default)]
    pub pm1_0_atm_b: f32,
    #[serde(rename = "p_2_5_um_b", default)]
    pub p_2_5_um_b: f32,
    #[serde(rename = "pm2_5_atm_b", default)]
    pub pm2_5_atm_b: f32,
    #[serde(rename = "p_5_0_um_b", default)]
    pub p_5_0_um_b: f32,
    #[serde(rename = "pm10_0_atm_b", default)]
    pub pm10_0_atm_b: f32,
    #[serde(rename = "p_10_0_um_b", default)]
    pub p_10_0_um_b: f32,
    pub p25aqic: String,
    #[serde(rename = "pm2.5_aqi")]
    pub pm2_5_aqi: f32,
    #[serde(rename = "pm1_0_cf_1")]
    pub pm1_0_cf_1: f32,
    #[serde(rename = "p_0_3_um")]
    pub p_0_3_um: f32,
    #[serde(rename = "pm2_5_cf_1")]
    pub pm2_5_cf_1: f32,
    #[serde(rename = "p_0_5_um")]
    pub p_0_5_um: f32,
    #[serde(rename = "pm10_0_cf_1")]
    pub pm10_0_cf_1: f32,
    #[serde(rename = "p_1_0_um")]
    pub p_1_0_um: f32,
    #[serde(rename = "pm1_0_atm")]
    pub pm1_0_atm: f32,
    #[serde(rename = "p_2_5_um")]
    pub p_2_5_um: f32,
    #[serde(rename = "pm2_5_atm")]
    pub pm2_5_atm: f32,
    #[serde(rename = "p_5_0_um")]
    pub p_5_0_um: f32,
    #[serde(rename = "pm10_0_atm")]
    pub pm10_0_atm: f32,
    #[serde(rename = "p_10_0_um")]
    pub p_10_0_um: f32,
    #[serde(rename = "pa_latency")]
    pub pa_latency: i32,
    pub response: i32,
    #[serde(rename = "response_date")]
    pub response_date: i32,
    pub latency: i32,
    #[serde(rename = "key1_response")]
    pub key1_response: i32,
    #[serde(rename = "key1_response_date")]
    pub key1_response_date: i32,
    #[serde(rename = "key1_count")]
    pub key1_count: i32,
    #[serde(rename = "ts_latency")]
    pub ts_latency: i32,
    #[serde(rename = "key2_response", default)]
    pub key2_response: i32,
    #[serde(rename = "key2_response_date", default)]
    pub key2_response_date: i32,
    #[serde(rename = "key2_count", default)]
    pub key2_count: i32,
    #[serde(rename = "ts_s_latency")]
    pub ts_s_latency: i32,
    #[serde(rename = "key1_response_b")]
    pub key1_response_b: i32,
    #[serde(rename = "key1_response_date_b")]
    pub key1_response_date_b: i32,
    #[serde(rename = "key1_count_b")]
    pub key1_count_b: i32,
    #[serde(rename = "ts_latency_b")]
    pub ts_latency_b: i32,
    #[serde(rename = "key2_response_b", default)]
    pub key2_response_b: i32,
    #[serde(rename = "key2_response_date_b", default)]
    pub key2_response_date_b: i32,
    #[serde(rename = "key2_count_b", default)]
    pub key2_count_b: i32,
    #[serde(rename = "ts_s_latency_b", default)]
    pub ts_s_latency_b: i32,
    pub wlstate: String,
    #[serde(rename = "status_0")]
    pub status_0: i32,
    #[serde(rename = "status_1")]
    pub status_1: i32,
    #[serde(rename = "status_2")]
    pub status_2: i32,
    #[serde(rename = "status_3")]
    pub status_3: i32,
    #[serde(rename = "status_4")]
    pub status_4: i32,
    #[serde(rename = "status_5")]
    pub status_5: i32,
    #[serde(rename = "status_6")]
    pub status_6: i32,
    #[serde(rename = "status_7")]
    pub status_7: i32,
    #[serde(rename = "status_8")]
    pub status_8: i32,
    #[serde(rename = "status_9")]
    pub status_9: i32,
    pub ssid: String,
}

#[derive(Metric)]
#[measurement = "purpleair"]
struct PurpleAirMetric {
    #[telegraf(tag)]
    sensor_id: String,
    #[telegraf(tag)]
    place: String,
    #[telegraf(tag)]
    version: String,
    current_temp_f: i32,
    current_humidity: f32,
    current_dewpoint_f: i32,
    pressure: f32,
    pm2_5_aqi: f32,
    pm2_5_aqi_b: f32,
    p_0_3_um: u32,
    p_0_3_um_b: u32,
    p_0_5_um: u32,
    p_0_5_um_b: u32,
    p_1_0_um: u32,
    p_1_0_um_b: u32,
    p_2_5_um: u32,
    p_2_5_um_b: u32,
    p_5_0_um: u32,
    p_5_0_um_b: u32,
    p_10_0_um: u32,
    p_10_0_um_b: u32,
}

fn retrive_from_sensor(id: &str) -> Result<SensorData, reqwest::Error> {
    let sensor_url = format!("http://purpleair-{}.lan/json?live=true", &id);
    reqwest::blocking::get(&sensor_url)?.json::<SensorData>()
}

fn convert_sensor_data(data: SensorData) -> PurpleAirMetric {
    PurpleAirMetric {
        sensor_id: data.sensor_id,
        place: data.place,
        version: data.version,
        current_temp_f: data.current_temp_f,
        current_humidity: data.current_humidity,
        current_dewpoint_f: data.current_dewpoint_f,
        pressure: data.pressure,
        pm2_5_aqi: data.pm2_5_aqi,
        pm2_5_aqi_b: data.pm2_5_aqi_b,
        p_0_3_um: data.p_0_3_um as u32,
        p_0_3_um_b: data.p_0_3_um_b as u32,
        p_0_5_um: data.p_0_5_um as u32,
        p_0_5_um_b: data.p_0_5_um_b as u32,
        p_1_0_um: data.p_1_0_um as u32,
        p_1_0_um_b: data.p_1_0_um_b as u32,
        p_2_5_um: data.p_2_5_um as u32,
        p_2_5_um_b: data.p_2_5_um_b as u32,
        p_5_0_um: data.p_5_0_um as u32,
        p_5_0_um_b: data.p_5_0_um_b as u32,
        p_10_0_um: data.p_10_0_um as u32,
        p_10_0_um_b: data.p_10_0_um_b as u32,
    }
}

struct Config<'a> {
    sensor_ids: Vec<&'a str>,
    hostname: String,
}

fn parse_config() -> Config<'static> {
    let sensor_ids = env!("SENSOR_IDS", "Please set SENSOR_IDS")
        .split(",")
        .collect();
    let hostname = env!("TELEGRAF_HOST", "Please set TELEGRAF_HOST").to_string();

    Config {
        sensor_ids,
        hostname,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = parse_config();

    let mut client = Client::new(&config.hostname).unwrap();

    loop {
        for sensor in &config.sensor_ids {
            let sensor_data = retrive_from_sensor(&sensor)?;
            let metric = convert_sensor_data(sensor_data);
            client.write(&metric).unwrap();
        }
        println!("Successfully wrote data, sleeping for 60 seconds ^.^");
        thread::sleep(time::Duration::from_secs(60));
    }
}
