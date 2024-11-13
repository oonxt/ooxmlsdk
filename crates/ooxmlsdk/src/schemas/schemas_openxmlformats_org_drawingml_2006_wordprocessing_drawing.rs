#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum WrapTextValues {
    #[default]
    BothSides,
    Left,
    Right,
    Largest,
}
crate::__string_enum! {
    WrapTextValues { BothSides = "bothSides", Left = "left", Right = "right", Largest =
    "largest", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HorizontalAlignmentValues {
    #[default]
    Left,
    Right,
    Center,
    Inside,
    Outside,
}
crate::__string_enum! {
    HorizontalAlignmentValues { Left = "left", Right = "right", Center = "center", Inside
    = "inside", Outside = "outside", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HorizontalRelativePositionValues {
    #[default]
    Margin,
    Page,
    Column,
    Character,
    LeftMargin,
    RightMargin,
    InsideMargin,
    OutsideMargin,
}
crate::__string_enum! {
    HorizontalRelativePositionValues { Margin = "margin", Page = "page", Column =
    "column", Character = "character", LeftMargin = "leftMargin", RightMargin =
    "rightMargin", InsideMargin = "insideMargin", OutsideMargin = "outsideMargin", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VerticalAlignmentValues {
    #[default]
    Top,
    Bottom,
    Center,
    Inside,
    Outside,
}
crate::__string_enum! {
    VerticalAlignmentValues { Top = "top", Bottom = "bottom", Center = "center", Inside =
    "inside", Outside = "outside", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VerticalRelativePositionValues {
    #[default]
    Margin,
    Page,
    Paragraph,
    Line,
    TopMargin,
    BottomMargin,
    InsideMargin,
    OutsideMargin,
}
crate::__string_enum! {
    VerticalRelativePositionValues { Margin = "margin", Page = "page", Paragraph =
    "paragraph", Line = "line", TopMargin = "topMargin", BottomMargin = "bottomMargin",
    InsideMargin = "insideMargin", OutsideMargin = "outsideMargin", }
}
/// No Text Wrapping.
/// When the object is serialized out as xml, it's qualified name is wp:wrapNone.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:wrapNone")]
pub struct WrapNone {}
/// Square Wrapping.
/// When the object is serialized out as xml, it's qualified name is wp:wrapSquare.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:wrapSquare")]
pub struct WrapSquare {
    /// Text Wrapping Location
    /// Represents the following attribute in the schema: :wrapText
    #[xml(attr = "wrapText")]
    pub wrap_text: WrapTextValues,
    /// Distance From Text (Top)
    /// Represents the following attribute in the schema: :distT
    #[xml(attr = "distT")]
    pub distance_from_top: Option<i32>,
    /// Distance From Text on Bottom Edge
    /// Represents the following attribute in the schema: :distB
    #[xml(attr = "distB")]
    pub distance_from_bottom: Option<i32>,
    /// Distance From Text on Left Edge
    /// Represents the following attribute in the schema: :distL
    #[xml(attr = "distL")]
    pub distance_from_left: Option<i32>,
    /// Distance From Text on Right Edge
    /// Represents the following attribute in the schema: :distR
    #[xml(attr = "distR")]
    pub distance_from_right: Option<i32>,
    ///Object Extents Including Effects
    #[xml(child = "wp:effectExtent")]
    pub effect_extent: Option<EffectExtent>,
}
/// Tight Wrapping.
/// When the object is serialized out as xml, it's qualified name is wp:wrapTight.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:wrapTight")]
pub struct WrapTight {
    /// Text Wrapping Location
    /// Represents the following attribute in the schema: :wrapText
    #[xml(attr = "wrapText")]
    pub wrap_text: WrapTextValues,
    /// Distance From Test on Left Edge
    /// Represents the following attribute in the schema: :distL
    #[xml(attr = "distL")]
    pub distance_from_left: Option<i32>,
    /// Distance From Text on Right Edge
    /// Represents the following attribute in the schema: :distR
    #[xml(attr = "distR")]
    pub distance_from_right: Option<i32>,
    ///Tight Wrapping Extents Polygon
    #[xml(child = "wp:wrapPolygon")]
    pub wrap_polygon: WrapPolygon,
}
/// Through Wrapping.
/// When the object is serialized out as xml, it's qualified name is wp:wrapThrough.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:wrapThrough")]
pub struct WrapThrough {
    /// Text Wrapping Location
    /// Represents the following attribute in the schema: :wrapText
    #[xml(attr = "wrapText")]
    pub wrap_text: WrapTextValues,
    /// Distance From Text on Left Edge
    /// Represents the following attribute in the schema: :distL
    #[xml(attr = "distL")]
    pub distance_from_left: Option<i32>,
    /// Distance From Text on Right Edge
    /// Represents the following attribute in the schema: :distR
    #[xml(attr = "distR")]
    pub distance_from_right: Option<i32>,
    ///Wrapping Polygon
    #[xml(child = "wp:wrapPolygon")]
    pub wrap_polygon: WrapPolygon,
}
/// Top and Bottom Wrapping.
/// When the object is serialized out as xml, it's qualified name is wp:wrapTopAndBottom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:wrapTopAndBottom")]
pub struct WrapTopBottom {
    /// Distance From Text on Top Edge
    /// Represents the following attribute in the schema: :distT
    #[xml(attr = "distT")]
    pub distance_from_top: Option<i32>,
    /// Distance From Text on Bottom Edge
    /// Represents the following attribute in the schema: :distB
    #[xml(attr = "distB")]
    pub distance_from_bottom: Option<i32>,
    ///Wrapping Boundaries
    #[xml(child = "wp:effectExtent")]
    pub effect_extent: Option<EffectExtent>,
}
/// Inline DrawingML Object.
/// When the object is serialized out as xml, it's qualified name is wp:inline.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:inline")]
pub struct Inline {
    /// Distance From Text on Top Edge
    /// Represents the following attribute in the schema: :distT
    #[xml(attr = "distT")]
    pub distance_from_top: Option<i32>,
    /// Distance From Text on Bottom Edge
    /// Represents the following attribute in the schema: :distB
    #[xml(attr = "distB")]
    pub distance_from_bottom: Option<i32>,
    /// Distance From Text on Left Edge
    /// Represents the following attribute in the schema: :distL
    #[xml(attr = "distL")]
    pub distance_from_left: Option<i32>,
    /// Distance From Text on Right Edge
    /// Represents the following attribute in the schema: :distR
    #[xml(attr = "distR")]
    pub distance_from_right: Option<i32>,
    /// anchorId
    /// Represents the following attribute in the schema: wp14:anchorId
    #[xml(attr = "wp14:anchorId")]
    pub wp14_anchor_id: Option<String>,
    /// editId
    /// Represents the following attribute in the schema: wp14:editId
    #[xml(attr = "wp14:editId")]
    pub edit_id: Option<String>,
    ///Drawing Object Size
    #[xml(child = "wp:extent")]
    pub extent: Extent,
    ///Inline Wrapping Extent
    #[xml(child = "wp:effectExtent")]
    pub effect_extent: Option<EffectExtent>,
    ///Drawing Object Non-Visual Properties
    #[xml(child = "wp:docPr")]
    pub doc_properties: DocProperties,
    ///Common DrawingML Non-Visual Properties
    #[xml(child = "wp:cNvGraphicFramePr")]
    pub non_visual_graphic_frame_drawing_properties: Option<
        NonVisualGraphicFrameDrawingProperties,
    >,
    /// _
    #[xml(child = "a:graphic")]
    pub graphic: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Graphic,
}
/// Anchor for Floating DrawingML Object.
/// When the object is serialized out as xml, it's qualified name is wp:anchor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:anchor")]
pub struct Anchor {
    /// Distance From Text on Top Edge
    /// Represents the following attribute in the schema: :distT
    #[xml(attr = "distT")]
    pub distance_from_top: Option<i32>,
    /// Distance From Text on Bottom Edge
    /// Represents the following attribute in the schema: :distB
    #[xml(attr = "distB")]
    pub distance_from_bottom: Option<i32>,
    /// Distance From Text on Left Edge
    /// Represents the following attribute in the schema: :distL
    #[xml(attr = "distL")]
    pub distance_from_left: Option<i32>,
    /// Distance From Text on Right Edge
    /// Represents the following attribute in the schema: :distR
    #[xml(attr = "distR")]
    pub distance_from_right: Option<i32>,
    /// Page Positioning
    /// Represents the following attribute in the schema: :simplePos
    #[xml(attr = "simplePos")]
    pub simple_pos: Option<bool>,
    /// Relative Z-Ordering Position
    /// Represents the following attribute in the schema: :relativeHeight
    #[xml(attr = "relativeHeight")]
    pub relative_height: i32,
    /// Display Behind Document Text
    /// Represents the following attribute in the schema: :behindDoc
    #[xml(attr = "behindDoc")]
    pub behind_doc: bool,
    /// Lock Anchor
    /// Represents the following attribute in the schema: :locked
    #[xml(attr = "locked")]
    pub locked: bool,
    /// Layout In Table Cell
    /// Represents the following attribute in the schema: :layoutInCell
    #[xml(attr = "layoutInCell")]
    pub layout_in_cell: bool,
    /// Hidden
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    /// Allow Objects to Overlap
    /// Represents the following attribute in the schema: :allowOverlap
    #[xml(attr = "allowOverlap")]
    pub allow_overlap: bool,
    /// editId
    /// Represents the following attribute in the schema: wp14:editId
    #[xml(attr = "wp14:editId")]
    pub edit_id: Option<String>,
    /// anchorId
    /// Represents the following attribute in the schema: wp14:anchorId
    #[xml(attr = "wp14:anchorId")]
    pub wp14_anchor_id: Option<String>,
    #[xml(
        child = "wp:simplePos",
        child = "wp:positionH",
        child = "wp:positionV",
        child = "wp:extent",
        child = "wp:effectExtent",
        child = "wp:wrapNone",
        child = "wp:wrapSquare",
        child = "wp:wrapTight",
        child = "wp:wrapThrough",
        child = "wp:wrapTopAndBottom",
        child = "wp:docPr",
        child = "wp:cNvGraphicFramePr",
        child = "a:graphic",
        child = "wp14:sizeRelH",
        child = "wp14:sizeRelV",
    )]
    pub children: Vec<AnchorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum AnchorChildChoice {
    #[xml(tag = "wp:simplePos")]
    WpSimplePos(SimplePosition),
    #[xml(tag = "wp:positionH")]
    WpPositionH(HorizontalPosition),
    #[xml(tag = "wp:positionV")]
    WpPositionV(VerticalPosition),
    #[xml(tag = "wp:extent")]
    WpExtent(Extent),
    #[xml(tag = "wp:effectExtent")]
    WpEffectExtent(EffectExtent),
    #[xml(tag = "wp:wrapNone")]
    WpWrapNone(WrapNone),
    #[xml(tag = "wp:wrapSquare")]
    WpWrapSquare(WrapSquare),
    #[xml(tag = "wp:wrapTight")]
    WpWrapTight(WrapTight),
    #[xml(tag = "wp:wrapThrough")]
    WpWrapThrough(WrapThrough),
    #[xml(tag = "wp:wrapTopAndBottom")]
    WpWrapTopAndBottom(WrapTopBottom),
    #[xml(tag = "wp:docPr")]
    WpDocPr(DocProperties),
    #[xml(tag = "wp:cNvGraphicFramePr")]
    WpCNvGraphicFramePr(NonVisualGraphicFrameDrawingProperties),
    #[xml(tag = "a:graphic")]
    AGraphic(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Graphic),
    #[xml(tag = "wp14:sizeRelH")]
    Wp14SizeRelH(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_drawing::RelativeWidth,
    ),
    #[xml(tag = "wp14:sizeRelV")]
    Wp14SizeRelV(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_drawing::RelativeHeight,
    ),
}
/// Wrapping Polygon Start.
/// When the object is serialized out as xml, it's qualified name is wp:start.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:start")]
pub struct StartPoint {
    /// X-Axis Coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i64,
    /// Y-Axis Coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i64,
}
/// Wrapping Polygon Line End Position.
/// When the object is serialized out as xml, it's qualified name is wp:lineTo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:lineTo")]
pub struct LineTo {
    /// X-Axis Coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i64,
    /// Y-Axis Coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i64,
}
/// Simple Positioning Coordinates.
/// When the object is serialized out as xml, it's qualified name is wp:simplePos.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:simplePos")]
pub struct SimplePosition {
    /// X-Axis Coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i64,
    /// Y-Axis Coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i64,
}
/// Defines the Point2DType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct Point2DType {
    /// X-Axis Coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i64,
    /// Y-Axis Coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i64,
}
/// Object Extents Including Effects.
/// When the object is serialized out as xml, it's qualified name is wp:effectExtent.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:effectExtent")]
pub struct EffectExtent {
    /// Additional Extent on Left Edge
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub left_edge: i64,
    /// Additional Extent on Top Edge
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub top_edge: i64,
    /// Additional Extent on Right Edge
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub right_edge: i64,
    /// Additional Extent on Bottom Edge
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bottom_edge: i64,
}
/// Tight Wrapping Extents Polygon.
/// When the object is serialized out as xml, it's qualified name is wp:wrapPolygon.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:wrapPolygon")]
pub struct WrapPolygon {
    /// Wrapping Points Modified
    /// Represents the following attribute in the schema: :edited
    #[xml(attr = "edited")]
    pub edited: Option<bool>,
    ///Wrapping Polygon Start
    #[xml(child = "wp:start")]
    pub start_point: StartPoint,
    /// _
    #[xml(child = "wp:lineTo")]
    pub wp_line_to: Vec<LineTo>,
}
/// Horizontal Positioning.
/// When the object is serialized out as xml, it's qualified name is wp:positionH.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:positionH")]
pub struct HorizontalPosition {
    /// Horizontal Position Relative Base
    /// Represents the following attribute in the schema: :relativeFrom
    #[xml(attr = "relativeFrom")]
    pub relative_from: HorizontalRelativePositionValues,
    #[xml(child = "wp:align", child = "wp:posOffset", child = "wp14:pctPosHOffset")]
    pub children: Vec<HorizontalPositionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HorizontalPositionChildChoice {
    #[xml(tag = "wp:align")]
    WpAlign(HorizontalAlignment),
    #[xml(tag = "wp:posOffset")]
    WpPosOffset(PositionOffset),
    #[xml(tag = "wp14:pctPosHOffset")]
    Wp14PctPosHOffset(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_drawing::PercentagePositionHeightOffset,
    ),
}
/// Vertical Positioning.
/// When the object is serialized out as xml, it's qualified name is wp:positionV.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:positionV")]
pub struct VerticalPosition {
    /// Vertical Position Relative Base
    /// Represents the following attribute in the schema: :relativeFrom
    #[xml(attr = "relativeFrom")]
    pub relative_from: VerticalRelativePositionValues,
    #[xml(child = "wp:align", child = "wp:posOffset", child = "wp14:pctPosVOffset")]
    pub children: Vec<VerticalPositionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum VerticalPositionChildChoice {
    #[xml(tag = "wp:align")]
    WpAlign(VerticalAlignment),
    #[xml(tag = "wp:posOffset")]
    WpPosOffset(PositionOffset),
    #[xml(tag = "wp14:pctPosVOffset")]
    Wp14PctPosVOffset(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_drawing::PercentagePositionVerticalOffset,
    ),
}
/// Inline Drawing Object Extents.
/// When the object is serialized out as xml, it's qualified name is wp:extent.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:extent")]
pub struct Extent {
    /// Extent Length
    /// Represents the following attribute in the schema: :cx
    #[xml(attr = "cx")]
    pub cx: i64,
    /// Extent Width
    /// Represents the following attribute in the schema: :cy
    #[xml(attr = "cy")]
    pub cy: i64,
}
/// Drawing Object Non-Visual Properties.
/// When the object is serialized out as xml, it's qualified name is wp:docPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:docPr")]
pub struct DocProperties {
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
/// Defines the NonVisualGraphicFrameDrawingProperties Class.
/// When the object is serialized out as xml, it's qualified name is wp:cNvGraphicFramePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:cNvGraphicFramePr")]
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
/// Relative Vertical Alignment.
/// When the object is serialized out as xml, it's qualified name is wp:align.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:align")]
pub struct VerticalAlignment {
    #[xml(text)]
    pub child: VerticalAlignmentValues,
}
/// Defines the PositionOffset Class.
/// When the object is serialized out as xml, it's qualified name is wp:posOffset.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:posOffset")]
pub struct PositionOffset {
    #[xml(text)]
    pub child: i32,
}
/// Relative Horizontal Alignment.
/// When the object is serialized out as xml, it's qualified name is wp:align.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp:align")]
pub struct HorizontalAlignment {
    #[xml(text)]
    pub child: HorizontalAlignmentValues,
}
