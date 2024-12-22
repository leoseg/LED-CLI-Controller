use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use rumqttc::{MqttOptions, Client, QoS, Event, Packet, Transport, TlsConfiguration};
use std::time::Duration;
use tokio_rustls::rustls::ClientConfig;

#[derive(Debug)]
struct MqttClientConfig {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
}

pub fn set_mqtt_config(host: &str, port: u16, username: Option<&str>, password: Option<&str>) {
    let home = std::env::var("HOME").unwrap();
    let path = Path::new(&home).join(".CLI_MQTT");
    fs::create_dir_all(&path).unwrap_or_else(|err| {
        println!("! {:?}", err.kind());
    });
    let mut file = File::create(path.join("config")).expect("Unable to create config file");
    let file_content;
    if username.is_some() && password.is_some() {
        file_content = format!("host: {}\nport: {}\nusername: {}\npassword: {}", host, port, username.unwrap(), password.unwrap());
    } else {
        file_content = format!("host: {}\nport: {}", host, port);
    }
    file.write_all(file_content
        .as_bytes()).expect("Error writing to config file");
    
}

fn get_mqtt_config() -> MqttClientConfig {
    let home = std::env::var("HOME").unwrap();
    let path = Path::new(&home).join(".CLI_MQTT").join("config");
    let contents = fs::read_to_string(path).expect("Unable to read config file if not already created run 'set' first");
    
    let mut config = MqttClientConfig {
        host: String::new(),
        port: 0,
        username: None,
        password: None,
    };

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        match parts[0].trim() {
            "host" => config.host = parts[1].trim().to_string(),
            "port" => config.port = parts[1].trim().to_string().parse().expect("Port must be a number"),
            "username" => config.username = Some(parts[1].trim().to_string()),
            "password" => config.password = Some(parts[1].trim().to_string()),
            _ => println!("Invalid line in config file: {}. Please fix by running set again", line),
        }
    }
    config
}

pub fn send_message(message:&str) {
    let config = get_mqtt_config();
    println!("Starting MQTT client with config: {:?}", config);
    let mut mqttoptions = MqttOptions::new("cli-app", config.host, config.port);
    if config.username.is_some() && config.password.is_some() {
        mqttoptions.set_credentials(
            config.username.unwrap(),
            config.password.unwrap()
        );
        let client_config = create_tls_config();
        mqttoptions.set_transport(Transport::tls_with_config(TlsConfiguration::from(client_config)));
    }
    mqttoptions.set_keep_alive(Duration::from_secs(10));
    
    let (client, mut connection) = Client::new(mqttoptions, 10);
    let connection_thread =std::thread::spawn(move || {
        for notification in connection.iter() {
             match notification {
                Ok(Event::Incoming(Packet::PubAck(_))) => {
                    println!("Message published");
                    break;
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    break;
                }
                _ => {}
            }
        }
    });
    client.publish("led", QoS::AtLeastOnce,  false,message).unwrap();
    connection_thread.join().unwrap();
}

fn create_tls_config() -> ClientConfig {
    let mut root_cert_store = tokio_rustls::rustls::RootCertStore::empty();
    root_cert_store.add_parsable_certificates(
        rustls_native_certs::load_native_certs().expect("could not load platform certs"),
    );

    ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth()
}


