use std::ffi::CString;

use chdb_bindings_rs::query_stable;

use crate::{options::QueryOption, result::QueryResult};

#[derive(Debug)]
pub struct Query {
    query: String,
    params: Vec<QueryOption>,
}

impl Query {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            params: Vec::new(),
        }
    }

    pub fn option(self, param: QueryOption) -> Self {
        Self {
            query: self.query,
            params: self
                .params
                .into_iter()
                .chain(std::iter::once(param))
                .collect(),
        }
    }

    pub fn exec(self) -> Option<QueryResult> {
        let mut args: Vec<String> =
            Vec::from(["clickhouse".to_string(), format!("--query={}", self.query)]);

        args.extend(
            self.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>(),
        );

        let c_strings: Vec<CString> = args
            .iter()
            .map(|s| CString::new(s.as_str()).expect("Failed to convert &str to CString"))
            .collect();

        let raw_pointers: Vec<*const std::os::raw::c_char> =
            c_strings.iter().map(|c_string| c_string.as_ptr()).collect();

        let result = unsafe { query_stable(c_strings.len() as i32, raw_pointers.as_ptr()) };

        if result.is_null() {
            return None;
        }

        let result = unsafe { &*result };

        Some(QueryResult {
            elapsed: result.elapsed,
            rows_read: result.rows_read,
            bytes_read: result.bytes_read,
            buf: unsafe { Vec::from_raw_parts(result.buf as *mut u8, result.size, 16) },
        })
    }
}
