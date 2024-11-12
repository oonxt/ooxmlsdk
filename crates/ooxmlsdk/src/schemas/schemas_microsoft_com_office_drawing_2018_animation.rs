#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Indefinite {
    #[default]
    Indefinite,
}
crate::__string_enum! {
    Indefinite { Indefinite = "indefinite", }
}
/// Defines the AnimationProperties Class.
/// When the object is serialized out as xml, it's qualified name is aanim:animPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "aanim:animPr")]
pub struct AnimationProperties {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// length
    /// Represents the following attribute in the schema: :length
    #[xml(attr = "length")]
    pub length: String,
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<String>,
    /// auto
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// offset
    /// Represents the following attribute in the schema: :offset
    #[xml(attr = "offset")]
    pub offset: Option<String>,
    /// st
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub st: Option<String>,
    /// end
    /// Represents the following attribute in the schema: :end
    #[xml(attr = "end")]
    pub end: Option<String>,
    /// _
    #[xml(child = "aanim:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the OfficeArtExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is aanim:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "aanim:extLst")]
pub struct OfficeArtExtensionList {
    #[xml(child = "a:ext")]
    pub children: Vec<OfficeArtExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeArtExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension),
}
