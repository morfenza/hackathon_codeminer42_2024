use crate::database::establish_connection;
use crate::models::vault::Vault;
use diesel::prelude::*;
use actix_web::Error;
use crate::schema::vaults;
use uuid::Uuid;

pub fn create_vault(
    name: String,
    tags: Option<Vec<String>>,
    iv: Option<String>,
    salt: Option<String>,
    username: Option<String>,
    password: Option<String>,
    setup_key: Option<String>,
) -> Result<Uuid, Error> {

    let mut conn = establish_connection();

    let new_vault = Vault {
        id: Default::default(), // TODO: DB will generate this UUID or Us? aaaaaa
        name,
        tags,
        iv: iv.unwrap_or_default(),
        salt: salt.unwrap_or_default(),
        username: username.unwrap_or_default(),
        password: password.unwrap_or_default(),
        setup_key: setup_key.unwrap_or_default(),
    };

    diesel::insert_into(vaults::table)
    .values(&new_vault)
    .execute(&mut conn);

    Ok(new_vault.id)
}
