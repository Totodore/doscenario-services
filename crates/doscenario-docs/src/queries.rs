use crate::database::POOL;
use doscenario_models::{user::UserModel, document::DocumentModel, sheet::SheetModel};
use uuid::Uuid;

use tonic::Status;
pub async fn get_user(id: &String) -> Result<UserModel, Status> {
    let user = sqlx::query_as("SELECT * FROM user WHERE id = ?")
        .bind(id)
        .fetch_one(POOL.get().unwrap())
        .await
        .map_err(|e| Status::data_loss(e.to_string()))?;
    Ok(user)
}

pub async fn create_document(
    title: &String,
    project_id: &i32,
    user_id: &String,
) -> Result<i32, Status> {
    let doc = sqlx::query(
        r#"
		INSERT INTO document (title, createdById, lastEditorId, projectId, uid) VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(title)
    .bind(user_id)
    .bind(user_id)
    .bind(project_id)
	.bind(Uuid::new_v4())
    .execute(POOL.get().unwrap())
    .await
    .map_err(|e| Status::data_loss(e.to_string()))?;
    Ok(doc.last_insert_id() as i32)
}
pub async fn get_document(id: &i32) -> Result<DocumentModel, Status> {
    let doc = sqlx::query_as(
        r#"SELECT id, createdDate,
		projectId,
		lastEditing,
		createdById,
		lastEditorId,
		title,
		uid,
		color FROM document WHERE id = ?"#,
    )
    .bind(id)
    .fetch_one(POOL.get().unwrap())
    .await
    .map_err(|e| Status::data_loss(e.to_string()))?;
    Ok(doc)
}
#[derive(sqlx::FromRow)]
struct ContentResult {
    content: Option<String>,
}
pub async fn get_document_content(id: &i32) -> Result<String, Status> {
    let ContentResult { content } = sqlx::query_as("SELECT content FROM document WHERE id = ?")
        .bind(id)
        .fetch_one(POOL.get().unwrap())
        .await
        .map_err(|e| Status::data_loss(e.to_string()))?;
    Ok(content.unwrap_or_default())
}

pub async fn get_doc_sheets(doc_id: &i32) -> Result<Vec<SheetModel>, Status> {
    let sheets = sqlx::query_as(
        r#"SELECT id, createdDate,
		documentId,
		lastEditing,
		projectId,
		createdById,
		lastEditorId,
		title,
		uid,
		color FROM sheet WHERE documentId = ?"#,
    )
    .bind(doc_id)
    .fetch_all(POOL.get().unwrap())
    .await
    .map_err(|e| Status::data_loss(e.to_string()))?;
    Ok(sheets)
}

pub async fn set_doc_content(id: &i32, content: &String) -> Result<(), Status> {
    sqlx::query("UPDATE document SET content = ? WHERE id = ?")
        .bind(content)
        .bind(id)
        .execute(POOL.get().unwrap())
        .await
        .map_err(|e| Status::data_loss(e.to_string()))?;
    Ok(())
}

pub async fn delete_doc(id: &i32) -> Result<(), Status> {
	sqlx::query("DELETE FROM document WHERE id = ?")
		.bind(id)
		.execute(POOL.get().unwrap())
		.await
		.map_err(|e| Status::data_loss(e.to_string()))?;
	Ok(())
}