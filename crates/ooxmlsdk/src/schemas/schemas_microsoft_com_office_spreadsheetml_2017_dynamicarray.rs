/// Defines the DynamicArrayProperties Class.
/// When the object is serialized out as xml, it's qualified name is xda:dynamicArrayProperties.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xda:dynamicArrayProperties")]
pub struct DynamicArrayProperties {
    /// fDynamic
    /// Represents the following attribute in the schema: :fDynamic
    #[xml(attr = "fDynamic")]
    pub f_dynamic: Option<bool>,
    /// fCollapsed
    /// Represents the following attribute in the schema: :fCollapsed
    #[xml(attr = "fCollapsed")]
    pub f_collapsed: Option<bool>,
    /// _
    #[xml(child = "xda:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is xda:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xda:extLst")]
pub struct ExtensionList {
    #[xml(child = "x:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "x:ext")]
    XExt(crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension),
}
