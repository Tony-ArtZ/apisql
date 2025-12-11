use serde_json::Value as Json;
use std::time::{Duration, Instant};

use crate::cache::{Cache, CacheEntry};
use crate::errors::RuntimeError;
use crate::http::HttpRuntime;
use core_lib::*;

pub struct ExecutionRuntime {
    http: HttpRuntime,
    cache: Cache,
}

impl ExecutionRuntime {
    pub fn new() -> Self {
        Self {
            http: HttpRuntime::new(),
            cache: Cache::default(),
        }
    }

    pub fn run_source(&mut self, source: &str) -> Result<Json, RuntimeError> {
        let program = parse_program(source).map_err(|e| RuntimeError::Parse(e))?;
        let req = program
            .request_blocks
            .first()
            .expect("You must have at least one Request Block");
        let resp = program
            .response_blocks
            .first()
            .expect("You must have at least one Response Block");

        self.run_request(req, resp)
    }

    pub fn fetch_data(&mut self, req: &RequestBlock) -> Result<Json, RuntimeError> {
        let cache_key = format!("{}:{}", Self::method_to_string(&req.method), req.url);
        if let Some(body) = self.try_cache(req, &cache_key)? {
            Ok(body.0)
        } else {
            let headers: Vec<(&str, &str)> = req
                .headers
                .iter()
                .map(|h| (h.key.as_str(), h.value.as_str()))
                .collect();

            let (json, status) = self.http.get_json(&req.url, &headers)?;
            self.store_cache(req, cache_key, json.clone(), status);
            Ok(json)
        }
    }

    fn run_request(
        &mut self,
        req: &RequestBlock,
        resp: &ResponseBlock,
    ) -> Result<Json, RuntimeError> {
        let cache_key = format!("{}:{}", Self::method_to_string(&req.method), req.url);
        let (body_json, _status) = if let Some(body) = self.try_cache(req, &cache_key)? {
            body
        } else {
            let headers: Vec<(&str, &str)> = req
                .headers
                .iter()
                .map(|h| (h.key.as_str(), h.value.as_str()))
                .collect();

            let (json, status) = self.http.get_json(&req.url, &headers)?;
            self.store_cache(req, cache_key, json.clone(), status);
            (json, status)
        };

        // TODO: Handle status codes appropriately
        let result = execute_query(&resp.query, &body_json)?;
        Ok(result)
    }

    fn try_cache(
        &self,
        _req: &RequestBlock,
        key: &str,
    ) -> Result<Option<(Json, u16)>, RuntimeError> {
        if let Some(entry) = self.cache.get(key) {
            return Ok(Some((entry.value.clone(), entry.status_code)));
        }
        Ok(None)
    }

    fn store_cache(&mut self, req: &RequestBlock, key: String, body: Json, status: u16) {
        let entry = CacheEntry {
            status_code: status,
            value: body.clone(),
            timestamp: Instant::now(),
            ttl: match req.cache {
                CacheDuration::None => Duration::from_secs(0),
                CacheDuration::DurationInSeconds(secs) => Duration::from_secs(secs as u64),
            },
        };
        self.cache.insert(key, entry);
    }

    fn method_to_string(method: &HttpMethods) -> &str {
        match method {
            HttpMethods::Get => "GET",
            HttpMethods::Post => "POST",
            HttpMethods::Put => "PUT",
            HttpMethods::Delete => "DELETE",
            HttpMethods::Patch => "PATCH",
        }
    }
}
