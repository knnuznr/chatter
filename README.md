# Chatter

Chatter is a real-time chat application built with Rust. It allows multiple clients to connect to a server and chat with each other in real-time.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Installation

Follow these steps to install and run Chatter.

### Prerequisites

Ensure you have Rust and Cargo installed. You can install Rust using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installing

Clone the repository and build the project:

```bash
# Clone the repo
git clone https://github.com/knnuznr/chatter.git

# Navigate into the project directory
cd chatter

# Build the project
cargo build --release
```

### Usage
Open a new terminal and run this command to connect a client to the server:

```bash
telnet 127.0.0.1 7007
```
After running the server, client(s) can connect and start chatting by typing messages and pressing Enter. Messages from client(s) are displayed in real time.
## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.
