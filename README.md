# centrifuge-rs
centrifugo server rust client

## Feature matrix

- [ ] connect to server using JSON protocol format
- [x] connect to server using Protobuf protocol format
- [x] connect with token (JWT)
- [x] connect with custom header
- [ ] automatic reconnect in case of errors, network problems etc
- [ ] an exponential backoff for reconnect
- [ ] connect and disconnect events
- [ ] handle disconnect reason
- [ ] subscribe on a channel and handle asynchronous Publications
- [ ] handle Join and Leave messages
- [ ] handle Unsubscribe notifications
- [ ] reconnect on subscribe timeout
- [ ] publish method of Subscription
- [ ] unsubscribe method of Subscription
- [ ] presence method of Subscription
- [ ] presence stats method of Subscription
- [ ] history method of Subscription
- [ ] top-level publish method
- [ ] top-level presence method
- [ ] top-level presence stats method
- [ ] top-level history method
- [ ] top-level unsubscribe method
- [ ] send asynchronous messages to server
- [ ] handle asynchronous messages from server
- [ ] send RPC commands
- [ ] publish to channel without being subscribed
- [ ] subscribe to private channels with token (JWT)
- [ ] connection token (JWT) refresh
- [ ] private channel subscription token (JWT) refresh
- [ ] handle connection expired error
- [ ] handle subscription expired error
- [ ] ping/pong to find broken connection
- [ ] message recovery mechanism for client-side subscriptions
- [ ] server-side subscriptions
- [ ] message recovery mechanism for server-side subscriptions
- [ ] history stream pagination
