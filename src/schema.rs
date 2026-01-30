//! Data models for Standard Information Block (SIB) types.
//!
//! This module contains Rust structs that mirror the SIB YAML schema definitions.
//! Each block type has a corresponding URN constant (e.g., [`PRODUCT_BASE_BLOCK_URN`])
//! that identifies the block type when querying SIB endpoints.
//!
//! The schemas are defined in the upstream `sib` repository.
//!
//! # Block Categories
//!
//! - **Product blocks**: Core product data (base, brand, texts, attributes, features, images, documents)
//! - **Relations block**: Product relationships (successors, spare parts, accessories, components)
//! - **Logistics block**: Packaging, shipping, and dangerous goods information
//! - **Regulations block**: Compliance data (WEEE, REACH, CLP, ERP, battery, German UBA)
//! - **Maintenance blocks**: FAQ, maintenance schedules, and life cycle assessment (experimental)
//! - **Links block**: Entity linking for external system integration

use crate::BlockDescriptor;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;

/// A file or image asset with URLs for different sizes.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    /// File size in bytes.
    #[serde(rename = "contentSize")]
    pub content_size: u64,
    /// Human-readable file size (e.g., "1.2 MB").
    pub content_size_string: Option<String>,
    /// File extension (e.g., "pdf", "jpg").
    #[serde(rename = "extension")]
    pub extension: Option<String>,
    /// Original filename.
    pub filename: Option<String>,
    /// URL to large thumbnail image.
    #[serde(rename = "largeThumbnail")]
    pub large_thumbnail: Option<String>,
    /// URL to medium thumbnail image.
    #[serde(rename = "mediumThumbnail")]
    pub medium_thumbnail: Option<String>,
    /// URL to small thumbnail image.
    pub small_thumbnail: Option<String>,
    /// URL to the original file.
    pub url: Option<String>,
}

// =============================================================================
// urn:sib:product-base-1 - Core product identification
// =============================================================================

/// URN for the product base block containing core identification and availability data.
pub const PRODUCT_BASE_BLOCK_URN: &str = "urn:sib:product-base-1";

/// Core product identification and availability information.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductBaseBlock {
    /// Unique product identifier.
    pub item_number: Option<String>,
    /// Model number.
    pub model: Option<String>,
    /// Global Trade Item Number (EAN/UPC barcode).
    pub gtin: Option<String>,
    /// Concise product description (may be translated).
    pub short_description: Option<String>,
    /// Main product image.
    pub main_image: Option<Asset>,
    /// Whether the product is discontinued.
    #[serde(default)]
    pub discontinued: bool,
    /// Expected date when stock will no longer be available.
    pub expected_availability_end: Option<NaiveDate>,
    /// ISO 3166-1 alpha-2 country code of origin.
    pub country_of_origin: Option<String>,
    /// Identifier for grouping product variants.
    pub variant_group: Option<String>,
}

impl BlockDescriptor for ProductBaseBlock {
    fn urn() -> &'static str {
        PRODUCT_BASE_BLOCK_URN
    }
}

// =============================================================================
// urn:sib:item-base-1 - Physical item-specific data (Experimental)
// =============================================================================

/// URN for the item base block containing physical item instance data (experimental).
pub const ITEM_BASE_BLOCK_URN: &str = "urn:sib:item-base-1";

/// Physical item-specific data for individual product instances.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemBaseBlock {
    /// Serial number of the individual item.
    pub serial_number: Option<String>,
    /// Manufacturing date.
    pub manufactured_at: Option<NaiveDate>,
}

impl BlockDescriptor for ItemBaseBlock {
    fn urn() -> &'static str {
        ITEM_BASE_BLOCK_URN
    }
}

// =============================================================================
// urn:sib:product-brand-1 - Brand and manufacturer information
// =============================================================================

/// URN for the product brand block containing brand and manufacturer information.
pub const PRODUCT_BRAND_BLOCK_URN: &str = "urn:sib:product-brand-1";

/// Brand and manufacturer information.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductBrandBlock {
    /// Brand name.
    pub brand_name: Option<String>,
    /// Brand logo image.
    pub brand_logo: Option<Asset>,
    /// Brand slogan (may be translated).
    pub slogan: Option<String>,
    /// Brand website URL (may be translated for different regions).
    pub website: Option<String>,
    /// Manufacturer information.
    pub manufacturer: Option<CompanyInfo>,
    /// Distributor information.
    pub distributor: Option<CompanyInfo>,
}

impl BlockDescriptor for ProductBrandBlock {
    fn urn() -> &'static str {
        PRODUCT_BRAND_BLOCK_URN
    }
}

/// Company contact information for manufacturer or distributor.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompanyInfo {
    /// Company name (may be translated).
    pub name: Option<String>,
    /// Postal address.
    pub postal_address: Option<String>,
    /// Electronic address (email).
    pub electronic_address: Option<String>,
}

// =============================================================================
// urn:sib:product-texts-1 - Extended text descriptions
// =============================================================================

/// URN for the product texts block containing extended descriptions.
pub const PRODUCT_TEXTS_BLOCK_URN: &str = "urn:sib:product-texts-1";

/// Extended text descriptions for the product.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductTextsBlock {
    /// Plain text description (may be translated).
    pub description: Option<String>,
    /// HTML-formatted description (may be translated).
    pub description_html: Option<String>,
    /// Plain text usage notes (may be translated).
    pub usage_note: Option<String>,
    /// HTML-formatted usage notes (may be translated).
    pub usage_note_html: Option<String>,
}

impl BlockDescriptor for ProductTextsBlock {
    fn urn() -> &'static str {
        PRODUCT_TEXTS_BLOCK_URN
    }
}

// =============================================================================
// urn:sib:product-attributes-1 - Custom product attributes
// =============================================================================

/// URN for the product attributes block containing custom key-value attributes.
pub const PRODUCT_ATTRIBUTES_BLOCK_URN: &str = "urn:sib:product-attributes-1";

/// Custom (non-standardized) product attributes.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductAttributesBlock {
    /// List of custom attributes.
    #[serde(default)]
    pub attributes: Vec<ProductAttribute>,
}

impl BlockDescriptor for ProductAttributesBlock {
    fn urn() -> &'static str {
        PRODUCT_ATTRIBUTES_BLOCK_URN
    }
}

/// A custom product attribute with label and value.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductAttribute {
    /// Attribute label (may be translated).
    pub label: String,
    /// Attribute value (may be translated).
    pub value: String,
}

// =============================================================================
// urn:sib:product-features-1 - Standardized technical features
// =============================================================================

/// URN for the product features block containing classification system features (ETIM, ECLASS).
pub const PRODUCT_FEATURES_BLOCK_URN: &str = "urn:sib:product-features-1";

/// Standardized technical features from classification systems (e.g., ETIM, ECLASS).
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductFeaturesBlock {
    /// List of classification systems with their features.
    #[serde(default)]
    pub systems: Vec<FeatureSystem>,
}

impl BlockDescriptor for ProductFeaturesBlock {
    fn urn() -> &'static str {
        PRODUCT_FEATURES_BLOCK_URN
    }
}

/// A classification system with its features.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeatureSystem {
    /// Name of the classification system (e.g., "ETIM", "ECLASS").
    pub system_name: String,
    /// Version of the classification system.
    pub system_version: String,
    /// Classification identifier within the system.
    pub classification: String,
    /// List of features in this classification.
    #[serde(default)]
    pub features: Vec<Feature>,
}

/// A single technical feature.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    /// Feature name.
    pub name: String,
    /// Feature value.
    pub value: String,
    /// Second value for ranges.
    pub value2: Option<String>,
    /// Unit of measurement.
    pub unit: Option<String>,
}

// =============================================================================
// urn:sib:product-images-1 - Product images
// =============================================================================

/// URN for the product images block containing multiple product images.
pub const PRODUCT_IMAGES_BLOCK_URN: &str = "urn:sib:product-images-1";

/// Multiple product images.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductImagesBlock {
    /// List of product images.
    #[serde(default)]
    pub images: Vec<ProductImage>,
}

impl BlockDescriptor for ProductImagesBlock {
    fn urn() -> &'static str {
        PRODUCT_IMAGES_BLOCK_URN
    }
}

/// A product image with metadata.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductImage {
    /// Image label (may be translated).
    pub label: Option<String>,
    /// Image type identifier.
    #[serde(rename = "type")]
    pub image_type: String,
    /// The image asset.
    pub image: Asset,
    /// ISO 639-1 language codes for text in the image.
    #[serde(default)]
    pub content_languages: Vec<String>,
}

// =============================================================================
// urn:sib:product-relations-1 - Product relationships
// =============================================================================

/// URN for the product relations block containing successors, spare parts, accessories, and components.
pub const PRODUCT_RELATIONS_BLOCK_URN: &str = "urn:sib:product-relations-1";

/// Product relationships: successors, spare parts, accessories, and components.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductRelationsBlock {
    /// Successor products (replacements).
    #[serde(default)]
    pub successors: Vec<Successor>,
    /// Spare parts organized by modules.
    #[serde(default)]
    pub spareparts: Vec<SparepartsModule>,
    /// Accessories for the product.
    #[serde(default)]
    pub accessories: Vec<Accessory>,
    /// Components that make up this product.
    #[serde(default)]
    pub components: Vec<Component>,
}

impl BlockDescriptor for ProductRelationsBlock {
    fn urn() -> &'static str {
        PRODUCT_RELATIONS_BLOCK_URN
    }
}

/// A successor/replacement product.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Successor {
    /// Item number of the successor product.
    pub item_number: String,
    /// Description of the successor (may be translated).
    pub short_description: Option<String>,
    /// Type of replacement: "DIRECT_REPLACEMENT" or "SIMILAR_PRODUCT".
    #[serde(rename = "type")]
    pub successor_type: String,
}

/// A module containing spare parts.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SparepartsModule {
    /// Module name (may be translated).
    pub module: Option<String>,
    /// Drawings showing part locations.
    #[serde(default)]
    pub drawings: Vec<SparepartsDrawing>,
    /// List of spare parts in this module.
    #[serde(default)]
    pub parts: Vec<Sparepart>,
}

/// A drawing showing spare part locations.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SparepartsDrawing {
    /// Drawing label (may be translated).
    pub label: Option<String>,
    /// The drawing image.
    pub image: Asset,
}

/// A spare part.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sparepart {
    /// Position number in drawing.
    pub number: Option<String>,
    /// Item number of the spare part.
    pub item_number: String,
    /// Description (may be translated).
    pub short_description: Option<String>,
}

/// An accessory for the product.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Accessory {
    /// Item number of the accessory.
    pub item_number: String,
    /// Description (may be translated).
    pub short_description: Option<String>,
    /// Recommended quantity.
    pub recommended_quantity: Option<Decimal>,
    /// Accessory group (may be translated).
    pub group: Option<String>,
    /// "optional" or "mandatory".
    #[serde(rename = "type")]
    pub accessory_type: Option<String>,
}

/// A component of the product.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    /// Item number of the component.
    pub item_number: String,
    /// Description (may be translated).
    pub short_description: Option<String>,
    /// Quantity of this component in the product.
    pub quantity: Decimal,
}

// =============================================================================
// urn:sib:product-documents-1 - Product documentation
// =============================================================================

/// URN for the product documents block containing documentation files.
pub const PRODUCT_DOCUMENTS_BLOCK_URN: &str = "urn:sib:product-documents-1";

/// Product documentation files.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductDocumentsBlock {
    /// List of product documents.
    #[serde(default)]
    pub documents: Vec<Document>,
}

impl BlockDescriptor for ProductDocumentsBlock {
    fn urn() -> &'static str {
        PRODUCT_DOCUMENTS_BLOCK_URN
    }
}

/// A product document with metadata.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    /// Document label (may be translated).
    pub label: Option<String>,
    /// Document type identifier.
    #[serde(rename = "type")]
    pub document_type: String,
    /// The document file.
    pub document: Asset,
    /// ISO 639-1 language codes for the document content.
    #[serde(default)]
    pub content_languages: Vec<String>,
}

/// Document type for installation manuals.
pub const DOCUMENT_TYPE_INSTALLATION_MANUAL: &str = "INSTALLATION_MANUAL";
/// Document type for maintenance manuals.
pub const DOCUMENT_TYPE_MAINTENANCE_MANUAL: &str = "MAINTENANCE_MANUAL";
/// Document type for maintenance instructions.
pub const DOCUMENT_TYPE_MAINTENANCE_INSTRUCTION: &str = "MAINTENANCE_INSTRUCTION";
/// Document type for planning documents.
pub const DOCUMENT_TYPE_PLANNING: &str = "PLANNING";
/// Document type for EU energy labels.
pub const DOCUMENT_TYPE_ENERGY_LABEL: &str = "ENERGY_LABEL";
/// Document type for Material Safety Data Sheets.
pub const DOCUMENT_TYPE_MSDS: &str = "MSDS";

impl ProductDocumentsBlock {
    /// Find a document by its type identifier.
    pub fn find_document_by_type(&self, document_type: &str) -> Option<&Document> {
        self.documents
            .iter()
            .find(|doc| doc.document_type == document_type)
    }
}

// =============================================================================
// urn:sib:product-logistics-1 - Logistics and dangerous goods
// =============================================================================

/// URN for the product logistics block containing packaging and dangerous goods data.
pub const PRODUCT_LOGISTICS_BLOCK_URN: &str = "urn:sib:product-logistics-1";

/// Transportation, storage, and dangerous goods information.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductLogisticsBlock {
    /// Shelf life in months.
    pub shelf_life: Option<Decimal>,
    /// Number of packages.
    pub number_of_packages: Option<Decimal>,
    /// Base package dimensions and weight.
    pub base_package: Option<BasePackage>,
    /// Dangerous goods information.
    pub dangerous_goods: Option<DangerousGoods>,
}

impl BlockDescriptor for ProductLogisticsBlock {
    fn urn() -> &'static str {
        PRODUCT_LOGISTICS_BLOCK_URN
    }
}

/// Package dimensions and weight.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BasePackage {
    /// Length in millimeters.
    pub length: Option<Decimal>,
    /// Width in millimeters.
    pub width: Option<Decimal>,
    /// Height in millimeters.
    pub height: Option<Decimal>,
    /// Weight in kilograms.
    pub weight: Option<Decimal>,
}

/// Dangerous goods information for transportation.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DangerousGoods {
    /// Whether the product contains dangerous goods.
    #[serde(default)]
    pub contains_dangerous_goods: bool,
    /// UN numbers (4-digit codes).
    #[serde(default)]
    pub un_numbers: Vec<String>,
    /// ADR hazard class (1-9).
    pub hazard_class: Option<String>,
    /// Hazard division (e.g., "1", "2").
    pub hazard_division: Option<String>,
    /// Transport category (0-4).
    pub transport_category: Option<String>,
    /// Hazard identification number (Kemler code).
    pub hazard_identification_number: Option<String>,
    /// Packing group (I, II, or III).
    pub packing_group: Option<String>,
    /// Proper shipping name (may be translated).
    pub proper_shipping_name: Option<String>,
    /// Technical name (may be translated).
    pub technical_name: Option<String>,
    /// Hazard label codes.
    #[serde(default)]
    pub hazard_labels: Vec<String>,
    /// ADR tunnel restriction code.
    pub tunnel_restriction_code: Option<String>,
    /// German water hazard class (nwg, awg, WGK 1, WGK 2, WGK 3).
    pub water_hazard_class: Option<String>,
    /// Whether limited quantity rules apply.
    #[serde(default)]
    pub limited_quantity: bool,
    /// Package description.
    pub package_description: Option<String>,
    /// Number of packages for dangerous goods shipment.
    pub number_of_packages: Option<Decimal>,
    /// Battery weight in kilograms.
    pub battery_weight: Option<Decimal>,
}

// =============================================================================
// urn:sib:product-regulations-1 - Regulatory compliance
// =============================================================================

/// URN for the product regulations block containing compliance data (WEEE, REACH, CLP, ERP, battery).
pub const PRODUCT_REGULATIONS_BLOCK_URN: &str = "urn:sib:product-regulations-1";

/// Regulatory compliance information for various jurisdictions and regulations.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductRegulationsBlock {
    /// German-specific regulations (UBA).
    pub germany: Option<GermanRegulations>,
    /// WEEE (Waste Electrical and Electronic Equipment) compliance.
    pub weee: Option<WeeeRegulations>,
    /// REACH (Registration, Evaluation, Authorisation and Restriction of Chemicals).
    pub reach: Option<ReachRegulations>,
    /// CLP (Classification, Labelling and Packaging) regulation.
    pub clp: Option<ClpRegulations>,
    /// Battery regulations.
    pub battery: Option<BatteryRegulations>,
    /// ERP (Energy-related Products) regulations.
    pub erp: Option<ErpRegulations>,
}

impl BlockDescriptor for ProductRegulationsBlock {
    fn urn() -> &'static str {
        PRODUCT_REGULATIONS_BLOCK_URN
    }
}

/// German UBA (Umweltbundesamt) regulations.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GermanRegulations {
    /// Whether the product is relevant for UBA registration.
    pub uba_relevant: Option<bool>,
    /// Whether the product is compliant with UBA requirements.
    pub uba_compliant: Option<bool>,
}

/// WEEE (Waste Electrical and Electronic Equipment) compliance.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WeeeRegulations {
    /// WEEE registration number.
    pub weee_number: Option<String>,
    /// WEEE category (temperatureExchangeEquipment, screensAndMonitors, lamps,
    /// largeEquipment, smallEquipment, smallITAndTelecommunicationEquipment).
    pub weee_category: Option<String>,
}

/// REACH (Registration, Evaluation, Authorisation and Restriction of Chemicals).
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReachRegulations {
    /// Whether the product is subject to REACH regulation.
    #[serde(default)]
    pub is_subject_to_reach: bool,
    /// Date of last REACH compliance check.
    pub last_check: Option<NaiveDate>,
    /// SCIP database number.
    pub scip_number: Option<String>,
    /// Substances of Very High Concern (SVHC) present in the product.
    #[serde(default)]
    pub svhc_contents: Vec<SvhcContent>,
}

/// A Substance of Very High Concern (SVHC) contained in the product.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SvhcContent {
    /// Name of the substance.
    pub substance_name: String,
    /// CAS (Chemical Abstracts Service) registry number.
    pub cas_number: Option<String>,
    /// Concentration of the substance.
    pub concentration: Option<String>,
}

/// CLP (Classification, Labelling and Packaging) regulation.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClpRegulations {
    /// Whether the product is classified as hazardous.
    #[serde(default)]
    pub is_hazardous: bool,
    /// Whether the "Warning" signal word applies.
    #[serde(default)]
    pub signal_word_warning: bool,
    /// Whether the "Danger" signal word applies.
    #[serde(default)]
    pub signal_word_danger: bool,
    /// H-statements (hazard statements).
    #[serde(default)]
    pub hazard_statements: Vec<String>,
    /// P-statements (precautionary statements).
    #[serde(default)]
    pub precautionary_statements: Vec<String>,
    /// GHS pictogram codes.
    #[serde(default)]
    pub pictograms: Vec<String>,
    /// Trade name of the product.
    pub trade_name: Option<String>,
    /// Supplemental hazard information.
    pub supplemental_information: Option<String>,
    /// Storage class.
    pub storage_class: Option<String>,
    /// Whether the product is restricted to B2B sales only.
    #[serde(default)]
    pub b2b_only: bool,
}

/// Battery regulations.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatteryRegulations {
    /// Whether the product contains a battery.
    #[serde(default)]
    pub contains_battery: bool,
    /// List of batteries in the product.
    #[serde(default)]
    pub batteries: Vec<Battery>,
}

/// Information about a battery in the product.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Battery {
    /// Battery identifier.
    pub id: Option<String>,
    /// Battery type description.
    pub battery_type: Option<String>,
    /// IEC battery code (e.g., "CR2032").
    pub battery_iec_code: Option<String>,
    /// Battery symbol/pictogram identifier.
    pub battery_pictogram: Option<String>,
    /// Battery location: BATTERY, INTERNAL, or SEPARATELY_ENCLOSED.
    pub battery_location: Option<String>,
    /// Number of batteries of this type.
    pub quantity: Option<Decimal>,
    /// Battery weight in kilograms.
    pub weight: Option<Decimal>,
    /// Battery capacity in ampere-hours.
    pub capacity: Option<Decimal>,
    /// Battery voltage.
    pub voltage: Option<Decimal>,
}

/// ERP (Energy-related Products) regulations.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErpRegulations {
    /// EU energy label image.
    pub erp_label: Option<Asset>,
    /// Product datasheet for energy labeling.
    pub erp_datasheet: Option<Asset>,
    /// Energy efficiency classifications.
    #[serde(default)]
    pub classifications: Vec<ErpClassification>,
}

/// An energy efficiency classification.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErpClassification {
    /// Category of energy efficiency (e.g., "seasonalSpaceHeating", "waterHeating").
    pub category: String,
    /// Energy efficiency class (A+++, A++, A+, A, B, C, D, E, F, G, or IE1-IE5).
    pub class: String,
}

// =============================================================================
// urn:sib:product-faq-1 - Frequently asked questions (Experimental)
// =============================================================================

/// URN for the product FAQ block containing frequently asked questions (experimental).
pub const PRODUCT_FAQ_BLOCK_URN: &str = "urn:sib:product-faq-1";

/// Frequently asked questions about the product.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductFaqBlock {
    /// List of FAQ entries.
    #[serde(default)]
    pub faqs: Vec<FaqEntry>,
}

impl BlockDescriptor for ProductFaqBlock {
    fn urn() -> &'static str {
        PRODUCT_FAQ_BLOCK_URN
    }
}

/// A frequently asked question with answer.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FaqEntry {
    /// The question (may be translated).
    pub question: String,
    /// The answer (may be translated).
    pub answer: String,
    /// Related documents.
    #[serde(default)]
    pub related_documents: Vec<RelatedDocument>,
}

/// A document related to a FAQ entry or maintenance task.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RelatedDocument {
    /// Document label (may be translated).
    pub label: Option<String>,
    /// The document file (PDF only).
    pub document: Asset,
}

// =============================================================================
// urn:sib:product-maintenance-1 - Maintenance information (Experimental)
// =============================================================================

/// URN for the product maintenance block containing maintenance schedules (experimental).
pub const PRODUCT_MAINTENANCE_BLOCK_URN: &str = "urn:sib:product-maintenance-1";

/// Maintenance and replacement schedules.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductMaintenanceBlock {
    /// Product replacement cycle.
    pub replacement_cycle: Option<ReplacementCycle>,
    /// List of maintenance tasks.
    #[serde(default)]
    pub maintenance_tasks: Vec<MaintenanceTask>,
}

impl BlockDescriptor for ProductMaintenanceBlock {
    fn urn() -> &'static str {
        PRODUCT_MAINTENANCE_BLOCK_URN
    }
}

/// Product replacement cycle information.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReplacementCycle {
    /// Interval value.
    pub interval_value: Decimal,
    /// Interval unit: DAYS, MONTHS, or YEARS.
    pub interval_unit: String,
    /// Additional notes (may be translated).
    pub note: Option<String>,
}

/// A scheduled maintenance task.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceTask {
    /// Task name (may be translated).
    pub name: String,
    /// Task description (may be translated).
    pub description: Option<String>,
    /// Interval value.
    pub interval_value: Decimal,
    /// Interval unit: DAYS, MONTHS, or YEARS.
    pub interval_unit: String,
    /// Parts required for this maintenance task.
    #[serde(default)]
    pub required_parts: Vec<RequiredPart>,
    /// Related documents.
    #[serde(default)]
    pub related_documents: Vec<RelatedDocument>,
}

/// A part required for a maintenance task.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequiredPart {
    /// Item number of the required part.
    pub item_number: String,
    /// Part description (may be translated).
    pub description: Option<String>,
}

// =============================================================================
// urn:sib:product-lca-1 - Life Cycle Assessment (Experimental)
// =============================================================================

/// URN for the product LCA block containing life cycle assessment data (experimental).
pub const PRODUCT_LCA_BLOCK_URN: &str = "urn:sib:product-lca-1";

/// Life Cycle Assessment data (EN 15804:2012+A2:2019).
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductLcaBlock {
    /// Environmental Product Declaration ID (ILCD UUID).
    pub epd_id: Option<String>,
    /// EPD version.
    pub epd_version: Option<String>,
    /// EPD title (may be translated).
    pub title: Option<String>,
    /// Publication date.
    pub published_at: Option<NaiveDate>,
    /// Validity start date.
    pub valid_from: Option<NaiveDate>,
    /// Validity end date.
    pub valid_to: Option<NaiveDate>,
    /// Declared unit quantity.
    pub declared_unit_quantity: Decimal,
    /// Declared unit: PCE, KGM, MTK, MTQ, MTR, KWH, or SET.
    pub declared_unit_unit: String,
    /// Functional unit description.
    pub functional_unit_description: Option<String>,
    /// Reference service life in years.
    pub reference_service_life: Option<Decimal>,
    /// Third party verification: none, internal, or external.
    pub third_party_verification: Option<String>,
    /// LCA scenarios.
    #[serde(default)]
    pub scenarios: Vec<LcaScenario>,
    /// Environmental impact indicators.
    #[serde(default)]
    pub impacts: Vec<LcaImpact>,
}

impl BlockDescriptor for ProductLcaBlock {
    fn urn() -> &'static str {
        PRODUCT_LCA_BLOCK_URN
    }
}

/// An LCA scenario.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LcaScenario {
    /// Scenario identifier.
    pub id: String,
    /// Scenario description (may be translated).
    pub description: Option<String>,
}

/// Environmental impact data for a life cycle stage.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LcaImpact {
    /// Life cycle stage (A1, A2, A3, A1-A3, A4, A5, B1-B7, C1-C4, D).
    pub stage: String,
    /// Whether this stage is declared.
    #[serde(default)]
    pub declared: bool,
    /// Scenario ID if applicable.
    pub scenario: Option<String>,
    // Environmental impact indicators (EN 15804:2012+A2:2019)
    /// Global Warming Potential - fossil (kg CO2 eq).
    #[serde(rename = "GWP_FOSSIL")]
    pub gwp_fossil: Option<Decimal>,
    /// Global Warming Potential - biogenic (kg CO2 eq).
    #[serde(rename = "GWP_BIOGENIC")]
    pub gwp_biogenic: Option<Decimal>,
    /// Global Warming Potential - land use (kg CO2 eq).
    #[serde(rename = "GWP_LULUC")]
    pub gwp_luluc: Option<Decimal>,
    /// Global Warming Potential - total (kg CO2 eq).
    #[serde(rename = "GWP_TOTAL")]
    pub gwp_total: Option<Decimal>,
    /// Ozone Depletion Potential (kg CFC-11 eq).
    #[serde(rename = "ODP")]
    pub odp: Option<Decimal>,
    /// Acidification Potential (mol H+ eq).
    #[serde(rename = "AP")]
    pub ap: Option<Decimal>,
    /// Eutrophication Potential - freshwater (kg P eq).
    #[serde(rename = "EP_FRESHWATER")]
    pub ep_freshwater: Option<Decimal>,
    /// Eutrophication Potential - marine (kg N eq).
    #[serde(rename = "EP_MARINE")]
    pub ep_marine: Option<Decimal>,
    /// Eutrophication Potential - terrestrial (mol N eq).
    #[serde(rename = "EP_TERRESTRIAL")]
    pub ep_terrestrial: Option<Decimal>,
    /// Photochemical Ozone Creation Potential (kg NMVOC eq).
    #[serde(rename = "POCP")]
    pub pocp: Option<Decimal>,
    /// Abiotic Depletion Potential - minerals and metals (kg Sb eq).
    #[serde(rename = "ADP_MINERALS_METALS")]
    pub adp_minerals_metals: Option<Decimal>,
    /// Abiotic Depletion Potential - fossil fuels (MJ).
    #[serde(rename = "ADP_FOSSIL")]
    pub adp_fossil: Option<Decimal>,
    /// Water Deprivation Potential (m³ world eq).
    #[serde(rename = "WDP")]
    pub wdp: Option<Decimal>,
    // Resource use indicators
    /// Primary Energy - renewable, energy (MJ).
    #[serde(rename = "PERE")]
    pub pere: Option<Decimal>,
    /// Primary Energy - renewable, material (MJ).
    #[serde(rename = "PERM")]
    pub perm: Option<Decimal>,
    /// Primary Energy - renewable, total (MJ).
    #[serde(rename = "PERT")]
    pub pert: Option<Decimal>,
    /// Primary Energy - non-renewable, energy (MJ).
    #[serde(rename = "PENRE")]
    pub penre: Option<Decimal>,
    /// Primary Energy - non-renewable, material (MJ).
    #[serde(rename = "PENRM")]
    pub penrm: Option<Decimal>,
    /// Primary Energy - non-renewable, total (MJ).
    #[serde(rename = "PENRT")]
    pub penrt: Option<Decimal>,
    /// Secondary Material (kg).
    #[serde(rename = "SM")]
    pub sm: Option<Decimal>,
    /// Renewable Secondary Fuels (MJ).
    #[serde(rename = "RSF")]
    pub rsf: Option<Decimal>,
    /// Non-renewable Secondary Fuels (MJ).
    #[serde(rename = "NRSF")]
    pub nrsf: Option<Decimal>,
    /// Fresh Water (m³).
    #[serde(rename = "FW")]
    pub fw: Option<Decimal>,
    // Waste indicators
    /// Hazardous Waste Disposed (kg).
    #[serde(rename = "HWD")]
    pub hwd: Option<Decimal>,
    /// Non-hazardous Waste Disposed (kg).
    #[serde(rename = "NHWD")]
    pub nhwd: Option<Decimal>,
    /// Radioactive Waste Disposed (kg).
    #[serde(rename = "RWD")]
    pub rwd: Option<Decimal>,
    // Output flows
    /// Components for Reuse (kg).
    #[serde(rename = "CRU")]
    pub cru: Option<Decimal>,
    /// Materials for Recycling (kg).
    #[serde(rename = "MFR")]
    pub mfr: Option<Decimal>,
    /// Materials for Energy Recovery (kg).
    #[serde(rename = "MER")]
    pub mer: Option<Decimal>,
    // Additional indicators
    /// Particulate Matter emissions (disease incidence).
    #[serde(rename = "PM")]
    pub pm: Option<Decimal>,
    /// Ionising Radiation (kBq U235 eq).
    #[serde(rename = "IR")]
    pub ir: Option<Decimal>,
    /// Ecotoxicity - freshwater (CTUe).
    #[serde(rename = "ETP_FW")]
    pub etp_fw: Option<Decimal>,
    /// Human Toxicity - cancer (CTUh).
    #[serde(rename = "HTP_C")]
    pub htp_c: Option<Decimal>,
    /// Human Toxicity - non-cancer (CTUh).
    #[serde(rename = "HTP_NC")]
    pub htp_nc: Option<Decimal>,
    /// Soil Quality Potential (dimensionless).
    #[serde(rename = "SQP")]
    pub sqp: Option<Decimal>,
}

// =============================================================================
// urn:sib:links-1 - Entity linking
// =============================================================================

/// URN for the links block containing entity references for external system integration.
pub const LINKS_BLOCK_URN: &str = "urn:sib:links-1";

/// Entity linking mechanism for connecting to other systems.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinksBlock {
    /// Upward links to parent entities.
    #[serde(default)]
    pub uplinks: Vec<EntityLink>,
    /// Effective date of the links.
    pub effective_date: Option<NaiveDate>,
}

impl BlockDescriptor for LinksBlock {
    fn urn() -> &'static str {
        LINKS_BLOCK_URN
    }
}

/// A link to an external entity.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EntityLink {
    /// Type of the linked entity.
    pub entity_type: String,
    /// Identifier of the linked entity.
    pub entity_id: String,
}
