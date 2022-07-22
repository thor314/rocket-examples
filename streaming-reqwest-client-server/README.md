# Demo: using reqwest to stream responses 

## Usage
```sh
# term1: mount server
cr --bin server
# term2: subscribe to events with curl or reqwest client:
curl http://127.0.0.1:8000/events # or
cr --bin client

# term3: post messages
curl -d "room=23&username=Al&message=Hi Bob" http://127.0.0.1:8000/message
```

## Usage: Server and Reqwest
just like before, but now use `reqwest`'s `bytes_stream` on term2
```sh
# term1: mount server
cr --bin server
# term2: subscribe to events
curl http://127.0.0.1:8000/events
# term3: post messages
curl -d "room=23&username=Al&message=Hi Bob" http://127.0.0.1:8000/message
```

## Important to filter the crap!
Occasionally an empty message pipes from the stream. Filter messages like these:

Chunk: Ok(b"\n")
Chunk: Ok(b":\n")