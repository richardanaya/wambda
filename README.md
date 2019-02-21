# wambda

Write lambda handlers that can run anywhere.
* Plain old servers
* AWS Lambda
* Embedded
* Edge networks
* Protobuffs/Flatbuffers

# A simple spec

To create a lambda, your web assembly module simply needs to adhere to a simple spec:

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

// Called when the host needs to allocate memory to send to module
// len - length in bytes needed by host
int malloc(int len){
  ...
}
```

# Define an HTTP
```swagger
swagger: "2.0"
info:
  description: "A minimal API that says hello world"
  version: "1.0.0"
  title: "Swagger HelloWorld"
paths:
  /hello:
    get:
      x-wasm-lambda: "hello.wasm"
      responses:
        200:
          description: "Returns hello world"
```

# Run a local server

```
wambda run
```

# Run Anywhere

## Server
```rust
use wambda;

fn main() -> () {
  let w = wambda.load("hello.swagger");
  w.runHTTPServer("hello.swagger")
}
```

## Lambda
```js
const wambda = require("wambda")
const w = wambda.load("hello.swagger");
async function handler(event,ctx){
  return await w.handleAPIGateway(event,ctx)
}
```
