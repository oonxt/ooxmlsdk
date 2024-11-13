#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EditAsValues {
    #[default]
    TwoCell,
    OneCell,
    Absolute,
}
crate::__string_enum! {
    EditAsValues { TwoCell = "twoCell", OneCell = "oneCell", Absolute = "absolute", }
}
/// Two Cell Anchor Shape Size.
/// When the object is serialized out as xml, it's qualified name is xdr:twoCellAnchor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:twoCellAnchor")]
pub struct TwoCellAnchor {
    /// Positioning and Resizing Behaviors
    /// Represents the following attribute in the schema: :editAs
    #[xml(attr = "editAs")]
    pub edit_as: Option<EditAsValues>,
    #[xml(
        child = "xdr:from",
        child = "xdr:to",
        child = "xdr:sp",
        child = "xdr:grpSp",
        child = "xdr:graphicFrame",
        child = "xdr:cxnSp",
        child = "xdr:pic",
        child = "xdr:contentPart",
        child = "xdr:clientData",
    )]
    pub children: Vec<TwoCellAnchorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TwoCellAnchorChildChoice {
    #[xml(tag = "xdr:from")]
    XdrFrom(FromMarker),
    #[xml(tag = "xdr:to")]
    XdrTo(ToMarker),
    #[xml(tag = "xdr:sp")]
    XdrSp(Shape),
    #[xml(tag = "xdr:grpSp")]
    XdrGrpSp(GroupShape),
    #[xml(tag = "xdr:graphicFrame")]
    XdrGraphicFrame(GraphicFrame),
    #[xml(tag = "xdr:cxnSp")]
    XdrCxnSp(ConnectionShape),
    #[xml(tag = "xdr:pic")]
    XdrPic(Picture),
    #[xml(tag = "xdr:contentPart")]
    XdrContentPart(ContentPart),
    #[xml(tag = "xdr:clientData")]
    XdrClientData(ClientData),
}
/// One Cell Anchor Shape Size.
/// When the object is serialized out as xml, it's qualified name is xdr:oneCellAnchor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:oneCellAnchor")]
pub struct OneCellAnchor {
    #[xml(
        child = "xdr:from",
        child = "xdr:ext",
        child = "xdr:sp",
        child = "xdr:grpSp",
        child = "xdr:graphicFrame",
        child = "xdr:cxnSp",
        child = "xdr:pic",
        child = "xdr:contentPart",
        child = "xdr:clientData",
    )]
    pub children: Vec<OneCellAnchorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OneCellAnchorChildChoice {
    #[xml(tag = "xdr:from")]
    XdrFrom(FromMarker),
    #[xml(tag = "xdr:ext")]
    XdrExt(Extent),
    #[xml(tag = "xdr:sp")]
    XdrSp(Shape),
    #[xml(tag = "xdr:grpSp")]
    XdrGrpSp(GroupShape),
    #[xml(tag = "xdr:graphicFrame")]
    XdrGraphicFrame(GraphicFrame),
    #[xml(tag = "xdr:cxnSp")]
    XdrCxnSp(ConnectionShape),
    #[xml(tag = "xdr:pic")]
    XdrPic(Picture),
    #[xml(tag = "xdr:contentPart")]
    XdrContentPart(ContentPart),
    #[xml(tag = "xdr:clientData")]
    XdrClientData(ClientData),
}
/// Absolute Anchor Shape Size.
/// When the object is serialized out as xml, it's qualified name is xdr:absoluteAnchor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:absoluteAnchor")]
pub struct AbsoluteAnchor {
    #[xml(
        child = "xdr:pos",
        child = "xdr:ext",
        child = "xdr:sp",
        child = "xdr:grpSp",
        child = "xdr:graphicFrame",
        child = "xdr:cxnSp",
        child = "xdr:pic",
        child = "xdr:contentPart",
        child = "xdr:clientData",
    )]
    pub children: Vec<AbsoluteAnchorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum AbsoluteAnchorChildChoice {
    #[xml(tag = "xdr:pos")]
    XdrPos(Position),
    #[xml(tag = "xdr:ext")]
    XdrExt(Extent),
    #[xml(tag = "xdr:sp")]
    XdrSp(Shape),
    #[xml(tag = "xdr:grpSp")]
    XdrGrpSp(GroupShape),
    #[xml(tag = "xdr:graphicFrame")]
    XdrGraphicFrame(GraphicFrame),
    #[xml(tag = "xdr:cxnSp")]
    XdrCxnSp(ConnectionShape),
    #[xml(tag = "xdr:pic")]
    XdrPic(Picture),
    #[xml(tag = "xdr:contentPart")]
    XdrContentPart(ContentPart),
    #[xml(tag = "xdr:clientData")]
    XdrClientData(ClientData),
}
/// Shape.
/// When the object is serialized out as xml, it's qualified name is xdr:sp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:sp")]
pub struct Shape {
    /// Reference to Custom Function
    /// Represents the following attribute in the schema: :macro
    #[xml(attr = "macro")]
    pub r#macro: Option<String>,
    /// Text Link
    /// Represents the following attribute in the schema: :textlink
    #[xml(attr = "textlink")]
    pub text_link: Option<String>,
    /// Lock Text Flag
    /// Represents the following attribute in the schema: :fLocksText
    #[xml(attr = "fLocksText")]
    pub lock_text: Option<bool>,
    /// Publish to Server Flag
    /// Represents the following attribute in the schema: :fPublished
    #[xml(attr = "fPublished")]
    pub published: Option<bool>,
    ///Non-Visual Properties for a Shape
    #[xml(child = "xdr:nvSpPr")]
    pub non_visual_shape_properties: NonVisualShapeProperties,
    ///Shape Properties
    #[xml(child = "xdr:spPr")]
    pub shape_properties: ShapeProperties,
    /// _
    #[xml(child = "xdr:style")]
    pub shape_style: Option<ShapeStyle>,
    ///Shape Text Body
    #[xml(child = "xdr:txBody")]
    pub text_body: Option<TextBody>,
}
/// Group Shape.
/// When the object is serialized out as xml, it's qualified name is xdr:grpSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:grpSp")]
pub struct GroupShape {
    #[xml(
        child = "xdr:nvGrpSpPr",
        child = "xdr:grpSpPr",
        child = "xdr:sp",
        child = "xdr:grpSp",
        child = "xdr:graphicFrame",
        child = "xdr:cxnSp",
        child = "xdr:pic",
        child = "xdr14:contentPart",
    )]
    pub children: Vec<GroupShapeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GroupShapeChildChoice {
    #[xml(tag = "xdr:nvGrpSpPr")]
    XdrNvGrpSpPr(NonVisualGroupShapeProperties),
    #[xml(tag = "xdr:grpSpPr")]
    XdrGrpSpPr(GroupShapeProperties),
    #[xml(tag = "xdr:sp")]
    XdrSp(Shape),
    #[xml(tag = "xdr:grpSp")]
    XdrGrpSp(GroupShape),
    #[xml(tag = "xdr:graphicFrame")]
    XdrGraphicFrame(GraphicFrame),
    #[xml(tag = "xdr:cxnSp")]
    XdrCxnSp(ConnectionShape),
    #[xml(tag = "xdr:pic")]
    XdrPic(Picture),
    #[xml(tag = "xdr14:contentPart")]
    Xdr14ContentPart(
        crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::ContentPart,
    ),
}
/// Graphic Frame.
/// When the object is serialized out as xml, it's qualified name is xdr:graphicFrame.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:graphicFrame")]
pub struct GraphicFrame {
    /// Reference To Custom Function
    /// Represents the following attribute in the schema: :macro
    #[xml(attr = "macro")]
    pub r#macro: Option<String>,
    /// Publish to Server Flag
    /// Represents the following attribute in the schema: :fPublished
    #[xml(attr = "fPublished")]
    pub published: Option<bool>,
    ///Non-Visual Properties for a Graphic Frame
    #[xml(child = "xdr:nvGraphicFramePr")]
    pub non_visual_graphic_frame_properties: NonVisualGraphicFrameProperties,
    ///2D Transform for Graphic Frames
    #[xml(child = "xdr:xfrm")]
    pub transform: Transform,
    /// _
    #[xml(child = "a:graphic")]
    pub graphic: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Graphic,
}
/// Connection Shape.
/// When the object is serialized out as xml, it's qualified name is xdr:cxnSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:cxnSp")]
pub struct ConnectionShape {
    /// Reference to Custom Function
    /// Represents the following attribute in the schema: :macro
    #[xml(attr = "macro")]
    pub r#macro: Option<String>,
    /// Publish to Server Flag
    /// Represents the following attribute in the schema: :fPublished
    #[xml(attr = "fPublished")]
    pub published: Option<bool>,
    ///Non-Visual Properties for a Connection Shape
    #[xml(child = "xdr:nvCxnSpPr")]
    pub non_visual_connection_shape_properties: NonVisualConnectionShapeProperties,
    ///Connector Shape Properties
    #[xml(child = "xdr:spPr")]
    pub shape_properties: ShapeProperties,
    /// _
    #[xml(child = "xdr:style")]
    pub shape_style: Option<ShapeStyle>,
}
/// Defines the Picture Class.
/// When the object is serialized out as xml, it's qualified name is xdr:pic.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:pic")]
pub struct Picture {
    /// Reference To Custom Function
    /// Represents the following attribute in the schema: :macro
    #[xml(attr = "macro")]
    pub r#macro: Option<String>,
    /// Publish to Server Flag
    /// Represents the following attribute in the schema: :fPublished
    #[xml(attr = "fPublished")]
    pub published: Option<bool>,
    ///Non-Visual Properties for a Picture
    #[xml(child = "xdr:nvPicPr")]
    pub non_visual_picture_properties: NonVisualPictureProperties,
    ///Picture Fill
    #[xml(child = "xdr:blipFill")]
    pub blip_fill: BlipFill,
    /// _
    #[xml(child = "xdr:spPr")]
    pub shape_properties: ShapeProperties,
    ///Shape Style
    #[xml(child = "xdr:style")]
    pub shape_style: Option<ShapeStyle>,
}
/// Defines the ContentPart Class.
/// When the object is serialized out as xml, it's qualified name is xdr:contentPart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:contentPart")]
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
    #[xml(child = "xdr14:nvContentPartPr")]
    pub excel_non_visual_content_part_shape_properties: Option<
        crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::ExcelNonVisualContentPartShapeProperties,
    >,
    /// _
    #[xml(child = "xdr14:nvPr")]
    pub application_non_visual_drawing_properties: Option<
        crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::ApplicationNonVisualDrawingProperties,
    >,
    /// _
    #[xml(child = "xdr14:xfrm")]
    pub transform2_d: Option<
        crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::Transform2D,
    >,
    /// _
    #[xml(child = "xdr14:extLst")]
    pub office_art_extension_list: Option<
        crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::OfficeArtExtensionList,
    >,
}
/// Worksheet Drawing.
/// When the object is serialized out as xml, it's qualified name is xdr:wsDr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:wsDr")]
pub struct WorksheetDrawing {
    #[xml(attr = "xmlns", with = "worksheet_drawing_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "xdr:twoCellAnchor",
        child = "xdr:oneCellAnchor",
        child = "xdr:absoluteAnchor",
    )]
    pub children: Vec<WorksheetDrawingChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum WorksheetDrawingChildChoice {
    #[xml(tag = "xdr:twoCellAnchor")]
    XdrTwoCellAnchor(TwoCellAnchor),
    #[xml(tag = "xdr:oneCellAnchor")]
    XdrOneCellAnchor(OneCellAnchor),
    #[xml(tag = "xdr:absoluteAnchor")]
    XdrAbsoluteAnchor(AbsoluteAnchor),
}
mod worksheet_drawing_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing")
    }
}
/// Non-Visual Properties for a Shape.
/// When the object is serialized out as xml, it's qualified name is xdr:nvSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:nvSpPr")]
pub struct NonVisualShapeProperties {
    ///Non-Visual Drawing Properties
    #[xml(child = "xdr:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Connection Non-Visual Shape Properties
    #[xml(child = "xdr:cNvSpPr")]
    pub non_visual_shape_drawing_properties: NonVisualShapeDrawingProperties,
}
/// Shape Properties.
/// When the object is serialized out as xml, it's qualified name is xdr:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:spPr")]
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
/// When the object is serialized out as xml, it's qualified name is xdr:style.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:style")]
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
/// Shape Text Body.
/// When the object is serialized out as xml, it's qualified name is xdr:txBody.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:txBody")]
pub struct TextBody {
    ///Body Properties
    #[xml(child = "a:bodyPr")]
    pub body_properties: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties,
    ///Text List Styles
    #[xml(child = "a:lstStyle")]
    pub list_style: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle,
    >,
    /// _
    #[xml(child = "a:p")]
    pub a_p: Vec<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph,
    >,
}
/// Non-Visual Properties for a Connection Shape.
/// When the object is serialized out as xml, it's qualified name is xdr:nvCxnSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:nvCxnSpPr")]
pub struct NonVisualConnectionShapeProperties {
    ///Connection Non-Visual Properties
    #[xml(child = "xdr:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Connector Shape Drawing Properties
    #[xml(child = "xdr:cNvCxnSpPr")]
    pub non_visual_connector_shape_drawing_properties: NonVisualConnectorShapeDrawingProperties,
}
/// Non-Visual Properties for a Picture.
/// When the object is serialized out as xml, it's qualified name is xdr:nvPicPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:nvPicPr")]
pub struct NonVisualPictureProperties {
    /// _
    #[xml(child = "xdr:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Picture Drawing Properties
    #[xml(child = "xdr:cNvPicPr")]
    pub non_visual_picture_drawing_properties: NonVisualPictureDrawingProperties,
}
/// Picture Fill.
/// When the object is serialized out as xml, it's qualified name is xdr:blipFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:blipFill")]
pub struct BlipFill {
    /// Rotate With Shape
    /// Represents the following attribute in the schema: :rotWithShape
    #[xml(attr = "rotWithShape")]
    pub rotate_with_shape: Option<bool>,
    #[xml(child = "a:blip", child = "a:srcRect", child = "a:tile", child = "a:stretch")]
    pub children: Vec<BlipFillChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BlipFillChildChoice {
    #[xml(tag = "a:blip")]
    ABlip(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blip),
    #[xml(tag = "a:srcRect")]
    ASrcRect(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SourceRectangle,
    ),
    #[xml(tag = "a:tile")]
    ATile(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Tile),
    #[xml(tag = "a:stretch")]
    AStretch(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Stretch),
}
/// Non-Visual Properties for a Graphic Frame.
/// When the object is serialized out as xml, it's qualified name is xdr:nvGraphicFramePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:nvGraphicFramePr")]
pub struct NonVisualGraphicFrameProperties {
    ///Connection Non-Visual Properties
    #[xml(child = "xdr:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Graphic Frame Drawing Properties
    #[xml(child = "xdr:cNvGraphicFramePr")]
    pub non_visual_graphic_frame_drawing_properties: NonVisualGraphicFrameDrawingProperties,
}
/// 2D Transform for Graphic Frames.
/// When the object is serialized out as xml, it's qualified name is xdr:xfrm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:xfrm")]
pub struct Transform {
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
/// Column).
/// When the object is serialized out as xml, it's qualified name is xdr:col.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:col")]
pub struct ColumnId {
    #[xml(text)]
    pub child: i32,
}
/// Column Offset.
/// When the object is serialized out as xml, it's qualified name is xdr:colOff.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:colOff")]
pub struct ColumnOffset {
    #[xml(text)]
    pub child: i32,
}
/// Row Offset.
/// When the object is serialized out as xml, it's qualified name is xdr:rowOff.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:rowOff")]
pub struct RowOffset {
    #[xml(text)]
    pub child: i32,
}
/// Row.
/// When the object is serialized out as xml, it's qualified name is xdr:row.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:row")]
pub struct RowId {
    #[xml(text)]
    pub child: i32,
}
/// Starting Anchor Point.
/// When the object is serialized out as xml, it's qualified name is xdr:from.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:from")]
pub struct FromMarker {
    ///Column)
    #[xml(child = "xdr:col")]
    pub column_id: ColumnId,
    ///Column Offset
    #[xml(child = "xdr:colOff")]
    pub column_offset: ColumnOffset,
    ///Row
    #[xml(child = "xdr:row")]
    pub row_id: RowId,
    ///Row Offset
    #[xml(child = "xdr:rowOff")]
    pub row_offset: RowOffset,
}
/// Ending Anchor Point.
/// When the object is serialized out as xml, it's qualified name is xdr:to.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:to")]
pub struct ToMarker {
    ///Column)
    #[xml(child = "xdr:col")]
    pub column_id: ColumnId,
    ///Column Offset
    #[xml(child = "xdr:colOff")]
    pub column_offset: ColumnOffset,
    ///Row
    #[xml(child = "xdr:row")]
    pub row_id: RowId,
    ///Row Offset
    #[xml(child = "xdr:rowOff")]
    pub row_offset: RowOffset,
}
/// Defines the MarkerType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct MarkerType {}
/// Client Data.
/// When the object is serialized out as xml, it's qualified name is xdr:clientData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:clientData")]
pub struct ClientData {
    /// Locks With Sheet Flag
    /// Represents the following attribute in the schema: :fLocksWithSheet
    #[xml(attr = "fLocksWithSheet")]
    pub lock_with_sheet: Option<bool>,
    /// Prints With Sheet Flag
    /// Represents the following attribute in the schema: :fPrintsWithSheet
    #[xml(attr = "fPrintsWithSheet")]
    pub print_with_sheet: Option<bool>,
}
/// Defines the Extent Class.
/// When the object is serialized out as xml, it's qualified name is xdr:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:ext")]
pub struct Extent {
    /// Extent Length
    /// Represents the following attribute in the schema: :cx
    #[xml(attr = "cx")]
    pub cx: i32,
    /// Extent Width
    /// Represents the following attribute in the schema: :cy
    #[xml(attr = "cy")]
    pub cy: i32,
}
/// Position.
/// When the object is serialized out as xml, it's qualified name is xdr:pos.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:pos")]
pub struct Position {
    /// X-Axis Coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i32,
    /// Y-Axis Coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i32,
}
/// Non-Visual Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is xdr:cNvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:cNvPr")]
pub struct NonVisualDrawingProperties {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: u32,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// descr
    /// Represents the following attribute in the schema: :descr
    #[xml(attr = "descr")]
    pub description: Option<String>,
    /// hidden
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// _
    #[xml(child = "a:hlinkClick")]
    pub hyperlink_on_click: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnClick,
    >,
    /// _
    #[xml(child = "a:hlinkHover")]
    pub hyperlink_on_hover: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnHover,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub non_visual_drawing_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualDrawingPropertiesExtensionList,
    >,
}
/// Connection Non-Visual Shape Properties.
/// When the object is serialized out as xml, it's qualified name is xdr:cNvSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:cNvSpPr")]
pub struct NonVisualShapeDrawingProperties {
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
/// Non-Visual Connector Shape Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is xdr:cNvCxnSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:cNvCxnSpPr")]
pub struct NonVisualConnectorShapeDrawingProperties {
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
/// Non-Visual Picture Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is xdr:cNvPicPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:cNvPicPr")]
pub struct NonVisualPictureDrawingProperties {
    /// preferRelativeResize
    /// Represents the following attribute in the schema: :preferRelativeResize
    #[xml(attr = "preferRelativeResize")]
    pub prefer_relative_resize: Option<bool>,
    /// _
    #[xml(child = "a:picLocks")]
    pub picture_locks: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PictureLocks,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub non_visual_picture_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualPicturePropertiesExtensionList,
    >,
}
/// Non-Visual Graphic Frame Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is xdr:cNvGraphicFramePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:cNvGraphicFramePr")]
pub struct NonVisualGraphicFrameDrawingProperties {
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
/// Non-Visual Group Shape Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is xdr:cNvGrpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:cNvGrpSpPr")]
pub struct NonVisualGroupShapeDrawingProperties {
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
/// Non-Visual Properties for a Group Shape.
/// When the object is serialized out as xml, it's qualified name is xdr:nvGrpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:nvGrpSpPr")]
pub struct NonVisualGroupShapeProperties {
    ///Connection Non-Visual Properties
    #[xml(child = "xdr:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Group Shape Drawing Properties
    #[xml(child = "xdr:cNvGrpSpPr")]
    pub non_visual_group_shape_drawing_properties: NonVisualGroupShapeDrawingProperties,
}
/// Group Shape Properties.
/// When the object is serialized out as xml, it's qualified name is xdr:grpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xdr:grpSpPr")]
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
