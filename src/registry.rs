#[derive(Debug, Deserialize, Clone)]
pub struct ConfigRegistry {
    #[serde(rename = "modelList")]
    pub models: KeyboardModelList,
    #[serde(rename = "layoutList")]
    pub layouts: LayoutList,
    #[serde(rename = "optionList")]
    pub options: XkbOptionGroupList,
}
