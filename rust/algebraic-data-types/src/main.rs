#[derive(Debug)]
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
    let _manager = Employee::Manager {
        name: "Alex".to_string(),
        subordinates: Vec::new(),
    };
    let employee = Employee::Worker {
        name: "Bob".to_string(),
        manager: "Alex".to_string(),
    };

    match employee {
        Employee::Manager { name, subordinates } => {
            println!("manager: {}, {:?}", name, subordinates)
        }
        Employee::Worker { name, manager } => {
            println!("worker: {}, {}", name, manager)
        }
    }
}
