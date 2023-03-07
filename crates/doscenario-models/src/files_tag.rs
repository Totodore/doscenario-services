use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct FilesTagModel {
    
    pub file_id: String,
    
    pub tag_id: i32,
}