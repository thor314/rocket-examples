# Demo: using reqwest to stream responses 
Demo of a client-server with Rocket and Reqwest, where the server exposes:
- an endpoint (events) for a client to subscribe to all messages passed into the message enpoint
- an endpoint (message) for a client to pass messages into

## Usage
```sh
# term1: mount server
cr --bin server
# term2: with curl, subscribe to events and pass messages in
curl http://127.0.0.1:8000/events 
curl -d "room=23&username=Al&message=Hi Bob" http://127.0.0.1:8000/message

# Or, using the reqwest client, which subscribes and passes messages every 5 seconds.
cr --bin client # this will pass messages
```
## Important to filter the crap!
Occasionally an empty message pipes from the stream. Filter messages like these:
```
Chunk: Ok(b"\n")
Chunk: Ok(b":\n")
```