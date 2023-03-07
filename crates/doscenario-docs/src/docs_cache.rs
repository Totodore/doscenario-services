use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use crate::{docs::doc_write_request::Change};
use crate::{queries, utils::RemoveRange};
use dashmap::DashMap;
use futures::future::join_all;
use log::warn;
use tokio::time::{self};
use tonic::Status;

#[derive(Debug, Clone)]
struct ChangeEntry {
    changes: Vec<Change>,
    session: i64,
}
#[derive(Debug, Clone)]
struct DocCacheEntry {
    changes: Vec<ChangeEntry>,
    last_update: SystemTime,
    change_id: u64,
}

#[derive(Debug, Clone)]
pub struct DocsCache {
    doc_cache: DashMap<i32, DocCacheEntry>,
}

impl DocsCache {
    pub fn new_arc() -> Arc<Self> {
        let inst = Arc::new(Self {
            doc_cache: DashMap::new(),
        });

        // Start interval update task
        let update_task_inst = inst.clone();
        tokio::spawn(async move {
            update_task_inst.interval_update().await;
        });

        inst
    }
}

impl DocsCache {
    /**
     * Save documents to DB if it has been unactive for more than 30 seconds or has more than 100 changes
     */
    async fn interval_update(&self) {
        let delay = Duration::from_secs(2);
        loop {
            time::sleep(delay).await;
			log::info!("Cache state: {:?}", self.doc_cache.len());
            join_all(
                self.doc_cache
                    .iter_mut()
                    .filter(|entry| {
                        entry.changes.len() > 0
                            && (entry.last_update.elapsed().unwrap_or_default().as_secs() > 30
                                || entry.changes.len() > 100)
                    })
                    .map(|entry| self.apply_doc_changes(entry.key().clone())),
            )
            .await;
        }
    }

	/// Build the document content from the list of changes
    async fn build_doc_changes(&self, id: i32) -> Result<String, Status> {
        let mut content = queries::get_document_content(&id).await?;
        let entry = self
            .doc_cache
            .get(&id)
            .ok_or(Status::data_loss("Document not found"))?;
        for change_entry in entry.changes.iter() {
            for change in change_entry.changes.iter() {
                match change {
                    Change::Insert(ref insert) => {
                        content.insert_str(insert.position as usize, &insert.content);
                    }
                    Change::Remove(ref remove) => {
                        content = content.to_remove_range(
                            remove.position as usize,
                            (remove.position + remove.size) as usize,
                        );
                    }
                    Change::Replace(ref replace) => {
                        content = replace.content.clone();
                    }
                }
            }
        }
        Ok::<String, Status>(content)
    }

    async fn apply_doc_changes(&self, id: i32) -> Result<(), Status> {
        let content = self.build_doc_changes(id).await?;

		queries::set_doc_content(&id, &content).await?;
        Ok(())
    }

	/// Register a document to the cache and return the content and change id
	/// If the document is already in the cache, it will return the cached content and change id
    pub async fn register_doc(&self, doc_id: i32) -> Result<(String, u64), Status> {
        if !self.doc_cache.contains_key(&doc_id) {
            self.doc_cache.insert(
                doc_id,
                DocCacheEntry {
                    changes: Vec::new(),
                    last_update: SystemTime::now(),
                    change_id: 0,
                },
            );
        }
        let content = self.build_doc_changes(doc_id).await?;
        Ok((content, self.doc_cache.get(&doc_id).unwrap().change_id))
    }

    pub async fn remove_doc(&self, doc_id: i32) -> Result<(), Status> {
        self.apply_doc_changes(doc_id).await?;
        self.doc_cache.remove(&doc_id);
        Ok(())
    }
    pub async fn update_doc(&self, session: i64, doc_id: i32, change: Change, change_id: u64) {
        if let Some(mut doc) = self.doc_cache.get_mut(&doc_id) {
            if doc
                .changes
                .last_mut()
                .map(|last| last.session == session)
                .unwrap_or(false)
            {
                doc.changes.last_mut().unwrap().changes.push(change);
            } else {
                doc.changes.push(ChangeEntry {
                    changes: vec![change],
                    session,
                });
            }
            doc.last_update = SystemTime::now();
            doc.change_id += 1;
        } else {
            warn!("Trying to modify a doc not found: {doc_id}!");
        }
    }

    // Apply change to the content and get a crc and compare it with client
    pub async fn crc_check(&self, doc_id: i32, crc: u32) -> Result<bool, Status> {
        let content = self.build_doc_changes(doc_id).await?;
        let hash = crc32fast::hash(content.as_bytes());
        Ok(hash == crc)
    }
}
