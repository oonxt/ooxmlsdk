/// Defines the ClassificationLabelList Class.
/// When the object is serialized out as xml, it's qualified name is clbl:labelList.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "clbl:labelList")]
pub struct ClassificationLabelList {
    #[xml(attr = "xmlns", with = "classification_label_list_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "clbl:label")]
    pub clbl_label: Vec<ClassificationLabel>,
    /// _
    #[xml(child = "clbl:extLst")]
    pub clbl_ext_lst: Option<ClassificationExtensionList>,
}
mod classification_label_list_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/2020/mipLabelMetadata")
    }
}
/// Defines the ClassificationExtension Class.
/// When the object is serialized out as xml, it's qualified name is clbl:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "clbl:ext")]
pub struct ClassificationExtension {
    /// uri
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
}
/// Defines the ClassificationLabel Class.
/// When the object is serialized out as xml, it's qualified name is clbl:label.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "clbl:label")]
pub struct ClassificationLabel {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: bool,
    /// setDate
    /// Represents the following attribute in the schema: :setDate
    #[xml(attr = "setDate")]
    pub set_date: Option<String>,
    /// method
    /// Represents the following attribute in the schema: :method
    #[xml(attr = "method")]
    pub method: String,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// siteId
    /// Represents the following attribute in the schema: :siteId
    #[xml(attr = "siteId")]
    pub site_id: String,
    /// actionId
    /// Represents the following attribute in the schema: :actionId
    #[xml(attr = "actionId")]
    pub action_id: Option<String>,
    /// contentBits
    /// Represents the following attribute in the schema: :contentBits
    #[xml(attr = "contentBits")]
    pub content_bits: Option<u32>,
    /// removed
    /// Represents the following attribute in the schema: :removed
    #[xml(attr = "removed")]
    pub removed: bool,
}
/// Defines the ClassificationExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is clbl:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "clbl:extLst")]
pub struct ClassificationExtensionList {
    /// _
    #[xml(child = "clbl:ext")]
    pub clbl_ext: Vec<ClassificationExtension>,
}
