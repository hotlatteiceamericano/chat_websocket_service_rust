# Chat Service WebSocket Service
## Fcuntionality
1. log in by entering user id and will provide a hash/secret url for use to log in. this url shall later be only sent to the registered email address
1. connect to the chat service through a visit of a url
1. http request to list chat history about which receipients were chatted before
1. http request when provide receipient id would like to chat with
1. return the chat history between you and the receipient
1. send the message through a real time message queue (inbox)
1. if recepient is online, recepient consume its inbox
1. if receipien is offline, store the message until it come back online
1. service keeps detecting if sender or receipient are still online
1. indicate if receipient is online or not based on detection result
1. chat can send images, will store the image using object_store_rust service (https://github.com/hotlatteiceamericano/object_store_rust)

## Operational Requirements
1. log when user log in when user visit the hash/secret url

## System Components
### Message Storage
1. Supports fast append operation, message are frequently being written
1. Supports time range query so that user can easily see the message with another user in decendent time order

### User Storage
1. keep it minimal, save the id and name first

### Message Queue
1. Planning to use a message queue as the way to handle offline scenario

### Chat Protocol
1. Use websocket to implement real-time chat

## FLows
### Message send flow
1. Alice send message to Bob
  1. Alice 
═══════════════════════════════════════════════════════════════════════════════════
  ALICE SENDS "hi bob" TO BOB
═══════════════════════════════════════════════════════════════════════════════════

  Alice's Browser ┊ alice_ws_receiver  alice_tx/rx   bob_tx/rx   bob_ws_sender  ┊ Bob's Browser
                  ┊                            Server                           ┊
  ················┊·····························································┊················
       │          ┊         │               │             │            │        ┊        │
  1.   │── "hi bob" ──→     │               │             │            │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  2.   │          ┊   Task 1 reads it       │             │            │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  3.   │          ┊   store in database     │             │            │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  4.   │          ┊   connections.get("bob")│             │            │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  5.   │          ┊         │───────────────────────→ tx.send()        │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  6.   │          ┊         │               │         rx.recv()        │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  7.   │          ┊         │               │       Task 2 reads it    │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  8.   │          ┊         │               │             │──→ ws_sender.send() ──→      │
       │          ┊         │               │             │            │        ┊  "hi bob"


═══════════════════════════════════════════════════════════════════════════════════
  BOB REPLIES "hey alice" TO ALICE
═══════════════════════════════════════════════════════════════════════════════════

  Alice's Browser ┊ alice_ws_sender  alice_tx/rx   bob_tx/rx   bob_ws_receiver  ┊ Bob's Browser
                  ┊                            Server                           ┊
  ················┊·····························································┊················
       │          ┊         │               │             │            │        ┊        │
  1.   │          ┊         │               │             │      ←──────── "hey alice"   │
       │          ┊         │               │             │            │        ┊        │
  2.   │          ┊         │               │             │      Task 1 reads it┊        │
       │          ┊         │               │             │            │        ┊        │
  3.   │          ┊         │               │             │      store in database       │
       │          ┊         │               │             │            │        ┊        │
  4.   │          ┊         │               │  connections.get("alice")│        ┊        │
       │          ┊         │               │             │            │        ┊        │
  5.   │          ┊         │          tx.send() ←────────│            │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  6.   │          ┊         │          rx.recv()          │            │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  7.   │          ┊         │       Task 2 reads it       │            │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  8.   │    ←── ws_sender.send() ←──│       │             │            │        ┊        │
       │ "hey alice"        │               │             │            │        ┊        │


═══════════════════════════════════════════════════════════════════════════════════
  EACH USER'S TWO TASKS (same structure for everyone)
═══════════════════════════════════════════════════════════════════════════════════

  Task 1 (read from this user, route to others):

    ws_receiver ──→ parse recipient ──→ store in db ──→ recipient_tx.send()

  Task 2 (receive from others, write to this user):

    rx.recv() ──→ ws_sender.send() ──→ out to browser

## FLows
### Message send flow
1. User send two request, one websocket request to the receipient, one to the message queue.
1. Would it be wasteful to send the same message in two different ways?
1. Message queue will later save the message to message storage
## Implementation Steps
1. decide which storage to store users
1. users able to connect to websocket server
1. decide how to store message between two users
1. the storing should support fast lookup and sort by time, so that user can easily see the history chat with another user
1. chat message storage should be optimized for appending operation, as new message are written frequently
1. send message method take an user id and message content
1. when two users are online, both should be able to see the message sent to them
