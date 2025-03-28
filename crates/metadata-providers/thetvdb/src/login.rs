use std::time::{SystemTime, UNIX_EPOCH};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use metadata_provider::{fetcher::Client, Error};
use serde::Deserialize;
use serde_json::json;

use crate::Instance;
#[derive(Deserialize, Default)]
struct Claims {
    exp: usize,
}

impl Instance {
    pub async fn login(&self, client: &Client) -> Result<String, Error> {
        let url = self.server.join("/v4/login").unwrap();
        let client: Root1 = client
            .post(url)
            .json(&json!({
                "apikey": self.key,
                "pin": self.pin
            }))
            .send()
            .await?
            .json()
            .await?;
        Ok(client.data.token)
    }

    pub async fn get_token(&self, client: &Client) -> Result<String, Error> {
        let mut token = self.access_token.lock().await;
        if let Some(token) = &*token {
            let payload = token.split('.').nth(1);
            let exp = payload
                .and_then(|v| STANDARD.decode(v).ok())
                .and_then(|v| serde_json::from_slice::<Claims>(&v).ok())
                .unwrap_or_default();
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() as usize;
            if exp.exp > now {
                return Ok(token.clone());
            }
        }
        let t = self.login(client).await?;
        *token = Some(t.clone());
        Ok(t)
    }
}

#[derive(Deserialize)]
struct Data1 {
    token: String,
}
#[derive(Deserialize)]
struct Root1 {
    data: Data1,
}
