# Naive Serve

This is a tiny web server designed for laboratory testing purposes.

It provides a minimal setup with two routes:
- `/`: Returns a 200 OK response
- `/crash`: Returns a 200 OK response and then shuts down the server

This server is intended for controlled testing environments and should not be used in production.

## Usage

To run the server:

```bash
cargo run
```

The server will start and listen on `http://127.0.0.1:8080`.




