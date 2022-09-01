## TODO
- [x] Simple 2 threads that can communicate with each other and listen on a socket
- [x] Learn tokio
- [ ] struct that handles the communication of the different processes
- [ ] Rewrite socket_range to retry connecting to other ports and do it on a different thread
- [ ] Use Tokio watch to send note to call an election to different threads. Or maybe use single consumer/producer?
- [ ] Send messages while leader

## Thoughts
- Threads should always be pinging the available ports (even if failed) in a continuous loop
  - Should have an idea of the connected servers
  - Communicate between different channels in main. Implement MPSC
- How does port binding work under the hood? Is a message sent on disconnect and connect? What happens if disconnect message never sent?
- Need to delegate a unique ID for each server

## Procedure
Term = 1
Thread sleep randomized for all servers. When sleep is over send message to elect as leader

Send messages to the other server's on their port. Listen for messages on own port and always respond
When election timer runs out is the first time a message is sent from the server