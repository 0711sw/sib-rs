# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

sib-rs is a Rust library for querying and parsing Standard Information Blocks (SIB) URLs. SIB is a standardized format for digital product passports, providing structured product and regulatory compliance information.

## Build Commands

```bash
cargo build           # Build the project
cargo test            # Run tests
cargo check           # Check for compilation errors without building
```

## Architecture

### Core Components

**lib.rs** - Entry point with the main query interface:
- `query_sib(url, source, blocks)` - Async function that queries SIB endpoints with optional filtering by source and block types
- `SibResponse` - Response container with `get_block::<T>(urn)` method for type-safe block extraction
- `Block` - Generic container holding block type and raw JSON data

**schema.rs** - Data models for all SIB block types (see Block URN Reference below). The schema mirrors the YAML definitions in `../sib/`. Each block has comprehensive doc comments explaining fields and their units/formats.

**product_info.rs** - High-level convenience API:
- `fetch_product_info(url)` - Simplified product info retrieval returning essential fields

### Key Design Decisions

- **Arbitrary precision decimals**: Uses `rust_decimal::Decimal` with `serde_json`'s `arbitrary_precision` feature for accurate regulatory/logistics data (concentrations, weights, dimensions)
- **Async-first**: All HTTP operations use `async`/`await` with reqwest
- **Error context**: Uses `anyhow::Context` to provide detailed error messages with request context
- **JSON mapping**: All structs use `#[serde(rename_all = "camelCase")]` for automatic property name conversion

### Block URN Reference

| Block Type | URN | Status |
|------------|-----|--------|
| ProductBaseBlock | urn:sib:product-base-1 | Active |
| ItemBaseBlock | urn:sib:item-base-1 | Experimental |
| ProductBrandBlock | urn:sib:product-brand-1 | Active |
| ProductTextsBlock | urn:sib:product-texts-1 | Active |
| ProductAttributesBlock | urn:sib:product-attributes-1 | Active |
| ProductFeaturesBlock | urn:sib:product-features-1 | Active |
| ProductImagesBlock | urn:sib:product-images-1 | Active |
| ProductRelationsBlock | urn:sib:product-relations-1 | Active |
| ProductDocumentsBlock | urn:sib:product-documents-1 | Active |
| ProductLogisticsBlock | urn:sib:product-logistics-1 | Active |
| ProductRegulationsBlock | urn:sib:product-regulations-1 | Active |
| ProductFaqBlock | urn:sib:product-faq-1 | Experimental |
| ProductMaintenanceBlock | urn:sib:product-maintenance-1 | Experimental |
| ProductLcaBlock | urn:sib:product-lca-1 | Experimental |
| LinksBlock | urn:sib:links-1 | Active |

### Schema Source

The Rust structs in `schema.rs` are derived from the YAML schema definitions in `../sib/`. When updating the schema, refer to `../sib/SIB_SCHEMA_FORMAT.md` for the schema definition format.
