/*
 * Copyright 2022-2023 Jochen Kupperschmidt
 * License: MIT
 */

use anyhow::Result;
use serde::Deserialize;
use std::fs::read_to_string;
use std::net::IpAddr;
use std::path::Path;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Config {
    pub ip_address: IpAddr,
    pub port: u16,
    pub destination: Destination,
    pub notion: NotionConfig,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Destination {
    Notion,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct NotionConfig {
    pub bearer_token: String,
    pub page_id: String,
    pub block_type: NotionBlockType,
}

#[derive(Clone, Debug, Deserialize)]
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
