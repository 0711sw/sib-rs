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
    pub tunnel_restriction_code: Option<String>,
    pub water_hazard_class: Option<String>,
    #[serde(default)]
    pub limited_quantity: bool,

    pub battery_weight: Option<Decimal>,

    #[serde(default)]
    pub hazard_labels: Vec<String>,
}

pub const PRODUCT_REGULATIONS_BLOCK_URN: &str = "urn:sib:product-regulations-1";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductRegulationsBlock {
    pub germany: Option<GermanRegulations>,
    pub weee: Option<WeeeRegulations>,
    pub reach: Option<ReachRegulations>,
    pub clp: Option<ClpRegulations>,
    pub battery: Option<BatteryRegulations>,
    pub erp: Option<ErpRegulations>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GermanRegulations {
    pub uba_relevant: Option<bool>,
    pub uba_compliant: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeeeRegulations {
    pub weee_number: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReachRegulations {
    #[serde(default)]
    pub is_subject_to_reach: bool,
    pub scip_number: Option<String>,
    #[serde(default)]
    pub svhc_contents: Vec<SVHCContent>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SVHCContent {
    pub substance_name: String,
    pub cas_number: Option<String>,
    pub concentration: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClpRegulations {
    #[serde(default)]
    pub is_hazardous: bool,
    #[serde(default)]
    pub signal_word_warning: bool,
    #[serde(default)]
    pub signal_word_danger: bool,
    #[serde(default)]
    pub b2b_only: bool,

    #[serde(default)]
    pub hazard_statements: Vec<String>,
    #[serde(default)]
    pub precautionary_statements: Vec<String>,
    #[serde(default)]
    pub pictograms: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatteryRegulations {
    #[serde(default)]
    pub contains_battery: bool,
    #[serde(default)]
    pub batteries: Vec<Battery>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battery {
    pub battery_pictogram: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErpRegulations {
    pub seasonal_space_heating: Option<String>,
    pub seasonal_space_heating_medium_temp_55_c: Option<String>,
    pub seasonal_space_heating_low_temp_35_c: Option<String>,
    pub water_heating: Option<String>,
    pub seasonal_space_heating_package: Option<String>,
    pub seasonal_combined_heating_package: Option<String>,
    pub hot_water_storage_tank: Option<String>,
    pub cooling: Option<String>,
    pub heating_colder_climate: Option<String>,
    pub heating_average_climate: Option<String>,
    pub heating_warmer_climate: Option<String>,
    pub solid_fuel_boiler: Option<String>,
    pub solid_fuel_package: Option<String>,
    pub lamp: Option<String>,
    pub luminaire_approved_range: Option<String>,
    pub led_approved_range: Option<String>,
    pub light_source_20192015: Option<String>,
    pub local_space_heater: Option<String>,
    pub two_direction_ventilation_unit: Option<String>,
}
