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