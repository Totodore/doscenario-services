use sqlx::types::time::PrimitiveDateTime;
use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct BlueprintModel {
    
    pub id: i32,
    
    pub project_id: i32,
    
    pub created_by_id: Option<String>,
    
    pub last_editor_id: Option<String>,
    
    pub created_date: PrimitiveDateTime,
    
    pub last_editing: PrimitiveDateTime,
    
    pub uid: String,
    pub color: Option<String>,
    pub title: String,
}