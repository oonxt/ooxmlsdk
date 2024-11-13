/// Relative Anchor Shape Size.
/// When the object is serialized out as xml, it's qualified name is cdr:relSizeAnchor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:relSizeAnchor")]
pub struct RelativeAnchorSize {
    #[xml(
        child = "cdr:from",
        child = "cdr:to",
        child = "cdr:sp",
        child = "cdr:grpSp",
        child = "cdr:graphicFrame",
        child = "cdr:cxnSp",
        child = "cdr:pic",
        child = "cdr14:contentPart",
    )]
    pub children: Vec<RelativeAnchorSizeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RelativeAnchorSizeChildChoice {
    #[xml(tag = "cdr:from")]
    CdrFrom(FromAnchor),
    #[xml(tag = "cdr:to")]
    CdrTo(ToAnchor),
    #[xml(tag = "cdr:sp")]
    CdrSp(Shape),
    #[xml(tag = "cdr:grpSp")]
    CdrGrpSp(GroupShape),
    #[xml(tag = "cdr:graphicFrame")]
    CdrGraphicFrame(GraphicFrame),
    #[xml(tag = "cdr:cxnSp")]
    CdrCxnSp(ConnectionShape),
    #[xml(tag = "cdr:pic")]
    CdrPic(Picture),
    #[xml(tag = "cdr14:contentPart")]
    Cdr14ContentPart(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_chart_drawing::ContentPart,
    ),
}
/// Absolute Anchor Shape Size.
/// When the object is serialized out as xml, it's qualified name is cdr:absSizeAnchor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:absSizeAnchor")]
pub struct AbsoluteAnchorSize {
    #[xml(
        child = "cdr:from",
        child = "cdr:ext",
        child = "cdr:sp",
        child = "cdr:grpSp",
        child = "cdr:graphicFrame",
        child = "cdr:cxnSp",
        child = "cdr:pic",
        child = "cdr14:contentPart",
    )]
    pub children: Vec<AbsoluteAnchorSizeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum AbsoluteAnchorSizeChildChoice {
    #[xml(tag = "cdr:from")]
    CdrFrom(FromAnchor),
    #[xml(tag = "cdr:ext")]
    CdrExt(Extent),
    #[xml(tag = "cdr:sp")]
    CdrSp(Shape),
    #[xml(tag = "cdr:grpSp")]
    CdrGrpSp(GroupShape),
    #[xml(tag = "cdr:graphicFrame")]
    CdrGraphicFrame(GraphicFrame),
    #[xml(tag = "cdr:cxnSp")]
    CdrCxnSp(ConnectionShape),
    #[xml(tag = "cdr:pic")]
    CdrPic(Picture),
    #[xml(tag = "cdr14:contentPart")]
    Cdr14ContentPart(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_chart_drawing::ContentPart,
    ),
}
/// Shape Definition.
/// When the object is serialized out as xml, it's qualified name is cdr:sp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:sp")]
pub struct Shape {
    /// Reference to Custom Function
    /// Represents the following attribute in the schema: :macro
    #[xml(attr = "macro")]
    pub r#macro: Option<String>,
    /// Text Link
    /// Represents the following attribute in the schema: :textlink
    #[xml(attr = "textlink")]
    pub text_link: Option<String>,
    /// Lock Text
    /// Represents the following attribute in the schema: :fLocksText
    #[xml(attr = "fLocksText")]
    pub lock_text: Option<bool>,
    /// Publish to Server
    /// Represents the following attribute in the schema: :fPublished
    #[xml(attr = "fPublished")]
    pub published: Option<bool>,
    ///Non-Visual Shape Properties
    #[xml(child = "cdr:nvSpPr")]
    pub non_visual_shape_properties: NonVisualShapeProperties,
    ///Shape Properties
    #[xml(child = "cdr:spPr")]
    pub shape_properties: ShapeProperties,
    ///Shape Style
    #[xml(child = "cdr:style")]
    pub style: Option<Style>,
    ///Shape Text Body
    #[xml(child = "cdr:txBody")]
    pub text_body: Option<TextBody>,
}
/// Group Shape.
/// When the object is serialized out as xml, it's qualified name is cdr:grpSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:grpSp")]
pub struct GroupShape {
    #[xml(
        child = "cdr:nvGrpSpPr",
        child = "cdr:grpSpPr",
        child = "cdr:sp",
        child = "cdr:grpSp",
        child = "cdr:graphicFrame",
        child = "cdr:cxnSp",
        child = "cdr:pic",
        child = "cdr14:contentPart",
    )]
    pub children: Vec<GroupShapeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GroupShapeChildChoice {
    #[xml(tag = "cdr:nvGrpSpPr")]
    CdrNvGrpSpPr(NonVisualGroupShapeProperties),
    #[xml(tag = "cdr:grpSpPr")]
    CdrGrpSpPr(GroupShapeProperties),
    #[xml(tag = "cdr:sp")]
    CdrSp(Shape),
    #[xml(tag = "cdr:grpSp")]
    CdrGrpSp(GroupShape),
    #[xml(tag = "cdr:graphicFrame")]
    CdrGraphicFrame(GraphicFrame),
    #[xml(tag = "cdr:cxnSp")]
    CdrCxnSp(ConnectionShape),
    #[xml(tag = "cdr:pic")]
    CdrPic(Picture),
    #[xml(tag = "cdr14:contentPart")]
    Cdr14ContentPart(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_chart_drawing::ContentPart,
    ),
}
/// Graphic Frame.
/// When the object is serialized out as xml, it's qualified name is cdr:graphicFrame.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:graphicFrame")]
pub struct GraphicFrame {
    /// Reference to Custom Function
    /// Represents the following attribute in the schema: :macro
    #[xml(attr = "macro")]
    pub r#macro: Option<String>,
    /// Publish To Server
    /// Represents the following attribute in the schema: :fPublished
    #[xml(attr = "fPublished")]
    pub published: Option<bool>,
    ///Non-Visual Graphic Frame Properties
    #[xml(child = "cdr:nvGraphicFramePr")]
    pub non_visual_graphic_frame_properties: NonVisualGraphicFrameProperties,
    ///Graphic Frame Transform
    #[xml(child = "cdr:xfrm")]
    pub transform: Transform,
    ///Graphical Object
    #[xml(child = "a:graphic")]
    pub graphic: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Graphic,
}
/// Connection Shape.
/// When the object is serialized out as xml, it's qualified name is cdr:cxnSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:cxnSp")]
pub struct ConnectionShape {
    /// Reference to Custom Function
    /// Represents the following attribute in the schema: :macro
    #[xml(attr = "macro")]
    pub r#macro: Option<String>,
    /// Publish to Server
    /// Represents the following attribute in the schema: :fPublished
    #[xml(attr = "fPublished")]
    pub published: Option<bool>,
    ///Connector Non Visual Properties
    #[xml(child = "cdr:nvCxnSpPr")]
    pub non_visual_connector_shape_drawing_properties: NonVisualConnectorShapeDrawingProperties,
    ///Shape Properties
    #[xml(child = "cdr:spPr")]
    pub shape_properties: ShapeProperties,
    ///Connection Shape Style
    #[xml(child = "cdr:style")]
    pub style: Option<Style>,
}
/// Defines the Picture Class.
/// When the object is serialized out as xml, it's qualified name is cdr:pic.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:pic")]
pub struct Picture {
    /// Reference to Custom Function
    /// Represents the following attribute in the schema: :macro
    #[xml(attr = "macro")]
    pub r#macro: Option<String>,
    /// Publish to Server
    /// Represents the following attribute in the schema: :fPublished
    #[xml(attr = "fPublished")]
    pub published: Option<bool>,
    ///Non-Visual Picture Properties
    #[xml(child = "cdr:nvPicPr")]
    pub non_visual_picture_properties: NonVisualPictureProperties,
    ///Picture Fill
    #[xml(child = "cdr:blipFill")]
    pub blip_fill: BlipFill,
    /// _
    #[xml(child = "cdr:spPr")]
    pub shape_properties: ShapeProperties,
    /// _
    #[xml(child = "cdr:style")]
    pub style: Option<Style>,
}
/// Chart Non Visual Properties.
/// When the object is serialized out as xml, it's qualified name is cdr:cNvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:cNvPr")]
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
/// Non-Visual Shape Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is cdr:cNvSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:cNvSpPr")]
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
/// Non-Visual Shape Properties.
/// When the object is serialized out as xml, it's qualified name is cdr:nvSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:nvSpPr")]
pub struct NonVisualShapeProperties {
    ///Chart Non Visual Properties
    #[xml(child = "cdr:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Shape Drawing Properties
    #[xml(child = "cdr:cNvSpPr")]
    pub non_visual_shape_drawing_properties: NonVisualShapeDrawingProperties,
}
/// Shape Properties.
/// When the object is serialized out as xml, it's qualified name is cdr:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:spPr")]
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
/// Shape Style.
/// When the object is serialized out as xml, it's qualified name is cdr:style.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:style")]
pub struct Style {
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
/// When the object is serialized out as xml, it's qualified name is cdr:txBody.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:txBody")]
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
/// Non-Visual Connection Shape Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is cdr:cNvCxnSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:cNvCxnSpPr")]
pub struct NonVisualConnectionShapeProperties {
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
/// Connector Non Visual Properties.
/// When the object is serialized out as xml, it's qualified name is cdr:nvCxnSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:nvCxnSpPr")]
pub struct NonVisualConnectorShapeDrawingProperties {
    ///Chart Non Visual Properties
    #[xml(child = "cdr:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Connection Shape Drawing Properties
    #[xml(child = "cdr:cNvCxnSpPr")]
    pub non_visual_connection_shape_properties: NonVisualConnectionShapeProperties,
}
/// Non-Visual Picture Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is cdr:cNvPicPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:cNvPicPr")]
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
/// Non-Visual Picture Properties.
/// When the object is serialized out as xml, it's qualified name is cdr:nvPicPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:nvPicPr")]
pub struct NonVisualPictureProperties {
    /// _
    #[xml(child = "cdr:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Picture Drawing Properties
    #[xml(child = "cdr:cNvPicPr")]
    pub non_visual_picture_drawing_properties: NonVisualPictureDrawingProperties,
}
/// Picture Fill.
/// When the object is serialized out as xml, it's qualified name is cdr:blipFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:blipFill")]
pub struct BlipFill {
    /// DPI Setting
    /// Represents the following attribute in the schema: :dpi
    #[xml(attr = "dpi")]
    pub dpi: Option<u32>,
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
/// Non-Visual Graphic Frame Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is cdr:cNvGraphicFramePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:cNvGraphicFramePr")]
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
/// Non-Visual Graphic Frame Properties.
/// When the object is serialized out as xml, it's qualified name is cdr:nvGraphicFramePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:nvGraphicFramePr")]
pub struct NonVisualGraphicFrameProperties {
    ///Non-Visual Drawing Properties
    #[xml(child = "cdr:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Graphic Frame Drawing Properties
    #[xml(child = "cdr:cNvGraphicFramePr")]
    pub non_visual_graphic_frame_drawing_properties: NonVisualGraphicFrameDrawingProperties,
}
/// Graphic Frame Transform.
/// When the object is serialized out as xml, it's qualified name is cdr:xfrm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:xfrm")]
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
/// Non-Visual Group Shape Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is cdr:cNvGrpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:cNvGrpSpPr")]
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
/// Relative X Coordinate.
/// When the object is serialized out as xml, it's qualified name is cdr:x.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:x")]
pub struct XPosition {
    #[xml(text)]
    pub child: f64,
}
/// Relative Y Coordinate.
/// When the object is serialized out as xml, it's qualified name is cdr:y.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:y")]
pub struct YPosition {
    #[xml(text)]
    pub child: f64,
}
/// Starting Anchor Point.
/// When the object is serialized out as xml, it's qualified name is cdr:from.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:from")]
pub struct FromAnchor {
    ///Relative X Coordinate
    #[xml(child = "cdr:x")]
    pub x_position: XPosition,
    ///Relative Y Coordinate
    #[xml(child = "cdr:y")]
    pub y_position: YPosition,
}
/// Ending Anchor Point.
/// When the object is serialized out as xml, it's qualified name is cdr:to.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:to")]
pub struct ToAnchor {
    ///Relative X Coordinate
    #[xml(child = "cdr:x")]
    pub x_position: XPosition,
    ///Relative Y Coordinate
    #[xml(child = "cdr:y")]
    pub y_position: YPosition,
}
/// Defines the MarkerType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct MarkerType {}
/// Shape Extent.
/// When the object is serialized out as xml, it's qualified name is cdr:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:ext")]
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
/// Non-Visual Group Shape Properties.
/// When the object is serialized out as xml, it's qualified name is cdr:nvGrpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:nvGrpSpPr")]
pub struct NonVisualGroupShapeProperties {
    ///Chart Non Visual Properties
    #[xml(child = "cdr:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Group Shape Drawing Properties
    #[xml(child = "cdr:cNvGrpSpPr")]
    pub non_visual_group_shape_drawing_properties: NonVisualGroupShapeDrawingProperties,
}
/// Group Shape Properties.
/// When the object is serialized out as xml, it's qualified name is cdr:grpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdr:grpSpPr")]
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
