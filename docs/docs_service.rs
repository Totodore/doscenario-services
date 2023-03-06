use std::{collections::HashMap, sync::Arc};

use crate::{
    broadcast_message,
    docs::{doc_event::Event, *},
    docs_cache::DocsCache,
    queries,
    utils::get_snowflake,
};
use dashmap::DashMap;
use tokio::sync::mpsc::{self, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub type SenderChan = Sender<Result<DocEvent, Status>>;

#[derive(Debug, Clone)]
pub struct DocsService {
    // Doc streams, map a doc id to a map of session id with a sender channel
    doc_streams: DashMap<i32, HashMap<i64, SenderChan>>,
    doc_cache: Arc<DocsCache>,
}
impl DocsService {
    pub fn new() -> Self {
        Self {
            doc_streams: DashMap::new(),
            doc_cache: DocsCache::new_arc(),
        }
    }
}

#[tonic::async_trait]
impl docs_server::Docs for DocsService {
    // Doc event stream
    type SubscribeDocStream = ReceiverStream<Result<DocEvent, Status>>;

    async fn subscribe_doc(
        &self,
        request: Request<DocIdentityRequest>,
    ) -> Result<Response<Self::SubscribeDocStream>, Status> {
        let (tx, rx) = mpsc::channel(64);
        let data = request.into_inner();
        let user = queries::get_user(&data.user_id).await?;

        let session_id = get_snowflake().await;

        if let Some(mut subs) = self.doc_streams.get_mut(&data.id) {
            broadcast_message!(
                subs.values(),
                Ok(DocEvent {
                    event: Some(doc_event::Event::Open(DocEventOpen {
                        user_id: data.user_id.clone(),
                        user_name: user.name.clone(),
                        id: data.id,
                    })),
                })
            )
            .await;
            subs.insert(session_id, tx);
        } else {
            let mut map = HashMap::new();
            map.insert(session_id, tx);
            self.doc_streams.insert(data.id, map);
        }
        log::info!("Doc stream created session_id{}", session_id);
        self.doc_streams
            .get(&data.id)
            .unwrap()
            .get(&session_id)
            .unwrap()
            .send(Ok(DocEvent {
                event: Some(doc_event::Event::Subscribed(DocEventSubscribed {
                    id: data.id,
                    session_id,
                })),
            }))
            .await
            .map_err(|_| Status::internal("Cannot send message"))?;
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn open_doc(
        &self,
        request: Request<OpenDocRequest>,
    ) -> Result<Response<OpenDocResponse>, Status> {
        let data = request.into_inner();
		log::info!("Open doc request: {:?}", data);
        let (doc, sheets, (content, change_id)) = tokio::try_join!(
            queries::get_document(&data.id),
            queries::get_doc_sheets(&data.id),
            self.doc_cache.register_doc(data.id)
        ).map_err(|e| {
			log::error!("Error opening doc: {:?}", e);
			e
		})?;

        let mut res: OpenDocResponse = doc.into();
        res.sheets = sheets.into_iter().map(|s| s.into()).collect();
        res.content = content;
        res.change_id = change_id;
        Ok(Response::new(res))
    }

    async fn write_doc(
        &self,
        request: Request<DocWriteRequest>,
    ) -> Result<Response<()>, Status> {
        let body = request.into_inner();
        if let Some(change) = body.change {
            self.doc_cache
                .update_doc(body.session_id, body.id, change.clone(), body.change_id)
                .await;
            if let Some(subs) = self.doc_streams.get(&body.id) {
                broadcast_message!(
                    subs.values(),
                    Ok(DocEvent {
                        event: Some(doc_event::Event::Write(DocEventWrite {
                            user_id: body.user_id.clone(),
                            id: body.id,
                            change: Some(change.clone().into()),
                        })),
                    })
                )
                .await;
            }
        }
        Ok(Response::new(()))
    }

    async fn close_doc(
        &self,
        request: Request<DocIdentityRequest>,
    ) -> Result<Response<()>, Status> {
        let data = request.into_inner();
        if let Some(mut subs) = self.doc_streams.get_mut(&data.id) {
            subs.remove(&data.session_id);
            broadcast_message!(
                subs.values(),
                Ok(DocEvent {
                    event: Some(Event::Close(DocEventClose {
                        user_id: data.user_id.clone(),
                        id: data.id,
                    })),
                })
            )
            .await;
            if subs.is_empty() {
                self.doc_streams.remove(&data.id);
            }
        }
        Ok(Response::new(()))
    }

    async fn remove_doc(
        &self,
        request: Request<DocIdentityRequest>,
    ) -> Result<Response<()>, Status> {
        let data = request.into_inner();
        if let Some(subs) = self.doc_streams.get(&data.id) {
            self.doc_streams
                .get_mut(&data.id)
                .unwrap()
                .remove(&data.session_id);
            broadcast_message!(
                subs.values(),
                Ok(DocEvent {
                    event: Some(Event::Remove(DocEventRemove {
                        user_id: data.user_id.clone(),
                        id: data.id,
                    })),
                })
            )
            .await;
            self.doc_streams.remove(&data.id);
            self.doc_cache.remove_doc(data.id).await?;
        }
        Ok(Response::new(()))
    }

    async fn crc_check(
        &self,
        request: Request<CrcCheckRequest>,
    ) -> Result<Response<CrcCheckResponse>, Status> {
        let data = request.into_inner();
        let valid = self.doc_cache.crc_check(data.id, data.crc).await?;
        Ok(Response::new(CrcCheckResponse { valid }))
    }
}
