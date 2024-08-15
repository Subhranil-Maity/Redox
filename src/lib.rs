
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub mod myconst;
pub mod utils;
pub mod schema;
pub struct ReturnedData {
    pub datatype: String,
    pub data: Vec<u8>,
}
