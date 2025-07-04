use once_cell::sync::OnceCell;
use sea_orm::DatabaseConnection;
pub mod entities;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::new();

pub fn get_db() -> &'static DatabaseConnection {
    DB.get().unwrap()
}
