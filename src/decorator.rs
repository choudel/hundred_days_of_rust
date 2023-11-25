trait Beverage {
    fn get_desc(&self) -> String;
    fn get_cost(&self) -> i32;
}
#[derive(Clone)]
struct Espresso {
    desc: String,
    cost: i32,
}
impl Beverage for Espresso {
    fn get_desc(&self) -> String {
        self.desc.clone()
    }
    fn get_cost(&self) -> i32 {
        self.cost
    }
}
trait Decorator {
    fn get_beverage(&self) -> &dyn Beverage;
}
struct Caramel {
    desc: String,
    cost: i32,
    beverage: Box<dyn Beverage>,
}
impl Beverage for Caramel {
    fn get_desc(&self) -> String {
        format!("{}, {}", self.beverage.get_desc(), self.desc)
    }
    fn get_cost(&self) -> i32 {
        self.beverage.get_cost() + self.cost
    }
}
impl Decorator for Caramel {
    fn get_beverage(&self) -> &dyn Beverage {
        self.beverage.as_ref()
    }
}
struct Soy {
    desc: String,
    cost: i32,
    beverage: Box<dyn Beverage>,
}
impl Beverage for Soy {
    fn get_desc(&self) -> String {
        format!("{}, {}", self.beverage.get_desc(), self.desc)
    }

    fn get_cost(&self) -> i32 {
        self.beverage.get_cost() + self.cost
    }
}

impl Decorator for Soy {
    fn get_beverage(&self) -> &dyn Beverage {
        self.beverage.as_ref()
    }
}
fn main() {
    let espresso = Espresso {
        desc: String::from("Espresso"),
        cost: 50,
    };

    let caramel_espresso = Caramel {
        desc: String::from("Caramel"),
        cost: 20,
        beverage: Box::new(espresso),
    };

    let soy_caramel_espresso = Soy {
        desc: String::from("Soy"),
        cost: 15,
        beverage: Box::new(caramel_espresso),
    };

    println!("Beverage: {}", soy_caramel_espresso.get_desc());
    println!("Cost: {}", soy_caramel_espresso.get_cost());
}
