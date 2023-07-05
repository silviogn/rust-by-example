enum Employee {
    Manager {
        name: String,
        subordinates: Vec<Box<Employee>>,
    },
    Worker {
        name: String,
        manager: String,
    },
}
fn main() {
    let employee = Employee::Worker {
        name: "Bob".to_string(),
        manager: "Silvio".to_string(),
    };

    match employee {
        Employee::Manager { name, subordinates } => {
            println!("{} is a manager {} subordinates.", name, subordinates.len())
        }
        Employee::Worker { name, manager } => {
            println!("{} is worder under the management of {}", name, manager)
        }
    }
}
