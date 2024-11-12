/// Defines the WordprocessingGroup Class.
/// When the object is serialized out as xml, it's qualified name is wpg:wgp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wpg:wgp")]
pub struct WordprocessingGroup {
    #[xml(
        child = "wpg:cNvPr",
        child = "wpg:cNvGrpSpPr",
        child = "wpg:grpSpPr",
        child = "wps:wsp",
        child = "wpg:grpSp",
        child = "wpg:graphicFrame",
        child = "pic:pic",
        child = "w14:contentPart",
        child = "wpg:extLst",
    )]
    pub children: Vec<WordprocessingGroupChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum WordprocessingGroupChildChoice {
    #[xml(tag = "wpg:cNvPr")]
    WpgCNvPr(NonVisualDrawingProperties),
    #[xml(tag = "wpg:cNvGrpSpPr")]
    WpgCNvGrpSpPr(NonVisualGroupDrawingShapeProperties),
    #[xml(tag = "wpg:grpSpPr")]
    WpgGrpSpPr(GroupShapeProperties),
    #[xml(tag = "wps:wsp")]
    WpsWsp(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_shape::WordprocessingShape,
    ),
    #[xml(tag = "wpg:grpSp")]
    WpgGrpSp(GroupShape),
    #[xml(tag = "wpg:graphicFrame")]
    WpgGraphicFrame(GraphicFrame),
    #[xml(tag = "pic:pic")]
    PicPic(crate::schemas::schemas_openxmlformats_org_drawingml_2006_picture::Picture),
    #[xml(tag = "w14:contentPart")]
    W14ContentPart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ContentPart,
    ),
    #[xml(tag = "wpg:extLst")]
    WpgExtLst(OfficeArtExtensionList),
}
/// Defines the GroupShape Class.
/// When the object is serialized out as xml, it's qualified name is wpg:grpSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wpg:grpSp")]
pub struct GroupShape {
    #[xml(
        child = "wpg:cNvPr",
        child = "wpg:cNvGrpSpPr",
        child = "wpg:grpSpPr",
        child = "wps:wsp",
        child = "wpg:grpSp",
        child = "wpg:graphicFrame",
        child = "pic:pic",
        child = "w14:contentPart",
        child = "wpg:extLst",
    )]
    pub children: Vec<GroupShapeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GroupShapeChildChoice {
    #[xml(tag = "wpg:cNvPr")]
    WpgCNvPr(NonVisualDrawingProperties),
    #[xml(tag = "wpg:cNvGrpSpPr")]
    WpgCNvGrpSpPr(NonVisualGroupDrawingShapeProperties),
    #[xml(tag = "wpg:grpSpPr")]
    WpgGrpSpPr(GroupShapeProperties),
    #[xml(tag = "wps:wsp")]
    WpsWsp(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_shape::WordprocessingShape,
    ),
    #[xml(tag = "wpg:grpSp")]
    WpgGrpSp(GroupShape),
    #[xml(tag = "wpg:graphicFrame")]
    WpgGraphicFrame(GraphicFrame),
    #[xml(tag = "pic:pic")]
    PicPic(crate::schemas::schemas_openxmlformats_org_drawingml_2006_picture::Picture),
    #[xml(tag = "w14:contentPart")]
    W14ContentPart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ContentPart,
    ),
    #[xml(tag = "wpg:extLst")]
    WpgExtLst(OfficeArtExtensionList),
}
/// Defines the WordprocessingGroupType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct WordprocessingGroupType {}
/// Defines the NonVisualDrawingProperties Class.
/// When the object is serialized out as xml, it's qualified name is wpg:cNvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wpg:cNvPr")]
pub struct NonVisualDrawingProperties {
    /// Application defined unique identifier.
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
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
/// Defines the NonVisualGraphicFrameProperties Class.
/// When the object is serialized out as xml, it's qualified name is wpg:cNvFrPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wpg:cNvFrPr")]
pub struct NonVisualGraphicFrameProperties {
    ///Graphic Frame Locks
    #[xml(child = "a:graphicFrameLocks")]
    pub graphic_frame_locks: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GraphicFrameLocks,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Defines the Transform2D Class.
/// When the object is serialized out as xml, it's qualified name is wpg:xfrm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wpg:xfrm")]
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
/// When the object is serialized out as xml, it's qualified name is wpg:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wpg:extLst")]
pub struct OfficeArtExtensionList {
    #[xml(child = "a:ext")]
    pub children: Vec<OfficeArtExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeArtExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension),
}
/// Defines the NonVisualGroupDrawingShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is wpg:cNvGrpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wpg:cNvGrpSpPr")]
pub struct NonVisualGroupDrawingShapeProperties {
    /// _
    #[xml(child = "a:grpSpLocks")]
    pub group_shape_locks: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupShapeLocks,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub non_visual_group_drawing_shape_props_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualGroupDrawingShapePropsExtensionList,
    >,
}
/// Defines the GroupShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is wpg:grpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wpg:grpSpPr")]
pub struct GroupShapeProperties {
    /// Black and White Mode
    /// Represents the following attribute in the schema: :bwMode
    #[xml(attr = "bwMode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    #[xml(
        child = "a:xfrm",
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
        child = "a:effectLst",
        child = "a:effectDag",
        child = "a:scene3d",
        child = "a:extLst",
    )]
    pub children: Vec<GroupShapePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GroupShapePropertiesChildChoice {
    #[xml(tag = "a:xfrm")]
    AXfrm(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TransformGroup,
    ),
    #[xml(tag = "a:noFill")]
    ANoFill(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill,
    ),
    #[xml(tag = "a:gradFill")]
    AGradFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill,
    ),
    #[xml(tag = "a:blipFill")]
    ABlipFill(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill,
    ),
    #[xml(tag = "a:grpFill")]
    AGrpFill(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupFill),
    #[xml(tag = "a:effectLst")]
    AEffectLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList,
    ),
    #[xml(tag = "a:effectDag")]
    AEffectDag(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag,
    ),
    #[xml(tag = "a:scene3d")]
    AScene3d(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType,
    ),
    #[xml(tag = "a:extLst")]
    AExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    ),
}
/// Defines the GraphicFrame Class.
/// When the object is serialized out as xml, it's qualified name is wpg:graphicFrame.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wpg:graphicFrame")]
pub struct GraphicFrame {
    /// _
    #[xml(child = "wpg:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    /// _
    #[xml(child = "wpg:cNvFrPr")]
    pub non_visual_graphic_frame_properties: NonVisualGraphicFrameProperties,
    /// _
    #[xml(child = "wpg:xfrm")]
    pub transform2_d: Transform2D,
    /// _
    #[xml(child = "a:graphic")]
    pub graphic: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Graphic,
    /// _
    #[xml(child = "wpg:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
