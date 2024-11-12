/// Defines the RichValueRels Class.
/// When the object is serialized out as xml, it's qualified name is xlrvrel:richValueRels.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrvrel:richValueRels")]
pub struct RichValueRels {
    /// _
    #[xml(child = "xlrvrel:rel")]
    pub xlrvrel_rel: Vec<RichValueRelRelationship>,
    /// _
    #[xml(child = "xlrvrel:extLst")]
    pub xlrvrel_ext_lst: Option<ExtensionList>,
}
/// Defines the RichValueRelRelationship Class.
/// When the object is serialized out as xml, it's qualified name is xlrvrel:rel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrvrel:rel")]
pub struct RichValueRelRelationship {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is xlrvrel:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrvrel:extLst")]
pub struct ExtensionList {
    #[xml(child = "x:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "x:ext")]
    XExt(crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension),
}
