use std::borrow::Borrow;
use std::env;
use std::error::Error;
use mongodb::bson::{self, doc};
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use mongodb::options::{ClientOptions, Collation, FindOneOptions, ResolverConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataBaseConfig {
    pub uri_id: String,
    pub database: String,
    pub collection: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Count {
    _id: ObjectId,
    count: isize
}

pub async fn count(config: &DataBaseConfig) -> Result<isize, Box<dyn Error>> {
    let collection = get_collection(config).await?;
    let first = collection.find_one(doc! {}, None).await?;
    let first = first.unwrap();
    let first_count = first.count;
    let count_documents = collection.count_documents(doc! {}, None).await?;
    let max_documents = env!("MAX_DOCUMENTS").parse().unwrap();
    let number = first.count + max_documents as isize;
    if count_documents > max_documents {
        let option = FindOneOptions::builder().skip(max_documents);
        let last = collection.find_one(doc! {}, option.build()).await?;
        let fid = first._id;
        let lid = last.unwrap()._id;
        collection.delete_many(doc! {"_id": {"$gt" : &fid, "$lte" : &lid}}, None).await;
        collection.update_one(
            doc! {"_id": &fid},
            doc! {"$set": { "count":  bson::to_bson(&number).expect("Unable to convert orders to bson")}},
            None
        ).await.expect("Unable to update orders of the turtle");
    }
    Ok(first_count + count_documents as isize - 1)
}

pub async fn add(config: &DataBaseConfig, add: isize) -> Result<(), Box<dyn Error>> {
    let collection = get_collection(config).await?;
    let new_one = Count {
        _id: Default::default(),
        count: add
    };
    collection.insert_one( new_one.borrow(), None).await;
    Ok(())
}

pub async fn get_collection(config: &DataBaseConfig) -> Result<Collection<Count>, Box<dyn Error>> {
    let uri = env::var(&config.uri_id).unwrap();
    let options =
        ClientOptions::parse_with_resolver_config(uri, ResolverConfig::cloudflare())
            .await?;
    let client = mongodb::Client::with_options(options)?;
    let db = client.database(config.database.as_str());
    let collection = db.collection::<Count>(config.collection.as_str());
    Ok(collection)
}