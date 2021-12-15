use bevy::prelude::{info, EventReader, EventWriter};
use bevy_console::{ConsoleCommandEntered, PrintConsoleLine};

pub fn listen_to_console_events(
    mut console_events: EventReader<ConsoleCommandEntered>,
    mut console_output: EventWriter<PrintConsoleLine>,
) {
    for event in console_events.iter() {
        let event: &ConsoleCommandEntered = event;
        let command_args: Vec<&str> = event.args.as_str().split(" ").collect();
        info!("Commands: {:?}", event);
        match event.command.as_str() {
            "god" => {
                console_output.send(PrintConsoleLine::new("god mode acitvated".to_string()));
            }
            "give_gold" => {
                info!("Args: {}", command_args[1]);
            }
            _ => console_output.send(PrintConsoleLine::new("Unknown command".to_string())),
        }
    }
}
