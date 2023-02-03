/*
 * Copyright 2022-2023 Jochen Kupperschmidt
 * License: MIT
 */

#[derive(Clone, Debug)]
pub(crate) struct ApiClient {
    pub(crate) webhook_url: String,
}

impl ApiClient {
    pub(crate) async fn send_message(&self, content: String) {
        let json_value = &serde_json::json!({ "text": content });
        reqwest::Client::new()
            .post(&self.webhook_url)
            .json(json_value)
            .send()
            .await
            .unwrap();
    }
}
