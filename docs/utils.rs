use tokio::sync::Mutex;

use crate::docs::{doc_event_write, doc_write_request};

pub trait RemoveRange {
    fn to_remove_range(&mut self, start: usize, stop: usize) -> Self;
}

impl RemoveRange for String {
    fn to_remove_range(&mut self, start: usize, stop: usize) -> Self {
        let mut rslt = "".to_string();
        for (i, c) in self.chars().enumerate() {
            if start > i || stop < i + 1 {
                rslt.push(c);
            }
        }
        rslt
    }
}

impl Into<doc_event_write::Change> for doc_write_request::Change {
    fn into(self) -> doc_event_write::Change {
        match self {
            doc_write_request::Change::Insert(insert) => doc_event_write::Change::Insert(insert),
            doc_write_request::Change::Remove(remove) => doc_event_write::Change::Remove(remove),
            doc_write_request::Change::Replace(replace) => {
                doc_event_write::Change::Replace(replace)
            }
        }
    }
}

pub async fn get_snowflake() -> i64 {
    use lazy_static::lazy_static;
    use snowflake::SnowflakeIdGenerator;
    lazy_static! {
        static ref ID_GENERATOR: Mutex<SnowflakeIdGenerator> =
            Mutex::new(SnowflakeIdGenerator::new(1, 1));
    }
    ID_GENERATOR.lock().await.real_time_generate()
}
