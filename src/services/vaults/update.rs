use crate::database::establish_connection;
use crate::schema::vaults;
use actix_web::Error;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(AsChangeset, Deserialize, Serialize)]
#[table_name = "vaults"]
pub struct VaultChangeset {
    pub name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub iv: Option<String>,
    pub salt: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub setup_key: Option<String>,
}

pub fn update_vault(
    vault_id: Uuid,
    name: Option<String>,
    tags: Option<Vec<String>>,
    iv: Option<String>,
    salt: Option<String>,
    username: Option<String>,
    password: Option<String>,
    setup_key: Option<String>,
) -> Result<Uuid, Error> {
    let mut conn = establish_connection();

    let changeset = VaultChangeset {
        name: if let Some(name) = name {
            if !name.is_empty() {
                Some(name)
            } else {
                None
            }
        } else {
            None
        },
        tags: if let Some(tags) = tags {
            if !tags.is_empty() {
                Some(tags)
            } else {
                None
            }
        } else {
            None
        },
        iv: if let Some(iv) = iv {
            if !iv.is_empty() {
                Some(iv)
            } else {
                None
            }
        } else {
            None
        },
        salt: if let Some(salt) = salt {
            if !salt.is_empty() {
                Some(salt)
            } else {
                None
            }
        } else {
            None
        },
        username: if let Some(username) = username {
            if !username.is_empty() {
                Some(username)
            } else {
                None
            }
        } else {
            None
        },
        password: if let Some(password) = password {
            if !password.is_empty() {
                Some(password)
            } else {
                None
            }
        } else {
            None
        },
        setup_key: if let Some(setup_key) = setup_key {
            if !setup_key.is_empty() {
                Some(setup_key)
            } else {
                None
            }
        } else {
            None
        },
    };

    diesel::update(vaults::table)
        .filter(vaults::id.eq(vault_id))
        .set(&changeset)
        .execute(&mut conn);

    Ok(vault_id)
}
