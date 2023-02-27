use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct NodeTagModel {
    
    pub node_id: i32,
    
    pub tag_id: i32,
}