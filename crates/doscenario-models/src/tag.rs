use sqlx::FromRow;


#[derive(Clone, Debug, PartialEq, Eq, FromRow)]

pub struct TagModel {
    
    pub id: i32,
    pub title: String,
    pub primary: i8,
    pub color: Option<String>,
    
    pub project_id: Option<i32>,
    
    pub created_by_id: Option<String>,
}