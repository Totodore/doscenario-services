use sqlx::FromRow;
#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProjectUsersUserModel {
    
    pub project_id: i32,
    
    pub user_id: String,
}