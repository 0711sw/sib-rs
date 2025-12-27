use anyhow::Context;
use http::HeaderValue;
use reqwest::Client;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use url::Url;

pub mod product_info;
pub mod schema;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub block_type: String,
    pub data: Value,
}

pub struct SibResponse {
    url: String,
    blocks: HashMap<String, Block>,
}

impl SibResponse {
    pub fn get_block<'s, T>(&'s self, block_type: &str) -> anyhow::Result<Option<T>>
    where
        T: Deserialize<'s>,
    {
        match self.blocks.get(block_type) {
            Some(block) => T::deserialize(&block.data)
                .with_context(|| format!("Failed to deserialize {} for {}", block_type, &self.url))
                .map(|data| Some(data)),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SibRawResponse {
    blocks: Vec<Block>,
}

pub async fn query_sib(
    url: &str,
    types: &[&str],
    source: Option<&str>,
) -> anyhow::Result<SibResponse> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        http::header::ACCEPT,
        HeaderValue::from_static("application/json"),
    );

    let original_url = url.to_owned();
    let url = Url::parse(url)
        .ok()
        .map(|mut url| {
            if let Some(source) = source {
                url.query_pairs_mut().append_pair("source", source);
            }
            if !types.is_empty() {
                url.query_pairs_mut()
                    .append_pair("blocks", &types.join(","));
            }

            url.into()
        })
        .unwrap_or_else(|| url.to_string());

    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .error_for_status()?;

    let response = response
        .json::<SibRawResponse>()
        .await
        .context("Failed parsing data from server")?;

    let blocks = response
        .blocks
        .into_iter()
        .map(|block| (block.block_type.clone(), block))
        .collect::<HashMap<String, Block>>();

    Ok(SibResponse {
        url: original_url,
        blocks,
    })
}
