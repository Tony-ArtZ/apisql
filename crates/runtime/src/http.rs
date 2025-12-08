use reqwest::blocking::Client;
use serde_json::Value as Json;

pub struct HttpRuntime {
    client: Client,
}

impl HttpRuntime {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn get_json(
        &self,
        url: &str,
        headers: &[(&str, &str)],
    ) -> Result<(Json, u16), reqwest::Error> {
        let mut req = self.client.get(url);
        for (key, value) in headers {
            req = req.header(*key, *value);
        }

        let resp = req.send()?;
        let status = resp.status().as_u16();
        let json = resp.json::<Json>()?;

        Ok((json, status))
    }
}
