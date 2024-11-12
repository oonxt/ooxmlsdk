/// Defines the XsdunsignedInt Class.
/// When the object is serialized out as xml, it's qualified name is xltc2:checksum.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xltc2:checksum")]
pub struct XsdunsignedInt {
    #[xml(text)]
    pub child: i32,
}
/// Defines the CommentHyperlink Class.
/// When the object is serialized out as xml, it's qualified name is xltc2:hyperlink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xltc2:hyperlink")]
pub struct CommentHyperlink {
    /// startIndex
    /// Represents the following attribute in the schema: :startIndex
    #[xml(attr = "startIndex")]
    pub start_index: i32,
    /// length
    /// Represents the following attribute in the schema: :length
    #[xml(attr = "length")]
    pub length: i32,
    /// url
    /// Represents the following attribute in the schema: :url
    #[xml(attr = "url")]
    pub url: String,
    /// _
    #[xml(child = "xltc2:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is xltc2:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xltc2:extLst")]
pub struct ExtensionList {
    #[xml(child = "x:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "x:ext")]
    XExt(crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension),
}
