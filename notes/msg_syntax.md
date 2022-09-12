# Message Syntax
Temporary syntax until I decide to implement protocol buffers

## Message
Starts w '*', message type, '*' length of content in bytes, '*', content in ascii.

### Ping
No content. Example: '*0*0*'
Response. '*0*1*1'

### Election
Call an election. One piece of data representing the id (port) in ascii of the server voting. '*1*4*3001'

# Message Syntax 2.0
Should actually implement the paper lol

## RequestVote
### Args: 
term - candidate's term
candidateId - candidate requesting vote
lastLogIndex - index of candidate's last log entry
lastLogTerm - term of candidate's last log entry

### Results:
term - currentTerm, for candidate to update itself
voteGranted - true means candidate received vote

### Receiver Implementation:
1. Reply false term < currentTerm
2. If votedFor is null or candidateId, and candidate's log is at least up-to-date as receiver's log, grant vote (reply true)