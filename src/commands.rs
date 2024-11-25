use clap::{Parser, ValueEnum,Subcommand};
use serde::Serialize;
use crate::mqttclient::{send_message, set_mqtt_config};

#[derive(Parser, Debug, Serialize)]
#[command(name = "led-cli", version = "1.0", about = "A CLI app to controll Led lights")]
pub struct CommandCLI {
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug,Serialize)]
#[serde(tag = "led_state")]
enum Commands {

    On {
        #[arg(long, default_value_t = 50, value_parser = clap::value_parser!(u8).range(1..=100))]
        percentage: u8,
        
        #[arg(long, default_value = "white")]
        color: Color
    },

    Off {
        #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=100))]
        percentage: u8,

        #[arg(long, default_value = "white")]
        color: Color
    },

    Set {
        
        #[arg(short, default_value_t = 1884)]
        port: u16,
        
        #[arg(short, default_value = "localhost")]
        mq_host: String,
    }
}

#[derive(ValueEnum, Debug, Clone,Serialize)]
enum CommandName {
    On,
    Off,
    Set
}

#[derive(ValueEnum, Debug, Clone,Serialize)]
pub enum Color {
    White,
    Red,
    Green,
    Blue,
    Yellow,
}

impl CommandCLI {
    
    pub fn execute(& mut self) {
        match &self.command {
            Commands::On { percentage, color } => {
                let message = serde_json::to_string(&self.command).expect("Error serializing message");
                send_message(&message);
                println!("Turning on LED with color {:?} and percentage {}", color, percentage);
            }
            Commands::Off {percentage, color} => {
                let message = serde_json::to_string(&self.command).expect("Error serializing message");
                send_message(&message);
                println!("Turning off LED");
            }
            Commands::Set { port, mq_host } => {
                set_mqtt_config(mq_host, *port);
                println!("Setting with port {} and host {}", port, mq_host);
            }
        }
    }
}
