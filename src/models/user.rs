use mongodb::{
    bson::{
        doc, oid::ObjectId, DateTime,
    }, options::IndexOptions, Collection, Database, IndexModel
};

use crate::error::{self, Result};

use super::token::Token;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub id: ObjectId,
    pub email: String,
    pub password: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl User {
    pub fn collection(db: &Database) -> Collection<Self> {
        db.collection::<Self>("users")
    }

    pub async fn create_indexes(db: &Database) {
        // Unique on "Email"
        let options = IndexOptions::builder()
            .unique(true)
            .build();
        let index = IndexModel::builder()
            .keys(doc! { "email": 1 })
            .options(options)
            .build();

        match Self::collection(&db).create_index(index, None).await {
            Err(err) => match *err.kind {
                mongodb::error::ErrorKind::Command(cmd_err)
                    if cmd_err.code_name == "IndexOptionsConflict" =>
                {
                    println!("User: Index already exists");
                }
                _ => panic!("{:?}", err),
            },
            Ok(_) => (),
        }
    }

    pub async fn create(db: &Database, email: String, hashed_password: String) -> Result<Self> {
        let user = Self {
            id: ObjectId::new(),
            email: email.to_string(),
            password: hashed_password.to_string(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };

        Self::collection(db).insert_one(user.clone(), None)
            .await
            .map_err(error::from_mongodb_error)?;

        Ok(user)
    }

    pub async fn find_by_email(db: &Database, email: &str) -> Result<Option<Self>> {
        let find = doc! { "email": email };

        Ok(Self::collection(db).find_one(find, None)
            .await
            .map_err(error::from_mongodb_error)?)
    }

    pub async fn find_by_id(db: &Database, id: ObjectId) -> Result<Option<Self>> {
        let find = doc! { "_id": id };

        Ok(Self::collection(db).find_one(find, None)
            .await
            .map_err(error::from_mongodb_error)?)
    }

    pub async fn create_token(&self, db: &Database) -> Result<Token> {
        Token::create(db, self.id).await
    }

    pub async fn tokens(&self, db: &Database) -> Result<Vec<Token>> {
        Token::find_all_by_user_id(db, self.id).await
    }
}
