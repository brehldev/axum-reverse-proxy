# Axum Reverse Proxy

This is a simple reverse proxy server built using the Axum web framework in Rust. It forwards incoming HTTP requests to a specified target server and returns the response back to the client.


## Try it out! 

You can try out the reverse proxy server by running the following command and proxying to httpbin.org:

```bash
cargo clean && cargo build && REVERSE_PROXY_TARGET_URL=https://httpbin.org cargo run
```

Then, you can make a request to the proxy server:

```bash
curl -i http://localhost:3000/get
```
