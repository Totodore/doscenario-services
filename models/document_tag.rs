use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct DocumentTagModel {
    pub document_id: i32,
    pub tag_id: i32,
}