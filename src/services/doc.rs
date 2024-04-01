use std::fmt::format;
use std::ops::Deref;
use anyhow::Context as AnyhowContext;
use chrono::{NaiveDateTime, Utc};
use log::debug;
use mongodb::bson;
use mongodb::bson::oid::ObjectId;
use redis::ToRedisArgs;
use serde_json::Value;
use crate::error::{Code, Error};
use crate::model::{Context, ZpData, ZPJob};
use crate::providers::Providers;

#[derive(Clone)]
pub struct DocService {
    ctx: Context,
    pvd: Providers,
}

impl DocService {
    pub fn new(context: Context, providers: Providers) -> Self {
        Self {
            ctx: context,
            pvd: providers,
        }
    }
}


impl DocService {
    pub async fn upload_doc(&self, collection: String, req: Value) -> Result<bool, Error> {
        let svc = self.clone();
        tokio::spawn(async move {
            let res = svc.pvd.doc().add_doc(collection, req).await;
            match res {
                Ok(res) => {debug!("inserted: {res} doc")},
                Err(err) => {debug!("inserted error: {err}")},
            }
        });
        Ok(true)
    }
}