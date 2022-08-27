# Thoughts on Raft

Timebox until 11:30 am

- Idea of distributed systems is that there is not one point of failure and you can scale up with demand
- Paxos dominant

### Features
- Strong leader - Log entries flow only from leader to other servers
- Leader election - Randomized timers. How does that compare to other algorithms??
- Membership changes - Joint consensus - Majority of two different configs overlap during transitions. Clusters continue operating normally during config changes

Actual algo is sections 5 - 8

### Replicated state machines
- Collection of servers compute identical copies of the same state and continue operating even if some of the servers are down

- Properties
 - Safety - never return incorrect result from network delays, partitions, packet loss, duplication, re-ordering
 - Fully functional (available) - as long as any *majority* of servers are operational. Communicate w each other and client. 5 servers tolerate 2 failures.
 - Do not depend on timing
 - Commonly, command can complete as soon as majority of cluster has responded to a single round of RPC. minority of slow servers not impact system performance

### The algorithm
- Distinguished leader - complete responsiblity for managing replicated log
  - ledare can decide where to place new entries without consulting other servers
  - Simple data flows
  - Failed leader results in new election

- What do they mean by volatile?
- Socket library in Rust? Idk much about sockets and networking
- How often is there an election?

### The basics
- The leader handles all client requests. So it doesn't scale that well? What is the throughput? Bottleneck
- Followers are passive. Issue no requests but simply respond to requests from leaders and candidates
- Time divided into terms of arbitrary length
- Term begins with an election, one or more candidates attempt to become leader
- On a split vote, term will end with no leader and a new term begins
- One leader per term

- Different servers observe transitions terms at different times or not at all. Skipping terms
- terms -> logical clock
- Current term exchanged always with server communication. If one servers current term smaller than others, it updates current term to larger value.
- If candidate/leader discovers term out of date term, revert to follower state
- RPCs are repeated if no response in timely matter

- Election timeout - If a follower receives no communication over a period of time then it assumes to viable leader and begins election to choose a new leader
- Randomized election timeouts - ensures split votes are rare and can be resolved quickly (150 - 300 ms)

- Leader decides when it is safe to apply a log entry to state machine. Entry is called committed.

Plan - take a break at noon for 45 minutes

Come back and look at source code/any questions
Compare source code to paper