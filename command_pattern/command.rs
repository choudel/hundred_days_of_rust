struct Remote {
    command: Box<dyn Command>,
}
impl Remote {
    fn new(command: Box<dyn Command>) -> Remote {
        Remote { command }
    }
    fn press_button(&self) {
        self.command.execute();
    }
    fn press_undo(&self) {
        self.command.undo();
    }
}
trait Command {
    fn execute(&self);
    fn undo(&self);
}

struct Line {
    receiver: Receiver,
    start: (i32, i32),
    end: (i32, i32),
}
impl Line {
    fn new(receiver: Receiver, start: (i32, i32), end: (i32, i32)) -> Line {
        Line {
            receiver,
            start,
            end,
        }
    }
}
impl Command for Line {
    fn execute(&self) {
        self.receiver.draw_line(self.start, self.end);
    }
    fn undo(&self) {
        self.receiver.erase_line(self.start, self.end);
    }
}

struct Receiver;

impl Receiver {
    fn draw_line(&self, start: (i32, i32), end: (i32, i32)) {
        println!("Draw line from {:?} to {:?}", start, end);
    }

    fn erase_line(&self, start: (i32, i32), end: (i32, i32)) {
        println!("Erase line from {:?} to {:?}", start, end);
    }
}
fn main() {
    let receiver = Receiver;
    let line = Line::new(receiver, (0, 0), (20, 10));
    let remote = Remote::new(Box::new(line));

    remote.press_button();
    remote.press_undo();
}
