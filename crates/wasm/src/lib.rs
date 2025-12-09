use core_lib::*;
use serde_json::Value as Json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn run(source: String) -> Result<JsValue, JsValue> {
    let program = parse_program(&source).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    let req = program
        .request_blocks
        .first()
        .ok_or_else(|| JsValue::from_str("You must have at least one Request Block"))?;
    let resp = program
        .response_blocks
        .first()
        .ok_or_else(|| JsValue::from_str("You must have at least one Response Block"))?;

    // Execute request
    let client = reqwest::Client::new();
    let mut request_builder = client.request(
        match req.method {
            HttpMethods::Get => reqwest::Method::GET,
            HttpMethods::Post => reqwest::Method::POST,
            HttpMethods::Put => reqwest::Method::PUT,
            HttpMethods::Delete => reqwest::Method::DELETE,
            HttpMethods::Patch => reqwest::Method::PATCH,
        },
        &req.url,
    );

    for header in &req.headers {
        request_builder = request_builder.header(&header.key, &header.value);
    }

    let response = request_builder
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let _status = response.status().as_u16();
    let body_json: Json = response
        .json()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Execute query
    let result = execute_query(&resp.query, &body_json)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    let serializer = serde_wasm_bindgen::Serializer::json_compatible();
    use serde::Serialize;
    Ok(result.serialize(&serializer)?)
}
