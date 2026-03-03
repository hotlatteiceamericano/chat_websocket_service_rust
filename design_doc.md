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

## Storages
### Message Storage
A wide column db sort data physically using timestamp such as cassandra is ideal, but hard to host. Hence choosing MongoDB Atlas as the first implementation, and index it using timestamp.

*Shema*
todo

### User Storage
A document based nosql can support fast lookup and less frequent writes. Choosing MongoDB Atlas as the hosing platform.

*Shema*
todo

### Conversation Storage
A wide column nosql would be a good fit because it requires heavy read (everytime user login) and heavy write (everytime user sends message). Cassandra or ScyllaDB would be a good choice but former requires self-host, and the latter has no generous free tier. Hence still choose MongoDB Atls as the hosting platform

*Shema*
todo

## Chat Protocol
1. Use websocket to implement real-time chat

### Chat FLows
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

## Milesstones
1. Users are able to connect to the websocket server
1. Users are able to send message to the websocket server
1. Server is able to send message to recipient websocket client
1. store and fetch conversations for the logged in user
1. user can see the previous message with a given user
1. terminal UI

## Notes
1. decided to use MongoDB Atlas for 1) user information, 2) conversations, 3) messages

## Next
* Which db to store user information?
* Which db to store messages
