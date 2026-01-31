use std::collections::HashMap;

pub struct Database {
    data: HashMap<String, Vec<String>>
}

impl Database {
    pub fn new() -> Database {
        Database {
            data: HashMap::new()
        }
    }

    pub fn list_departments(&self) {
        println!("These departments are represented in the database:");
        let mut keys: Vec<String> = self.data.keys().cloned().collect();
        keys.sort();
        println!("{:?}", keys);
    }

    pub fn list_employees(&self) {
        println!("These are all registered employees:");
        let mut all_employees: Vec<&String> = Vec::new();
        for key in self.data.keys() {
            all_employees.extend(&self.data[key]);
        }
        all_employees.sort();
        println!("{:?}", all_employees);
    }

    pub fn list_department_employees(&self, department_name: &str) -> Result<(), &'static str> {
        println!("Following employees are part of department {}:", department_name);
        println!("{:?}", &self.data.get(department_name).ok_or("department not found")?);
        Ok(())
    }

    pub fn add_entry(&mut self, department_name: &str, employee_name: &str) {
        let employees = self.data.entry(department_name.to_string()).or_insert_with(Vec::new);
        employees.push(employee_name.to_string());
        employees.sort();
    }
}
