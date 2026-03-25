# Chat Service WebSocket Service
## Fcuntionality
1. client connect to websocket server using the websocket url, and the JWT token provided by HTTP server.
1. when received message from the clients, send this message to Redis Pub/Sub.
1. subscribe to the Redis Pub/Sub, and consume message:
  1. if recepient is online, recepient consume its inbox
  1. if receipien is offline, store the message to permanent storage

## System Requirements
1. Separate HTTP and WebSocket server, so that they can scale independently
1. WebSocket server are communicated through a Redis Pub/Sub, for cross server communications
1. log when user log in when user visit the hash/secret url

## Connection Lookup
It is important for this project to scale up websocket server easily. That we can decrease or increase the number of websocket server to reduce cost or to accomodate with increasing usages.

It results different users may have their websocket connections on different server, making us to implement a cross-server communication method. Redis Pub/Sub is chosen because, we don't need persistent storage on message sending. We allow messages to not be sent to the client, and client should be notified when such happened and choose to resend or not.

To start with single server implementation, we should abstract the connection lookup in a trait and choose in-server implementation or cross-server implementation.

## Storages
### User Storage
A document based nosql can support fast lookup and less frequent writes. Choosing MongoDB Atlas as the hosing platform.

*Shema*
id: 
display_name: String
server_id: String

### Message Storage
A wide column db sort data physically using timestamp such as cassandra is ideal, but it is hard to host. Hence choosing MongoDB Atlas as the first implementation, and index it using timestamp.

*Shema*
sender: User
receiver: User
payload: Option<Vec<u64>>
msg_type: message type enum, including image, text and a type indicating user is typing
timestamp: 

### Conversation Storage
A wide column nosql would be a good fit because it requires heavy read (everytime user login) and heavy write (everytime user sends message). Cassandra or ScyllaDB would be a good choice but former requires self-host, and the latter has no generous free tier. Hence still choose MongoDB Atls as the hosting platform

*Shema*
sender: User
receiver: User
last_message: Message

## Chat Protocol
1. Use websocket to implement real-time chat

### Chat FLows
* spawning two tasks for sending messages and receiving messages respectively. so that they don't block each other
═══════════════════════════════════════════════════════════════════════════════════
  ALICE SENDS "hi bob" TO BOB
═══════════════════════════════════════════════════════════════════════════════════

  Alice's Browser ┊ alice_ws_receiver  alice_tx/rx   bob_tx/rx   bob_ws_sender  ┊ Bob's Browser
                  ┊                            Server                           ┊
  ················┊·····························································┊················
       │          ┊         │               │             │            │        ┊        │
  1.   │── "hi bob" ──→     │               │             │            │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  2.   │          ┊   Sender Task reads it  │             │            │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  3.   │          ┊   store in database     │             │            │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  4.   │          ┊   connections.get("bob")│             │            │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  5.   │          ┊         │───────────────────────→ tx.send()        │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  6.   │          ┊         │               │         rx.recv()        │        ┊        │
       │          ┊         │               │             │            │        ┊        │
  7.   │          ┊         │               │       Receiver Task reads it      │        ┊        │
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


## Deployment
To-research, can I deployt WebSocket server and HTTP server altogether using Docker?

## Milesstones
[x]. Users are able to connect to the websocket server
[x]. Users are able to send message to the websocket server
[x]. Server is able to send message to recipient websocket client
[]. terminal UI to connect to the websocket server
[]. terminal UI can send and receive messages
[]. store and fetch conversations for the logged in user
[]. user can see the previous message with a given user

## Notes
1. decided to use MongoDB Atlas for 1) user information, 2) conversations, 3) messages

## Future Phases
1. User's online/active indicator
1. User's typing or not
1. Send images and store images using object_store_rust service (https://github.com/hotlatteiceamericano/object_store_rust)
