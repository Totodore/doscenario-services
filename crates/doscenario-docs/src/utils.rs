use tokio::sync::Mutex;
use tonic::Request;

use crate::UserId;

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

pub async fn get_snowflake() -> i64 {
    use lazy_static::lazy_static;
    use snowflake::SnowflakeIdGenerator;
    lazy_static! {
        static ref ID_GENERATOR: Mutex<SnowflakeIdGenerator> =
            Mutex::new(SnowflakeIdGenerator::new(1, 1));
    }
    ID_GENERATOR.lock().await.real_time_generate()
}


pub fn unpack_req<T>(req: Request<T>) -> (T, UserId) {
	let user_id = req.extensions().get::<UserId>().unwrap().clone();
	(req.into_inner(), user_id)
}