use sqlx::{types::time::PrimitiveDateTime, FromRow};

#[derive(Clone, Debug, PartialEq, FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct SheetModel {
    
    pub id: i32,
    
	#[sqlx(default)]
    pub content: Option<String>,
    
    pub uid: String,
    pub color: Option<String>,
    
    pub created_date: PrimitiveDateTime,
    
    pub last_editing: PrimitiveDateTime,
    
    pub project_id: i32,
    pub title: String,
    
    pub document_id: i32,
    
    pub created_by_id: Option<String>,
    
    pub last_editor_id: Option<String>,
}