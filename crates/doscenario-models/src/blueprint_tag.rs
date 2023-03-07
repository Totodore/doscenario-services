use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]

pub struct BlueprintTagModel {
    pub blueprint_id: i32,
    pub tag_id: i32,
}