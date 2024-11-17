use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use rumqttc::{MqttOptions, Client, QoS};
use std::time::Duration;

#[derive(Debug)]
struct ClientConfig {
    host: String,
    port: u16,
}

pub fn set_mqtt_config(host: &str, port: &str){
    let home = std::env::var("HOME").unwrap();
    let path = Path::new(&home).join(".CLI_MQTT");
    fs::create_dir_all(&path).unwrap_or_else(|err| {
        println!("! {:?}", err.kind());
    });
    let mut file = File::create(path.join("config")).expect("Unable to create config file");
    file.write_all(format!("host: {}\nport: {}", host, port)
        .as_bytes()).expect("Error writing to config file");
    
}

fn get_mqtt_config() -> ClientConfig {
    let home = std::env::var("HOME").unwrap();
    let path = Path::new(&home).join(".CLI_MQTT").join("config");
    let contents = fs::read_to_string(path).expect("Unable to read config file if not already created run 'set' first");
    
    let mut config = ClientConfig {
        host: String::new(),
        port: 0,
    };

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        match parts[0].trim() {
            "host" => config.host = parts[1].trim().to_string(),
            "port" => config.port = parts[1].trim().to_string().parse().expect("Port must be a number"),
            _ => println!("Invalid line in config file: {}. Please fix by running set again", line),
        }
    }
    config
}

pub(crate) fn start_mqqt_client() {
    let config = get_mqtt_config();
    println!("Starting MQTT client with config: {:?}", config);
    let mut mqttoptions = MqttOptions::new("cli-app", config.host, config.port);
    mqttoptions.set_keep_alive(Duration::from_secs(10));
    
    let (client, _connection) = Client::new(mqttoptions, 10);
    client.publish("hello/world", QoS::AtLeastOnce,  false,"Hello world".to_string()).unwrap();
    println!("Client started")
    
}
