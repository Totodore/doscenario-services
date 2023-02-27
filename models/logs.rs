use sqlx::types::time::PrimitiveDateTime;
use sqlx::FromRow;


#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct LogsModel {
    
    pub id: i32,
    
    pub message: String,
    
    pub logs: String,
    pub solved: i8,
    
    pub created_date: PrimitiveDateTime,
    
    pub user_id: Option<String>,
}