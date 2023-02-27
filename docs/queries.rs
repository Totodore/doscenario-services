use models::{document::*, sheet::SheetModel, user::*};
use tonic::Status;

use crate::database::POOL;

pub async fn get_user(id: &String) -> Result<UserModel, Status> {
    let user = sqlx::query_as("SELECT * FROM user WHERE id = ?")
        .bind(id)
        .fetch_one(POOL.get().unwrap())
        .await
        .map_err(|e| Status::data_loss(e.to_string()))?;
	Ok(user)
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
