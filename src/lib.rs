//! Client library for querying Standard Information Block (SIB) endpoints.
//!
//! SIB is a standardized format for digital product passports, providing structured
//! product and regulatory compliance information.
//!
//! # Example
//!
//! ```ignore
//! use sib_rs::{query_sib, schema::ProductBaseBlock};
//!
//! let response = query_sib("https://example.com/sib/product123", &[], None).await?;
//! if let Some(base) = response.get_block::<ProductBaseBlock>()? {
//!     println!("Product: {}", base.item_number.unwrap_or_default());
//! }
//! ```
//!

#![deny(
    // Code Quality
    warnings,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    // Safety
    unsafe_code,
    // Robustness
    rust_2018_idioms,
    nonstandard_style,
    future_incompatible,
    // Clippy - Panic Prevention
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]
// Relax some lints for test code
#![cfg_attr(
    test,
    allow(
        unused_results,
        missing_docs,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::indexing_slicing
    )
)]

use anyhow::Context;
use chrono::{DateTime, NaiveDate, Utc};
use http::HeaderValue;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use url::Url;

/// High-level product information retrieval.
pub mod product_info;
/// Data models for all SIB block types.
pub mod schema;

/// Trait for SIB block types that provides their URN identifier.
///
/// Implement this trait for block structs to enable type-safe block retrieval
/// via [`SibResponse::get_block`] and [`SibResponse::has_block`].
///
/// # Example
///
/// ```ignore
/// use sib_rs::{BlockDescriptor, schema::ProductBaseBlock};
///
/// // The URN is automatically used when retrieving blocks:
/// let base: Option<ProductBaseBlock> = response.get_block()?;
/// ```
pub trait BlockDescriptor {
    /// Returns the URN that identifies this block type (e.g., "urn:sib:product-base-1").
    fn urn() -> &'static str;
}

/// A single block from a SIB response.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    /// The block type URN (e.g., "urn:sib:product-base-1").
    pub block_type: String,
    /// The block schema version.
    pub block_version: u32,
    /// The date from which this block is valid.
    pub valid_from: NaiveDate,
    /// The timestamp when this block was last updated.
    pub last_updated: DateTime<Utc>,
    /// The raw JSON data for this block.
    pub data: Value,
}

/// An entry in the change history timeline.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntry {
    /// The date when this change became effective.
    pub effective_date: NaiveDate,
    /// The type of history entry (Ignored, Merged, Used).
    pub history_type: HistoryEntryType,
    /// Duration in days until the next change.
    pub duration_days: u32,
    /// Percentage of total timeline this entry covers.
    pub duration_percentage: f32,
    /// Whether this entry is the currently selected one for the lookup date.
    pub selected: bool,
}

/// Type of history entry indicating how a block was processed.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum HistoryEntryType {
    /// The block was ignored (e.g., not yet valid or superseded).
    Ignored,
    /// The block was merged with another block.
    Merged,
    /// The block was used directly.
    Used,
}

/// Response from a SIB endpoint containing multiple blocks.
pub struct SibResponse {
    blocks: HashMap<String, Block>,

    /// The url which has been requested.
    pub url: String,
    /// The lookup date used for the query.
    pub lookup_date: NaiveDate,
    /// The country code used for the query.
    pub country: String,
    /// The language code used for the query.
    pub language: String,
    /// Timeline of block changes (only populated if compute_timeline was requested).
    pub change_history: Vec<HistoryEntry>,
    /// The earliest update date across all used blocks.
    pub first_update: Option<NaiveDate>,
    /// The latest update date across all used blocks.
    pub last_update: Option<NaiveDate>,
    /// Any problems encountered during block resolution.
    pub problems: Vec<String>,
    /// Number of entities scanned during resolution.
    pub num_entities_scanned: u32,
    /// Number of blocks scanned during resolution.
    pub num_blocks_scanned: u32,
    /// Number of blocks actually used in the response.
    pub num_blocks_used: u32,
    /// Number of blocks loaded from storage.
    pub num_blocks_loaded: u32,
    /// Query execution time in microseconds.
    pub runtime_micros: u32,
    /// Application name that served the request.
    pub app: String,
    /// Application version that served the request.
    pub version: String,
    /// Cluster ID that served the request.
    pub cluster_id: String,
}

impl SibResponse {
    /// Deserialize a block into a typed struct.
    ///
    /// The block type is determined by the [`BlockDescriptor`] implementation,
    /// which provides the URN used to look up the block in the response.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(block))` if the block is present and deserialization succeeds
    /// - `Ok(None)` if the block is not present in the response
    /// - `Err(...)` if deserialization fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// use sib_rs::schema::ProductBaseBlock;
    ///
    /// let response = query_sib("https://example.com/sib/product123", &[], None).await?;
    /// if let Some(base) = response.get_block::<ProductBaseBlock>()? {
    ///     println!("Product: {}", base.item_number.unwrap_or_default());
    /// }
    /// ```
    pub fn get_block<D>(&self) -> anyhow::Result<Option<D>>
    where
        D: serde::de::DeserializeOwned + BlockDescriptor,
    {
        match self.blocks.get(D::urn()) {
            Some(block) => {
                // WORKAROUND: Roundtrip through JSON string to use serde_json's stream
                // deserializer instead of the Value deserializer.
                //
                // The Value deserializer in serde_json (with `arbitrary_precision`) tries
                // visit_f64 for numbers that roundtrip as f64 before falling back to
                // visit_map. rust_decimal's visit_f64 (with `serde-arbitrary-precision`)
                // uses ryu, which formats small numbers in scientific notation (e.g. "5.06e-6").
                // Decimal::from_str cannot parse scientific notation → deserialization fails.
                //
                // The stream deserializer does not have this problem: non-integer numbers
                // always go through visit_map → DecimalFromString → from_scientific fallback.
                //
                // Upstream issues:
                //   - https://github.com/paupino/rust-decimal/issues/722 (from_str + scientific)
                //   - https://github.com/paupino/rust-decimal/issues/760 (serde_json roundtrip)
                //
                // This workaround can be removed once rust_decimal's visit_f64 handles
                // scientific notation or from_str supports it natively.
                let json = serde_json::to_string(&block.data)
                    .with_context(|| format!("Failed to serialize {} for {}", D::urn(), &self.url))?;
                serde_json::from_str(&json)
                    .with_context(|| format!("Failed to deserialize {} for {}", D::urn(), &self.url))
                    .map(Some)
            }
            None => Ok(None),
        }
    }

    /// Check if a block type is present in the response.
    ///
    /// This is useful for conditional logic without deserializing the block data.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use sib_rs::schema::{ProductRegulationsBlock, ProductLogisticsBlock};
    ///
    /// if response.has_block::<ProductRegulationsBlock>() {
    ///     // Product has regulatory compliance data
    /// }
    /// ```
    pub fn has_block<D>(&self) -> bool
    where
        D: BlockDescriptor,
    {
        self.blocks.contains_key(D::urn())
    }

    // =========================================================================
    // Test utilities - for building mock SibResponse instances in tests
    // =========================================================================

    /// Create an empty `SibResponse` with default metadata.
    ///
    /// **For testing purposes.** Use this to build mock responses for unit tests.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let response = SibResponse::empty()
    ///     .with_block_json::<ProductBaseBlock>(r#"{"itemNumber": "12345"}"#)?;
    /// ```
    pub fn empty() -> Self {
        Self {
            blocks: HashMap::new(),
            url: String::new(),
            lookup_date: Utc::now().date_naive(),
            country: "DE".to_owned(),
            language: "de".to_owned(),
            change_history: Vec::new(),
            first_update: None,
            last_update: None,
            problems: Vec::new(),
            num_entities_scanned: 0,
            num_blocks_scanned: 0,
            num_blocks_used: 0,
            num_blocks_loaded: 0,
            runtime_micros: 0,
            app: String::new(),
            version: String::new(),
            cluster_id: String::new(),
        }
    }

    /// Add a block from a JSON string.
    ///
    /// **For testing purposes.** The block type is inferred from `D::urn()`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let response = SibResponse::empty()
    ///     .with_block_json::<ProductBaseBlock>(r#"{"itemNumber": "ABC-123"}"#)?;
    /// ```
    pub fn with_block_json<D>(mut self, json: &str) -> anyhow::Result<Self>
    where
        D: BlockDescriptor,
    {
        let data: Value = serde_json::from_str(json)
            .with_context(|| format!("Failed to parse JSON for {}", D::urn()))?;
        self.insert_block::<D>(data);
        Ok(self)
    }

    /// Add a block from a `serde_json::Value`.
    ///
    /// **For testing purposes.** The block type is inferred from `D::urn()`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use serde_json::json;
    ///
    /// let response = SibResponse::empty()
    ///     .with_block_value::<ProductBaseBlock>(json!({"itemNumber": "ABC-123"}));
    /// ```
    pub fn with_block_value<D>(mut self, data: Value) -> Self
    where
        D: BlockDescriptor,
    {
        self.insert_block::<D>(data);
        self
    }

    /// Add a block by serializing a typed struct.
    ///
    /// **For testing purposes.** Useful when you want to construct the block
    /// data programmatically.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let base = ProductBaseBlock {
    ///     item_number: Some("ABC-123".to_owned()),
    ///     ..Default::default()
    /// };
    /// let response = SibResponse::empty()
    ///     .with_block_struct(&base)?;
    /// ```
    pub fn with_block_struct<D>(mut self, block: &D) -> anyhow::Result<Self>
    where
        D: BlockDescriptor + serde::Serialize,
    {
        let data = serde_json::to_value(block)
            .with_context(|| format!("Failed to serialize {}", D::urn()))?;
        self.insert_block::<D>(data);
        Ok(self)
    }

    /// Insert a block with the given data (internal helper).
    fn insert_block<D>(&mut self, data: Value)
    where
        D: BlockDescriptor,
    {
        let block = Block {
            block_type: D::urn().to_owned(),
            block_version: 1,
            valid_from: self.lookup_date,
            last_updated: Utc::now(),
            data,
        };
        let _ = self.blocks.insert(D::urn().to_owned(), block);
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SibRawResponse {
    lookup_date: String,
    country: String,
    language: String,
    blocks: Vec<Block>,
    change_history: Vec<HistoryEntry>,
    first_update: Option<NaiveDate>,
    last_update: Option<NaiveDate>,
    problems: Vec<String>,
    num_entities_scanned: u32,
    num_blocks_scanned: u32,
    num_blocks_used: u32,
    num_blocks_loaded: u32,
    runtime_micros: u32,
    app: String,
    version: String,
    cluster_id: String,
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

    let lookup_date = NaiveDate::parse_from_str(&response.lookup_date, "%Y-%m-%d")
        .context("Failed to parse lookup_date")?;

    Ok(SibResponse {
        url: original_url,
        blocks,
        lookup_date,
        country: response.country,
        language: response.language,
        change_history: response.change_history,
        first_update: response.first_update,
        last_update: response.last_update,
        problems: response.problems,
        num_entities_scanned: response.num_entities_scanned,
        num_blocks_scanned: response.num_blocks_scanned,
        num_blocks_used: response.num_blocks_used,
        num_blocks_loaded: response.num_blocks_loaded,
        runtime_micros: response.runtime_micros,
        app: response.app,
        version: response.version,
        cluster_id: response.cluster_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use schema::{ProductLcaBlock, ProductLogisticsBlock};
    use std::str::FromStr;

    fn d(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    /// Verifies that scientific notation decimals (e.g. 5.06e-6) are correctly
    /// deserialized through get_block. This is a regression test for a bug where
    /// serde_json's Value deserializer routes f64-representable numbers through
    /// visit_f64, and rust_decimal's visit_f64 fails on scientific notation
    /// produced by ryu.
    #[test]
    fn get_block_handles_scientific_notation_in_lca() {
        let response = SibResponse::empty()
            .with_block_json::<ProductLcaBlock>(
                r#"{
                    "declaredUnitQuantity": 1,
                    "declaredUnitUnit": "KGM",
                    "impacts": [{
                        "stage": "A1-A3",
                        "declared": true,
                        "GWP_FOSSIL": 5.06e-6,
                        "GWP_BIOGENIC": -1.23e-10,
                        "ODP": 3.45e-15,
                        "AP": 0.00000789
                    }]
                }"#,
            )
            .unwrap();

        let block = response
            .get_block::<ProductLcaBlock>()
            .expect("deserialization must not fail")
            .expect("block must be present");

        assert_eq!(block.impacts.len(), 1);
        let impact = &block.impacts[0];
        assert_eq!(impact.stage, "A1-A3");
        assert_eq!(impact.gwp_fossil, Some(d("0.00000506")));
        assert_eq!(impact.odp, Some(d("0.00000000000000345")));
        assert_eq!(impact.ap, Some(d("0.00000789")));
    }

    /// Verifies that trailing-zero decimals like 67.000000 are correctly
    /// deserialized. These fail with the Value deserializer because
    /// serde_json's f64 roundtrip check (format_finite(67.0) = "67.0") does
    /// not match the raw string "67.000000", causing it to fall through to
    /// visit_map which requires serde-arbitrary-precision.
    #[test]
    fn get_block_handles_trailing_zero_decimals_in_logistics() {
        let response = SibResponse::empty()
            .with_block_json::<ProductLogisticsBlock>(
                r#"{
                    "shelfLife": 24.000000,
                    "numberOfPackages": 1.00,
                    "basePackage": {
                        "length": 400.000000,
                        "width": 200.000000,
                        "height": 87.000000,
                        "weight": 2.500000
                    }
                }"#,
            )
            .unwrap();

        let block = response
            .get_block::<ProductLogisticsBlock>()
            .expect("deserialization must not fail")
            .expect("block must be present");

        assert_eq!(block.shelf_life, Some(d("24.000000")));
        assert_eq!(block.number_of_packages, Some(d("1.00")));

        let pkg = block.base_package.expect("basePackage must be present");
        assert_eq!(pkg.height, Some(d("87.000000")));
        assert_eq!(pkg.weight, Some(d("2.500000")));
    }

    /// Verifies that both scientific notation and trailing-zero decimals work
    /// together in the same block, as they would in real SIB data.
    #[test]
    fn get_block_handles_mixed_decimal_formats_in_lca() {
        let response = SibResponse::empty()
            .with_block_json::<ProductLcaBlock>(
                r#"{
                    "declaredUnitQuantity": 1.000000,
                    "declaredUnitUnit": "PCE",
                    "referenceServiceLife": 50.00,
                    "impacts": [
                        {
                            "stage": "A1-A3",
                            "declared": true,
                            "GWP_TOTAL": 123.456000,
                            "GWP_FOSSIL": 1.2e2,
                            "EP_FRESHWATER": 9.87e-8,
                            "PERE": 0.00,
                            "SM": 0
                        },
                        {
                            "stage": "D",
                            "declared": true,
                            "GWP_TOTAL": -4.56e-3
                        }
                    ]
                }"#,
            )
            .unwrap();

        let block = response
            .get_block::<ProductLcaBlock>()
            .expect("deserialization must not fail")
            .expect("block must be present");

        assert_eq!(block.declared_unit_quantity, Some(d("1.000000")));
        assert_eq!(block.reference_service_life, Some(d("50.00")));
        assert_eq!(block.impacts.len(), 2);

        let a = &block.impacts[0];
        assert_eq!(a.gwp_total, Some(d("123.456000")));
        assert_eq!(a.gwp_fossil, Some(d("120")));
        assert_eq!(a.pere, Some(d("0.00")));
        assert_eq!(a.sm, Some(d("0")));

        let stage_d = &block.impacts[1];
        assert_eq!(stage_d.gwp_total, Some(d("-0.00456")));
    }
}
