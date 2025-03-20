use reqwest::Error;
use serde::Deserialize;
use serde_json::json;

use crate::Instance;

impl Instance {
    pub fn login(&self) -> Result<String, Error> {
        let url = self.server.join("/v4/login").unwrap();
        let client: Root1 = self
            .client
            .post(url)
            .json(&json!({
                "apikey": self.key,
                "pin": self.pin
            }))
            .send()?
            .json()?;
        Ok(client.data.token)
    }

    pub fn get_token(&self) -> Result<String, Error> {
        let mut token = self.access_token.lock().unwrap();
        if let Some(token) = &*token {
            //TODO: check if valid
            Ok(token.clone())
        } else {
            let t = self.login()?;
            *token = Some(t.clone());
            Ok(t)
        }
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
