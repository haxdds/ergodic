# Ergodic

I'm learning Rust and so I decided to work on this for fun.

A high-performance exchange built with Rust, featuring a multi-threaded architecture and HTTP API.

## Overview

Ergodic is an exchange that processes orders and executes trades using a multi-threaded architecture. It's built with Rust for performance and reliability, using modern async/await patterns and channels for inter-thread communication.

## Features

- Multi-threaded architecture for high performance
- HTTP API for order submission and trade monitoring
- In-memory order book and matching engine
- Crossbeam channels for efficient inter-thread communication
- Built with Tokio for async runtime
- Axum for HTTP server implementation

## Architecture

The project is organized into several modules:

- `core.rs`: Core trading logic, order book, and matching engine
- `engine.rs`: Main trading engine that processes orders and executes trades
- `api.rs`: HTTP API endpoints for order submission and trade monitoring
- `main.rs`: Application entry point and thread orchestration

## Dependencies

- tokio: Async runtime
- axum: HTTP server framework
- serde: Serialization/deserialization
- crossbeam: Thread-safe channels and synchronization primitives

## Getting Started

### Prerequisites

- Rust 2024 edition
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/ergodic.git
cd ergodic
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
cargo run --release
```

The server will start on `0.0.0.0:8080`.

## API Endpoints

### Submit Order
```
POST /order
Content-Type: application/json

{
    "symbol": "BTC/USD",
    "side": "buy",
    "price": 50000.0,
    "quantity": 1.0
}
```

### Get Trades
```
GET /trades
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running in Debug Mode

```bash
cargo run
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with Rust's excellent concurrency primitives
- Inspired by modern trading system architectures 
