// @generated automatically by Diesel CLI.

diesel::table! {
    vaults (id) {
        id -> Uuid,
        name -> Varchar,
        tags -> Nullable<Array<Nullable<Text>>>,
        iv -> Nullable<Varchar>,
        salt -> Nullable<Varchar>,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        setup_key -> Nullable<Varchar>,
    }
}
