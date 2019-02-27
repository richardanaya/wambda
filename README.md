# wambda

Write HTTP handlers that can run anywhere.
* Plain old servers
* AWS Lambda
* Embedded
* Edge networks
* Binary protocols (protobuffs,websockets,etc.)

# A simple spec

To create a HTTP handler, your web assembly module simply needs to adhere to a simple spec:

```C
// For making data requests from host to outside world
* data - pointer in memory to json strong for request
* returns a handle that will be used when calling back
extern int request(void* target, void* data);

// Called when request is complete
// * requestID - the ID that was returned when request was called 
// * data - a pointer in memory to json string of response data
void response(int requestID, void* data){

}

// Send data to host to deliver response
// * data - pointer in memory to json string of response
extern void send(void* data);

// Called when served
// * event - pointer in memory to json string of event
// * context - pointer in memory to json string of context in which this server is run ( host specific )
void execute(void* data, void* context) {
  send(&"{\"status\":200, \"body\":\"hello world!\"")
}

// Called when the host needs to allocate memory to send to web assembly module
// len - length in bytes needed by host
void* malloc(int len){
  ...
}
```

# Define an HTTP api
Writing a swagger will allow routing and input validation to the appropriate web assembly module

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
wambda run -p 8080
```

# Run Anywhere

## Server
```rust
use wambda;

fn main() -> () {
  let w = wambda.load("hello.swagger");
  w.runHTTPServer()
}
```

## AWS Lambda
```js
const wambda = require("wambda")
const w = wambda.load("hello.swagger");
async function handler(event,ctx){
  return await w.handleAPIGateway(event,ctx)
}
```

## Custom 
```js
const wambda = require("wambda")
const w = wambda.load("hello.swagger");
...
let response = w.run({path:"/hello",method:"GET"},{env:"my custom host"});
...
```
