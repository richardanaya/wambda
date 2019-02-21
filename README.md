# wambda

Write lambda handlers that can run anywhere.
* Plain old servers
* AWS Lambda
* Embedded
* Edge networks
* Protobuffs/Flatbuffers

# A simple spec

```C
// For making HTTP requests to outside world
* request - pointer in memory to json string of request
* returns a handle that will be used when calling back
extern int httpRequest(int request);

// Called when HTTP request is complete
// * requestID - the ID that was returned when httpRequest was called 
// * response - a pointer in memory to json string of response
void httpResponse(int requestID, int response){

}

// Respond to server to deliver response
// * response - pointer in memory to json string of response
extern void callback(int response);

// Called when served
// * event - pointer in memory to json string of event
// * context - pointer in memory to json string of context in which this server is run ( host specific )
void execute(int event, int context) {
  callback(&"{\"status\":200, \"body\":\"hello world!\"")
}
```

# Define an HTTP
```swagger
swagger: "2.0"
info:
  description: "this is my api"
  version: "1.0.0"
  title: "Swagger Petstore"
paths:
  /hello:
    get:
      x-wasm-lambda: "hello.wasm"
      responses:
        200:
          description: "Invalid input"
```

# Run a local server

```
wambda run
```

# Run Anywhere

```rust
use wambda;

fn main() -> () {
  wambda.run("hello.swagger")
}
```
