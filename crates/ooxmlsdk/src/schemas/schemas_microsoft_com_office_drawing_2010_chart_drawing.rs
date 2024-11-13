/// Defines the ContentPart Class.
/// When the object is serialized out as xml, it's qualified name is cdr14:contentPart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr14:contentPart")]
pub struct ContentPart {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub relationship_id: String,
    /// bwMode
    /// Represents the following attribute in the schema: :bwMode
    #[xml(attr = "bwMode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    /// _
    #[xml(child = "cdr14:nvContentPartPr")]
    pub non_visual_content_part_properties: Option<NonVisualContentPartProperties>,
    /// _
    #[xml(child = "cdr14:nvPr")]
    pub application_non_visual_drawing_properties: Option<
        ApplicationNonVisualDrawingProperties,
    >,
    /// _
    #[xml(child = "cdr14:xfrm")]
    pub transform2_d: Option<Transform2D>,
    /// _
    #[xml(child = "cdr14:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the NonVisualDrawingProperties Class.
/// When the object is serialized out as xml, it's qualified name is cdr14:cNvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr14:cNvPr")]
pub struct NonVisualDrawingProperties {
    /// Application defined unique identifier.
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: u32,
    /// Name compatible with Object Model (non-unique).
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Description of the drawing element.
    /// Represents the following attribute in the schema: :descr
    #[xml(attr = "descr")]
    pub description: Option<String>,
    /// Flag determining to show or hide this element.
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    /// Title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    ///Hyperlink associated with clicking or selecting the element.
    #[xml(child = "a:hlinkClick")]
    pub hyperlink_on_click: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnClick,
    >,
    ///Hyperlink associated with hovering over the element.
    #[xml(child = "a:hlinkHover")]
    pub hyperlink_on_hover: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnHover,
    >,
    ///Future extension
    #[xml(child = "a:extLst")]
    pub non_visual_drawing_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualDrawingPropertiesExtensionList,
    >,
}
/// Defines the NonVisualInkContentPartProperties Class.
/// When the object is serialized out as xml, it's qualified name is cdr14:cNvContentPartPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr14:cNvContentPartPr")]
pub struct NonVisualInkContentPartProperties {
    /// isComment
    /// Represents the following attribute in the schema: :isComment
    #[xml(attr = "isComment")]
    pub is_comment: Option<bool>,
    /// _
    #[xml(child = "a14:cpLocks")]
    pub content_part_locks: Option<
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::ContentPartLocks,
    >,
    /// _
    #[xml(child = "a14:extLst")]
    pub office_art_extension_list: Option<
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::OfficeArtExtensionList,
    >,
}
/// Defines the NonVisualContentPartProperties Class.
/// When the object is serialized out as xml, it's qualified name is cdr14:nvContentPartPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr14:nvContentPartPr")]
pub struct NonVisualContentPartProperties {
    /// _
    #[xml(child = "cdr14:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    /// _
    #[xml(child = "cdr14:cNvContentPartPr")]
    pub non_visual_ink_content_part_properties: Option<
        NonVisualInkContentPartProperties,
    >,
}
/// Defines the ApplicationNonVisualDrawingProperties Class.
/// When the object is serialized out as xml, it's qualified name is cdr14:nvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr14:nvPr")]
pub struct ApplicationNonVisualDrawingProperties {
    /// macro
    /// Represents the following attribute in the schema: :macro
    #[xml(attr = "macro")]
    pub r#macro: Option<String>,
    /// fPublished
    /// Represents the following attribute in the schema: :fPublished
    #[xml(attr = "fPublished")]
    pub published: Option<bool>,
}
/// Defines the Transform2D Class.
/// When the object is serialized out as xml, it's qualified name is cdr14:xfrm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr14:xfrm")]
pub struct Transform2D {
    /// Rotation
    /// Represents the following attribute in the schema: :rot
    #[xml(attr = "rot")]
    pub rotation: Option<i32>,
    /// Horizontal Flip
    /// Represents the following attribute in the schema: :flipH
    #[xml(attr = "flipH")]
    pub horizontal_flip: Option<bool>,
    /// Vertical Flip
    /// Represents the following attribute in the schema: :flipV
    #[xml(attr = "flipV")]
    pub vertical_flip: Option<bool>,
    ///Offset
    #[xml(child = "a:off")]
    pub offset: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Offset,
    >,
    ///Extents
    #[xml(child = "a:ext")]
    pub extents: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extents,
    >,
}
/// Defines the OfficeArtExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is cdr14:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr14:extLst")]
pub struct OfficeArtExtensionList {
    #[xml(child = "a:ext")]
    pub children: Vec<OfficeArtExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeArtExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension),
}
