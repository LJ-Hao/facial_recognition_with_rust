pub mod database;
pub mod face_recognition;
pub mod photo_db;
pub mod monitor;
pub mod opencv_wrapper;

pub use database::{FaceDatabase, FaceRecord};
pub use face_recognition::DeepFaceRecognizer;
pub use photo_db::{PhotoDatabase, CustomerPhoto};
pub use monitor::{DatabaseMonitor, RecognitionResponse};
pub use opencv_wrapper::SimpleFaceDetector;