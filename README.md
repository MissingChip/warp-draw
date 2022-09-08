# Warp-Draw

A simple polygon drawing protocol I made to learn about Warp (Rust web framework) and WebSockets. Turns out they're both really easy to use!

To run:

```sh
# run the project
cargo run
```

Go to `localhost:8080` in a browser.

You should see some stuff! That stuff was specified by the server and sent over a WebSocket to your browser, and recieved/displayed on an HTML canvas using JavaScript.
