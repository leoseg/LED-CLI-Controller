# LED-CLI-Controller
A simple CLI tool to control my LED strip connected to an Esp32 microcontroller.
Run 
```shell
led-cli set --mqtt_host=localhost --port=1884
```
to set the host and port of the mqtt broker to which the 
client should connect.
Run 'on' or 'off' to publish messages at least once to the topic 'led'.
You can provide the message the option 'color' and 'percentage' this will result in a message string 
in json format like this:
```json
{
    "led-state": "on/off",
    "color": "red",
    "percentage": 100
}
```

### Broker
I used rumqttd to run a broker on my local machine, the configuration is in the `rumqttd.toml` file in `broker-config` folder.
