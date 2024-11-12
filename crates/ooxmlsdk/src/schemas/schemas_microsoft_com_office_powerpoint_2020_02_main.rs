/// Defines the DesignerTagList Class.
/// When the object is serialized out as xml, it's qualified name is p202:designTagLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p202:designTagLst")]
pub struct DesignerTagList {
    /// _
    #[xml(child = "p202:designTag")]
    pub p202_design_tag: Vec<DesignerTag>,
}
/// Defines the DesignerDrawingProps Class.
/// When the object is serialized out as xml, it's qualified name is p202:designPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p202:designPr")]
pub struct DesignerDrawingProps {
    /// edtDesignElem
    /// Represents the following attribute in the schema: :edtDesignElem
    #[xml(attr = "edtDesignElem")]
    pub edt_design_elem: Option<bool>,
    /// _
    #[xml(child = "p202:designTagLst")]
    pub designer_tag_list: Option<DesignerTagList>,
    /// _
    #[xml(child = "p202:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the DesignerTag Class.
/// When the object is serialized out as xml, it's qualified name is p202:designTag.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p202:designTag")]
pub struct DesignerTag {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p202:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p202:extLst")]
pub struct ExtensionList {
    #[xml(child = "p:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "p:ext")]
    PExt(crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension),
}
