use mongodb::{Client, Database, Collection};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerPhoto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub customer_name: String,
    pub photo_data: Vec<u8>, // Store actual image data
    pub created_at: DateTime<Utc>,
}

impl CustomerPhoto {
    pub fn new(customer_name: String, photo_data: Vec<u8>) -> Self {
        Self {
            id: None,
            customer_name,
            photo_data,
            created_at: Utc::now(),
        }
    }
}

pub struct PhotoDatabase {
    collection: Collection<CustomerPhoto>,
}

impl PhotoDatabase {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        // Get MongoDB connection string from environment or use default
        let mongodb_uri = env::var("MONGODB_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        
        let client = Client::with_uri_str(&mongodb_uri).await?;
        let database: Database = client.database("facial_recognition");
        let collection: Collection<CustomerPhoto> = database.collection("customer_photos");
        
        Ok(PhotoDatabase { collection })
    }
    
    pub async fn save_customer_photo(&self, photo: CustomerPhoto) -> Result<(), mongodb::error::Error> {
        self.collection.insert_one(photo, None).await?;
        Ok(())
    }
    
    pub async fn get_customer_photos(&self, customer_name: &str) -> Result<Vec<CustomerPhoto>, mongodb::error::Error> {
        use mongodb::bson::{doc, Document};
        use tokio_stream::StreamExt;
        
        let filter = doc! { "customer_name": customer_name };
        let mut cursor = self.collection.find(filter, None).await?;
        
        let mut photos = Vec::new();
        while let Some(photo) = cursor.next().await {
            photos.push(photo?);
        }
        
        Ok(photos)
    }
    
    pub async fn get_all_photos(&self) -> Result<Vec<CustomerPhoto>, mongodb::error::Error> {
        use tokio_stream::StreamExt;
        
        let mut cursor = self.collection.find(None, None).await?;
        
        let mut photos = Vec::new();
        while let Some(photo) = cursor.next().await {
            photos.push(photo?);
        }
        
        Ok(photos)
    }
}