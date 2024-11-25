use crate::services::vaults::create_or_update::create_or_update_vault;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct VaultData {
    name: String,
    tags: Option<Vec<String>>,
    iv: Option<String>,
    salt: Option<String>,
    username: Option<String>,
    password: Option<String>,
    setup_key: Option<String>,
}

#[derive(Serialize)]
pub struct VaultResponse {
    uuid: String,
}

pub async fn upsert_vault_handler(
    vault_id: web::Path<Option<String>>,
    vault_data: web::Json<VaultData>,
) -> impl Responder {
    let vault_id = match vault_id.into_inner() {
        Some(id) => Some(Uuid::parse_str(&id).unwrap()),
        None => None,
    };
    let vault = vault_data.into_inner();

    match create_or_update_vault(
        vault_id,
        vault.name,
        vault.tags,
        vault.iv,
        vault.salt,
        vault.username,
        vault.password,
        vault.setup_key,
    ) {
        Ok(uuid) => HttpResponse::Ok().json(VaultResponse { uuid: uuid.to_string() }), 
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
