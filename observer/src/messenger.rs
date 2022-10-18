use std::{collections::HashMap};
use std::any::{Any, TypeId};
use std::sync::Mutex;


trait Handler<M>
where M: Message {
  fn handle(&self, msg: M);
}

trait Message {
  type Response: 'static;
}

struct RequestVoteResponse {
}

#[derive(Debug, Clone, Copy)]
struct RequestVoteRequest {
  potato: u16
}

impl Message for RequestVoteRequest {
  type Response = RequestVoteResponse;
}

#[derive(Clone)]
struct RequestVoteActor;

impl Handler<RequestVoteRequest> for RequestVoteActor {
  fn handle(&self, msg: RequestVoteRequest) {
    println!("Message value: {:?}", msg.potato);
  }
}

struct Network
{
  //TypeId should be for the message type, Box<dyn Any + Send> should be actor with trait implementation
  observers: Mutex<HashMap<TypeId, Box<dyn Any + Send>>> 
}

static REQ_VOTE_HANDLER: RequestVoteActor = RequestVoteActor {};

impl Network {
  pub fn new() -> Self {
    Network { observers: Mutex::new(HashMap::new()) }
  }

  pub fn notify_observers<T, M>(&mut self, msg: M)
  where T: Handler<M> + 'static,
  M: Message
  {
    let observers = self.observers.get_mut().unwrap();
    if let Some(actor) = observers.get(&TypeId::of::<T>()) {
      match actor.downcast_ref::<T>() {
        Some(actor) => actor.handle(msg),
        None => panic!("Spaghetti!")
      }
    }
  }

  pub fn when<T, M>(&mut self, handler: &T)
  where T: Handler<M> + Clone + Send + 'static,
  M: Message,
  {
    let observers = self.observers.get_mut().unwrap();
    observers.insert(TypeId::of::<T>(), Box::new(handler.clone()));
  }
}

pub fn test() {
  println!("yup");
  let potato_request = RequestVoteRequest {potato: 1};
  let mut network = Network::new();
  network.when(&REQ_VOTE_HANDLER);
  network.notify_observers::<RequestVoteActor, RequestVoteRequest>(potato_request);
}
