use sqlx::types::time::PrimitiveDateTime;

pub struct ImageModel {
    
    pub id: String,
    pub size: i32,
    pub height: i32,
    pub width: i32,
    
    pub project_id: Option<i32>,
    
    pub added_by_id: Option<String>,
    
    pub last_editing: PrimitiveDateTime,
    
    pub uploaded_date: PrimitiveDateTime,
}