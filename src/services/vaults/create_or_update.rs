use crate::services::vaults::{create::create_vault, update::update_vault};
use actix_web::Error;
use uuid::Uuid;

pub fn create_or_update_vault(
    vault_id: Option<Uuid>,
    name: String,
    tags: Option<Vec<String>>,
    iv: Option<String>,
    salt: Option<String>,
    username: Option<String>,
    password: Option<String>,
    setup_key: Option<String>,
) -> Result<Uuid, Error> {
    if let Some(vault_id) = vault_id {
        update_vault(vault_id, Some(name), tags, iv, salt, username, password, setup_key)
    } else {
        create_vault(name, tags, iv, salt, username, password, setup_key)
    }
}
