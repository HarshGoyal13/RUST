use crate::database;

pub mod models;

pub fn login(cred : models::Credentials){
    database::get_user(cred);
}
