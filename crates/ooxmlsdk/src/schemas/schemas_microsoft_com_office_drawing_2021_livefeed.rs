/// Defines the BackgroundNormalProperties Class.
/// When the object is serialized out as xml, it's qualified name is alf:Normal.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "alf:Normal")]
pub struct BackgroundNormalProperties {
    /// _
    #[xml(child = "alf:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the BackgroundRemovedProperties Class.
/// When the object is serialized out as xml, it's qualified name is alf:Removed.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "alf:Removed")]
pub struct BackgroundRemovedProperties {
    /// _
    #[xml(child = "alf:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the BackgroundBlurProperties Class.
/// When the object is serialized out as xml, it's qualified name is alf:Blur.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "alf:Blur")]
pub struct BackgroundBlurProperties {
    /// _
    #[xml(child = "alf:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the BackgroundCustomProperties Class.
/// When the object is serialized out as xml, it's qualified name is alf:Custom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "alf:Custom")]
pub struct BackgroundCustomProperties {
    /// _
    #[xml(child = "alf:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LiveFeedProperties Class.
/// When the object is serialized out as xml, it's qualified name is alf:liveFeedProps.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "alf:liveFeedProps")]
pub struct LiveFeedProperties {
    /// _
    #[xml(child = "alf:backgroundProps")]
    pub live_feed_background_properties: Option<LiveFeedBackgroundProperties>,
    /// _
    #[xml(child = "alf:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the OfficeArtExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is alf:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "alf:extLst")]
pub struct OfficeArtExtensionList {
    #[xml(child = "a:ext")]
    pub children: Vec<OfficeArtExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeArtExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension),
}
/// Defines the LiveFeedBackgroundProperties Class.
/// When the object is serialized out as xml, it's qualified name is alf:backgroundProps.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "alf:backgroundProps")]
pub struct LiveFeedBackgroundProperties {
    #[xml(
        child = "alf:Normal",
        child = "alf:Removed",
        child = "alf:Blur",
        child = "alf:Custom",
        child = "alf:extLst",
    )]
    pub children: Vec<LiveFeedBackgroundPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LiveFeedBackgroundPropertiesChildChoice {
    #[xml(tag = "alf:Normal")]
    AlfNormal(BackgroundNormalProperties),
    #[xml(tag = "alf:Removed")]
    AlfRemoved(BackgroundRemovedProperties),
    #[xml(tag = "alf:Blur")]
    AlfBlur(BackgroundBlurProperties),
    #[xml(tag = "alf:Custom")]
    AlfCustom(BackgroundCustomProperties),
    #[xml(tag = "alf:extLst")]
    AlfExtLst(OfficeArtExtensionList),
}
