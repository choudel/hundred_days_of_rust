trait Beverage {
    fn get_desc(&self) -> String;
    fn get_price(&self) -> u32;
}

struct Coffee {
    desc: String,
    price: u32,
}

impl Beverage for Coffee {
    fn get_desc(&self) -> String {
        self.desc.clone()
    }
    fn get_price(&self) -> u32 {
        self.price
    }
}

trait Decorator: Beverage {
    fn get_beverage(&self) -> &dyn Beverage;
}

struct Milk<T: Beverage> {
    desc: String,
    price: u32,
    beverage: T,
}

impl<T: Beverage> Beverage for Milk<T> {
    fn get_desc(&self) -> String {
        format!("{}, {}", self.beverage.get_desc(), self.desc)
    }
    fn get_price(&self) -> u32 {
        self.beverage.get_price() + self.price
    }
}

impl<T: Beverage> Decorator for Milk<T> {
    fn get_beverage(&self) -> &dyn Beverage {
        &self.beverage
    }
}

fn main() {
    let coffee = Coffee {
        desc: String::from("Coffee"),
        price: 50,
    };
    let milk_coffee = Milk {
        desc: String::from("Milk"),
        price: 20,
        beverage: coffee,
    };
    println!("Beverage: {}", milk_coffee.get_desc());
    println!("Price: {}", milk_coffee.get_price());
}
