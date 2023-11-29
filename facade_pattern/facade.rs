struct Subsystem1;
impl Subsystem1 {
    fn operation1(&self) {
        println!("Subsystem1: Operation1");
    }
}

struct Subsystem2;
impl Subsystem2 {
    fn operation2(&self) {
        println!("Subsystem2: Operation2");
    }
}

struct Subsystem3;
impl Subsystem3 {
    fn operation3(&self) {
        println!("Subsystem3: Operation3");
    }
}

struct Facade {
    sub1: Subsystem1,
    sub2: Subsystem2,
    sub3: Subsystem3,
}

impl Facade {
    fn new() -> Facade {
        Facade {
            sub1: Subsystem1,
            sub2: Subsystem2,
            sub3: Subsystem3,
        }
    }

    fn operation(&self) {
        self.sub1.operation1();
        self.sub2.operation2();
        self.sub3.operation3();
    }
}
fn main() {
    let facade = Facade::new();
    facade.operation();
}
