use std::io;

//use crate::employee_database;

use super::database::Database;

pub struct Api {
    database: Database
}

impl Api {
    pub fn new() -> Api {
        Api {
            database: Database::new()
        }
    }

    pub fn run(&mut self) {
        println!(">> Employee database started <<");
        println!("type \"h\" for commands");

        loop {
            let input = Self::get_input();
            match self.process_input(input) {
                Ok(should_stop) => {
                    if should_stop {
                        break;
                    }
                },
                Err(e) => println!("Error: {}", e)
            }
        }
        println!(">> Employee database terminated <<");
    }

    fn get_input() -> String{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Could not parse input, please try again");
        input
    }

    fn process_input(&mut self, input: String) -> Result<bool, &str> {
        match input.split_ascii_whitespace().next().expect("Could not parse the first part of the command") {
            "ld" => self.database.list_departments(),
            "le" => self.database.list_employees(),
            "lde" => {
                let department_name = Self::parse_department_name(&input)?;
                let _ = self.database.list_department_employees(department_name)?;
            },
            "add" => {
                let department_name = Self::parse_department_name(&input)?;
                let employee_name = Self::parse_employee_name(&input)?;
                self.database.add_entry(department_name, employee_name);
                println!("entry added")
            },
            "h" => Self::print_help(),
            "exit" => return Ok(true),
            _ => println!("Unknown command")
        };
        Ok(false)
    }

    fn print_help() {
        println!("Following commands are supported:");
        println!("ld -> list all departments");
        println!("le -> list all employees in all departments");
        println!("lde [DEPARTMENT_NAME]-> list all employees in one department");
        println!("add [EMPLOYEE_NAME] [DEPARTMENT_NAME]");
        println!("h -> lisy all commands");
        println!("exit -> end the program");
    }

    fn parse_department_name(command: &String) -> Result<&str, &'static str> {
        command
            .split_ascii_whitespace()
            .nth(1)
            .ok_or("missing department name")
    }

    fn parse_employee_name(command: &String) -> Result<&str, &'static str> {
        command
            .split_ascii_whitespace()
            .nth(2)
            .ok_or("missing employee name")
    }
}


