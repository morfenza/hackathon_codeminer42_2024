use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::vaults;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "vaults"]
pub struct Vault {
    pub id: Uuid,
    pub name: String,
    pub tags: Option<Vec<String>>,
    pub iv: String,
    pub salt: String,
    pub username: String,
    pub password: String,
    pub setup_key: String,
}

impl Vault {
    pub fn new(
        name: String,
        tags: Option<Vec<String>>,
        iv: String,
        salt: String,
        username: String,
        password: String,
        setup_key: String,
    ) -> Self {
        Vault {
            id: Uuid::new_v4(),
            name,
            tags,
            iv,
            salt,
            username,
            password,
            setup_key,
        }
    }
}
