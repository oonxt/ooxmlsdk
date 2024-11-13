/// Defines the ThemeFamily Class.
/// When the object is serialized out as xml, it's qualified name is thm15:themeFamily.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "thm15:themeFamily")]
pub struct ThemeFamily {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// vid
    /// Represents the following attribute in the schema: :vid
    #[xml(attr = "vid")]
    pub vid: String,
    /// _
    #[xml(child = "thm15:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the OfficeArtExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is thm15:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "thm15:extLst")]
pub struct OfficeArtExtensionList {
    #[xml(child = "a:ext")]
    pub children: Vec<OfficeArtExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeArtExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension),
}
/// Defines the ThemeVariant Class.
/// When the object is serialized out as xml, it's qualified name is thm15:themeVariant.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "thm15:themeVariant")]
pub struct ThemeVariant {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// vid
    /// Represents the following attribute in the schema: :vid
    #[xml(attr = "vid")]
    pub vid: String,
    /// cx
    /// Represents the following attribute in the schema: :cx
    #[xml(attr = "cx")]
    pub x: i32,
    /// cy
    /// Represents the following attribute in the schema: :cy
    #[xml(attr = "cy")]
    pub y: i32,
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
    /// _
    #[xml(child = "thm15:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ThemeVariantList Class.
/// When the object is serialized out as xml, it's qualified name is thm15:themeVariantLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "thm15:themeVariantLst")]
pub struct ThemeVariantList {
    /// _
    #[xml(child = "thm15:themeVariant")]
    pub thm15_theme_variant: Vec<ThemeVariant>,
}
