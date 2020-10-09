mod commands;

use std::io;
use std::collections::HashMap;
use commands::commands::Commands;

fn main() {
    
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    let mut input = String::new();
    let exit = String::from("EXIT");
    let mut commands: Commands = Commands::new();
    loop {
        input = String::new();

        println!("Welcome to Head Tracker!");

        io::stdin()
            .read_line(&mut input)
            .expect("Fail to read input!");
        input.pop();

        if input.to_uppercase() == exit {
            println!("Bye!");
            break;
        }

        let sliced_commands = slicer(&input);
        
        match sliced_commands.get(0) {
            Some(val) => {
                commands.command = val.to_uppercase().to_string();
            },
            None => {}
        }

        match sliced_commands.get(1) {
            Some(val) => {
                let temp: String = String::from(val);
                commands.argument1 = temp;
            },
            None => {}
        }

        match sliced_commands.get(2) {
            Some(val) => {
                let temp: String = String::from(val);
                commands.argument2 = temp.clone();
            },
            None => {}
        }

        commands.call_command(&mut departments);
        println!("{:#?}", departments);
    }
        
}

fn slicer(command: &String) -> Vec<String> {
    let a: Vec<&str> = command.split(" ").collect();
    let mut b: Vec<String> = vec![];

    for i in a {
        b.push(String::from(i));
    }

    b
}

