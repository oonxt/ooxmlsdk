/// Defines the PresetTransition Class.
/// When the object is serialized out as xml, it's qualified name is p15:prstTrans.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p15:prstTrans")]
pub struct PresetTransition {
    /// prst
    /// Represents the following attribute in the schema: :prst
    #[xml(attr = "prst")]
    pub preset: Option<String>,
    /// invX
    /// Represents the following attribute in the schema: :invX
    #[xml(attr = "invX")]
    pub inv_x: Option<bool>,
    /// invY
    /// Represents the following attribute in the schema: :invY
    #[xml(attr = "invY")]
    pub inv_y: Option<bool>,
}
/// Defines the PresenceInfo Class.
/// When the object is serialized out as xml, it's qualified name is p15:presenceInfo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p15:presenceInfo")]
pub struct PresenceInfo {
    /// userId
    /// Represents the following attribute in the schema: :userId
    #[xml(attr = "userId")]
    pub user_id: String,
    /// providerId
    /// Represents the following attribute in the schema: :providerId
    #[xml(attr = "providerId")]
    pub provider_id: String,
}
/// Defines the ThreadingInfo Class.
/// When the object is serialized out as xml, it's qualified name is p15:threadingInfo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p15:threadingInfo")]
pub struct ThreadingInfo {
    /// timeZoneBias
    /// Represents the following attribute in the schema: :timeZoneBias
    #[xml(attr = "timeZoneBias")]
    pub time_zone_bias: Option<i32>,
    /// _
    #[xml(child = "p15:parentCm")]
    pub parent_comment_identifier: Option<ParentCommentIdentifier>,
}
/// Defines the SlideGuideList Class.
/// When the object is serialized out as xml, it's qualified name is p15:sldGuideLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p15:sldGuideLst")]
pub struct SlideGuideList {
    #[xml(child = "p15:guide", child = "p15:extLst")]
    pub children: Vec<SlideGuideListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SlideGuideListChildChoice {
    #[xml(tag = "p15:guide")]
    P15Guide(ExtendedGuide),
    #[xml(tag = "p15:extLst")]
    P15ExtLst(ExtensionList),
}
/// Defines the NotesGuideList Class.
/// When the object is serialized out as xml, it's qualified name is p15:notesGuideLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p15:notesGuideLst")]
pub struct NotesGuideList {
    #[xml(child = "p15:guide", child = "p15:extLst")]
    pub children: Vec<NotesGuideListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NotesGuideListChildChoice {
    #[xml(tag = "p15:guide")]
    P15Guide(ExtendedGuide),
    #[xml(tag = "p15:extLst")]
    P15ExtLst(ExtensionList),
}
/// Defines the ExtendedGuideList Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct ExtendedGuideList {
    #[xml(child = "p15:guide", child = "p15:extLst")]
    pub children: Vec<ExtendedGuideListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtendedGuideListChildChoice {
    #[xml(tag = "p15:guide")]
    P15Guide(ExtendedGuide),
    #[xml(tag = "p15:extLst")]
    P15ExtLst(ExtensionList),
}
/// Defines the ChartTrackingReferenceBased Class.
/// When the object is serialized out as xml, it's qualified name is p15:chartTrackingRefBased.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p15:chartTrackingRefBased")]
pub struct ChartTrackingReferenceBased {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: bool,
}
/// Defines the ParentCommentIdentifier Class.
/// When the object is serialized out as xml, it's qualified name is p15:parentCm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p15:parentCm")]
pub struct ParentCommentIdentifier {
    /// authorId
    /// Represents the following attribute in the schema: :authorId
    #[xml(attr = "authorId")]
    pub author_id: Option<i32>,
    /// idx
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: Option<i32>,
}
/// Defines the ColorType Class.
/// When the object is serialized out as xml, it's qualified name is p15:clr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p15:clr")]
pub struct ColorType {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<ColorTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ColorTypeChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    ),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    ),
    #[xml(tag = "a:hslClr")]
    AHslClr(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor,
    ),
    #[xml(tag = "a:prstClr")]
    APrstClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor,
    ),
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p15:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p15:extLst")]
pub struct ExtensionList {
    #[xml(child = "p:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "p:ext")]
    PExt(crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension),
}
/// Defines the ExtendedGuide Class.
/// When the object is serialized out as xml, it's qualified name is p15:guide.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p15:guide")]
pub struct ExtendedGuide {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// orient
    /// Represents the following attribute in the schema: :orient
    #[xml(attr = "orient")]
    pub orientation: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::DirectionValues,
    >,
    /// pos
    /// Represents the following attribute in the schema: :pos
    #[xml(attr = "pos")]
    pub position: Option<i32>,
    /// userDrawn
    /// Represents the following attribute in the schema: :userDrawn
    #[xml(attr = "userDrawn")]
    pub is_user_drawn: Option<bool>,
    /// _
    #[xml(child = "p15:clr")]
    pub color_type: ColorType,
    /// _
    #[xml(child = "p15:extLst")]
    pub extension_list: Option<ExtensionList>,
}
