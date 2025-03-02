use futures_util::TryStreamExt;
use mongodb::{bson::Document, error::Error, options::ClientOptions, Client, Collection};
use once_cell::sync::Lazy;
use std::sync::Mutex;
pub mod cache;
pub mod users;

pub async fn connection() -> Result<(Collection<Document>, Vec<Document>), Error> {
    let mongodb_uri = "mongodb+srv://waneexw:V1zQmtHOLhHonEwO@main.3thc3.mongodb.net/?retryWrites=true&w=majority&appName=main";
    let client_options = ClientOptions::parse(mongodb_uri).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("Pattern_Of_Doom");
    let collection = database.collection("Accounts");
    let collection_chat: Collection<Document> = database.collection("Chat");

    let mut cache = cache::GLOBAL_CACHE.lock().await;
    cache.set("Chat".to_string(), collection_chat);
    drop(cache);

    let mut global_database = cache::GLOBAL_DATABASE.lock().await;
    if let Some(database) = global_database.clone() {
    } else {
        *global_database = Some(database);
    }
    drop(global_database);
    let mut docs: Vec<Document> = Vec::new();
    let mut cursor = collection.find(None, None).await?;
    while let Some(doc) = cursor.try_next().await? {
        docs.push(doc);
    }
    Ok((collection, docs))
}

pub async fn get_chats() -> Option<Vec<Document>> {
    let cache = cache::GLOBAL_CACHE.lock().await;
    if !cache.is_empty() {
        let collection = cache.get_collection("Chat".to_string());
        if let Some(col) = collection {
            let mut docs: Vec<Document> = Vec::new();

            let mut cursor = match col.find(None, None).await {
                Ok(cursor) => cursor,
                Err(e) => {
                    eprintln!("Error querying collection: {}", e);
                    return None;
                }
            };

            while let Ok(Some(doc)) = cursor.try_next().await {
                docs.push(doc);
            }

            return Some(docs);
        } else {
            return None;
        }
    } else {
        return None;
    }
}
pub async fn get_all_docs() -> Option<Vec<Document>> {
    let cache = cache::GLOBAL_CACHE.lock().await;
    if !cache.is_empty() {
        let collection = cache.get_collection("UserAccounts".to_string());
        if let Some(col) = collection {
            let collection = col;
            let mut docs: Vec<Document> = Vec::new();

            let mut cursor = match collection.find(None, None).await {
                Ok(cursor) => cursor,
                Err(e) => {
                    eprintln!("Error querying collection: {}", e);
                    return None;
                }
            };

            while let Ok(Some(doc)) = cursor.try_next().await {
                docs.push(doc);
            }

            return Some(docs);
        } else {
            return None;
        }
    } else {
        println!("cache empty");
        return None;
    }
}

pub async fn get_connection() -> Result<(Collection<Document>, Vec<Document>), Error> {
    let (collection, docs) = connection().await?;
    let mut cache = cache::GLOBAL_CACHE.lock().await;
    cache.set("UserAccounts".to_string(), collection.clone());
    drop(cache);
    Ok((collection, docs))
}
