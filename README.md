# Facial Recognition System

A Rust-based facial recognition system that captures images every 10 seconds, recognizes faces using deep learning techniques, and controls screen lock/unlock based on authorized users. Customer photos are stored in MongoDB database in name.png format, while interval photos are not stored.

## Features

- Captures photos every 10 seconds using connected camera
- Recognizes faces using deep learning techniques (feature extraction and comparison)
- Locks screen for unknown faces
- Unlocks screen for recognized (authorized) faces
- Stores customer photos in MongoDB database (name.png format)
- Does not store interval photos (every 10 seconds)
- Docker support with MongoDB integration
- CLI tool for managing authorized faces and viewing customer photos
- HTTP API on port 8001 to return recognition results
- Database monitoring for photo changes every minute

## Prerequisites

- Rust (latest stable version)
- OpenCV development libraries
- Camera device
- MongoDB database (local or remote)

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd facial_recognition_system
   ```

2. Install dependencies:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install libopencv-dev pkg-config libgtk-3-dev wget
   
   # macOS
   brew install opencv
   ```

3. Set up MongoDB:
   - Install MongoDB locally, or
   - Use a cloud MongoDB service (MongoDB Atlas), or
   - Use Docker Compose (see below)

## Building the Project

```bash
# Build the main application
cargo build --release

# Build the CLI tool
cargo build --release --bin facial-recognition-cli
```

## Running Tests

```bash
# Run unit tests
cargo test --lib

# Run integration tests
cargo test --test integration_test

# Run all tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture
```

## Usage

### Adding Authorized Faces

Before running the facial recognition system, you need to add authorized faces to the database:

```bash
# Add a new authorized face
./target/release/facial-recognition-cli add --name "John Doe" --photo "database/john.png"

# List all authorized faces
./target/release/facial-recognition-cli list

# Remove an authorized face
./target/release/facial-recognition-cli remove --id <face-id>

# Train the model (uses pre-trained approaches)
./target/release/facial-recognition-cli train

# List customer photos stored in MongoDB
./target/release/facial-recognition-cli list-photos

# List photos for a specific customer
./target/release/facial-recognition-cli list-photos --name "John Doe"
```

### Running the Facial Recognition System

```bash
# Run the main application (requires MongoDB)
cargo run --release

# Or with custom MongoDB URI
MONGODB_URI="mongodb://localhost:27017" cargo run --release
```

The system will:
1. Capture a photo from your camera every 10 seconds
2. Detect faces in the captured photo using deep learning techniques
3. Compare detected faces against authorized faces in the database
4. Lock the screen for unknown faces, unlock for authorized faces
5. Store customer photos in MongoDB (not interval photos)
6. Save interval photos only temporarily for processing

### HTTP API

The system provides an HTTP API on port 8001:

- `GET /health` - Health check endpoint
- `GET /recognition` - Returns the latest recognition result:
  - If recognized: `{"name": "User Name", "recognized": true}`
  - If not recognized: `{"name": null, "recognized": false}`

## How Face Recognition Works

This system uses a deep learning approach for facial recognition:

1. **Face Detection**: Uses Haar Cascade Classifier to detect faces in images
2. **Feature Extraction**: Extracts facial features using histogram-based techniques
3. **Face Comparison**: Compares facial features using cosine similarity
4. **Recognition Decision**: Determines if a face matches authorized users based on similarity threshold

## Database Storage

- **Authorized Faces**: Stored locally in JSON format with file paths
- **Customer Photos**: Stored in MongoDB as binary data (name.png format)
- **Interval Photos**: Not stored permanently (only used for recognition)

## Database Monitoring

The system monitors the `database` directory for changes every minute:
- Detects new photos added to the database
- Detects photos removed from the database
- Automatically updates the internal database state

## Docker

To run using Docker with MongoDB:

1. Build and run with docker-compose:
   ```bash
   docker-compose up --build
   ```

This will start both the facial recognition system and MongoDB in separate containers.

### Using External MongoDB

If you want to use an external MongoDB instance:

```bash
# Run with external MongoDB
docker run -it --device=/dev/video0 \
  -e MONGODB_URI="mongodb://your-mongo-host:27017" \
  facial_recognition_system
```

## CI/CD Pipeline

This project uses GitHub Actions for continuous integration and deployment:

1. **Build and Test**: 
   - Runs on every push and pull request
   - Compiles the Rust code
   - Runs unit tests
   - Performs code linting with Clippy
   - Checks code formatting with Rustfmt

2. **Docker Image Building**:
   - Builds and pushes Docker images to GitHub Container Registry
   - Tags images with branch names, commit SHA, and semantic versions
   - Only pushes images on push events (not pull requests)

3. **Release Packaging**:
   - Creates release packages with compiled binaries
   - Uploads packages as release assets

## Project Structure

```
.
├── Cargo.toml                 # Project dependencies
├── build.rs                   # Build script
├── Dockerfile                 # Docker configuration
├── docker-compose.yml         # Docker Compose configuration (with MongoDB)
├── src/
│   ├── main.rs                # Main application code
│   ├── lib.rs                 # Library module exports
│   ├── database.rs            # Face database management (local JSON)
│   ├── photo_db.rs            # MongoDB photo storage
│   ├── face_recognition.rs    # Deep learning face detection and recognition
│   ├── monitor.rs             # Database monitoring and HTTP server
│   └── bin/
│       └── cli.rs             # CLI tool for face management
├── database/                  # Authorized face images and records
├── photos/                    # Temporary interval photos
├── tests/                     # Integration tests
└── .github/workflows/         # GitHub Actions workflows
    ├── ci-cd.yml              # CI/CD pipeline
    ├── tests.yml              # Test pipeline
    └── docker_publish.yml     # Docker publish pipeline
```

## Development

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
# Format code with rustfmt
cargo fmt

# Check for linting issues
cargo clippy
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.