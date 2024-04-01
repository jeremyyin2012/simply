use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use mongodb::bson;
use mongodb::bson::oid::ObjectId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::error::Error;
use bson::serde_helpers::hex_string_as_object_id;


pub type UserId = String;
pub type UserName = String;
pub type CreatedAt = NaiveDateTime;
pub type CreatedBy = UserId;
pub type UpdatedAt = Option<NaiveDateTime>;
pub type UpdatedBy = Option<UserId>;


#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}


#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Context {
    pub user: Option<User>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            user: None,
        }
    }
    pub fn with_user(user: User) -> Self {
        Self {
            user: Some(user),
        }
    }
}

