use once_cell::sync::OnceCell;
use sqlx::{mysql::{MySqlPool}};

pub static POOL: OnceCell<MySqlPool> = OnceCell::new();

pub async fn load_mysql_pool() {
	let conn_url = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let pool = MySqlPool::connect(conn_url.as_str())
        .await
        .expect("Failed to connect to database");
	POOL.set(pool).expect("Failed to set database pool");
}