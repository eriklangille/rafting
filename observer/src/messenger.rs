trait Handler<M>
where M: Message {
  fn handle(msg: M) -> M::Response;
}

trait Message {
  type Response: 'static;
}

struct RequestVoteResponse {
}

#[derive(Debug, Clone)]
struct RequestVoteRequest {
  potato: u16
}

impl Message for RequestVoteRequest {
  type Response = RequestVoteResponse;
}

struct RequestVoteActor;

impl Handler<RequestVoteRequest> for RequestVoteActor {
  fn handle(msg: RequestVoteRequest) -> RequestVoteResponse {
    println!("Message value: {:?}", msg.potato);
    RequestVoteResponse { }
  }
}

struct Network {}

impl Network {
  pub fn notify_observers<T, M>(msg: M) -> M::Response
  where T: Handler<M>,
  M: Message
  {
    T::handle(msg)
  }
}

pub fn test() {
  let potato_request = RequestVoteRequest {potato: 1};
  Network::notify_observers::<RequestVoteActor, RequestVoteRequest>(potato_request);
}
