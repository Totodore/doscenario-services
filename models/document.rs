use sqlx::types::time::PrimitiveDateTime;
use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct DocumentModel {
    
    pub id: i32,
    
	#[sqlx(default)]
    pub content: Option<String>,
    
    pub created_date: PrimitiveDateTime,
    
    pub project_id: i32,
    
    pub created_by_id: Option<String>,
    
    pub last_editor_id: Option<String>,
    pub title: String,
    
    pub last_editing: PrimitiveDateTime,
    
    pub uid: String,
    pub color: Option<String>,
}