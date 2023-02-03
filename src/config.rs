/*
 * Copyright 2022-2023 Jochen Kupperschmidt
 * License: MIT
 */

use anyhow::Result;
use serde::Deserialize;
use std::fs::read_to_string;
use std::net::IpAddr;
use std::path::Path;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Config {
    pub ip_address: IpAddr,
    pub port: u16,
    pub destination: Destination,
    pub discord: DiscordConfig,
    pub mattermost: MattermostConfig,
    pub notion: NotionConfig,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Destination {
    Discord,
    Mattermost,
    Notion,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct DiscordConfig {
    pub webhook_url: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct MattermostConfig {
    pub webhook_url: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct NotionConfig {
    pub bearer_token: String,
    pub page_id: String,
    pub block_type: NotionBlockType,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum NotionBlockType {
    BulletedListItem,
    NumberedListItem,
    Paragraph,
    ToDo,
}

/// Load configuration from TOML file.
pub(crate) fn load_config(path: &Path) -> Result<Config> {
    let text = read_to_string(path)?;
    let config: Config = toml::from_str(&text)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_load_config() {
        let expected = Config {
            ip_address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 3000,
            destination: Destination::Notion,
            discord: DiscordConfig {
                webhook_url: "INSERT-WEBHOOK-URL".to_owned(),
            },
            mattermost: MattermostConfig {
                webhook_url: "INSERT-WEBHOOK-URL".to_owned(),
            },
            notion: NotionConfig {
                bearer_token: "INSERT-VALUE".to_owned(),
                page_id: "INSERT-VALUE".to_owned(),
                block_type: NotionBlockType::ToDo,
            },
        };

        let actual = load_config(Path::new("config_example.toml"));
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }
}
