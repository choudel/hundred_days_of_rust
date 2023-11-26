trait Observable {
    fn add(&mut self, observer: Box<dyn Observer>);
    fn remove(&mut self, id: u32);
    fn notify(&self);
}
trait Observer {
    fn update(&self);
    fn get_id(&self) -> u32;
}
struct ConcreteObservable {
    observers: Vec<Box<dyn Observer>>,
}
impl ConcreteObservable {
    fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }
    fn get_state() {
        println!("this is the state")
    }
}
struct ConcreteObserver {
    id: u32,
}
impl ConcreteObserver {
    fn new(id: u32) -> Self {
        Self { id }
    }
}
impl Observer for ConcreteObserver {
    fn update(&self) {
        println!("ConcreteObserver {} has been updated!", self.id);
        ConcreteObservable::get_state();
    }
    fn get_id(&self) -> u32 {
        self.id
    }
}
impl Observable for ConcreteObservable {
    fn add(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer)
    }
    fn remove(&mut self, id: u32) {
        self.observers.retain(|observer| observer.get_id() != id);
    }

    fn notify(&self) {
        for observer in &self.observers {
            observer.update();
        }
    }
}
fn main() {
    let mut observable = ConcreteObservable::new();
    let observer1 = ConcreteObserver::new(1);
    let observer2 = ConcreteObserver::new(2);
    observable.add(Box::new(observer1));
    observable.add(Box::new(observer2));
    observable.notify();
    observable.remove(1);

    observable.notify();
}
