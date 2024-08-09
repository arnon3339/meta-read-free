
# Meta-Read-Free

**Meta-Read-Free** is a Rust project designed to generate a Python package for reading NFTs on the Solana blockchain. Utilizing the PyO3 library, this project bridges Rust's performance with Python's simplicity, making it easier for developers to interact with Solana NFTs in Python.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Tech Stack](#tech-stack)
- [Installation](#installation)
- [Contributing](#contributing)
- [License](#license)

## Overview

Meta-Read-Free is aimed at developers who want to integrate Solana NFT reading capabilities into their Python applications. This project leverages Rust for its speed and safety, while PyO3 allows seamless integration with Python, enabling developers to use the generated Python package to interact with Solana NFTs.

## Features

- **Rust-Powered**: Combines Rust's performance with Python's ease of use.
- **Solana Integration**: Provides functions to read NFTs directly from the Solana blockchain.
- **Python Package**: Generates a Python package that can be easily integrated into Python projects.

## Tech Stack

- **Language**: Rust
- **Library**: PyO3
- **Blockchain**: Solana

## Installation

### Prerequisites

- Rust (latest stable version)
- Python (version 3.10.0)
- Cargo

### Build the Project

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/arnon3339/meta-read-free.git
cd meta-read-free
cargo build --release
```

### Generate the Python Package

To generate the Python package, use the following command:

```bash
maturin develop
```

This command will build and install the package in your local Python environment.

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or improvements, please feel free to submit a Pull Request.

## License

This project is licensed under the [choose your license]. See the [LICENSE](LICENSE) file for more details.