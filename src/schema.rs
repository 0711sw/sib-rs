use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    #[serde(rename = "contentSize")]
    pub content_size: u64,
    pub content_size_string: Option<String>,
    #[serde(rename = "extension")]
    pub extension: Option<String>,
    pub filename: Option<String>,
    #[serde(rename = "largeThumbnail")]
    pub large_thumbnail: Option<String>,
    #[serde(rename = "mediumThumbnail")]
    pub medium_thumbnail: Option<String>,
    pub small_thumbnail: Option<String>,
    pub url: Option<String>,
}

pub const PRODUCT_BASE_BLOCK_URN: &str = "urn:sib:product-base-1";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductBaseBlock {
    pub item_number: Option<String>,
    pub model: Option<String>,
    pub short_description: Option<String>,
    pub main_image: Option<Asset>,
    #[serde(default)]
    pub discontinued: bool,
    pub expected_availability_end: Option<NaiveDate>,
}

pub const PRODUCT_BRAND_BLOCK_URN: &str = "urn:sib:product-brand-1";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductBrandBlock {
    pub brand_name: Option<String>,
    pub brand_logo: Option<Asset>,
}

pub const PRODUCT_RELATIONS_BLOCK_URN: &str = "urn:sib:product-relations-1";

#[derive(Debug, Deserialize)]
pub struct ProductRelationsBlock {
    #[serde(default)]
    pub successors: Vec<Successor>,
    #[serde(default)]
    pub spareparts: Vec<Value>,
}

#[derive(Debug, Deserialize)]
pub struct Successor {
    #[serde(rename = "itemNumber")]
    pub item_number: String,
    #[serde(rename = "type")]
    pub successor_type: String,
}

pub const PRODUCT_TEXTS_BLOCK_URN: &str = "urn:sib:product-texts-1";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductTextsBlock {
    pub description: Option<String>,
    pub usage_note: Option<String>,
}

pub const PRODUCT_DOCUMENTS_BLOCK_URN: &str = "urn:sib:product-documents-1";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductDocumentsBlock {
    pub documents: Vec<Document>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub document_type: String,
    pub document: Asset,
}

pub const DOCUMENT_TYPE_INSTALLATION_MANUAL: &str = "INSTALLATION_MANUAL";
pub const DOCUMENT_TYPE_MAINTENANCE_MANUAL: &str = "MAINTENANCE_MANUAL";
pub const DOCUMENT_TYPE_MAINTENANCE_INSTRUCTION: &str = "MAINTENANCE_INSTRUCTION";
pub const DOCUMENT_TYPE_PLANNING: &str = "PLANNING";
pub const DOCUMENT_TYPE_ENERGY_LABEL: &str = "ENERGY_LABEL";
pub const DOCUMENT_TYPE_MSDS: &str = "MSDS";

impl ProductDocumentsBlock {
    pub fn find_document_by_type(&self, document_type: &str) -> Option<&Document> {
        self.documents
            .iter()
            .find(|doc| doc.document_type == document_type)
    }
}

pub const PRODUCT_LOGISTICS_BLOCK_URN: &str = "urn:sib:product-logistics-1";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductLogisticsBlock {
    pub shelf_life: Option<Decimal>,
    pub number_of_packages: Option<Decimal>,
    pub base_package: Option<BasePackage>,
    pub dangerous_goods: Option<DangerousGoods>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasePackage {
    pub length: Option<Decimal>,
    pub width: Option<Decimal>,
    pub height: Option<Decimal>,
    pub weight: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DangerousGoods {
    #[serde(default)]
    pub contains_dangerous_goods: bool,
}

pub const PRODUCT_REGULATIONS_BLOCK_URN: &str = "urn:sib:product-regulations-1";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductRegulationsBlock {
    pub germany: Option<GermanRegulations>,
    pub weee: Option<WeeeRegulations>,
    pub reach: Option<ReachRegulations>,
    pub clp: Option<ClpRegulations>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GermanRegulations {
    #[serde(default)]
    pub uba_positive: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeeeRegulations {
    #[serde(default)]
    pub weee_number: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReachRegulations {
    #[serde(default)]
    pub svhc_contents: Vec<SVHCContent>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SVHCContent {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClpRegulations {
    #[serde(default)]
    pub is_hazardous: bool,
    #[serde(default)]
    pub signal_word_warning: bool,
    #[serde(default)]
    pub signal_word_danger: bool,
}
