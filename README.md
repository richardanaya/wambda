# wambda

Write lambda handlers that can run anywhere.
* Plain old servers
* AWS Lambda
* Embedded
* Edge networks

# A simple spec

```C
// For making HTTP requests to outside world
* request - pointer in memory to json string of request
* returns a handle that will be used when calling back
extern int httpRequest(int request);

// For receiving http
// * the handle that was used when httpRequest was called 
void httpResponse(int requestID, int response){

}

// Respond to server to deliver response
// * response - pointer in memory to json string of response
extern void callback(int response);

// Called when served
// * event - pointer in memory to json string of event
// * context - pointer in memory to json string of context in which this server is run ( host specific )
void execute(int event, int context) {

}
```
