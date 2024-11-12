/// Defines the WordprocessingShape Class.
/// When the object is serialized out as xml, it's qualified name is wps:wsp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wps:wsp")]
pub struct WordprocessingShape {
    /// normalEastAsianFlow
    /// Represents the following attribute in the schema: :normalEastAsianFlow
    #[xml(attr = "normalEastAsianFlow")]
    pub normal_east_asian_flow: Option<bool>,
    #[xml(
        child = "wps:cNvPr",
        child = "wps:cNvSpPr",
        child = "wps:cNvCnPr",
        child = "wps:spPr",
        child = "wps:style",
        child = "wps:extLst",
        child = "wps:txbx",
        child = "wps:linkedTxbx",
        child = "wps:bodyPr",
    )]
    pub children: Vec<WordprocessingShapeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum WordprocessingShapeChildChoice {
    #[xml(tag = "wps:cNvPr")]
    WpsCNvPr(NonVisualDrawingProperties),
    #[xml(tag = "wps:cNvSpPr")]
    WpsCNvSpPr(NonVisualDrawingShapeProperties),
    #[xml(tag = "wps:cNvCnPr")]
    WpsCNvCnPr(NonVisualConnectorProperties),
    #[xml(tag = "wps:spPr")]
    WpsSpPr(ShapeProperties),
    #[xml(tag = "wps:style")]
    WpsStyle(ShapeStyle),
    #[xml(tag = "wps:extLst")]
    WpsExtLst(OfficeArtExtensionList),
    #[xml(tag = "wps:txbx")]
    WpsTxbx(TextBoxInfo2),
    #[xml(tag = "wps:linkedTxbx")]
    WpsLinkedTxbx(LinkedTextBox),
    #[xml(tag = "wps:bodyPr")]
    WpsBodyPr(TextBodyProperties),
}
/// Defines the OfficeArtExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is wps:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wps:extLst")]
pub struct OfficeArtExtensionList {
    #[xml(child = "a:ext")]
    pub children: Vec<OfficeArtExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeArtExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension),
}
/// Defines the NonVisualDrawingProperties Class.
/// When the object is serialized out as xml, it's qualified name is wps:cNvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wps:cNvPr")]
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
/// Defines the NonVisualDrawingShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is wps:cNvSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wps:cNvSpPr")]
pub struct NonVisualDrawingShapeProperties {
    /// Text Box
    /// Represents the following attribute in the schema: :txBox
    #[xml(attr = "txBox")]
    pub text_box: Option<bool>,
    ///Shape Locks
    #[xml(child = "a:spLocks")]
    pub shape_locks: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapeLocks,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Defines the NonVisualConnectorProperties Class.
/// When the object is serialized out as xml, it's qualified name is wps:cNvCnPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wps:cNvCnPr")]
pub struct NonVisualConnectorProperties {
    ///Connection Shape Locks
    #[xml(child = "a:cxnSpLocks")]
    pub connection_shape_locks: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ConnectionShapeLocks,
    >,
    ///Connection Start
    #[xml(child = "a:stCxn")]
    pub start_connection: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::StartConnection,
    >,
    ///Connection End
    #[xml(child = "a:endCxn")]
    pub end_connection: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EndConnection,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Defines the ShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is wps:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wps:spPr")]
pub struct ShapeProperties {
    /// Black and White Mode
    /// Represents the following attribute in the schema: :bwMode
    #[xml(attr = "bwMode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    #[xml(
        child = "a:xfrm",
        child = "a:custGeom",
        child = "a:prstGeom",
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
        child = "a:ln",
        child = "a:effectLst",
        child = "a:effectDag",
        child = "a:scene3d",
        child = "a:sp3d",
        child = "a:extLst",
    )]
    pub children: Vec<ShapePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShapePropertiesChildChoice {
    #[xml(tag = "a:xfrm")]
    AXfrm(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Transform2D),
    #[xml(tag = "a:custGeom")]
    ACustGeom(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomGeometry,
    ),
    #[xml(tag = "a:prstGeom")]
    APrstGeom(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetGeometry,
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
    #[xml(tag = "a:ln")]
    ALn(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline),
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
    #[xml(tag = "a:sp3d")]
    ASp3d(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType),
    #[xml(tag = "a:extLst")]
    AExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapePropertiesExtensionList,
    ),
}
/// Defines the ShapeStyle Class.
/// When the object is serialized out as xml, it's qualified name is wps:style.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wps:style")]
pub struct ShapeStyle {
    /// _
    #[xml(child = "a:lnRef")]
    pub line_reference: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineReference,
    /// _
    #[xml(child = "a:fillRef")]
    pub fill_reference: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FillReference,
    /// _
    #[xml(child = "a:effectRef")]
    pub effect_reference: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectReference,
    ///Font Reference
    #[xml(child = "a:fontRef")]
    pub font_reference: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FontReference,
}
/// Defines the TextBoxInfo2 Class.
/// When the object is serialized out as xml, it's qualified name is wps:txbx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wps:txbx")]
pub struct TextBoxInfo2 {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<i16>,
    /// _
    #[xml(child = "w:txbxContent")]
    pub text_box_content: Option<
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::TextBoxContent,
    >,
    /// _
    #[xml(child = "wps:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LinkedTextBox Class.
/// When the object is serialized out as xml, it's qualified name is wps:linkedTxbx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wps:linkedTxbx")]
pub struct LinkedTextBox {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i16,
    /// seq
    /// Represents the following attribute in the schema: :seq
    #[xml(attr = "seq")]
    pub sequence: i16,
    /// _
    #[xml(child = "wps:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TextBodyProperties Class.
/// When the object is serialized out as xml, it's qualified name is wps:bodyPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wps:bodyPr")]
pub struct TextBodyProperties {
    /// Rotation
    /// Represents the following attribute in the schema: :rot
    #[xml(attr = "rot")]
    pub rotation: Option<i32>,
    /// Paragraph Spacing
    /// Represents the following attribute in the schema: :spcFirstLastPara
    #[xml(attr = "spcFirstLastPara")]
    pub use_paragraph_spacing: Option<bool>,
    /// Text Vertical Overflow
    /// Represents the following attribute in the schema: :vertOverflow
    #[xml(attr = "vertOverflow")]
    pub vertical_overflow: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalOverflowValues,
    >,
    /// Text Horizontal Overflow
    /// Represents the following attribute in the schema: :horzOverflow
    #[xml(attr = "horzOverflow")]
    pub horizontal_overflow: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextHorizontalOverflowValues,
    >,
    /// Vertical Text
    /// Represents the following attribute in the schema: :vert
    #[xml(attr = "vert")]
    pub vertical: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalValues,
    >,
    /// Text Wrapping Type
    /// Represents the following attribute in the schema: :wrap
    #[xml(attr = "wrap")]
    pub wrap: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextWrappingValues,
    >,
    /// Left Inset
    /// Represents the following attribute in the schema: :lIns
    #[xml(attr = "lIns")]
    pub left_inset: Option<i32>,
    /// Top Inset
    /// Represents the following attribute in the schema: :tIns
    #[xml(attr = "tIns")]
    pub top_inset: Option<i32>,
    /// Right Inset
    /// Represents the following attribute in the schema: :rIns
    #[xml(attr = "rIns")]
    pub right_inset: Option<i32>,
    /// Bottom Inset
    /// Represents the following attribute in the schema: :bIns
    #[xml(attr = "bIns")]
    pub bottom_inset: Option<i32>,
    /// Number of Columns
    /// Represents the following attribute in the schema: :numCol
    #[xml(attr = "numCol")]
    pub column_count: Option<i32>,
    /// Space Between Columns
    /// Represents the following attribute in the schema: :spcCol
    #[xml(attr = "spcCol")]
    pub column_spacing: Option<i32>,
    /// Columns Right-To-Left
    /// Represents the following attribute in the schema: :rtlCol
    #[xml(attr = "rtlCol")]
    pub right_to_left_columns: Option<bool>,
    /// From WordArt
    /// Represents the following attribute in the schema: :fromWordArt
    #[xml(attr = "fromWordArt")]
    pub from_word_art: Option<bool>,
    /// Anchor
    /// Represents the following attribute in the schema: :anchor
    #[xml(attr = "anchor")]
    pub anchor: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextAnchoringTypeValues,
    >,
    /// Anchor Center
    /// Represents the following attribute in the schema: :anchorCtr
    #[xml(attr = "anchorCtr")]
    pub anchor_center: Option<bool>,
    /// Force Anti-Alias
    /// Represents the following attribute in the schema: :forceAA
    #[xml(attr = "forceAA")]
    pub force_anti_alias: Option<bool>,
    /// Text Upright
    /// Represents the following attribute in the schema: :upright
    #[xml(attr = "upright")]
    pub up_right: Option<bool>,
    /// Compatible Line Spacing
    /// Represents the following attribute in the schema: :compatLnSpc
    #[xml(attr = "compatLnSpc")]
    pub compatible_line_spacing: Option<bool>,
    #[xml(
        child = "a:prstTxWarp",
        child = "a:noAutofit",
        child = "a:normAutofit",
        child = "a:spAutoFit",
        child = "a:scene3d",
        child = "a:sp3d",
        child = "a:flatTx",
        child = "a:extLst",
    )]
    pub children: Vec<TextBodyPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextBodyPropertiesChildChoice {
    #[xml(tag = "a:prstTxWarp")]
    APrstTxWarp(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetTextWarp,
    ),
    #[xml(tag = "a:noAutofit")]
    ANoAutofit(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoAutoFit,
    ),
    #[xml(tag = "a:normAutofit")]
    ANormAutofit(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NormalAutoFit,
    ),
    #[xml(tag = "a:spAutoFit")]
    ASpAutoFit(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapeAutoFit,
    ),
    #[xml(tag = "a:scene3d")]
    AScene3d(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType,
    ),
    #[xml(tag = "a:sp3d")]
    ASp3d(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType),
    #[xml(tag = "a:flatTx")]
    AFlatTx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FlatText),
    #[xml(tag = "a:extLst")]
    AExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    ),
}
