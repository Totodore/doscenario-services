use crate::sea_orm_active_enums::{ParentPole, ChildPole, Type};
use sqlx::FromRow;


#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct RelationshipModel {
    
    pub id: i32,
    
    pub parent_id: i32,
    
    pub child_id: i32,
    
    pub blueprint_id: Option<i32>,
    
    pub parent_pole: ParentPole,
    
    pub child_pole: ChildPole,
    pub r#type: Type,
}