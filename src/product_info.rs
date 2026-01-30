//! High-level product information retrieval.
//!
//! This module provides a simplified API for fetching basic product information
//! without needing to work with individual SIB blocks.

use crate::schema::{
    PRODUCT_BASE_BLOCK_URN, PRODUCT_BRAND_BLOCK_URN, ProductBaseBlock, ProductBrandBlock,
};
use crate::{SibResponse, query_sib};
use serde::Serialize;

/// Simplified product information combining data from multiple blocks.
#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProductInfo {
    /// Unique product identifier.
    pub item_number: Option<String>,
    /// Model number.
    pub model: Option<String>,
    /// Short product description.
    pub short_description: Option<String>,
    /// URL to the main product image (small thumbnail).
    pub main_image_url: Option<String>,
    /// URL to the brand logo (small thumbnail).
    pub brand_logo_url: Option<String>,
    /// Brand name.
    pub brand_name: Option<String>,
}

/// Fetch simplified product information from a SIB endpoint.
///
/// This function queries the ProductBase and ProductBrand blocks and combines
/// them into a single [`ProductInfo`] struct with the most commonly needed fields.
pub async fn fetch_product_info(url: &str, source: Option<&str>) -> anyhow::Result<ProductInfo> {
    let response = query_sib(
        url,
        &[PRODUCT_BASE_BLOCK_URN, PRODUCT_BRAND_BLOCK_URN],
        source,
    )
    .await?;

    extract_product_info(&response)
}

/// Extract product information from an existing [`SibResponse`].
///
/// This is the testable core of [`fetch_product_info`]. It extracts and combines
/// data from ProductBase and ProductBrand blocks into a [`ProductInfo`] struct.
///
/// # Example
///
/// ```ignore
/// use sib_rs::SibResponse;
/// use sib_rs::schema::ProductBaseBlock;
/// use sib_rs::product_info::extract_product_info;
///
/// let response = SibResponse::empty()
///     .with_block_json::<ProductBaseBlock>(r#"{"itemNumber": "12345"}"#)?;
/// let info = extract_product_info(&response)?;
/// assert_eq!(info.item_number, Some("12345".to_owned()));
/// ```
pub fn extract_product_info(response: &SibResponse) -> anyhow::Result<ProductInfo> {
    let base_block = response.get_block::<ProductBaseBlock>()?;
    let brand_block = response.get_block::<ProductBrandBlock>()?;

    Ok(ProductInfo {
        item_number: base_block
            .as_ref()
            .and_then(|block| block.item_number.clone()),
        model: base_block.as_ref().and_then(|block| block.model.clone()),
        short_description: base_block
            .as_ref()
            .and_then(|block| block.short_description.clone()),
        main_image_url: base_block.as_ref().and_then(|block| {
            block
                .main_image
                .as_ref()
                .and_then(|asset| asset.small_thumbnail.clone())
        }),
        brand_logo_url: brand_block.as_ref().and_then(|block| {
            block
                .brand_logo
                .as_ref()
                .and_then(|asset| asset.small_thumbnail.clone())
        }),
        brand_name: brand_block
            .as_ref()
            .and_then(|block| block.brand_name.clone()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_product_info_from_json() {
        let response = SibResponse::empty()
            .with_block_json::<ProductBaseBlock>(
                r#"{
                    "itemNumber": "ABC-123",
                    "model": "Model X",
                    "shortDescription": "A great product",
                    "mainImage": {
                        "contentSize": 12345,
                        "smallThumbnail": "https://example.com/thumb.jpg"
                    }
                }"#,
            )
            .unwrap()
            .with_block_json::<ProductBrandBlock>(
                r#"{
                    "brandName": "ACME Corp",
                    "brandLogo": {
                        "contentSize": 5000,
                        "smallThumbnail": "https://example.com/logo.jpg"
                    }
                }"#,
            )
            .unwrap();

        let info = extract_product_info(&response).unwrap();

        assert_eq!(info.item_number, Some("ABC-123".to_owned()));
        assert_eq!(info.model, Some("Model X".to_owned()));
        assert_eq!(info.short_description, Some("A great product".to_owned()));
        assert_eq!(
            info.main_image_url,
            Some("https://example.com/thumb.jpg".to_owned())
        );
        assert_eq!(info.brand_name, Some("ACME Corp".to_owned()));
        assert_eq!(
            info.brand_logo_url,
            Some("https://example.com/logo.jpg".to_owned())
        );
    }

    #[test]
    fn test_extract_product_info_empty_response() {
        let response = SibResponse::empty();
        let info = extract_product_info(&response).unwrap();

        assert_eq!(info.item_number, None);
        assert_eq!(info.model, None);
        assert_eq!(info.brand_name, None);
    }

    #[test]
    fn test_extract_product_info_partial_blocks() {
        // Only base block, no brand block
        let response = SibResponse::empty()
            .with_block_json::<ProductBaseBlock>(r#"{"itemNumber": "PARTIAL-001"}"#)
            .unwrap();

        let info = extract_product_info(&response).unwrap();

        assert_eq!(info.item_number, Some("PARTIAL-001".to_owned()));
        assert_eq!(info.brand_name, None);
    }
}
