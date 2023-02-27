use sqlx::types::time::PrimitiveDateTime;
use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct NodeModel {
    
    pub id: i32,
    
    pub is_root: i8,
    
    pub content: Option<String>,
    
    pub blueprint_id: Option<i32>,
    
    pub created_by_id: Option<String>,
    
    pub last_editor_id: Option<String>,
    pub x: i32,
    pub y: i32,
    pub locked: i8,
    
    pub summary: Option<String>,
    
    pub created_date: PrimitiveDateTime,
    
    pub last_editing: PrimitiveDateTime,
    pub color: Option<String>,
}