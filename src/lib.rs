//! Client library for querying Standard Information Block (SIB) endpoints.
//!
//! SIB is a standardized format for digital product passports, providing structured
//! product and regulatory compliance information.
//!
//! # Example
//!
//! ```ignore
//! use sib_rs::{query_sib, schema::PRODUCT_BASE_BLOCK_URN, schema::ProductBaseBlock};
//!
//! let response = query_sib("https://example.com/sib/product123", &[], None).await?;
//! let base_block: Option<ProductBaseBlock> = response.get_block(PRODUCT_BASE_BLOCK_URN)?;
//! ```

#![deny(
    warnings,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results
)]
// Relax some lints for test code
#![cfg_attr(test, allow(unused_results, dead_code, unused_imports, missing_docs))]

use anyhow::Context;
use http::HeaderValue;
use reqwest::Client;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use url::Url;

/// High-level product information retrieval.
pub mod product_info;
/// Data models for all SIB block types.
pub mod schema;

/// A single block from a SIB response.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    /// The block type URN (e.g., "urn:sib:product-base-1").
    pub block_type: String,
    /// The raw JSON data for this block.
    pub data: Value,
}

/// Response from a SIB endpoint containing multiple blocks.
pub struct SibResponse {
    url: String,
    blocks: HashMap<String, Block>,
}

impl SibResponse {
    /// Deserialize a block by its URN into a typed struct.
    ///
    /// Returns `Ok(None)` if the block is not present in the response.
    /// Returns an error if deserialization fails.
    pub fn get_block<'s, T>(&'s self, block_type: &str) -> anyhow::Result<Option<T>>
    where
        T: Deserialize<'s>,
    {
        match self.blocks.get(block_type) {
            Some(block) => T::deserialize(&block.data)
                .with_context(|| format!("Failed to deserialize {} for {}", block_type, &self.url))
                .map(Some),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SibRawResponse {
    blocks: Vec<Block>,
}

/// Query a SIB endpoint and return the response.
///
/// # Arguments
///
/// * `url` - The SIB endpoint URL
/// * `types` - Optional list of block type URNs to request (empty = all blocks)
/// * `source` - Optional source identifier for the request
pub async fn query_sib(
    url: &str,
    types: &[&str],
    source: Option<&str>,
) -> anyhow::Result<SibResponse> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let _ = headers.insert(
        http::header::ACCEPT,
        HeaderValue::from_static("application/json"),
    );

    let original_url = url.to_owned();
    let url = Url::parse(url)
        .ok()
        .map(|mut url| {
            if let Some(source) = source {
                let _ = url.query_pairs_mut().append_pair("source", source);
            }
            if !types.is_empty() {
                let _ = url
                    .query_pairs_mut()
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
