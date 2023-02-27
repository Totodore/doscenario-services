use sqlx::{types::time::PrimitiveDateTime, FromRow};

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Project {
    
    pub id: i32,
    pub name: String,
    
    pub created_by_id: Option<String>,
    
    pub created_date: PrimitiveDateTime,
    pub uid: String,
}