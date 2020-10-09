use std::collections::HashMap;

pub struct Commands {
    pub command: String,
    pub argument1: String,
    pub argument2: String,
}

impl Commands {
    pub fn new() -> Commands {
        Commands {
            argument1: String::new(),
            argument2: String::new(),
            command: String::new(),
        }
    }

    pub fn call_command(&self, departments: &mut HashMap<String, Vec<String>>) {
        if self.command == String::from("ADD") {
            Commands::add_employee(&self.argument1, &self.argument2, departments);
        } else if self.command == String::from("REMOVE") {
            Commands::remove_employee(&self.argument1, &self.argument2, departments);
        }
    }

    fn add_employee(employee_name: &String, dpt_name: &String ,departments: &mut HashMap<String, Vec<String>>) {
        let employees = departments.entry(String::from(dpt_name)).or_insert(Vec::new());
        
        if !employees.contains(employee_name) {
            employees.push(employee_name.to_string());
        } else {
            println!("Employee already registered");
        }
    }

    fn remove_employee(employee_name: &String, dpt_name: &String ,departments: &mut HashMap<String, Vec<String>>) {
        let employees = departments.entry(String::from(dpt_name)).or_insert(Vec::new());

        if employees.contains(employee_name) {
            employees.retain(|x| x != employee_name);
        } else {
            println!("Employee not found")
        }
    }
}