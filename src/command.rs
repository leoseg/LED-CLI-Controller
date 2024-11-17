use crate::mqttclient::{set_mqtt_config, start_mqqt_client};
pub trait Command {
    fn execute(&self,args: Vec<String>) -> Result<(), String>;
}

struct DefaultCommand;
struct Start;
struct Stop;
struct Set;
struct Publish;

impl Command for Publish {
    fn execute(&self,args: Vec<String>) -> Result<(), String> {
        assert_eq!(args.len(), 2,"Publish command requires 2 arguments");
        println!("Publishing message: {}", args[1]);
        Ok(())
    }
}

impl Command for DefaultCommand {
            fn execute(&self,_args: Vec<String>) -> Result<(), String> {
                println!("Please enter a valid command");
                Ok(())
            }
}

impl Command for Start {
    fn execute(&self,args: Vec<String>) -> Result<(), String> {
        assert_eq!(args.len(), 0,"Start command does not take any arguments");
        if args.len() != 0 {
            return Err("Stop command does not take any arguments".to_string());
        }
        start_mqqt_client();
        Ok(())
    }
}

impl Command for Stop {
    fn execute(&self,args: Vec<String>) -> Result<(), String> {
        assert_eq!(args.len(), 0,"Start command does not take any arguments");
        println!("Stopping MQTT client");
        Ok(())
    }
}

impl Command for Set {
    fn execute(&self, args: Vec<String>) -> Result<(), String> {
        assert_eq!(args.len(), 2,"Set command requires 2 arguments");
        set_mqtt_config(&args[0], &args[1] );
        Ok(())
    }
}



pub fn get_command(command_name: &str) -> Box<dyn Command>{
    match command_name {
        "set" => Box::new(Set),
        "start" => Box::new(Start),
        "stop" => Box::new(Stop),
        "publish" => Box::new(Publish),
        _ => Box::new(DefaultCommand)
    }
}