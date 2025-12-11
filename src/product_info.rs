use crate::query_sib;
use crate::schema::{
    PRODUCT_BASE_BLOCK_URN, PRODUCT_BRAND_BLOCK_URN, ProductBaseBlock, ProductBrandBlock,
};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductInfo {
    pub item_number: Option<String>,
    pub model: Option<String>,
    pub short_description: Option<String>,
    pub main_image_url: Option<String>,
    pub brand_logo_url: Option<String>,
    pub brand_name: Option<String>,
}

pub async fn fetch_product_info(url: &str, source: Option<&str>) -> anyhow::Result<ProductInfo> {
    let result = query_sib(
        url,
        &[PRODUCT_BASE_BLOCK_URN, PRODUCT_BRAND_BLOCK_URN],
        source,
    )
    .await?;

    let base_block = result.get_block::<ProductBaseBlock>(PRODUCT_BASE_BLOCK_URN);
    let brand_block = result.get_block::<ProductBrandBlock>(PRODUCT_BRAND_BLOCK_URN);

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
