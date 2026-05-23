# WebServer 1

This is a basic version of a Simple Webserver and a simulation built around it. 
Users can send multiple requests to this server, there are multiple senders, 
but there is only a single receiver. This receiver is a thread-safe component. 

## How to build

```bash
cargo build
```

## How to run 

```bash
cargo run
```

## How to use

Go to the browser, use following URLs

1. Base Route: http://127.0.0.1:7878/
Once the program is runnning when you go to this URL,
you can see a worker has started and handled the request.
This shows how a request is handled. The browser acting
as a client sends a request and the receiving server here
is our machine itself. Our server is listening to the TCP 
port 7878, and once a request comes that way, it is handled
by a worker, and the content is sent. Luckily as our client 
is a browser and also what we send is HTML, thus you can see
a nice web page. 

2. http://127.0.0.1:7878/sleep

This is an interesting case, some of the URLs may take time to load
and as a server, it cannot stand there without doing anything as it 
receives many other requests. To demonstrate that, we do a simple demo
here, where it pauses for a while and render the outcome. What happens
under the hood is that the server allocates a separate thread to watch 
for this and continue. So the program never get stuck unless all resources
consumed. 

3. http://127.0.0.1:7878/<any_route>

This is basically covering the error handling component. There are so many 
errors that can occur in a program. In this program we only handle Thread related
issues that could arise and specifically, when an unknown URL is requested. 
We handle it by looking into anything else other than `/` or `/sleep`, we render
a default `404` page. 

## Future Work

Thinking about re-writing this using `Future` trait. 