use rand::Rng;
trait Animal {
    fn make_sound(&self);
}
struct Pet;
struct Dog;
impl Animal for Dog {
    fn make_sound(&self) {
        println!("Haow Haow");
    }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) {
        println!("Miaou Miaou");
    }
}
enum AnimalType {
    Dog,
    Cat,
}
trait Factory {
    fn create_animal(animal_type: AnimalType) -> &'static dyn Animal;
    fn create_random_animal() -> &'static dyn Animal;
}
impl Factory for Pet {
    fn create_animal(animal_type: AnimalType) -> &'static dyn Animal {
        match animal_type {
            AnimalType::Dog => &Dog,
            AnimalType::Cat => &Cat,
        }
    }
    fn create_random_animal() -> &'static dyn Animal {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(1..=10);
        if random > 5 {
            return &Dog;
        } else {
            return &Cat;
        }
    }
}

fn main() {
    let dog = Pet::create_animal(AnimalType::Dog);
    dog.make_sound();
    let cat = Pet::create_animal(AnimalType::Cat);
    cat.make_sound();
    let random = Pet::create_random_animal();
    random.make_sound();
}
