use anyhow::{Ok, Result};

pub struct HttpClient {
    url: String,
}

impl HttpClient {
    //new creates a new httpclient with the url
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    //send an https post
    pub async fn post_request<T: serde::de::DeserializeOwned + Send>(
        &self,
        headers: Option<reqwest::header::HeaderMap>,
        body: serde_json::Value,
    ) -> Result<T> {

        let client = reqwest::Client::new();
        let mut request = client.post(&self.url);

        if let Some(h) = headers {
            request = request.headers(h);
        }

        // debug!("Body: {:#?}", &body);
        let response = request.json(&body).send().await?;

        let text = response.text().await?;
        // debug!("Raw response: {}", text);

        let result: T = serde_json::from_str(&text).map_err(|e| {
            anyhow::anyhow!("Failed to deserialize response: {}. Body: {}", e, text)
        })?;

        Ok(result)
    }
}
