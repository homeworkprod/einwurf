/*
 * Copyright 2022-2023 Jochen Kupperschmidt
 * License: MIT
 */

use reqwest::header::HeaderName;

const API_VERSION: &str = "2022-06-28";

#[derive(Clone, Debug)]
pub(crate) struct ApiClient {
    pub(crate) bearer_token: String,
    pub(crate) page_id: String,
}

impl ApiClient {
    pub(crate) async fn add_bulleted_list_item_block(&self, content: String) {
        self.add_block("bulleted_list_item".into(), content).await
    }

    pub(crate) async fn add_numbered_list_item_block(&self, content: String) {
        self.add_block("numbered_list_item".into(), content).await
    }

    pub(crate) async fn add_paragraph_block(&self, content: String) {
        self.add_block("paragraph".into(), content).await
    }

    pub(crate) async fn add_to_do_block(&self, content: String) {
        self.add_block("to_do".into(), content).await
    }

    async fn add_block(&self, block_type: String, content: String) {
        self.send_request(&serde_json::json!({
            "children": [
                {
                    "object": "block",
                    "type": block_type,
                    block_type: {
                        "rich_text": [
                            {
                                "type": "text",
                                "text": {
                                    "content": content,
                                }
                            }
                        ]
                    }
                }
            ]
        }))
        .await
    }

    async fn send_request(&self, json_value: &serde_json::Value) {
        let url = format!(
            "https://api.notion.com/v1/blocks/{page_id}/children",
            page_id = self.page_id
        );

        reqwest::Client::new()
            .patch(url)
            .bearer_auth(&self.bearer_token)
            .header(
                HeaderName::from_lowercase(b"notion-version").unwrap(),
                API_VERSION,
            )
            .json(json_value)
            .send()
            .await
            .unwrap();
    }
}
