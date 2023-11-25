fn main() {
    let robot = Robot {
        name: String::from("Eve"),
    };
    println!("My robot name is {}.", robot.name);
    robot.run();
}

trait Run {
    fn run(&self) {
        println!("I am running!");
    }
}

struct Robot {
    name: String,
}
impl Run for Robot {
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}
