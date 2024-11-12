#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TrueOnlyValues {
    #[default]
    True,
}
crate::__string_enum! {
    TrueOnlyValues { True = "true", }
}
/// Defines the Dummy Class.
/// When the object is serialized out as xml, it's qualified name is ma:DummyContentTypeElement.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ma:DummyContentTypeElement")]
pub struct Dummy {
    /// decimals
    /// Represents the following attribute in the schema: :decimals
    #[xml(attr = "decimals")]
    pub decimals: Option<String>,
    /// default
    /// Represents the following attribute in the schema: :default
    #[xml(attr = "default")]
    pub default: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// displayName
    /// Represents the following attribute in the schema: :displayName
    #[xml(attr = "displayName")]
    pub display_name: Option<String>,
    /// fieldsID
    /// Represents the following attribute in the schema: :fieldsID
    #[xml(attr = "fieldsID")]
    pub fields_id: Option<String>,
    /// format
    /// Represents the following attribute in the schema: :format
    #[xml(attr = "format")]
    pub format: Option<String>,
    /// hidden
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<String>,
    /// index
    /// Represents the following attribute in the schema: :index
    #[xml(attr = "index")]
    pub index: Option<i32>,
    /// internalName
    /// Represents the following attribute in the schema: :internalName
    #[xml(attr = "internalName")]
    pub internal_name: Option<String>,
    /// LCID
    /// Represents the following attribute in the schema: :LCID
    #[xml(attr = "LCID")]
    pub lcid: Option<i32>,
    /// list
    /// Represents the following attribute in the schema: :list
    #[xml(attr = "list")]
    pub list: Option<String>,
    /// percentage
    /// Represents the following attribute in the schema: :percentage
    #[xml(attr = "percentage")]
    pub percentage: Option<String>,
    /// readOnly
    /// Represents the following attribute in the schema: :readOnly
    #[xml(attr = "readOnly")]
    pub read_only: Option<String>,
    /// requiredMultiChoice
    /// Represents the following attribute in the schema: :requiredMultiChoice
    #[xml(attr = "requiredMultiChoice")]
    pub required_multi_choice: Option<String>,
    /// root
    /// Represents the following attribute in the schema: :root
    #[xml(attr = "root")]
    pub root: Option<TrueOnlyValues>,
    /// showField
    /// Represents the following attribute in the schema: :showField
    #[xml(attr = "showField")]
    pub show_field: Option<String>,
    /// web
    /// Represents the following attribute in the schema: :web
    #[xml(attr = "web")]
    pub web: Option<String>,
}
