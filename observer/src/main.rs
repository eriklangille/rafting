// Inspired by: https://github.com/lpxxn/rust-design-pattern/blob/master/behavioral/observer.rs
use std::sync::Weak;
use std::sync::Arc;

mod messenger;

pub struct Message {
    request: Request,
}

type Callback = Weak<Func>;
type Func = dyn Fn(Request) -> Response;

#[derive(Debug, Clone, Copy)]
pub enum Request {
    Option1,
    Option2 {random_param: u32}
}

#[derive(Debug, Clone, Copy)]
pub enum Response {
    Option1,
    Option2 {random_param2: u32}
}

trait INetwork {
    fn when(&mut self, msg: Request, observer: Callback);
    fn notify_observers(&self, msg: Request);
}

struct Network {
    observers: Vec<Callback>
}

impl Network {
    fn new() -> Network {
        Network {
            observers: Vec::new()
        }
    }
}

impl INetwork for Network {
    fn when(&mut self, msg: Request, observer: Callback) {
        self.observers.push(observer);
    }
    fn notify_observers(&self, msg: Request) {
        for item in self.observers.iter() {
            if let Some(function) = item.upgrade() {
                function(msg);
            }
        }
    }
}

fn code_test(msg: Request) -> Response {
    println!("hey a message {:?}", msg);
    return Response::Option1 {}
}

fn main() {
    // let mut network = Network::new();
    // let arc = Arc::new(code_test);
    // let downgrade = Arc::downgrade(&arc);

    // network.when(Request::Option1, downgrade);

    // let msg = Request::Option1 {};
    // let msg2 = Request::Option2 {random_param: 10};
    // network.notify_observers(msg);
    // network.notify_observers(msg2);

    messenger::test();
}
