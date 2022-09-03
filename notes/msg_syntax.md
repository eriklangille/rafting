# Message Syntax
Temporary syntax until I decide to implement protocol buffers

## Message
Starts w '*', message type, '*' length of content in bytes, '*', content in ascii.

### Ping
No content. Example: '*0*0*'

### Election
Call an election. One piece of data representing the id (port) in ascii of the server voting. '*1*4*3001'