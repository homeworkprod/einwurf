/*
 * Copyright 2022-2023 Jochen Kupperschmidt
 * License: MIT
 */

use anyhow::Result;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::{get, post};
use axum::{Form, Router, Server};
use serde::Deserialize;
use std::net::IpAddr;

mod cli;
mod config;
mod notion;

use config::{load_config, Config, Destination, NotionBlockType, NotionConfig};

const HTML_FORM: Html<&'static str> = Html(
    r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Einwurf</title>
    <style>
      html {
        box-sizing: border-box;
      }

      *,
      *::after,
      *::before {
        box-sizing: inherit;
      }

      body {
        background-color: #edede9;
        color: #222222;
        display: grid;
        font-family: sans-serif;
        font-size: 16px;
        gap: 0.75rem;
        grid-template:
          "body" 1fr
          "footer" auto;
        height: 100vh;
        margin: 0;
        padding: 1.5rem;
      }

      form {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        grid-area: body;
      }

      textarea {
        height: 12rem;
        width: 100%;
      }

      button {
        background-color: #d6ccc2;
        color: currentColor;
        cursor: pointer;
        font-weight: bold;
        padding: 1rem 2rem;
      }

      button:hover {
        background-color: #ccd5ae;
      }

      button,
      textarea {
        border: #bcb8b1 solid 1px;
        border-radius: 0.25rem;
      }

      button:hover,
      textarea:hover {
        border-color: #999999;
      }

      footer {
        font-size: 0.75rem;
        grid-area: footer;
        text-align: right;
      }

      footer a {
        color: currentColor;
      }

      footer a:not(:hover) {
        text-decoration: none;
      }
    </style>
  </head>
  <body>
    <form action="/form" method="post">
      <textarea name="content" type="text" required autofocus></textarea>
      <button type="submit">Submit</button>
    </form>
    <footer>
      <a href="https://homework.nwsnet.de/releases/e1ff/#einwurf"><strong>Einwurf</strong></a> by Jochen Kupperschmidt
    </footer>
  </body>
</html>
"#,
);

#[derive(Clone)]
struct AppState {
    config: Config,
    notion_api_client: notion::ApiClient,
}

#[derive(Deserialize, Debug)]
struct NewItem {
    content: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::parse_args();
    let config = load_config(&args.config_filename)?;

    let app = build_app(config.clone());

    serve_app(app, config.ip_address, config.port)
        .await
        .unwrap();

    Ok(())
}

fn build_app(config: Config) -> Router {
    let notion_api_client = create_notion_api_client(&config.notion);

    let app_state = AppState {
        config,
        notion_api_client,
    };

    Router::new()
        .route("/", get(root))
        .route("/form", post(accept_form))
        .with_state(app_state)
}

fn create_notion_api_client(config: &NotionConfig) -> notion::ApiClient {
    notion::ApiClient {
        bearer_token: config.bearer_token.to_owned(),
        page_id: config.page_id.to_owned(),
    }
}

async fn serve_app(app: Router, ip_address: IpAddr, port: u16) -> Result<()> {
    let addr = (ip_address, port).into();
    println!("Listening on http://{addr}/");
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}

async fn root() -> Html<&'static str> {
    HTML_FORM
}

async fn accept_form(
    State(app_state): State<AppState>,
    Form(new_item): Form<NewItem>,
) -> impl IntoResponse {
    let content = new_item.content.trim().into();

    match app_state.config.destination {
        Destination::Notion => send_to_notion(app_state, content).await,
    };

    Redirect::to("/")
}

async fn send_to_notion(app_state: AppState, content: String) {
    let api_client = app_state.notion_api_client;
    let block_type = app_state.config.notion.block_type;

    match block_type {
        NotionBlockType::BulletedListItem => api_client.add_bulleted_list_item_block(content).await,
        NotionBlockType::NumberedListItem => api_client.add_numbered_list_item_block(content).await,
        NotionBlockType::Paragraph => api_client.add_paragraph_block(content).await,
        NotionBlockType::ToDo => api_client.add_to_do_block(content).await,
    }
}
