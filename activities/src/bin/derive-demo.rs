#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}
#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };
    let boss = Employee {
        position: Position::Supervisor,
        work_hours: 40,
    };
    print_employee(boss);
    print_employee(boss);
    print_employee(me);
    print_employee(me);
}
