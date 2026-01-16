# sib-rs

Rust client library for querying [Standard Information Block (SIB)](https://github.com/0711sw/sib) endpoints.

SIB is a standardized format for digital product passports, providing structured product and regulatory compliance information.

## Usage

```rust
use sib_rs::{query_sib, schema::*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Query a SIB endpoint
    let response = query_sib(
        "https://example.com/sib/product123",
        &[PRODUCT_BASE_BLOCK_URN, PRODUCT_BRAND_BLOCK_URN],
        None,
    ).await?;

    // Deserialize specific blocks
    if let Some(base) = response.get_block::<ProductBaseBlock>(PRODUCT_BASE_BLOCK_URN)? {
        println!("Product: {}", base.item_number.unwrap_or_default());
    }

    Ok(())
}
```

### Simplified API

For basic product information, use the high-level `fetch_product_info` function:

```rust
use sib_rs::product_info::fetch_product_info;

let info = fetch_product_info("https://example.com/sib/product123", None).await?;
println!("{}: {}", info.item_number.unwrap_or_default(), info.short_description.unwrap_or_default());
```

## Available Blocks

| Block | URN | Description |
|-------|-----|-------------|
| `ProductBaseBlock` | `urn:sib:product-base-1` | Core identification (item number, GTIN, description) |
| `ProductBrandBlock` | `urn:sib:product-brand-1` | Brand and manufacturer information |
| `ProductTextsBlock` | `urn:sib:product-texts-1` | Extended descriptions (plain text and HTML) |
| `ProductAttributesBlock` | `urn:sib:product-attributes-1` | Custom key-value attributes |
| `ProductFeaturesBlock` | `urn:sib:product-features-1` | Classification features (ETIM, ECLASS) |
| `ProductImagesBlock` | `urn:sib:product-images-1` | Product images |
| `ProductDocumentsBlock` | `urn:sib:product-documents-1` | Documentation files (manuals, datasheets) |
| `ProductRelationsBlock` | `urn:sib:product-relations-1` | Successors, spare parts, accessories |
| `ProductLogisticsBlock` | `urn:sib:product-logistics-1` | Packaging, shipping, dangerous goods |
| `ProductRegulationsBlock` | `urn:sib:product-regulations-1` | Compliance (WEEE, REACH, CLP, ERP, battery) |
| `ProductFaqBlock` | `urn:sib:product-faq-1` | FAQ (experimental) |
| `ProductMaintenanceBlock` | `urn:sib:product-maintenance-1` | Maintenance schedules (experimental) |
| `ProductLcaBlock` | `urn:sib:product-lca-1` | Life cycle assessment (experimental) |
| `ItemBaseBlock` | `urn:sib:item-base-1` | Item instance data (experimental) |
| `LinksBlock` | `urn:sib:links-1` | Entity linking |

## License

MIT License