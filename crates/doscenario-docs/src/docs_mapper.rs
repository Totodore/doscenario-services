use doscenario_models::{document, sheet};

use crate::docs::{OpenDocResponse, SheetEntity};

impl Into<OpenDocResponse> for document::DocumentModel {
    fn into(self) -> OpenDocResponse {
        OpenDocResponse {
            color: self.color.unwrap_or_default(),
            created_date: self.created_date.to_string(),
            last_editing: self.last_editing.to_string(),
            id: self.id,
            title: self.title,
            uid: self.uid,
            content: self.content.unwrap_or_default(),
            sheets: vec![],
            change_id: 0,
        }
    }
}
impl Into<SheetEntity> for sheet::SheetModel {
    fn into(self) -> SheetEntity {
        SheetEntity {
            id: self.id,
            title: self.title,
            color: self.color.unwrap_or_default(),
            created_date: self.created_date.to_string(),
            last_editing: self.last_editing.to_string(),
            uid: self.uid,
            created_by_id: self.created_by_id.unwrap_or_default(),
            project_id: self.project_id,
        }
    }
}
