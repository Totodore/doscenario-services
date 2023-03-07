use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct UserModel {
    
    pub id: String,
    pub name: String,
    pub password: String,
}
