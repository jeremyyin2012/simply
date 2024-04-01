use std::cmp::max;
use std::str::FromStr;
use std::time::Duration;
use anyhow::Context;
use chrono::{DateTime, NaiveDateTime, NaiveTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::{DateTime as BsonDateTime, doc, to_document};
use mongodb::bson::oid::ObjectId;
use mongodb::options::{FindOneOptions, FindOptions};
use serde_json::Value;
use crate::error::Error;

use crate::store::api_client::ApiClients;
use crate::store::cache::Caches;
use crate::store::database::Databases;
use crate::store::Store;

#[derive(Clone)]
pub struct DocProvider {
    store: Store,
    db: Databases,
    cache: Caches,
    api: ApiClients,
}


impl DocProvider {
    pub fn new(store: Store) -> Self {
        Self {
            store: store.clone(),
            db: store.databases.clone(),
            cache: store.caches.clone(),
            api: store.api_clients.clone(),
        }
    }
}

impl DocProvider {

    pub async fn add_doc(&self, collection: String, req: Value) -> Result<usize, Error> {
        let res = self.db.doc(collection).insert_one(req, None).await
            .with_context(|| "insert_one".to_string())?;
        debug!("inserted: {:?}", res);
        Ok(res.inserted_id.to_string().len())
    }
}
