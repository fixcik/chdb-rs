use std::string::FromUtf8Error;

#[derive(Debug)]
pub struct QueryResult {
    pub buf: Vec<u8>,
    pub elapsed: f64,
    pub rows_read: u64,
    pub bytes_read: u64,
}

impl QueryResult {
    pub fn to_string(self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.buf)
    }
}
