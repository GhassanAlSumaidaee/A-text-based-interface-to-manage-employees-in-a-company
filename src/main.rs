use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Please choose an option:");
        println!("1. Add employee to department");
        println!("2. List employees in a department");
        println!("3. List all employees by department");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => add_employee(&mut company),
            2 => list_department(&company),
            3 => list_all(&company),
            4 => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn add_employee(company: &mut HashMap<String, Vec<String>>) {
    println!("Enter the employee's name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim().to_string();

    println!("Enter the department:");
    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read line");
    let department = department.trim().to_string();

    company
        .entry(department)
        .or_insert_with(Vec::new)
        .push(name);
    println!("Employee added.");
}

fn list_department(company: &HashMap<String, Vec<String>>) {
    println!("Enter the department:");
    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read line");
    let department = department.trim();

    match company.get(department) {
        Some(employees) => {
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();
            println!("Employees in {}: {:?}", department, sorted_employees);
        }
        None => println!("Department not found."),
    }
}

fn list_all(company: &HashMap<String, Vec<String>>) {
    let mut departments: Vec<_> = company.keys().collect();
    departments.sort();

    for department in departments {
        let mut employees = company.get(department).unwrap().clone();
        employees.sort();
        println!("{}: {:?}", department, employees);
    }
}
