use sqlx::types::time::PrimitiveDateTime;
use sqlx::FromRow;


#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct FileModel {
    
    pub id: String,
    pub mime: String,
    pub path: String,
    pub size: i32,
    
    pub project_id: Option<i32>,
    
    pub created_by_id: Option<String>,
    
    pub last_editing: PrimitiveDateTime,
    
    pub created_date: PrimitiveDateTime,
}