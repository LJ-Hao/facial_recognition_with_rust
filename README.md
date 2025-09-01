# Facial Recognition with Rust

This project is a facial recognition system implemented in Rust. It provides a robust and efficient solution for detecting and recognizing faces in images.

## Features

- Face detection in images
- Face recognition and matching
- High-performance implementation using Rust
- Docker support for easy deployment
- CI/CD pipeline for automated testing and deployment

## Project Structure

```
src/
├── main.rs                 # Entry point for CLI application
├── lib.rs                  # Library crate root, exposes public API
├── models/
│   ├── mod.rs              # Models module declaration
│   ├── face.rs             # Face data structure and methods
│   └── detection.rs        # Face detection result structures
├── processors/
│   ├── mod.rs              # Processors module declaration
│   ├── image_loader.rs     # Handles loading and basic preprocessing of images
│   └── face_detector.rs    # Core logic for detecting faces in images
├── utils/
│   ├── mod.rs              # Utilities module declaration
│   └── helpers.rs          # Helper functions used across the crate
└── cli/
    ├── mod.rs              # CLI module declaration
    └── app.rs              # CLI application setup and argument parsing
```

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager and build tool)

### Building the Project

To build the project, run:

```bash
cargo build
```

To build with optimizations, run:

```bash
cargo build --release
```

### Running Tests

To run the tests, use:

```bash
cargo test
```

### Running the Application

To run the application, use:

```bash
cargo run -- [arguments]
```

For example:

```bash
cargo run -- --input path/to/image.jpg
```

### Using Docker

To build the Docker image, run:

```bash
docker build -t facial_recognition .
```

To run the application in a Docker container, use:

```bash
docker run --rm -v /path/to/images:/images facial_recognition --input /images/input.jpg
```

## CI/CD

This project uses GitHub Actions for continuous integration and deployment. The pipeline includes:

- Building and testing on multiple platforms
- Code formatting and linting checks
- Docker image building and publishing

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.