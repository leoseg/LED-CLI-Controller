use clap::{Parser, ValueEnum, Subcommand};
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

    On(OnCommand),

    Rotate(RotateOrBlinkCommand),

    Blink(RotateOrBlinkCommand),

    Off,

    Set {
        
        #[arg(short, default_value_t = 1884)]
        port: u16,
        
        #[arg(short, default_value = "localhost")]
        mq_host: String,

        #[arg(short)]
        username: Option<String>,

        #[arg(short)]
        password: Option<String>
    }
}

#[derive(Debug, Parser, Serialize)]
struct OnCommand {

    #[arg(long, default_value_t = 50, value_parser = clap::value_parser!(u8).range(1..=100))]
    percentage: u8,

    #[arg(long, default_value = "white")]
    color: String,
}

#[derive(Debug, Parser,Serialize)]
struct RotateOrBlinkCommand {

    #[arg(long, default_value_t = 50, value_parser = clap::value_parser!(u8).range(1..=100))]
    percentage: u8,

    #[arg(long, default_value = "white")]
    color: String,

    #[arg(long, default_value_t = 50, value_parser = clap::value_parser!(u8).range(1..=100))]
    speed: u8,
}



#[derive(ValueEnum, Debug, Clone,Serialize)]
enum CommandName {
    On,
    Rotate,
    Off,
    Set
}

#[derive(ValueEnum, Debug, Clone,Serialize)]
pub enum Color {
    White,
    Red,
    Green,
    Blue,
    Purple,
    Yellow,
}

impl CommandCLI {
    
    pub fn execute(& mut self) {
        match &self.command {
            Commands::On(on_command) => {
                let message = serde_json::to_string(&self.command).expect("Error serializing message");
                send_message(&message);
                println!("Turning on LED with color {:?} and percentage {}", on_command.color, on_command.percentage);
            }
            Commands::Off {} => {
                let message = serde_json::to_string(&self.command).expect("Error serializing message");
                send_message(&message);
                println!("Turning off LED");
            }
            Commands::Set { port, mq_host, username, password} => {
                set_mqtt_config(mq_host, *port, username.as_deref(), password.as_deref());
                println!("Setting with port {} and host {}", port, mq_host);
            }
            Commands::Rotate(on_command)
                => {
                let message = serde_json::to_string(&self.command).expect("Error serializing message");
                send_message(&message);
                println!("Rotating LED with color {:?} and percentage {} and speed {}", on_command.color, on_command.percentage, on_command.speed);
            }
            Commands::Blink(on_command)
                => {
                let message = serde_json::to_string(&self.command).expect("Error serializing message");
                send_message(&message);
                println!("Blinking LED with color {:?} and percentage {} and speed {}", on_command.color, on_command.percentage, on_command.speed);
            }
        }
    }
}
