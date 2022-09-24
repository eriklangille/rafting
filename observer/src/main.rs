// Inspired heavily by: https://github.com/lpxxn/rust-design-pattern/blob/master/behavioral/observer.rs
pub enum Message {
    Option1,
    Option2 {random_param: u32}
}

trait IObserver {
    fn update(&self, msg: &Message);
}

trait INetwork<'a, T: IObserver> {
    fn when(&mut self, msg: Message, observer: &'a T);
    fn notify_observers(&self, msg: &Message);
}

struct Network<'a, T: IObserver> {
    observers: Vec<&'a T>
}

impl <'a, T: IObserver + PartialEq> Network<'a, T> {
    fn new() -> Network <'a, T> {
        Network {
            observers: Vec::new()
        }
    }
}

impl <'a, T: IObserver + PartialEq> INetwork<'a, T> for Network<'a, T> {
    fn when(&mut self, msg: Message, observer: &'a T) {
        self.observers.push(observer);
    }
    fn notify_observers(&self, msg: &Message) {
        for item in self.observers.iter() {
            item.update(msg);
        }
    }
}

#[derive(PartialEq)]
struct MessageObserver {
}
impl IObserver for MessageObserver {
    fn update(&self, msg: &Message) {
        match msg {
            Message::Option1 => println!("Option 1"),
            Message::Option2 {..} => println!("Ayy yo we don't do Option 2"),
        }
    }
}

fn main() {
    let mut network = Network::new();
    let observer = MessageObserver {};
    network.when(Message::Option1, &observer);

    let msg = Message::Option1 {};
    let msg2 = Message::Option2 {random_param: 10};
    network.notify_observers(&msg);
    network.notify_observers(&msg2);
}
