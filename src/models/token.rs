use std::time::Duration;

use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    options::IndexOptions,
    Collection, Database, IndexModel,
};
use futures::TryStreamExt;
use crate::{
    error::{self, Result},
    helpers::security::generate_token,
};

use super::user::User;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub value: String,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Token {
    pub fn collection(db: &Database) -> Collection<Self> {
        db.collection::<Self>("tokens")
    }

    pub async fn create_indexes(db: &Database) {
        // TTL on "createdAt"
        let options = IndexOptions::builder()
            .expire_after(Duration::from_secs(7 * 24 * 60 * 60))
            .build();
        let index = IndexModel::builder()
            .keys(doc! { "createdAt": 1 })
            .options(options)
            .build();

        match Self::collection(&db).create_index(index, None).await {
            Err(err) => match *err.kind {
                mongodb::error::ErrorKind::Command(cmd_err)
                    if cmd_err.code_name == "IndexOptionsConflict" =>
                {
                    println!("Token: Index already exists");
                }
                _ => panic!("{:?}", err),
            },
            Ok(_) => (),
        }
    }

    pub async fn create(db: &Database, user_id: ObjectId) -> Result<Self> {
        let token = Token {
            id: ObjectId::new(),
            user_id,
            value: generate_token(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };

        Self::collection(db)
            .insert_one(token.clone(), None)
            .await
            .map_err(error::from_mongodb_error)?;

        Ok(token)
    }

    pub async fn find_by_value(db: &Database, value: &str) -> Result<Option<Self>> {
        let find = doc! { "value": value };

        Ok(Self::collection(db)
            .find_one(find, None)
            .await
            .map_err(error::from_mongodb_error)?)
    }

    pub async fn find_all_by_user_id(db: &Database, user_id: ObjectId) -> Result<Vec<Self>> {
        let find = doc! { "userId": user_id };

        let mut cursor = Self::collection(db)
            .find(find, None)
            .await
            .map_err(error::from_mongodb_error)?;

        let mut documents = vec![];

        while let Some(doc) = cursor.try_next().await.map_err(error::from_mongodb_error)? {
            documents.push(doc);
        }

        Ok(documents)
    }

    pub async fn delete(&self, db: &Database) -> Result<()> {
        let find = doc! { "_id": self.id };

        Self::collection(db)
            .delete_one(find, None)
            .await
            .map_err(error::from_mongodb_error)?;

        Ok(())
    }

    pub async fn user(&self, db: &Database) -> Result<Option<User>> {
        User::find_by_id(db, self.user_id).await
    }
}
