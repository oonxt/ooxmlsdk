#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LayoutTargetValues {
    #[default]
    Inner,
    Outer,
}
crate::__string_enum! {
    LayoutTargetValues { Inner = "inner", Outer = "outer", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LayoutModeValues {
    #[default]
    Edge,
    Factor,
}
crate::__string_enum! {
    LayoutModeValues { Edge = "edge", Factor = "factor", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SizeRepresentsValues {
    #[default]
    Area,
    Width,
}
crate::__string_enum! {
    SizeRepresentsValues { Area = "area", Width = "w", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LabelAlignmentValues {
    #[default]
    Center,
    Left,
    Right,
}
crate::__string_enum! {
    LabelAlignmentValues { Center = "ctr", Left = "l", Right = "r", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataLabelPositionValues {
    #[default]
    BestFit,
    Bottom,
    Center,
    InsideBase,
    InsideEnd,
    Left,
    OutsideEnd,
    Right,
    Top,
}
crate::__string_enum! {
    DataLabelPositionValues { BestFit = "bestFit", Bottom = "b", Center = "ctr",
    InsideBase = "inBase", InsideEnd = "inEnd", Left = "l", OutsideEnd = "outEnd", Right
    = "r", Top = "t", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TrendlineValues {
    #[default]
    Exponential,
    Linear,
    Logarithmic,
    MovingAverage,
    Polynomial,
    Power,
}
crate::__string_enum! {
    TrendlineValues { Exponential = "exp", Linear = "linear", Logarithmic = "log",
    MovingAverage = "movingAvg", Polynomial = "poly", Power = "power", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ErrorBarDirectionValues {
    #[default]
    X,
    Y,
}
crate::__string_enum! {
    ErrorBarDirectionValues { X = "x", Y = "y", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ErrorBarValues {
    #[default]
    Both,
    Minus,
    Plus,
}
crate::__string_enum! {
    ErrorBarValues { Both = "both", Minus = "minus", Plus = "plus", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ErrorValues {
    #[default]
    Custom,
    FixedValue,
    Percentage,
    StandardDeviation,
    StandardError,
}
crate::__string_enum! {
    ErrorValues { Custom = "cust", FixedValue = "fixedVal", Percentage = "percentage",
    StandardDeviation = "stdDev", StandardError = "stdErr", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GroupingValues {
    #[default]
    PercentStacked,
    Standard,
    Stacked,
}
crate::__string_enum! {
    GroupingValues { PercentStacked = "percentStacked", Standard = "standard", Stacked =
    "stacked", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RadarStyleValues {
    #[default]
    Standard,
    Marker,
    Filled,
}
crate::__string_enum! {
    RadarStyleValues { Standard = "standard", Marker = "marker", Filled = "filled", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BarGroupingValues {
    #[default]
    PercentStacked,
    Clustered,
    Standard,
    Stacked,
}
crate::__string_enum! {
    BarGroupingValues { PercentStacked = "percentStacked", Clustered = "clustered",
    Standard = "standard", Stacked = "stacked", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BarDirectionValues {
    #[default]
    Bar,
    Column,
}
crate::__string_enum! {
    BarDirectionValues { Bar = "bar", Column = "col", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ShapeValues {
    #[default]
    Cone,
    ConeToMax,
    Box,
    Cylinder,
    Pyramid,
    PyramidToMaximum,
}
crate::__string_enum! {
    ShapeValues { Cone = "cone", ConeToMax = "coneToMax", Box = "box", Cylinder =
    "cylinder", Pyramid = "pyramid", PyramidToMaximum = "pyramidToMax", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OfPieValues {
    #[default]
    Pie,
    Bar,
}
crate::__string_enum! {
    OfPieValues { Pie = "pie", Bar = "bar", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AxisPositionValues {
    #[default]
    Bottom,
    Left,
    Right,
    Top,
}
crate::__string_enum! {
    AxisPositionValues { Bottom = "b", Left = "l", Right = "r", Top = "t", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CrossesValues {
    #[default]
    AutoZero,
    Maximum,
    Minimum,
}
crate::__string_enum! {
    CrossesValues { AutoZero = "autoZero", Maximum = "max", Minimum = "min", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CrossBetweenValues {
    #[default]
    Between,
    MidpointCategory,
}
crate::__string_enum! {
    CrossBetweenValues { Between = "between", MidpointCategory = "midCat", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TickMarkValues {
    #[default]
    Cross,
    Inside,
    None,
    Outside,
}
crate::__string_enum! {
    TickMarkValues { Cross = "cross", Inside = "in", None = "none", Outside = "out", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TickLabelPositionValues {
    #[default]
    High,
    Low,
    NextTo,
    None,
}
crate::__string_enum! {
    TickLabelPositionValues { High = "high", Low = "low", NextTo = "nextTo", None =
    "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TimeUnitValues {
    #[default]
    Days,
    Months,
    Years,
}
crate::__string_enum! {
    TimeUnitValues { Days = "days", Months = "months", Years = "years", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BuiltInUnitValues {
    #[default]
    Hundreds,
    Thousands,
    TenThousands,
    HundredThousands,
    Millions,
    TenMillions,
    HundredMillions,
    Billions,
    Trillions,
}
crate::__string_enum! {
    BuiltInUnitValues { Hundreds = "hundreds", Thousands = "thousands", TenThousands =
    "tenThousands", HundredThousands = "hundredThousands", Millions = "millions",
    TenMillions = "tenMillions", HundredMillions = "hundredMillions", Billions =
    "billions", Trillions = "trillions", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PictureFormatValues {
    #[default]
    Stretch,
    Stack,
    StackScale,
}
crate::__string_enum! {
    PictureFormatValues { Stretch = "stretch", Stack = "stack", StackScale =
    "stackScale", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OrientationValues {
    #[default]
    MaxMin,
    MinMax,
}
crate::__string_enum! {
    OrientationValues { MaxMin = "maxMin", MinMax = "minMax", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LegendPositionValues {
    #[default]
    Bottom,
    TopRight,
    Left,
    Right,
    Top,
}
crate::__string_enum! {
    LegendPositionValues { Bottom = "b", TopRight = "tr", Left = "l", Right = "r", Top =
    "t", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DisplayBlanksAsValues {
    #[default]
    Span,
    Gap,
    Zero,
}
crate::__string_enum! {
    DisplayBlanksAsValues { Span = "span", Gap = "gap", Zero = "zero", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PageSetupOrientationValues {
    #[default]
    Default,
    Portrait,
    Landscape,
}
crate::__string_enum! {
    PageSetupOrientationValues { Default = "default", Portrait = "portrait", Landscape =
    "landscape", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ScatterStyleValues {
    #[default]
    Line,
    LineMarker,
    Marker,
    Smooth,
    SmoothMarker,
}
crate::__string_enum! {
    ScatterStyleValues { Line = "line", LineMarker = "lineMarker", Marker = "marker",
    Smooth = "smooth", SmoothMarker = "smoothMarker", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MarkerStyleValues {
    #[default]
    Auto,
    Circle,
    Dash,
    Diamond,
    Dot,
    None,
    Picture,
    Plus,
    Square,
    Star,
    Triangle,
    X,
}
crate::__string_enum! {
    MarkerStyleValues { Auto = "auto", Circle = "circle", Dash = "dash", Diamond =
    "diamond", Dot = "dot", None = "none", Picture = "picture", Plus = "plus", Square =
    "square", Star = "star", Triangle = "triangle", X = "x", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SplitValues {
    #[default]
    Custom,
    Percent,
    Position,
    Value,
}
crate::__string_enum! {
    SplitValues { Custom = "cust", Percent = "percent", Position = "pos", Value = "val",
    }
}
/// Number Format.
/// When the object is serialized out as xml, it's qualified name is c:numFmt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:numFmt")]
pub struct NumberingFormat {
    /// Number Format Code
    /// Represents the following attribute in the schema: :formatCode
    #[xml(attr = "formatCode")]
    pub format_code: String,
    /// Linked to Source
    /// Represents the following attribute in the schema: :sourceLinked
    #[xml(attr = "sourceLinked")]
    pub source_linked: Option<bool>,
}
/// Defines the ChartShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is c:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:spPr")]
pub struct ChartShapeProperties {
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
        child = "a:ln",
        child = "a:effectLst",
        child = "a:effectDag",
        child = "a:scene3d",
        child = "a:sp3d",
        child = "a:extLst",
    )]
    pub children: Vec<ChartShapePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ChartShapePropertiesChildChoice {
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
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    ),
}
/// Defines the TextProperties Class.
/// When the object is serialized out as xml, it's qualified name is c:txPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:txPr")]
pub struct TextProperties {
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
/// Rich Text.
/// When the object is serialized out as xml, it's qualified name is c:rich.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:rich")]
pub struct RichText {
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
/// Defines the TextBodyType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TextBodyType {}
/// Data Label Position.
/// When the object is serialized out as xml, it's qualified name is c:dLblPos.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:dLblPos")]
pub struct DataLabelPosition {
    /// Data Label Position Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: DataLabelPositionValues,
}
/// Show Legend Key.
/// When the object is serialized out as xml, it's qualified name is c:showLegendKey.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:showLegendKey")]
pub struct ShowLegendKey {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Show Value.
/// When the object is serialized out as xml, it's qualified name is c:showVal.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:showVal")]
pub struct ShowValue {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Show Category Name.
/// When the object is serialized out as xml, it's qualified name is c:showCatName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:showCatName")]
pub struct ShowCategoryName {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Show Series Name.
/// When the object is serialized out as xml, it's qualified name is c:showSerName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:showSerName")]
pub struct ShowSeriesName {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Show Percent.
/// When the object is serialized out as xml, it's qualified name is c:showPercent.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:showPercent")]
pub struct ShowPercent {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Show Bubble Size.
/// When the object is serialized out as xml, it's qualified name is c:showBubbleSize.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:showBubbleSize")]
pub struct ShowBubbleSize {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Show Leader Lines.
/// When the object is serialized out as xml, it's qualified name is c:showLeaderLines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:showLeaderLines")]
pub struct ShowLeaderLines {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the VaryColors Class.
/// When the object is serialized out as xml, it's qualified name is c:varyColors.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:varyColors")]
pub struct VaryColors {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Wireframe.
/// When the object is serialized out as xml, it's qualified name is c:wireframe.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:wireframe")]
pub struct Wireframe {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Delete.
/// When the object is serialized out as xml, it's qualified name is c:delete.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:delete")]
pub struct Delete {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Overlay.
/// When the object is serialized out as xml, it's qualified name is c:overlay.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:overlay")]
pub struct Overlay {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Right Angle Axes.
/// When the object is serialized out as xml, it's qualified name is c:rAngAx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:rAngAx")]
pub struct RightAngleAxes {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Show Horizontal Border.
/// When the object is serialized out as xml, it's qualified name is c:showHorzBorder.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:showHorzBorder")]
pub struct ShowHorizontalBorder {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Show Vertical Border.
/// When the object is serialized out as xml, it's qualified name is c:showVertBorder.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:showVertBorder")]
pub struct ShowVerticalBorder {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Show Outline Border.
/// When the object is serialized out as xml, it's qualified name is c:showOutline.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:showOutline")]
pub struct ShowOutlineBorder {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Show Legend Keys.
/// When the object is serialized out as xml, it's qualified name is c:showKeys.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:showKeys")]
pub struct ShowKeys {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Invert if Negative.
/// When the object is serialized out as xml, it's qualified name is c:invertIfNegative.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:invertIfNegative")]
pub struct InvertIfNegative {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// 3D Bubble.
/// When the object is serialized out as xml, it's qualified name is c:bubble3D.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:bubble3D")]
pub struct Bubble3D {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Display R Squared Value.
/// When the object is serialized out as xml, it's qualified name is c:dispRSqr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:dispRSqr")]
pub struct DisplayRSquaredValue {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Display Equation.
/// When the object is serialized out as xml, it's qualified name is c:dispEq.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:dispEq")]
pub struct DisplayEquation {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// No End Cap.
/// When the object is serialized out as xml, it's qualified name is c:noEndCap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:noEndCap")]
pub struct NoEndCap {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Apply To Front.
/// When the object is serialized out as xml, it's qualified name is c:applyToFront.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:applyToFront")]
pub struct ApplyToFront {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Apply To Sides.
/// When the object is serialized out as xml, it's qualified name is c:applyToSides.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:applyToSides")]
pub struct ApplyToSides {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Apply to End.
/// When the object is serialized out as xml, it's qualified name is c:applyToEnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:applyToEnd")]
pub struct ApplyToEnd {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Chart Object.
/// When the object is serialized out as xml, it's qualified name is c:chartObject.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:chartObject")]
pub struct ChartObject {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Data Cannot Be Changed.
/// When the object is serialized out as xml, it's qualified name is c:data.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:data")]
pub struct Data {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Formatting.
/// When the object is serialized out as xml, it's qualified name is c:formatting.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:formatting")]
pub struct Formatting {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Selection.
/// When the object is serialized out as xml, it's qualified name is c:selection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:selection")]
pub struct Selection {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// User Interface.
/// When the object is serialized out as xml, it's qualified name is c:userInterface.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:userInterface")]
pub struct UserInterface {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Update Automatically.
/// When the object is serialized out as xml, it's qualified name is c:autoUpdate.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:autoUpdate")]
pub struct AutoUpdate {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the ShowMarker Class.
/// When the object is serialized out as xml, it's qualified name is c:marker.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:marker")]
pub struct ShowMarker {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the Smooth Class.
/// When the object is serialized out as xml, it's qualified name is c:smooth.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:smooth")]
pub struct Smooth {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the ShowNegativeBubbles Class.
/// When the object is serialized out as xml, it's qualified name is c:showNegBubbles.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:showNegBubbles")]
pub struct ShowNegativeBubbles {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the AutoLabeled Class.
/// When the object is serialized out as xml, it's qualified name is c:auto.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:auto")]
pub struct AutoLabeled {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the NoMultiLevelLabels Class.
/// When the object is serialized out as xml, it's qualified name is c:noMultiLvlLbl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:noMultiLvlLbl")]
pub struct NoMultiLevelLabels {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the Date1904 Class.
/// When the object is serialized out as xml, it's qualified name is c:date1904.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:date1904")]
pub struct Date1904 {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the RoundedCorners Class.
/// When the object is serialized out as xml, it's qualified name is c:roundedCorners.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:roundedCorners")]
pub struct RoundedCorners {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// True if the chart automatic title has been deleted..
/// When the object is serialized out as xml, it's qualified name is c:autoTitleDeleted.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:autoTitleDeleted")]
pub struct AutoTitleDeleted {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// True if only visible cells are plotted..
/// When the object is serialized out as xml, it's qualified name is c:plotVisOnly.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:plotVisOnly")]
pub struct PlotVisibleOnly {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// True if we should render datalabels over the maximum scale.
/// When the object is serialized out as xml, it's qualified name is c:showDLblsOverMax.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:showDLblsOverMax")]
pub struct ShowDataLabelsOverMaximum {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the BooleanType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct BooleanType {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Separator.
/// When the object is serialized out as xml, it's qualified name is c:separator.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:separator")]
pub struct Separator {
    #[xml(text)]
    pub child: String,
}
/// Trendline Name.
/// When the object is serialized out as xml, it's qualified name is c:name.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:name")]
pub struct TrendlineName {
    #[xml(text)]
    pub child: String,
}
/// Defines the Formula Class.
/// When the object is serialized out as xml, it's qualified name is c:f.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:f")]
pub struct Formula {
    #[xml(text)]
    pub child: String,
}
/// Layout.
/// When the object is serialized out as xml, it's qualified name is c:layout.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:layout")]
pub struct Layout {
    ///Manual Layout
    #[xml(child = "c:manualLayout")]
    pub manual_layout: Option<ManualLayout>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the ChartText Class.
/// When the object is serialized out as xml, it's qualified name is c:tx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:tx")]
pub struct ChartText {
    #[xml(child = "c:strRef", child = "c:rich", child = "c:strLit")]
    pub children: Vec<ChartTextChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ChartTextChildChoice {
    #[xml(tag = "c:strRef")]
    CStrRef(StringReference),
    #[xml(tag = "c:rich")]
    CRich(RichText),
    #[xml(tag = "c:strLit")]
    CStrLit(StringLiteral),
}
/// Leader Lines.
/// When the object is serialized out as xml, it's qualified name is c:leaderLines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:leaderLines")]
pub struct LeaderLines {
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
}
/// Drop Lines.
/// When the object is serialized out as xml, it's qualified name is c:dropLines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:dropLines")]
pub struct DropLines {
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
}
/// Major Gridlines.
/// When the object is serialized out as xml, it's qualified name is c:majorGridlines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:majorGridlines")]
pub struct MajorGridlines {
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
}
/// Minor Gridlines.
/// When the object is serialized out as xml, it's qualified name is c:minorGridlines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:minorGridlines")]
pub struct MinorGridlines {
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
}
/// Defines the SeriesLines Class.
/// When the object is serialized out as xml, it's qualified name is c:serLines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:serLines")]
pub struct SeriesLines {
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
}
/// Defines the HighLowLines Class.
/// When the object is serialized out as xml, it's qualified name is c:hiLowLines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:hiLowLines")]
pub struct HighLowLines {
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
}
/// Defines the ChartLinesType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct ChartLinesType {}
/// Index.
/// When the object is serialized out as xml, it's qualified name is c:idx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:idx")]
pub struct Index {
    /// Integer Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Order.
/// When the object is serialized out as xml, it's qualified name is c:order.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:order")]
pub struct Order {
    /// Integer Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Axis ID.
/// When the object is serialized out as xml, it's qualified name is c:axId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:axId")]
pub struct AxisId {
    /// Integer Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Crossing Axis ID.
/// When the object is serialized out as xml, it's qualified name is c:crossAx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:crossAx")]
pub struct CrossingAxis {
    /// Integer Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Point Count.
/// When the object is serialized out as xml, it's qualified name is c:ptCount.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ptCount")]
pub struct PointCount {
    /// Integer Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Second Pie Point.
/// When the object is serialized out as xml, it's qualified name is c:secondPiePt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:secondPiePt")]
pub struct SecondPiePoint {
    /// Integer Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Explosion.
/// When the object is serialized out as xml, it's qualified name is c:explosion.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:explosion")]
pub struct Explosion {
    /// Integer Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Format ID.
/// When the object is serialized out as xml, it's qualified name is c:fmtId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:fmtId")]
pub struct FormatId {
    /// Integer Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the UnsignedIntegerType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct UnsignedIntegerType {
    /// Integer Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Series Text.
/// When the object is serialized out as xml, it's qualified name is c:tx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:tx")]
pub struct SeriesText {
    #[xml(child = "c:strRef", child = "c:v")]
    pub children: Vec<SeriesTextChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SeriesTextChildChoice {
    #[xml(tag = "c:strRef")]
    CStrRef(StringReference),
    #[xml(tag = "c:v")]
    CV(NumericValue),
}
/// Grouping.
/// When the object is serialized out as xml, it's qualified name is c:grouping.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:grouping")]
pub struct Grouping {
    /// Grouping Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<GroupingValues>,
}
/// Defines the LineChartSeries Class.
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ser")]
pub struct LineChartSeries {
    #[xml(
        child = "c:idx",
        child = "c:order",
        child = "c:tx",
        child = "c:spPr",
        child = "c:marker",
        child = "c:pictureOptions",
        child = "c:dPt",
        child = "c:dLbls",
        child = "c:trendline",
        child = "c:errBars",
        child = "c:cat",
        child = "c:val",
        child = "c:smooth",
        child = "c:extLst",
    )]
    pub children: Vec<LineChartSeriesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LineChartSeriesChildChoice {
    #[xml(tag = "c:idx")]
    CIdx(Index),
    #[xml(tag = "c:order")]
    COrder(Order),
    #[xml(tag = "c:tx")]
    CTx(SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(ChartShapeProperties),
    #[xml(tag = "c:marker")]
    CMarker(Marker),
    #[xml(tag = "c:pictureOptions")]
    CPictureOptions(PictureOptions),
    #[xml(tag = "c:dPt")]
    CDPt(DataPoint),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:trendline")]
    CTrendline(Trendline),
    #[xml(tag = "c:errBars")]
    CErrBars(ErrorBars),
    #[xml(tag = "c:cat")]
    CCat(CategoryAxisData),
    #[xml(tag = "c:val")]
    CVal(Values),
    #[xml(tag = "c:smooth")]
    CSmooth(Smooth),
    #[xml(tag = "c:extLst")]
    CExtLst(LineSerExtensionList),
}
/// Data Labels.
/// When the object is serialized out as xml, it's qualified name is c:dLbls.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:dLbls")]
pub struct DataLabels {
    #[xml(
        child = "c:dLbl",
        child = "c:delete",
        child = "c:numFmt",
        child = "c:spPr",
        child = "c:txPr",
        child = "c:dLblPos",
        child = "c:showLegendKey",
        child = "c:showVal",
        child = "c:showCatName",
        child = "c:showSerName",
        child = "c:showPercent",
        child = "c:showBubbleSize",
        child = "c:separator",
        child = "c:showLeaderLines",
        child = "c:leaderLines",
        child = "c:extLst",
    )]
    pub children: Vec<DataLabelsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DataLabelsChildChoice {
    #[xml(tag = "c:dLbl")]
    CDLbl(DataLabel),
    #[xml(tag = "c:delete")]
    CDelete(Delete),
    #[xml(tag = "c:numFmt")]
    CNumFmt(NumberingFormat),
    #[xml(tag = "c:spPr")]
    CSpPr(ChartShapeProperties),
    #[xml(tag = "c:txPr")]
    CTxPr(TextProperties),
    #[xml(tag = "c:dLblPos")]
    CDLblPos(DataLabelPosition),
    #[xml(tag = "c:showLegendKey")]
    CShowLegendKey(ShowLegendKey),
    #[xml(tag = "c:showVal")]
    CShowVal(ShowValue),
    #[xml(tag = "c:showCatName")]
    CShowCatName(ShowCategoryName),
    #[xml(tag = "c:showSerName")]
    CShowSerName(ShowSeriesName),
    #[xml(tag = "c:showPercent")]
    CShowPercent(ShowPercent),
    #[xml(tag = "c:showBubbleSize")]
    CShowBubbleSize(ShowBubbleSize),
    #[xml(tag = "c:separator")]
    CSeparator(Separator),
    #[xml(tag = "c:showLeaderLines")]
    CShowLeaderLines(ShowLeaderLines),
    #[xml(tag = "c:leaderLines")]
    CLeaderLines(LeaderLines),
    #[xml(tag = "c:extLst")]
    CExtLst(DLblsExtensionList),
}
/// Bar Direction.
/// When the object is serialized out as xml, it's qualified name is c:barDir.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:barDir")]
pub struct BarDirection {
    /// Bar Direction Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: BarDirectionValues,
}
/// Bar Grouping.
/// When the object is serialized out as xml, it's qualified name is c:grouping.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:grouping")]
pub struct BarGrouping {
    /// Bar Grouping Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<BarGroupingValues>,
}
/// Bar Chart Series.
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ser")]
pub struct BarChartSeries {
    #[xml(
        child = "c:idx",
        child = "c:order",
        child = "c:tx",
        child = "c:spPr",
        child = "c:invertIfNegative",
        child = "c:pictureOptions",
        child = "c:dPt",
        child = "c:dLbls",
        child = "c:trendline",
        child = "c:errBars",
        child = "c:cat",
        child = "c:val",
        child = "c:shape",
        child = "c:extLst",
    )]
    pub children: Vec<BarChartSeriesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BarChartSeriesChildChoice {
    #[xml(tag = "c:idx")]
    CIdx(Index),
    #[xml(tag = "c:order")]
    COrder(Order),
    #[xml(tag = "c:tx")]
    CTx(SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(ChartShapeProperties),
    #[xml(tag = "c:invertIfNegative")]
    CInvertIfNegative(InvertIfNegative),
    #[xml(tag = "c:pictureOptions")]
    CPictureOptions(PictureOptions),
    #[xml(tag = "c:dPt")]
    CDPt(DataPoint),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:trendline")]
    CTrendline(Trendline),
    #[xml(tag = "c:errBars")]
    CErrBars(ErrorBars),
    #[xml(tag = "c:cat")]
    CCat(CategoryAxisData),
    #[xml(tag = "c:val")]
    CVal(Values),
    #[xml(tag = "c:shape")]
    CShape(Shape),
    #[xml(tag = "c:extLst")]
    CExtLst(BarSerExtensionList),
}
/// Area Chart Series.
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ser")]
pub struct AreaChartSeries {
    #[xml(
        child = "c:idx",
        child = "c:order",
        child = "c:tx",
        child = "c:spPr",
        child = "c:pictureOptions",
        child = "c:dPt",
        child = "c:dLbls",
        child = "c:trendline",
        child = "c:errBars",
        child = "c:cat",
        child = "c:val",
        child = "c:extLst",
    )]
    pub children: Vec<AreaChartSeriesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum AreaChartSeriesChildChoice {
    #[xml(tag = "c:idx")]
    CIdx(Index),
    #[xml(tag = "c:order")]
    COrder(Order),
    #[xml(tag = "c:tx")]
    CTx(SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(ChartShapeProperties),
    #[xml(tag = "c:pictureOptions")]
    CPictureOptions(PictureOptions),
    #[xml(tag = "c:dPt")]
    CDPt(DataPoint),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:trendline")]
    CTrendline(Trendline),
    #[xml(tag = "c:errBars")]
    CErrBars(ErrorBars),
    #[xml(tag = "c:cat")]
    CCat(CategoryAxisData),
    #[xml(tag = "c:val")]
    CVal(Values),
    #[xml(tag = "c:extLst")]
    CExtLst(AreaSerExtensionList),
}
/// Pie Chart Series.
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ser")]
pub struct PieChartSeries {
    #[xml(
        child = "c:idx",
        child = "c:order",
        child = "c:tx",
        child = "c:spPr",
        child = "c:pictureOptions",
        child = "c:explosion",
        child = "c:dPt",
        child = "c:dLbls",
        child = "c:cat",
        child = "c:val",
        child = "c:extLst",
    )]
    pub children: Vec<PieChartSeriesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PieChartSeriesChildChoice {
    #[xml(tag = "c:idx")]
    CIdx(Index),
    #[xml(tag = "c:order")]
    COrder(Order),
    #[xml(tag = "c:tx")]
    CTx(SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(ChartShapeProperties),
    #[xml(tag = "c:pictureOptions")]
    CPictureOptions(PictureOptions),
    #[xml(tag = "c:explosion")]
    CExplosion(Explosion),
    #[xml(tag = "c:dPt")]
    CDPt(DataPoint),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:cat")]
    CCat(CategoryAxisData),
    #[xml(tag = "c:val")]
    CVal(Values),
    #[xml(tag = "c:extLst")]
    CExtLst(PieSerExtensionList),
}
/// Surface Chart Series.
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ser")]
pub struct SurfaceChartSeries {
    #[xml(
        child = "c:idx",
        child = "c:order",
        child = "c:tx",
        child = "c:spPr",
        child = "c:pictureOptions",
        child = "c:cat",
        child = "c:val",
        child = "c:bubble3D",
        child = "c:extLst",
    )]
    pub children: Vec<SurfaceChartSeriesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SurfaceChartSeriesChildChoice {
    #[xml(tag = "c:idx")]
    CIdx(Index),
    #[xml(tag = "c:order")]
    COrder(Order),
    #[xml(tag = "c:tx")]
    CTx(SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(ChartShapeProperties),
    #[xml(tag = "c:pictureOptions")]
    CPictureOptions(PictureOptions),
    #[xml(tag = "c:cat")]
    CCat(CategoryAxisData),
    #[xml(tag = "c:val")]
    CVal(Values),
    #[xml(tag = "c:bubble3D")]
    CBubble3D(Bubble3D),
    #[xml(tag = "c:extLst")]
    CExtLst(SurfaceSerExtensionList),
}
/// Band Formats.
/// When the object is serialized out as xml, it's qualified name is c:bandFmts.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:bandFmts")]
pub struct BandFormats {
    /// _
    #[xml(child = "c:bandFmt")]
    pub c_band_fmt: Vec<BandFormat>,
}
/// Scaling.
/// When the object is serialized out as xml, it's qualified name is c:scaling.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:scaling")]
pub struct Scaling {
    ///Logarithmic Base
    #[xml(child = "c:logBase")]
    pub log_base: Option<LogBase>,
    ///Axis Orientation
    #[xml(child = "c:orientation")]
    pub orientation: Option<Orientation>,
    ///Maximum
    #[xml(child = "c:max")]
    pub max_axis_value: Option<MaxAxisValue>,
    ///Minimum
    #[xml(child = "c:min")]
    pub min_axis_value: Option<MinAxisValue>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Axis Position.
/// When the object is serialized out as xml, it's qualified name is c:axPos.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:axPos")]
pub struct AxisPosition {
    /// Axis Position Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: AxisPositionValues,
}
/// Title.
/// When the object is serialized out as xml, it's qualified name is c:title.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:title")]
pub struct Title {
    ///Chart Text
    #[xml(child = "c:tx")]
    pub chart_text: Option<ChartText>,
    ///Layout
    #[xml(child = "c:layout")]
    pub layout: Option<Layout>,
    ///Overlay
    #[xml(child = "c:overlay")]
    pub overlay: Option<Overlay>,
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
    /// _
    #[xml(child = "c:txPr")]
    pub text_properties: Option<TextProperties>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Major Tick Mark.
/// When the object is serialized out as xml, it's qualified name is c:majorTickMark.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:majorTickMark")]
pub struct MajorTickMark {
    /// Tick Mark Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<TickMarkValues>,
}
/// Minor Tick Mark.
/// When the object is serialized out as xml, it's qualified name is c:minorTickMark.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:minorTickMark")]
pub struct MinorTickMark {
    /// Tick Mark Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<TickMarkValues>,
}
/// Defines the TickMarkType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TickMarkType {
    /// Tick Mark Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<TickMarkValues>,
}
/// Tick Label Position.
/// When the object is serialized out as xml, it's qualified name is c:tickLblPos.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:tickLblPos")]
pub struct TickLabelPosition {
    /// Tick Label Position Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<TickLabelPositionValues>,
}
/// Crosses.
/// When the object is serialized out as xml, it's qualified name is c:crosses.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:crosses")]
pub struct Crosses {
    /// Crosses Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: CrossesValues,
}
/// Crossing Value.
/// When the object is serialized out as xml, it's qualified name is c:crossesAt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:crossesAt")]
pub struct CrossesAt {
    /// Floating Point Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Left.
/// When the object is serialized out as xml, it's qualified name is c:x.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:x")]
pub struct Left {
    /// Floating Point Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Top.
/// When the object is serialized out as xml, it's qualified name is c:y.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:y")]
pub struct Top {
    /// Floating Point Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Width.
/// When the object is serialized out as xml, it's qualified name is c:w.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:w")]
pub struct Width {
    /// Floating Point Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Height.
/// When the object is serialized out as xml, it's qualified name is c:h.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:h")]
pub struct Height {
    /// Floating Point Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Forward.
/// When the object is serialized out as xml, it's qualified name is c:forward.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:forward")]
pub struct Forward {
    /// Floating Point Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Backward.
/// When the object is serialized out as xml, it's qualified name is c:backward.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:backward")]
pub struct Backward {
    /// Floating Point Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Intercept.
/// When the object is serialized out as xml, it's qualified name is c:intercept.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:intercept")]
pub struct Intercept {
    /// Floating Point Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Error Bar Value.
/// When the object is serialized out as xml, it's qualified name is c:val.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:val")]
pub struct ErrorBarValue {
    /// Floating Point Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Split Position.
/// When the object is serialized out as xml, it's qualified name is c:splitPos.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:splitPos")]
pub struct SplitPosition {
    /// Floating Point Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Custom Display Unit.
/// When the object is serialized out as xml, it's qualified name is c:custUnit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:custUnit")]
pub struct CustomDisplayUnit {
    /// Floating Point Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Maximum.
/// When the object is serialized out as xml, it's qualified name is c:max.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:max")]
pub struct MaxAxisValue {
    /// Floating Point Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Minimum.
/// When the object is serialized out as xml, it's qualified name is c:min.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:min")]
pub struct MinAxisValue {
    /// Floating Point Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Defines the DoubleType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct DoubleType {
    /// Floating Point Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Chart Space.
/// When the object is serialized out as xml, it's qualified name is c:chartSpace.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:chartSpace")]
pub struct ChartSpace {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "c:date1904",
        child = "c:lang",
        child = "c:roundedCorners",
        child = "c14:style",
        child = "c:style",
        child = "c:clrMapOvr",
        child = "c:pivotSource",
        child = "c:protection",
        child = "c:chart",
        child = "c:spPr",
        child = "c:txPr",
        child = "c:externalData",
        child = "c:printSettings",
        child = "c:userShapes",
        child = "c:extLst",
    )]
    pub children: Vec<ChartSpaceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ChartSpaceChildChoice {
    #[xml(tag = "c:date1904")]
    CDate1904(Date1904),
    #[xml(tag = "c:lang")]
    CLang(EditingLanguage),
    #[xml(tag = "c:roundedCorners")]
    CRoundedCorners(RoundedCorners),
    #[xml(tag = "c14:style")]
    C14Style(crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::Style),
    #[xml(tag = "c:style")]
    CStyle(Style),
    #[xml(tag = "c:clrMapOvr")]
    CClrMapOvr(ColorMapOverride),
    #[xml(tag = "c:pivotSource")]
    CPivotSource(PivotSource),
    #[xml(tag = "c:protection")]
    CProtection(Protection),
    #[xml(tag = "c:chart")]
    CChart(Chart),
    #[xml(tag = "c:spPr")]
    CSpPr(ShapeProperties),
    #[xml(tag = "c:txPr")]
    CTxPr(TextProperties),
    #[xml(tag = "c:externalData")]
    CExternalData(ExternalData),
    #[xml(tag = "c:printSettings")]
    CPrintSettings(PrintSettings),
    #[xml(tag = "c:userShapes")]
    CUserShapes(UserShapesReference),
    #[xml(tag = "c:extLst")]
    CExtLst(ChartSpaceExtensionList),
}
/// User Shapes.
/// When the object is serialized out as xml, it's qualified name is c:userShapes.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:userShapes")]
pub struct UserShapes {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(child = "cdr:relSizeAnchor", child = "cdr:absSizeAnchor")]
    pub children: Vec<UserShapesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum UserShapesChildChoice {
    #[xml(tag = "cdr:relSizeAnchor")]
    CdrRelSizeAnchor(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart_drawing::RelativeAnchorSize,
    ),
    #[xml(tag = "cdr:absSizeAnchor")]
    CdrAbsSizeAnchor(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart_drawing::AbsoluteAnchorSize,
    ),
}
/// Reference to Chart Part.
/// When the object is serialized out as xml, it's qualified name is c:chart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:chart")]
pub struct ChartReference {
    /// Relationship Reference
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Legacy Drawing for Headers and Footers.
/// When the object is serialized out as xml, it's qualified name is c:legacyDrawingHF.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:legacyDrawingHF")]
pub struct LegacyDrawingHeaderFooter {
    /// Relationship Reference
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the UserShapesReference Class.
/// When the object is serialized out as xml, it's qualified name is c:userShapes.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:userShapes")]
pub struct UserShapesReference {
    /// Relationship Reference
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the RelationshipIdType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct RelationshipIdType {
    /// Relationship Reference
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Extension.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct Extension {
    /// Uniform Resource Identifier
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: Option<String>,
}
/// Numeric Value.
/// When the object is serialized out as xml, it's qualified name is c:v.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:v")]
pub struct NumericValue {
    #[xml(text)]
    pub child: String,
}
/// Format Code.
/// When the object is serialized out as xml, it's qualified name is c:formatCode.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:formatCode")]
pub struct FormatCode {
    #[xml(text)]
    pub child: String,
}
/// Odd Header.
/// When the object is serialized out as xml, it's qualified name is c:oddHeader.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:oddHeader")]
pub struct OddHeader {
    #[xml(text)]
    pub child: String,
}
/// Odd Footer.
/// When the object is serialized out as xml, it's qualified name is c:oddFooter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:oddFooter")]
pub struct OddFooter {
    #[xml(text)]
    pub child: String,
}
/// Even Header.
/// When the object is serialized out as xml, it's qualified name is c:evenHeader.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:evenHeader")]
pub struct EvenHeader {
    #[xml(text)]
    pub child: String,
}
/// Even Footer.
/// When the object is serialized out as xml, it's qualified name is c:evenFooter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:evenFooter")]
pub struct EvenFooter {
    #[xml(text)]
    pub child: String,
}
/// First Header.
/// When the object is serialized out as xml, it's qualified name is c:firstHeader.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:firstHeader")]
pub struct FirstHeader {
    #[xml(text)]
    pub child: String,
}
/// First Footer.
/// When the object is serialized out as xml, it's qualified name is c:firstFooter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:firstFooter")]
pub struct FirstFooter {
    #[xml(text)]
    pub child: String,
}
/// Pivot Name.
/// When the object is serialized out as xml, it's qualified name is c:name.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:name")]
pub struct PivotTableName {
    #[xml(text)]
    pub child: String,
}
/// Numeric Point.
/// When the object is serialized out as xml, it's qualified name is c:pt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:pt")]
pub struct NumericPoint {
    /// Index
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
    /// Number Format
    /// Represents the following attribute in the schema: :formatCode
    #[xml(attr = "formatCode")]
    pub format_code: Option<String>,
    ///Numeric Value
    #[xml(child = "c:v")]
    pub numeric_value: NumericValue,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct ExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<Extension>,
}
/// Number Reference.
/// When the object is serialized out as xml, it's qualified name is c:numRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:numRef")]
pub struct NumberReference {
    /// _
    #[xml(child = "c:f")]
    pub formula: Formula,
    /// _
    #[xml(child = "c:numCache")]
    pub numbering_cache: Option<NumberingCache>,
    /// _
    #[xml(child = "c:extLst")]
    pub num_ref_extension_list: Option<NumRefExtensionList>,
}
/// Number Literal.
/// When the object is serialized out as xml, it's qualified name is c:numLit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:numLit")]
pub struct NumberLiteral {
    ///Format Code
    #[xml(child = "c:formatCode")]
    pub format_code: Option<FormatCode>,
    ///Point Count
    #[xml(child = "c:ptCount")]
    pub point_count: Option<PointCount>,
    /// _
    #[xml(child = "c:pt")]
    pub c_pt: Vec<NumericPoint>,
    /// _
    #[xml(child = "c:extLst")]
    pub c_ext_lst: Option<ExtensionList>,
}
/// Defines the NumberingCache Class.
/// When the object is serialized out as xml, it's qualified name is c:numCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:numCache")]
pub struct NumberingCache {
    ///Format Code
    #[xml(child = "c:formatCode")]
    pub format_code: Option<FormatCode>,
    ///Point Count
    #[xml(child = "c:ptCount")]
    pub point_count: Option<PointCount>,
    /// _
    #[xml(child = "c:pt")]
    pub c_pt: Vec<NumericPoint>,
    /// _
    #[xml(child = "c:extLst")]
    pub c_ext_lst: Option<ExtensionList>,
}
/// Defines the NumberDataType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct NumberDataType {}
/// Level.
/// When the object is serialized out as xml, it's qualified name is c:lvl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:lvl")]
pub struct Level {
    /// _
    #[xml(child = "c:pt")]
    pub c_pt: Vec<StringPoint>,
}
/// Multi Level String Reference.
/// When the object is serialized out as xml, it's qualified name is c:multiLvlStrRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:multiLvlStrRef")]
pub struct MultiLevelStringReference {
    /// _
    #[xml(child = "c:f")]
    pub formula: Formula,
    /// _
    #[xml(child = "c:multiLvlStrCache")]
    pub multi_level_string_cache: Option<MultiLevelStringCache>,
    /// _
    #[xml(child = "c:extLst")]
    pub multi_lvl_str_ref_extension_list: Option<MultiLvlStrRefExtensionList>,
}
/// Defines the StringReference Class.
/// When the object is serialized out as xml, it's qualified name is c:strRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:strRef")]
pub struct StringReference {
    /// _
    #[xml(child = "c:f")]
    pub formula: Formula,
    /// _
    #[xml(child = "c:strCache")]
    pub string_cache: Option<StringCache>,
    /// _
    #[xml(child = "c:extLst")]
    pub str_ref_extension_list: Option<StrRefExtensionList>,
}
/// String Literal.
/// When the object is serialized out as xml, it's qualified name is c:strLit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:strLit")]
pub struct StringLiteral {
    /// _
    #[xml(child = "c:ptCount")]
    pub point_count: Option<PointCount>,
    /// _
    #[xml(child = "c:pt")]
    pub c_pt: Vec<StringPoint>,
    /// _
    #[xml(child = "c:extLst")]
    pub c_ext_lst: Option<StrDataExtensionList>,
}
/// Defines the StringCache Class.
/// When the object is serialized out as xml, it's qualified name is c:strCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:strCache")]
pub struct StringCache {
    /// _
    #[xml(child = "c:ptCount")]
    pub point_count: Option<PointCount>,
    /// _
    #[xml(child = "c:pt")]
    pub c_pt: Vec<StringPoint>,
    /// _
    #[xml(child = "c:extLst")]
    pub c_ext_lst: Option<StrDataExtensionList>,
}
/// Defines the StringDataType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct StringDataType {}
/// Layout Target.
/// When the object is serialized out as xml, it's qualified name is c:layoutTarget.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:layoutTarget")]
pub struct LayoutTarget {
    /// Layout Target Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<LayoutTargetValues>,
}
/// Left Mode.
/// When the object is serialized out as xml, it's qualified name is c:xMode.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:xMode")]
pub struct LeftMode {
    /// Layout Mode Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<LayoutModeValues>,
}
/// Top Mode.
/// When the object is serialized out as xml, it's qualified name is c:yMode.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:yMode")]
pub struct TopMode {
    /// Layout Mode Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<LayoutModeValues>,
}
/// Width Mode.
/// When the object is serialized out as xml, it's qualified name is c:wMode.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:wMode")]
pub struct WidthMode {
    /// Layout Mode Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<LayoutModeValues>,
}
/// Height Mode.
/// When the object is serialized out as xml, it's qualified name is c:hMode.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:hMode")]
pub struct HeightMode {
    /// Layout Mode Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<LayoutModeValues>,
}
/// Defines the LayoutModeType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct LayoutModeType {
    /// Layout Mode Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<LayoutModeValues>,
}
/// Manual Layout.
/// When the object is serialized out as xml, it's qualified name is c:manualLayout.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:manualLayout")]
pub struct ManualLayout {
    ///Layout Target
    #[xml(child = "c:layoutTarget")]
    pub layout_target: Option<LayoutTarget>,
    ///Left Mode
    #[xml(child = "c:xMode")]
    pub left_mode: Option<LeftMode>,
    ///Top Mode
    #[xml(child = "c:yMode")]
    pub top_mode: Option<TopMode>,
    ///Width Mode
    #[xml(child = "c:wMode")]
    pub width_mode: Option<WidthMode>,
    ///Height Mode
    #[xml(child = "c:hMode")]
    pub height_mode: Option<HeightMode>,
    ///Left
    #[xml(child = "c:x")]
    pub left: Option<Left>,
    ///Top
    #[xml(child = "c:y")]
    pub top: Option<Top>,
    ///Width
    #[xml(child = "c:w")]
    pub width: Option<Width>,
    ///Height
    #[xml(child = "c:h")]
    pub height: Option<Height>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// X Rotation.
/// When the object is serialized out as xml, it's qualified name is c:rotX.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:rotX")]
pub struct RotateX {
    /// X Rotation Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<u8>,
}
/// Height Percent.
/// When the object is serialized out as xml, it's qualified name is c:hPercent.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:hPercent")]
pub struct HeightPercent {
    /// Height Percent Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<i16>,
}
/// Y Rotation.
/// When the object is serialized out as xml, it's qualified name is c:rotY.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:rotY")]
pub struct RotateY {
    /// Y Rotation Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<i16>,
}
/// Depth Percent.
/// When the object is serialized out as xml, it's qualified name is c:depthPercent.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:depthPercent")]
pub struct DepthPercent {
    /// Depth Percent Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<i16>,
}
/// Perspective.
/// When the object is serialized out as xml, it's qualified name is c:perspective.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:perspective")]
pub struct Perspective {
    /// Perspective Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<u8>,
}
/// Symbol.
/// When the object is serialized out as xml, it's qualified name is c:symbol.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:symbol")]
pub struct Symbol {
    /// Marker Style Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: MarkerStyleValues,
}
/// Size.
/// When the object is serialized out as xml, it's qualified name is c:size.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:size")]
pub struct Size {
    /// Marker Size Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<u8>,
}
/// Marker.
/// When the object is serialized out as xml, it's qualified name is c:marker.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:marker")]
pub struct Marker {
    ///Symbol
    #[xml(child = "c:symbol")]
    pub symbol: Option<Symbol>,
    ///Size
    #[xml(child = "c:size")]
    pub size: Option<Size>,
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the PictureOptions Class.
/// When the object is serialized out as xml, it's qualified name is c:pictureOptions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:pictureOptions")]
pub struct PictureOptions {
    ///Apply To Front
    #[xml(child = "c:applyToFront")]
    pub apply_to_front: Option<ApplyToFront>,
    ///Apply To Sides
    #[xml(child = "c:applyToSides")]
    pub apply_to_sides: Option<ApplyToSides>,
    ///Apply to End
    #[xml(child = "c:applyToEnd")]
    pub apply_to_end: Option<ApplyToEnd>,
    ///Picture Format
    #[xml(child = "c:pictureFormat")]
    pub picture_format: Option<PictureFormat>,
    ///Picture Stack Unit
    #[xml(child = "c:pictureStackUnit")]
    pub picture_stack_unit: Option<PictureStackUnit>,
}
/// Trendline Type.
/// When the object is serialized out as xml, it's qualified name is c:trendlineType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:trendlineType")]
pub struct TrendlineType {
    /// Trendline Type Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<TrendlineValues>,
}
/// Polynomial Trendline Order.
/// When the object is serialized out as xml, it's qualified name is c:order.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:order")]
pub struct PolynomialOrder {
    /// Order Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: u8,
}
/// Period.
/// When the object is serialized out as xml, it's qualified name is c:period.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:period")]
pub struct Period {
    /// Period Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Trendline Label.
/// When the object is serialized out as xml, it's qualified name is c:trendlineLbl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:trendlineLbl")]
pub struct TrendlineLabel {
    ///Layout
    #[xml(child = "c:layout")]
    pub layout: Option<Layout>,
    /// _
    #[xml(child = "c:tx")]
    pub chart_text: Option<ChartText>,
    ///Number Format
    #[xml(child = "c:numFmt")]
    pub numbering_format: Option<NumberingFormat>,
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
    /// _
    #[xml(child = "c:txPr")]
    pub text_properties: Option<TextProperties>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Error Bar Direction.
/// When the object is serialized out as xml, it's qualified name is c:errDir.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:errDir")]
pub struct ErrorDirection {
    /// Error Bar Direction Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: ErrorBarDirectionValues,
}
/// Error Bar Type.
/// When the object is serialized out as xml, it's qualified name is c:errBarType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:errBarType")]
pub struct ErrorBarType {
    /// Error Bar Type Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: ErrorBarValues,
}
/// Error Bar Value Type.
/// When the object is serialized out as xml, it's qualified name is c:errValType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:errValType")]
pub struct ErrorBarValueType {
    /// Error Bar Type Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: ErrorValues,
}
/// Plus.
/// When the object is serialized out as xml, it's qualified name is c:plus.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:plus")]
pub struct Plus {
    #[xml(child = "c:numRef", child = "c:numLit")]
    pub children: Vec<PlusChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PlusChildChoice {
    #[xml(tag = "c:numRef")]
    CNumRef(NumberReference),
    #[xml(tag = "c:numLit")]
    CNumLit(NumberLiteral),
}
/// Minus.
/// When the object is serialized out as xml, it's qualified name is c:minus.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:minus")]
pub struct Minus {
    #[xml(child = "c:numRef", child = "c:numLit")]
    pub children: Vec<MinusChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MinusChildChoice {
    #[xml(tag = "c:numRef")]
    CNumRef(NumberReference),
    #[xml(tag = "c:numLit")]
    CNumLit(NumberLiteral),
}
/// Defines the Values Class.
/// When the object is serialized out as xml, it's qualified name is c:val.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:val")]
pub struct Values {
    #[xml(child = "c:numRef", child = "c:numLit")]
    pub children: Vec<ValuesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ValuesChildChoice {
    #[xml(tag = "c:numRef")]
    CNumRef(NumberReference),
    #[xml(tag = "c:numLit")]
    CNumLit(NumberLiteral),
}
/// Defines the YValues Class.
/// When the object is serialized out as xml, it's qualified name is c:yVal.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:yVal")]
pub struct YValues {
    #[xml(child = "c:numRef", child = "c:numLit")]
    pub children: Vec<YValuesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum YValuesChildChoice {
    #[xml(tag = "c:numRef")]
    CNumRef(NumberReference),
    #[xml(tag = "c:numLit")]
    CNumLit(NumberLiteral),
}
/// Defines the BubbleSize Class.
/// When the object is serialized out as xml, it's qualified name is c:bubbleSize.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:bubbleSize")]
pub struct BubbleSize {
    #[xml(child = "c:numRef", child = "c:numLit")]
    pub children: Vec<BubbleSizeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BubbleSizeChildChoice {
    #[xml(tag = "c:numRef")]
    CNumRef(NumberReference),
    #[xml(tag = "c:numLit")]
    CNumLit(NumberLiteral),
}
/// Defines the NumberDataSourceType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct NumberDataSourceType {
    #[xml(child = "c:numRef", child = "c:numLit")]
    pub children: Vec<NumberDataSourceTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NumberDataSourceTypeChildChoice {
    #[xml(tag = "c:numRef")]
    CNumRef(NumberReference),
    #[xml(tag = "c:numLit")]
    CNumLit(NumberLiteral),
}
/// Gap Width.
/// When the object is serialized out as xml, it's qualified name is c:gapWidth.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:gapWidth")]
pub struct GapWidth {
    /// Gap Size Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<i16>,
}
/// Defines the GapDepth Class.
/// When the object is serialized out as xml, it's qualified name is c:gapDepth.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:gapDepth")]
pub struct GapDepth {
    /// Gap Size Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<i16>,
}
/// Defines the GapAmountType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct GapAmountType {
    /// Gap Size Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<i16>,
}
/// Up Bars.
/// When the object is serialized out as xml, it's qualified name is c:upBars.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:upBars")]
pub struct UpBars {
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
}
/// Down Bars.
/// When the object is serialized out as xml, it's qualified name is c:downBars.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:downBars")]
pub struct DownBars {
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
}
/// Defines the UpDownBarType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct UpDownBarType {}
/// Pie of Pie or Bar of Pie Type.
/// When the object is serialized out as xml, it's qualified name is c:ofPieType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ofPieType")]
pub struct OfPieType {
    /// Pie of Pie or Bar of Pie Type Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: OfPieValues,
}
/// Split Type.
/// When the object is serialized out as xml, it's qualified name is c:splitType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:splitType")]
pub struct SplitType {
    /// Split Type Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: SplitValues,
}
/// Custom Split.
/// When the object is serialized out as xml, it's qualified name is c:custSplit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:custSplit")]
pub struct CustomSplit {
    /// _
    #[xml(child = "c:secondPiePt")]
    pub c_second_pie_pt: Vec<SecondPiePoint>,
}
/// Second Pie Size.
/// When the object is serialized out as xml, it's qualified name is c:secondPieSize.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:secondPieSize")]
pub struct SecondPieSize {
    /// Second Pie Size Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<i16>,
}
/// Band Format.
/// When the object is serialized out as xml, it's qualified name is c:bandFmt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:bandFmt")]
pub struct BandFormat {
    /// _
    #[xml(child = "c:idx")]
    pub index: Index,
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
}
/// Picture Format.
/// When the object is serialized out as xml, it's qualified name is c:pictureFormat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:pictureFormat")]
pub struct PictureFormat {
    /// Picture Format Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: PictureFormatValues,
}
/// Picture Stack Unit.
/// When the object is serialized out as xml, it's qualified name is c:pictureStackUnit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:pictureStackUnit")]
pub struct PictureStackUnit {
    /// Picture Stack Unit
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Built in Display Unit Value.
/// When the object is serialized out as xml, it's qualified name is c:builtInUnit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:builtInUnit")]
pub struct BuiltInUnit {
    /// Built In Unit Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<BuiltInUnitValues>,
}
/// Display Units Label.
/// When the object is serialized out as xml, it's qualified name is c:dispUnitsLbl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:dispUnitsLbl")]
pub struct DisplayUnitsLabel {
    ///Layout
    #[xml(child = "c:layout")]
    pub layout: Option<Layout>,
    /// _
    #[xml(child = "c:tx")]
    pub chart_text: Option<ChartText>,
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
    /// _
    #[xml(child = "c:txPr")]
    pub text_properties: Option<TextProperties>,
}
/// Logarithmic Base.
/// When the object is serialized out as xml, it's qualified name is c:logBase.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:logBase")]
pub struct LogBase {
    /// Logarithmic Base Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Axis Orientation.
/// When the object is serialized out as xml, it's qualified name is c:orientation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:orientation")]
pub struct Orientation {
    /// Orientation Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<OrientationValues>,
}
/// Pivot Format.
/// When the object is serialized out as xml, it's qualified name is c:pivotFmt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:pivotFmt")]
pub struct PivotFormat {
    ///Index
    #[xml(child = "c:idx")]
    pub index: Index,
    /// _
    #[xml(child = "c:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    ///Marker
    #[xml(child = "c:marker")]
    pub marker: Option<Marker>,
    ///Data Label
    #[xml(child = "c:dLbl")]
    pub data_label: Option<DataLabel>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Legend Position.
/// When the object is serialized out as xml, it's qualified name is c:legendPos.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:legendPos")]
pub struct LegendPosition {
    /// Legend Position Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<LegendPositionValues>,
}
/// Legend Entry.
/// When the object is serialized out as xml, it's qualified name is c:legendEntry.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:legendEntry")]
pub struct LegendEntry {
    #[xml(child = "c:idx", child = "c:delete", child = "c:txPr", child = "c:extLst")]
    pub children: Vec<LegendEntryChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LegendEntryChildChoice {
    #[xml(tag = "c:idx")]
    CIdx(Index),
    #[xml(tag = "c:delete")]
    CDelete(Delete),
    #[xml(tag = "c:txPr")]
    CTxPr(TextProperties),
    #[xml(tag = "c:extLst")]
    CExtLst(ExtensionList),
}
/// Header and Footer.
/// When the object is serialized out as xml, it's qualified name is c:headerFooter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:headerFooter")]
pub struct HeaderFooter {
    /// Align With Margins
    /// Represents the following attribute in the schema: :alignWithMargins
    #[xml(attr = "alignWithMargins")]
    pub align_with_margins: Option<bool>,
    /// Different Odd Even
    /// Represents the following attribute in the schema: :differentOddEven
    #[xml(attr = "differentOddEven")]
    pub different_odd_even: Option<bool>,
    /// Different First
    /// Represents the following attribute in the schema: :differentFirst
    #[xml(attr = "differentFirst")]
    pub different_first: Option<bool>,
    ///Odd Header
    #[xml(child = "c:oddHeader")]
    pub odd_header: Option<OddHeader>,
    ///Odd Footer
    #[xml(child = "c:oddFooter")]
    pub odd_footer: Option<OddFooter>,
    ///Even Header
    #[xml(child = "c:evenHeader")]
    pub even_header: Option<EvenHeader>,
    ///Even Footer
    #[xml(child = "c:evenFooter")]
    pub even_footer: Option<EvenFooter>,
    ///First Header
    #[xml(child = "c:firstHeader")]
    pub first_header: Option<FirstHeader>,
    ///First Footer
    #[xml(child = "c:firstFooter")]
    pub first_footer: Option<FirstFooter>,
}
/// Page Margins.
/// When the object is serialized out as xml, it's qualified name is c:pageMargins.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:pageMargins")]
pub struct PageMargins {
    /// Left
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub left: f64,
    /// Right
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub right: f64,
    /// Top
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub top: f64,
    /// Bottom
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bottom: f64,
    /// Header
    /// Represents the following attribute in the schema: :header
    #[xml(attr = "header")]
    pub header: f64,
    /// Footer
    /// Represents the following attribute in the schema: :footer
    #[xml(attr = "footer")]
    pub footer: f64,
}
/// Page Setup.
/// When the object is serialized out as xml, it's qualified name is c:pageSetup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:pageSetup")]
pub struct PageSetup {
    /// Page Size
    /// Represents the following attribute in the schema: :paperSize
    #[xml(attr = "paperSize")]
    pub paper_size: Option<i32>,
    /// First Page Number
    /// Represents the following attribute in the schema: :firstPageNumber
    #[xml(attr = "firstPageNumber")]
    pub first_page_number: Option<i32>,
    /// Orientation
    /// Represents the following attribute in the schema: :orientation
    #[xml(attr = "orientation")]
    pub orientation: Option<PageSetupOrientationValues>,
    /// Black and White
    /// Represents the following attribute in the schema: :blackAndWhite
    #[xml(attr = "blackAndWhite")]
    pub black_and_white: Option<bool>,
    /// Draft
    /// Represents the following attribute in the schema: :draft
    #[xml(attr = "draft")]
    pub draft: Option<bool>,
    /// Use First Page Number
    /// Represents the following attribute in the schema: :useFirstPageNumber
    #[xml(attr = "useFirstPageNumber")]
    pub use_first_page_number: Option<bool>,
    /// Horizontal DPI
    /// Represents the following attribute in the schema: :horizontalDpi
    #[xml(attr = "horizontalDpi")]
    pub horizontal_dpi: Option<i32>,
    /// Vertical DPI
    /// Represents the following attribute in the schema: :verticalDpi
    #[xml(attr = "verticalDpi")]
    pub vertical_dpi: Option<i32>,
    /// Copies
    /// Represents the following attribute in the schema: :copies
    #[xml(attr = "copies")]
    pub copies: Option<i32>,
}
/// Defines the ShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is c:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:spPr")]
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
/// Data Label.
/// When the object is serialized out as xml, it's qualified name is c:dLbl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:dLbl")]
pub struct DataLabel {
    #[xml(
        child = "c:idx",
        child = "c:delete",
        child = "c:layout",
        child = "c:tx",
        child = "c:numFmt",
        child = "c:spPr",
        child = "c:txPr",
        child = "c:dLblPos",
        child = "c:showLegendKey",
        child = "c:showVal",
        child = "c:showCatName",
        child = "c:showSerName",
        child = "c:showPercent",
        child = "c:showBubbleSize",
        child = "c:separator",
        child = "c:extLst",
    )]
    pub children: Vec<DataLabelChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DataLabelChildChoice {
    #[xml(tag = "c:idx")]
    CIdx(Index),
    #[xml(tag = "c:delete")]
    CDelete(Delete),
    #[xml(tag = "c:layout")]
    CLayout(Layout),
    #[xml(tag = "c:tx")]
    CTx(ChartText),
    #[xml(tag = "c:numFmt")]
    CNumFmt(NumberingFormat),
    #[xml(tag = "c:spPr")]
    CSpPr(ChartShapeProperties),
    #[xml(tag = "c:txPr")]
    CTxPr(TextProperties),
    #[xml(tag = "c:dLblPos")]
    CDLblPos(DataLabelPosition),
    #[xml(tag = "c:showLegendKey")]
    CShowLegendKey(ShowLegendKey),
    #[xml(tag = "c:showVal")]
    CShowVal(ShowValue),
    #[xml(tag = "c:showCatName")]
    CShowCatName(ShowCategoryName),
    #[xml(tag = "c:showSerName")]
    CShowSerName(ShowSeriesName),
    #[xml(tag = "c:showPercent")]
    CShowPercent(ShowPercent),
    #[xml(tag = "c:showBubbleSize")]
    CShowBubbleSize(ShowBubbleSize),
    #[xml(tag = "c:separator")]
    CSeparator(Separator),
    #[xml(tag = "c:extLst")]
    CExtLst(DLblExtensionList),
}
/// Area Charts.
/// When the object is serialized out as xml, it's qualified name is c:areaChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:areaChart")]
pub struct AreaChart {
    #[xml(
        child = "c:grouping",
        child = "c:varyColors",
        child = "c:ser",
        child = "c:dLbls",
        child = "c:dropLines",
        child = "c:axId",
        child = "c:extLst",
    )]
    pub children: Vec<AreaChartChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum AreaChartChildChoice {
    #[xml(tag = "c:grouping")]
    CGrouping(Grouping),
    #[xml(tag = "c:varyColors")]
    CVaryColors(VaryColors),
    #[xml(tag = "c:ser")]
    CSer(AreaChartSeries),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:dropLines")]
    CDropLines(DropLines),
    #[xml(tag = "c:axId")]
    CAxId(AxisId),
    #[xml(tag = "c:extLst")]
    CExtLst(AreaChartExtensionList),
}
/// 3D Area Charts.
/// When the object is serialized out as xml, it's qualified name is c:area3DChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:area3DChart")]
pub struct Area3DChart {
    #[xml(
        child = "c:grouping",
        child = "c:varyColors",
        child = "c:ser",
        child = "c:dLbls",
        child = "c:dropLines",
        child = "c:gapDepth",
        child = "c:axId",
        child = "c:extLst",
    )]
    pub children: Vec<Area3DChartChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Area3DChartChildChoice {
    #[xml(tag = "c:grouping")]
    CGrouping(Grouping),
    #[xml(tag = "c:varyColors")]
    CVaryColors(VaryColors),
    #[xml(tag = "c:ser")]
    CSer(AreaChartSeries),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:dropLines")]
    CDropLines(DropLines),
    #[xml(tag = "c:gapDepth")]
    CGapDepth(GapDepth),
    #[xml(tag = "c:axId")]
    CAxId(AxisId),
    #[xml(tag = "c:extLst")]
    CExtLst(Area3DChartExtensionList),
}
/// Line Charts.
/// When the object is serialized out as xml, it's qualified name is c:lineChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:lineChart")]
pub struct LineChart {
    #[xml(
        child = "c:grouping",
        child = "c:varyColors",
        child = "c:ser",
        child = "c:dLbls",
        child = "c:dropLines",
        child = "c:hiLowLines",
        child = "c:upDownBars",
        child = "c:marker",
        child = "c:smooth",
        child = "c:axId",
        child = "c:extLst",
    )]
    pub children: Vec<LineChartChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LineChartChildChoice {
    #[xml(tag = "c:grouping")]
    CGrouping(Grouping),
    #[xml(tag = "c:varyColors")]
    CVaryColors(VaryColors),
    #[xml(tag = "c:ser")]
    CSer(LineChartSeries),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:dropLines")]
    CDropLines(DropLines),
    #[xml(tag = "c:hiLowLines")]
    CHiLowLines(HighLowLines),
    #[xml(tag = "c:upDownBars")]
    CUpDownBars(UpDownBars),
    #[xml(tag = "c:marker")]
    CMarker(ShowMarker),
    #[xml(tag = "c:smooth")]
    CSmooth(Smooth),
    #[xml(tag = "c:axId")]
    CAxId(AxisId),
    #[xml(tag = "c:extLst")]
    CExtLst(LineChartExtensionList),
}
/// 3D Line Charts.
/// When the object is serialized out as xml, it's qualified name is c:line3DChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:line3DChart")]
pub struct Line3DChart {
    #[xml(
        child = "c:grouping",
        child = "c:varyColors",
        child = "c:ser",
        child = "c:dLbls",
        child = "c:dropLines",
        child = "c:gapDepth",
        child = "c:axId",
        child = "c:extLst",
    )]
    pub children: Vec<Line3DChartChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Line3DChartChildChoice {
    #[xml(tag = "c:grouping")]
    CGrouping(Grouping),
    #[xml(tag = "c:varyColors")]
    CVaryColors(VaryColors),
    #[xml(tag = "c:ser")]
    CSer(LineChartSeries),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:dropLines")]
    CDropLines(DropLines),
    #[xml(tag = "c:gapDepth")]
    CGapDepth(GapDepth),
    #[xml(tag = "c:axId")]
    CAxId(AxisId),
    #[xml(tag = "c:extLst")]
    CExtLst(Line3DChartExtensionList),
}
/// Stock Charts.
/// When the object is serialized out as xml, it's qualified name is c:stockChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:stockChart")]
pub struct StockChart {
    /// _
    #[xml(child = "c:ser")]
    pub c_ser: Vec<LineChartSeries>,
    /// _
    #[xml(child = "c:dLbls")]
    pub c_d_lbls: Option<DataLabels>,
    /// _
    #[xml(child = "c:dropLines")]
    pub c_drop_lines: Option<DropLines>,
    /// _
    #[xml(child = "c:hiLowLines")]
    pub c_hi_low_lines: Option<HighLowLines>,
    /// _
    #[xml(child = "c:upDownBars")]
    pub c_up_down_bars: Option<UpDownBars>,
    /// _
    #[xml(child = "c:axId")]
    pub c_ax_id: Vec<AxisId>,
    /// _
    #[xml(child = "c:extLst")]
    pub c_ext_lst: Option<StockChartExtensionList>,
}
/// Radar Charts.
/// When the object is serialized out as xml, it's qualified name is c:radarChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:radarChart")]
pub struct RadarChart {
    /// _
    #[xml(child = "c:radarStyle")]
    pub radar_style: RadarStyle,
    /// _
    #[xml(child = "c:varyColors")]
    pub vary_colors: Option<VaryColors>,
    /// _
    #[xml(child = "c:ser")]
    pub c_ser: Vec<RadarChartSeries>,
    /// _
    #[xml(child = "c:dLbls")]
    pub c_d_lbls: Option<DataLabels>,
    /// _
    #[xml(child = "c:axId")]
    pub c_ax_id: Vec<AxisId>,
    /// _
    #[xml(child = "c:extLst")]
    pub c_ext_lst: Option<RadarChartExtensionList>,
}
/// Scatter Charts.
/// When the object is serialized out as xml, it's qualified name is c:scatterChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:scatterChart")]
pub struct ScatterChart {
    /// _
    #[xml(child = "c:scatterStyle")]
    pub scatter_style: ScatterStyle,
    /// _
    #[xml(child = "c:varyColors")]
    pub vary_colors: Option<VaryColors>,
    /// _
    #[xml(child = "c:ser")]
    pub c_ser: Vec<ScatterChartSeries>,
    /// _
    #[xml(child = "c:dLbls")]
    pub c_d_lbls: Option<DataLabels>,
    /// _
    #[xml(child = "c:axId")]
    pub c_ax_id: Vec<AxisId>,
    /// _
    #[xml(child = "c:extLst")]
    pub c_ext_lst: Option<ScatterChartExtensionList>,
}
/// Pie Charts.
/// When the object is serialized out as xml, it's qualified name is c:pieChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:pieChart")]
pub struct PieChart {
    #[xml(
        child = "c:varyColors",
        child = "c:ser",
        child = "c:dLbls",
        child = "c:firstSliceAng",
        child = "c:extLst",
    )]
    pub children: Vec<PieChartChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PieChartChildChoice {
    #[xml(tag = "c:varyColors")]
    CVaryColors(VaryColors),
    #[xml(tag = "c:ser")]
    CSer(PieChartSeries),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:firstSliceAng")]
    CFirstSliceAng(FirstSliceAngle),
    #[xml(tag = "c:extLst")]
    CExtLst(PieChartExtensionList),
}
/// 3D Pie Charts.
/// When the object is serialized out as xml, it's qualified name is c:pie3DChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:pie3DChart")]
pub struct Pie3DChart {
    #[xml(
        child = "c:varyColors",
        child = "c:ser",
        child = "c:dLbls",
        child = "c:extLst",
    )]
    pub children: Vec<Pie3DChartChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Pie3DChartChildChoice {
    #[xml(tag = "c:varyColors")]
    CVaryColors(VaryColors),
    #[xml(tag = "c:ser")]
    CSer(PieChartSeries),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:extLst")]
    CExtLst(Pie3DChartExtensionList),
}
/// Doughnut Charts.
/// When the object is serialized out as xml, it's qualified name is c:doughnutChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:doughnutChart")]
pub struct DoughnutChart {
    #[xml(
        child = "c:varyColors",
        child = "c:ser",
        child = "c:dLbls",
        child = "c:firstSliceAng",
        child = "c:holeSize",
        child = "c:extLst",
    )]
    pub children: Vec<DoughnutChartChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DoughnutChartChildChoice {
    #[xml(tag = "c:varyColors")]
    CVaryColors(VaryColors),
    #[xml(tag = "c:ser")]
    CSer(PieChartSeries),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:firstSliceAng")]
    CFirstSliceAng(FirstSliceAngle),
    #[xml(tag = "c:holeSize")]
    CHoleSize(HoleSize),
    #[xml(tag = "c:extLst")]
    CExtLst(ExtensionList),
}
/// Bar Charts.
/// When the object is serialized out as xml, it's qualified name is c:barChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:barChart")]
pub struct BarChart {
    #[xml(
        child = "c:barDir",
        child = "c:grouping",
        child = "c:varyColors",
        child = "c:ser",
        child = "c:dLbls",
        child = "c:gapWidth",
        child = "c:overlap",
        child = "c:serLines",
        child = "c:axId",
        child = "c:extLst",
    )]
    pub children: Vec<BarChartChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BarChartChildChoice {
    #[xml(tag = "c:barDir")]
    CBarDir(BarDirection),
    #[xml(tag = "c:grouping")]
    CGrouping(BarGrouping),
    #[xml(tag = "c:varyColors")]
    CVaryColors(VaryColors),
    #[xml(tag = "c:ser")]
    CSer(BarChartSeries),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:gapWidth")]
    CGapWidth(GapWidth),
    #[xml(tag = "c:overlap")]
    COverlap(Overlap),
    #[xml(tag = "c:serLines")]
    CSerLines(SeriesLines),
    #[xml(tag = "c:axId")]
    CAxId(AxisId),
    #[xml(tag = "c:extLst")]
    CExtLst(BarChartExtensionList),
}
/// 3D Bar Charts.
/// When the object is serialized out as xml, it's qualified name is c:bar3DChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:bar3DChart")]
pub struct Bar3DChart {
    #[xml(
        child = "c:barDir",
        child = "c:grouping",
        child = "c:varyColors",
        child = "c:ser",
        child = "c:dLbls",
        child = "c:gapWidth",
        child = "c:gapDepth",
        child = "c:shape",
        child = "c:axId",
        child = "c:extLst",
    )]
    pub children: Vec<Bar3DChartChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Bar3DChartChildChoice {
    #[xml(tag = "c:barDir")]
    CBarDir(BarDirection),
    #[xml(tag = "c:grouping")]
    CGrouping(BarGrouping),
    #[xml(tag = "c:varyColors")]
    CVaryColors(VaryColors),
    #[xml(tag = "c:ser")]
    CSer(BarChartSeries),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:gapWidth")]
    CGapWidth(GapWidth),
    #[xml(tag = "c:gapDepth")]
    CGapDepth(GapDepth),
    #[xml(tag = "c:shape")]
    CShape(Shape),
    #[xml(tag = "c:axId")]
    CAxId(AxisId),
    #[xml(tag = "c:extLst")]
    CExtLst(Bar3DChartExtensionList),
}
/// Pie of Pie or Bar of Pie Charts.
/// When the object is serialized out as xml, it's qualified name is c:ofPieChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ofPieChart")]
pub struct OfPieChart {
    #[xml(
        child = "c:ofPieType",
        child = "c:varyColors",
        child = "c:ser",
        child = "c:dLbls",
        child = "c:gapWidth",
        child = "c:splitType",
        child = "c:splitPos",
        child = "c:custSplit",
        child = "c:secondPieSize",
        child = "c:serLines",
        child = "c:extLst",
    )]
    pub children: Vec<OfPieChartChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfPieChartChildChoice {
    #[xml(tag = "c:ofPieType")]
    COfPieType(OfPieType),
    #[xml(tag = "c:varyColors")]
    CVaryColors(VaryColors),
    #[xml(tag = "c:ser")]
    CSer(PieChartSeries),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:gapWidth")]
    CGapWidth(GapWidth),
    #[xml(tag = "c:splitType")]
    CSplitType(SplitType),
    #[xml(tag = "c:splitPos")]
    CSplitPos(SplitPosition),
    #[xml(tag = "c:custSplit")]
    CCustSplit(CustomSplit),
    #[xml(tag = "c:secondPieSize")]
    CSecondPieSize(SecondPieSize),
    #[xml(tag = "c:serLines")]
    CSerLines(SeriesLines),
    #[xml(tag = "c:extLst")]
    CExtLst(ExtensionList),
}
/// Surface Charts.
/// When the object is serialized out as xml, it's qualified name is c:surfaceChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:surfaceChart")]
pub struct SurfaceChart {
    #[xml(
        child = "c:wireframe",
        child = "c:ser",
        child = "c:bandFmts",
        child = "c:axId",
        child = "c:extLst",
    )]
    pub children: Vec<SurfaceChartChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SurfaceChartChildChoice {
    #[xml(tag = "c:wireframe")]
    CWireframe(Wireframe),
    #[xml(tag = "c:ser")]
    CSer(SurfaceChartSeries),
    #[xml(tag = "c:bandFmts")]
    CBandFmts(BandFormats),
    #[xml(tag = "c:axId")]
    CAxId(AxisId),
    #[xml(tag = "c:extLst")]
    CExtLst(SurfaceChartExtensionList),
}
/// 3D Surface Charts.
/// When the object is serialized out as xml, it's qualified name is c:surface3DChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:surface3DChart")]
pub struct Surface3DChart {
    /// _
    #[xml(child = "c:wireframe")]
    pub wireframe: Option<Wireframe>,
    /// _
    #[xml(child = "c:varyColors")]
    pub vary_colors: Option<VaryColors>,
    /// _
    #[xml(child = "c:ser")]
    pub c_ser: Vec<SurfaceChartSeries>,
    /// _
    #[xml(child = "c:bandFmts")]
    pub c_band_fmts: Option<BandFormats>,
    /// _
    #[xml(child = "c:axId")]
    pub c_ax_id: Vec<AxisId>,
    /// _
    #[xml(child = "c:extLst")]
    pub c_ext_lst: Option<Surface3DChartExtensionList>,
}
/// Bubble Charts.
/// When the object is serialized out as xml, it's qualified name is c:bubbleChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:bubbleChart")]
pub struct BubbleChart {
    /// _
    #[xml(child = "c:varyColors")]
    pub vary_colors: Option<VaryColors>,
    /// _
    #[xml(child = "c:ser")]
    pub c_ser: Vec<BubbleChartSeries>,
    /// _
    #[xml(child = "c:dLbls")]
    pub c_d_lbls: Option<DataLabels>,
    /// _
    #[xml(child = "c:bubble3D")]
    pub c_bubble3_d: Option<Bubble3D>,
    /// _
    #[xml(child = "c:bubbleScale")]
    pub c_bubble_scale: Option<BubbleScale>,
    /// _
    #[xml(child = "c:showNegBubbles")]
    pub c_show_neg_bubbles: Option<ShowNegativeBubbles>,
    /// _
    #[xml(child = "c:sizeRepresents")]
    pub c_size_represents: Option<SizeRepresents>,
    /// _
    #[xml(child = "c:axId")]
    pub c_ax_id: Vec<AxisId>,
    /// _
    #[xml(child = "c:extLst")]
    pub c_ext_lst: Option<BubbleChartExtensionList>,
}
/// Value Axis.
/// When the object is serialized out as xml, it's qualified name is c:valAx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:valAx")]
pub struct ValueAxis {
    #[xml(
        child = "c:axId",
        child = "c:scaling",
        child = "c:delete",
        child = "c:axPos",
        child = "c:majorGridlines",
        child = "c:minorGridlines",
        child = "c:title",
        child = "c:numFmt",
        child = "c:majorTickMark",
        child = "c:minorTickMark",
        child = "c:tickLblPos",
        child = "c:spPr",
        child = "c:txPr",
        child = "c:crossAx",
        child = "c:crosses",
        child = "c:crossesAt",
        child = "c:crossBetween",
        child = "c:majorUnit",
        child = "c:minorUnit",
        child = "c:dispUnits",
        child = "c:extLst",
    )]
    pub children: Vec<ValueAxisChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ValueAxisChildChoice {
    #[xml(tag = "c:axId")]
    CAxId(AxisId),
    #[xml(tag = "c:scaling")]
    CScaling(Scaling),
    #[xml(tag = "c:delete")]
    CDelete(Delete),
    #[xml(tag = "c:axPos")]
    CAxPos(AxisPosition),
    #[xml(tag = "c:majorGridlines")]
    CMajorGridlines(MajorGridlines),
    #[xml(tag = "c:minorGridlines")]
    CMinorGridlines(MinorGridlines),
    #[xml(tag = "c:title")]
    CTitle(Title),
    #[xml(tag = "c:numFmt")]
    CNumFmt(NumberingFormat),
    #[xml(tag = "c:majorTickMark")]
    CMajorTickMark(MajorTickMark),
    #[xml(tag = "c:minorTickMark")]
    CMinorTickMark(MinorTickMark),
    #[xml(tag = "c:tickLblPos")]
    CTickLblPos(TickLabelPosition),
    #[xml(tag = "c:spPr")]
    CSpPr(ChartShapeProperties),
    #[xml(tag = "c:txPr")]
    CTxPr(TextProperties),
    #[xml(tag = "c:crossAx")]
    CCrossAx(CrossingAxis),
    #[xml(tag = "c:crosses")]
    CCrosses(Crosses),
    #[xml(tag = "c:crossesAt")]
    CCrossesAt(CrossesAt),
    #[xml(tag = "c:crossBetween")]
    CCrossBetween(CrossBetween),
    #[xml(tag = "c:majorUnit")]
    CMajorUnit(MajorUnit),
    #[xml(tag = "c:minorUnit")]
    CMinorUnit(MinorUnit),
    #[xml(tag = "c:dispUnits")]
    CDispUnits(DisplayUnits),
    #[xml(tag = "c:extLst")]
    CExtLst(ValAxExtensionList),
}
/// Category Axis Data.
/// When the object is serialized out as xml, it's qualified name is c:catAx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:catAx")]
pub struct CategoryAxis {
    #[xml(
        child = "c:axId",
        child = "c:scaling",
        child = "c:delete",
        child = "c:axPos",
        child = "c:majorGridlines",
        child = "c:minorGridlines",
        child = "c:title",
        child = "c:numFmt",
        child = "c:majorTickMark",
        child = "c:minorTickMark",
        child = "c:tickLblPos",
        child = "c:spPr",
        child = "c:txPr",
        child = "c:crossAx",
        child = "c:crosses",
        child = "c:crossesAt",
        child = "c:auto",
        child = "c:lblAlgn",
        child = "c:lblOffset",
        child = "c:tickLblSkip",
        child = "c:tickMarkSkip",
        child = "c:noMultiLvlLbl",
        child = "c:extLst",
    )]
    pub children: Vec<CategoryAxisChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CategoryAxisChildChoice {
    #[xml(tag = "c:axId")]
    CAxId(AxisId),
    #[xml(tag = "c:scaling")]
    CScaling(Scaling),
    #[xml(tag = "c:delete")]
    CDelete(Delete),
    #[xml(tag = "c:axPos")]
    CAxPos(AxisPosition),
    #[xml(tag = "c:majorGridlines")]
    CMajorGridlines(MajorGridlines),
    #[xml(tag = "c:minorGridlines")]
    CMinorGridlines(MinorGridlines),
    #[xml(tag = "c:title")]
    CTitle(Title),
    #[xml(tag = "c:numFmt")]
    CNumFmt(NumberingFormat),
    #[xml(tag = "c:majorTickMark")]
    CMajorTickMark(MajorTickMark),
    #[xml(tag = "c:minorTickMark")]
    CMinorTickMark(MinorTickMark),
    #[xml(tag = "c:tickLblPos")]
    CTickLblPos(TickLabelPosition),
    #[xml(tag = "c:spPr")]
    CSpPr(ChartShapeProperties),
    #[xml(tag = "c:txPr")]
    CTxPr(TextProperties),
    #[xml(tag = "c:crossAx")]
    CCrossAx(CrossingAxis),
    #[xml(tag = "c:crosses")]
    CCrosses(Crosses),
    #[xml(tag = "c:crossesAt")]
    CCrossesAt(CrossesAt),
    #[xml(tag = "c:auto")]
    CAuto(AutoLabeled),
    #[xml(tag = "c:lblAlgn")]
    CLblAlgn(LabelAlignment),
    #[xml(tag = "c:lblOffset")]
    CLblOffset(LabelOffset),
    #[xml(tag = "c:tickLblSkip")]
    CTickLblSkip(TickLabelSkip),
    #[xml(tag = "c:tickMarkSkip")]
    CTickMarkSkip(TickMarkSkip),
    #[xml(tag = "c:noMultiLvlLbl")]
    CNoMultiLvlLbl(NoMultiLevelLabels),
    #[xml(tag = "c:extLst")]
    CExtLst(CatAxExtensionList),
}
/// Date Axis.
/// When the object is serialized out as xml, it's qualified name is c:dateAx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:dateAx")]
pub struct DateAxis {
    #[xml(
        child = "c:axId",
        child = "c:scaling",
        child = "c:delete",
        child = "c:axPos",
        child = "c:majorGridlines",
        child = "c:minorGridlines",
        child = "c:title",
        child = "c:numFmt",
        child = "c:majorTickMark",
        child = "c:minorTickMark",
        child = "c:tickLblPos",
        child = "c:spPr",
        child = "c:txPr",
        child = "c:crossAx",
        child = "c:crosses",
        child = "c:crossesAt",
        child = "c:auto",
        child = "c:lblOffset",
        child = "c:baseTimeUnit",
        child = "c:majorUnit",
        child = "c:majorTimeUnit",
        child = "c:minorUnit",
        child = "c:minorTimeUnit",
        child = "c:extLst",
    )]
    pub children: Vec<DateAxisChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DateAxisChildChoice {
    #[xml(tag = "c:axId")]
    CAxId(AxisId),
    #[xml(tag = "c:scaling")]
    CScaling(Scaling),
    #[xml(tag = "c:delete")]
    CDelete(Delete),
    #[xml(tag = "c:axPos")]
    CAxPos(AxisPosition),
    #[xml(tag = "c:majorGridlines")]
    CMajorGridlines(MajorGridlines),
    #[xml(tag = "c:minorGridlines")]
    CMinorGridlines(MinorGridlines),
    #[xml(tag = "c:title")]
    CTitle(Title),
    #[xml(tag = "c:numFmt")]
    CNumFmt(NumberingFormat),
    #[xml(tag = "c:majorTickMark")]
    CMajorTickMark(MajorTickMark),
    #[xml(tag = "c:minorTickMark")]
    CMinorTickMark(MinorTickMark),
    #[xml(tag = "c:tickLblPos")]
    CTickLblPos(TickLabelPosition),
    #[xml(tag = "c:spPr")]
    CSpPr(ChartShapeProperties),
    #[xml(tag = "c:txPr")]
    CTxPr(TextProperties),
    #[xml(tag = "c:crossAx")]
    CCrossAx(CrossingAxis),
    #[xml(tag = "c:crosses")]
    CCrosses(Crosses),
    #[xml(tag = "c:crossesAt")]
    CCrossesAt(CrossesAt),
    #[xml(tag = "c:auto")]
    CAuto(AutoLabeled),
    #[xml(tag = "c:lblOffset")]
    CLblOffset(LabelOffset),
    #[xml(tag = "c:baseTimeUnit")]
    CBaseTimeUnit(BaseTimeUnit),
    #[xml(tag = "c:majorUnit")]
    CMajorUnit(MajorUnit),
    #[xml(tag = "c:majorTimeUnit")]
    CMajorTimeUnit(MajorTimeUnit),
    #[xml(tag = "c:minorUnit")]
    CMinorUnit(MinorUnit),
    #[xml(tag = "c:minorTimeUnit")]
    CMinorTimeUnit(MinorTimeUnit),
    #[xml(tag = "c:extLst")]
    CExtLst(DateAxExtensionList),
}
/// Series Axis.
/// When the object is serialized out as xml, it's qualified name is c:serAx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:serAx")]
pub struct SeriesAxis {
    #[xml(
        child = "c:axId",
        child = "c:scaling",
        child = "c:delete",
        child = "c:axPos",
        child = "c:majorGridlines",
        child = "c:minorGridlines",
        child = "c:title",
        child = "c:numFmt",
        child = "c:majorTickMark",
        child = "c:minorTickMark",
        child = "c:tickLblPos",
        child = "c:spPr",
        child = "c:txPr",
        child = "c:crossAx",
        child = "c:crosses",
        child = "c:crossesAt",
        child = "c:tickLblSkip",
        child = "c:tickMarkSkip",
        child = "c:extLst",
    )]
    pub children: Vec<SeriesAxisChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SeriesAxisChildChoice {
    #[xml(tag = "c:axId")]
    CAxId(AxisId),
    #[xml(tag = "c:scaling")]
    CScaling(Scaling),
    #[xml(tag = "c:delete")]
    CDelete(Delete),
    #[xml(tag = "c:axPos")]
    CAxPos(AxisPosition),
    #[xml(tag = "c:majorGridlines")]
    CMajorGridlines(MajorGridlines),
    #[xml(tag = "c:minorGridlines")]
    CMinorGridlines(MinorGridlines),
    #[xml(tag = "c:title")]
    CTitle(Title),
    #[xml(tag = "c:numFmt")]
    CNumFmt(NumberingFormat),
    #[xml(tag = "c:majorTickMark")]
    CMajorTickMark(MajorTickMark),
    #[xml(tag = "c:minorTickMark")]
    CMinorTickMark(MinorTickMark),
    #[xml(tag = "c:tickLblPos")]
    CTickLblPos(TickLabelPosition),
    #[xml(tag = "c:spPr")]
    CSpPr(ChartShapeProperties),
    #[xml(tag = "c:txPr")]
    CTxPr(TextProperties),
    #[xml(tag = "c:crossAx")]
    CCrossAx(CrossingAxis),
    #[xml(tag = "c:crosses")]
    CCrosses(Crosses),
    #[xml(tag = "c:crossesAt")]
    CCrossesAt(CrossesAt),
    #[xml(tag = "c:tickLblSkip")]
    CTickLblSkip(TickLabelSkip),
    #[xml(tag = "c:tickMarkSkip")]
    CTickMarkSkip(TickMarkSkip),
    #[xml(tag = "c:extLst")]
    CExtLst(SerAxExtensionList),
}
/// Data Table.
/// When the object is serialized out as xml, it's qualified name is c:dTable.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:dTable")]
pub struct DataTable {
    ///Show Horizontal Border
    #[xml(child = "c:showHorzBorder")]
    pub show_horizontal_border: Option<ShowHorizontalBorder>,
    ///Show Vertical Border
    #[xml(child = "c:showVertBorder")]
    pub show_vertical_border: Option<ShowVerticalBorder>,
    ///Show Outline Border
    #[xml(child = "c:showOutline")]
    pub show_outline_border: Option<ShowOutlineBorder>,
    ///Show Legend Keys
    #[xml(child = "c:showKeys")]
    pub show_keys: Option<ShowKeys>,
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
    ///Text Properties
    #[xml(child = "c:txPr")]
    pub text_properties: Option<TextProperties>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// First Slice Angle.
/// When the object is serialized out as xml, it's qualified name is c:firstSliceAng.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:firstSliceAng")]
pub struct FirstSliceAngle {
    /// First Slice Angle Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<i16>,
}
/// Hole Size.
/// When the object is serialized out as xml, it's qualified name is c:holeSize.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:holeSize")]
pub struct HoleSize {
    /// Hole Size Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: u8,
}
/// String Point.
/// When the object is serialized out as xml, it's qualified name is c:pt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:pt")]
pub struct StringPoint {
    /// Index
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
    ///Text Value
    #[xml(child = "c:v")]
    pub numeric_value: NumericValue,
}
/// Thickness.
/// When the object is serialized out as xml, it's qualified name is c:thickness.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:thickness")]
pub struct Thickness {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<u8>,
}
/// Defines the StockChartExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct StockChartExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:filteredLineSeries")]
    pub children: Vec<StockChartExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum StockChartExtensionChildChoice {
    #[xml(tag = "c15:filteredLineSeries")]
    C15FilteredLineSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredLineSeriesExtension,
    ),
}
/// Defines the PieChartExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct PieChartExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:filteredPieSeries")]
    pub children: Vec<PieChartExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PieChartExtensionChildChoice {
    #[xml(tag = "c15:filteredPieSeries")]
    C15FilteredPieSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredPieSeries,
    ),
}
/// Defines the Pie3DChartExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct Pie3DChartExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:filteredPieSeries")]
    pub children: Vec<Pie3DChartExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Pie3DChartExtensionChildChoice {
    #[xml(tag = "c15:filteredPieSeries")]
    C15FilteredPieSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredPieSeries,
    ),
}
/// Defines the NumRefExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct NumRefExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:fullRef", child = "c15:levelRef", child = "c15:formulaRef")]
    pub children: Vec<NumRefExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NumRefExtensionChildChoice {
    #[xml(tag = "c15:fullRef")]
    C15FullRef(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FullReference,
    ),
    #[xml(tag = "c15:levelRef")]
    C15LevelRef(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::LevelReference,
    ),
    #[xml(tag = "c15:formulaRef")]
    C15FormulaRef(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FormulaReference,
    ),
}
/// Defines the StrDataExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct StrDataExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:autoCat")]
    pub children: Vec<StrDataExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum StrDataExtensionChildChoice {
    #[xml(tag = "c15:autoCat")]
    C15AutoCat(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::AutoGeneneratedCategories,
    ),
}
/// Defines the StrRefExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct StrRefExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:fullRef", child = "c15:levelRef", child = "c15:formulaRef")]
    pub children: Vec<StrRefExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum StrRefExtensionChildChoice {
    #[xml(tag = "c15:fullRef")]
    C15FullRef(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FullReference,
    ),
    #[xml(tag = "c15:levelRef")]
    C15LevelRef(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::LevelReference,
    ),
    #[xml(tag = "c15:formulaRef")]
    C15FormulaRef(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FormulaReference,
    ),
}
/// Defines the MultiLvlStrRefExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct MultiLvlStrRefExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:fullRef", child = "c15:levelRef", child = "c15:formulaRef")]
    pub children: Vec<MultiLvlStrRefExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MultiLvlStrRefExtensionChildChoice {
    #[xml(tag = "c15:fullRef")]
    C15FullRef(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FullReference,
    ),
    #[xml(tag = "c15:levelRef")]
    C15LevelRef(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::LevelReference,
    ),
    #[xml(tag = "c15:formulaRef")]
    C15FormulaRef(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FormulaReference,
    ),
}
/// Defines the DLblsExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct DLblsExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "c15:tx",
        child = "c15:dlblFieldTable",
        child = "c15:showDataLabelsRange",
        child = "c15:spPr",
        child = "c15:layout",
        child = "c15:showLeaderLines",
        child = "c15:leaderLines",
    )]
    pub children: Vec<DLblsExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DLblsExtensionChildChoice {
    #[xml(tag = "c15:tx")]
    C15Tx(crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ChartText),
    #[xml(tag = "c15:dlblFieldTable")]
    C15DlblFieldTable(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelFieldTable,
    ),
    #[xml(tag = "c15:showDataLabelsRange")]
    C15ShowDataLabelsRange(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShowDataLabelsRange,
    ),
    #[xml(tag = "c15:spPr")]
    C15SpPr(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShapeProperties,
    ),
    #[xml(tag = "c15:layout")]
    C15Layout(crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::Layout),
    #[xml(tag = "c15:showLeaderLines")]
    C15ShowLeaderLines(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShowLeaderLines,
    ),
    #[xml(tag = "c15:leaderLines")]
    C15LeaderLines(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::LeaderLines,
    ),
}
/// Defines the LineChartExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct LineChartExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:filteredLineSeries")]
    pub children: Vec<LineChartExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LineChartExtensionChildChoice {
    #[xml(tag = "c15:filteredLineSeries")]
    C15FilteredLineSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredLineSeriesExtension,
    ),
}
/// Defines the Line3DChartExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct Line3DChartExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:filteredLineSeries")]
    pub children: Vec<Line3DChartExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Line3DChartExtensionChildChoice {
    #[xml(tag = "c15:filteredLineSeries")]
    C15FilteredLineSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredLineSeriesExtension,
    ),
}
/// Defines the ScatterChartExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct ScatterChartExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:filteredScatterSeries")]
    pub children: Vec<ScatterChartExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ScatterChartExtensionChildChoice {
    #[xml(tag = "c15:filteredScatterSeries")]
    C15FilteredScatterSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredScatterSeries,
    ),
}
/// Defines the RadarChartExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct RadarChartExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:filteredRadarSeries")]
    pub children: Vec<RadarChartExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RadarChartExtensionChildChoice {
    #[xml(tag = "c15:filteredRadarSeries")]
    C15FilteredRadarSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredRadarSeries,
    ),
}
/// Defines the BarChartExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct BarChartExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:filteredBarSeries")]
    pub children: Vec<BarChartExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BarChartExtensionChildChoice {
    #[xml(tag = "c15:filteredBarSeries")]
    C15FilteredBarSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredBarSeries,
    ),
}
/// Defines the Bar3DChartExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct Bar3DChartExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:filteredBarSeries")]
    pub children: Vec<Bar3DChartExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Bar3DChartExtensionChildChoice {
    #[xml(tag = "c15:filteredBarSeries")]
    C15FilteredBarSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredBarSeries,
    ),
}
/// Defines the AreaChartExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct AreaChartExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:filteredAreaSeries")]
    pub children: Vec<AreaChartExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum AreaChartExtensionChildChoice {
    #[xml(tag = "c15:filteredAreaSeries")]
    C15FilteredAreaSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredAreaSeries,
    ),
}
/// Defines the Area3DChartExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct Area3DChartExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:filteredAreaSeries")]
    pub children: Vec<Area3DChartExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Area3DChartExtensionChildChoice {
    #[xml(tag = "c15:filteredAreaSeries")]
    C15FilteredAreaSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredAreaSeries,
    ),
}
/// Defines the BubbleChartExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct BubbleChartExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:filteredBubbleSeries")]
    pub children: Vec<BubbleChartExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BubbleChartExtensionChildChoice {
    #[xml(tag = "c15:filteredBubbleSeries")]
    C15FilteredBubbleSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredBubbleSeries,
    ),
}
/// Defines the SurfaceChartExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct SurfaceChartExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:filteredSurfaceSeries")]
    pub children: Vec<SurfaceChartExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SurfaceChartExtensionChildChoice {
    #[xml(tag = "c15:filteredSurfaceSeries")]
    C15FilteredSurfaceSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSurfaceSeries,
    ),
}
/// Defines the Surface3DChartExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct Surface3DChartExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:filteredSurfaceSeries")]
    pub children: Vec<Surface3DChartExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Surface3DChartExtensionChildChoice {
    #[xml(tag = "c15:filteredSurfaceSeries")]
    C15FilteredSurfaceSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSurfaceSeries,
    ),
}
/// Defines the CatAxExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct CatAxExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:numFmt")]
    pub children: Vec<CatAxExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CatAxExtensionChildChoice {
    #[xml(tag = "c15:numFmt")]
    C15NumFmt(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::NumberingFormat,
    ),
}
/// Defines the DateAxExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct DateAxExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:numFmt")]
    pub children: Vec<DateAxExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DateAxExtensionChildChoice {
    #[xml(tag = "c15:numFmt")]
    C15NumFmt(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::NumberingFormat,
    ),
}
/// Defines the SerAxExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct SerAxExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:numFmt")]
    pub children: Vec<SerAxExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SerAxExtensionChildChoice {
    #[xml(tag = "c15:numFmt")]
    C15NumFmt(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::NumberingFormat,
    ),
}
/// Defines the ValAxExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct ValAxExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "c15:numFmt")]
    pub children: Vec<ValAxExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ValAxExtensionChildChoice {
    #[xml(tag = "c15:numFmt")]
    C15NumFmt(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::NumberingFormat,
    ),
}
/// Defines the UpDownBars Class.
/// When the object is serialized out as xml, it's qualified name is c:upDownBars.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:upDownBars")]
pub struct UpDownBars {
    ///Gap Width
    #[xml(child = "c:gapWidth")]
    pub gap_width: Option<GapWidth>,
    ///Up Bars
    #[xml(child = "c:upBars")]
    pub up_bars: Option<UpBars>,
    ///Down Bars
    #[xml(child = "c:downBars")]
    pub down_bars: Option<DownBars>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the StockChartExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct StockChartExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<StockChartExtension>,
}
/// Defines the PieChartExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct PieChartExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<PieChartExtension>,
}
/// Defines the Pie3DChartExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct Pie3DChartExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<Pie3DChartExtension>,
}
/// Defines the NumRefExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct NumRefExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<NumRefExtension>,
}
/// Defines the StrDataExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct StrDataExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<StrDataExtension>,
}
/// Defines the StrRefExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct StrRefExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<StrRefExtension>,
}
/// Defines the MultiLevelStringCache Class.
/// When the object is serialized out as xml, it's qualified name is c:multiLvlStrCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:multiLvlStrCache")]
pub struct MultiLevelStringCache {
    /// _
    #[xml(child = "c:ptCount")]
    pub point_count: Option<PointCount>,
    /// _
    #[xml(child = "c:lvl")]
    pub c_lvl: Vec<Level>,
    /// _
    #[xml(child = "c:extLst")]
    pub c_ext_lst: Option<ExtensionList>,
}
/// Defines the MultiLvlStrRefExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct MultiLvlStrRefExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<MultiLvlStrRefExtension>,
}
/// Defines the DLblsExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct DLblsExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<DLblsExtension>,
}
/// Defines the LineChartExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct LineChartExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<LineChartExtension>,
}
/// Defines the Line3DChartExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct Line3DChartExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<Line3DChartExtension>,
}
/// Defines the ScatterStyle Class.
/// When the object is serialized out as xml, it's qualified name is c:scatterStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:scatterStyle")]
pub struct ScatterStyle {
    /// Scatter Style Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<ScatterStyleValues>,
}
/// Defines the ScatterChartSeries Class.
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ser")]
pub struct ScatterChartSeries {
    #[xml(
        child = "c:idx",
        child = "c:order",
        child = "c:tx",
        child = "c:spPr",
        child = "c:marker",
        child = "c:dPt",
        child = "c:dLbls",
        child = "c:trendline",
        child = "c:errBars",
        child = "c:xVal",
        child = "c:yVal",
        child = "c:smooth",
        child = "c:extLst",
    )]
    pub children: Vec<ScatterChartSeriesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ScatterChartSeriesChildChoice {
    #[xml(tag = "c:idx")]
    CIdx(Index),
    #[xml(tag = "c:order")]
    COrder(Order),
    #[xml(tag = "c:tx")]
    CTx(SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(ChartShapeProperties),
    #[xml(tag = "c:marker")]
    CMarker(Marker),
    #[xml(tag = "c:dPt")]
    CDPt(DataPoint),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:trendline")]
    CTrendline(Trendline),
    #[xml(tag = "c:errBars")]
    CErrBars(ErrorBars),
    #[xml(tag = "c:xVal")]
    CXVal(XValues),
    #[xml(tag = "c:yVal")]
    CYVal(YValues),
    #[xml(tag = "c:smooth")]
    CSmooth(Smooth),
    #[xml(tag = "c:extLst")]
    CExtLst(ScatterSerExtensionList),
}
/// Defines the ScatterChartExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct ScatterChartExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<ScatterChartExtension>,
}
/// Defines the RadarStyle Class.
/// When the object is serialized out as xml, it's qualified name is c:radarStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:radarStyle")]
pub struct RadarStyle {
    /// Radar Style Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: RadarStyleValues,
}
/// Defines the RadarChartSeries Class.
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ser")]
pub struct RadarChartSeries {
    #[xml(
        child = "c:idx",
        child = "c:order",
        child = "c:tx",
        child = "c:spPr",
        child = "c:pictureOptions",
        child = "c:marker",
        child = "c:dPt",
        child = "c:dLbls",
        child = "c:cat",
        child = "c:val",
        child = "c:extLst",
    )]
    pub children: Vec<RadarChartSeriesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RadarChartSeriesChildChoice {
    #[xml(tag = "c:idx")]
    CIdx(Index),
    #[xml(tag = "c:order")]
    COrder(Order),
    #[xml(tag = "c:tx")]
    CTx(SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(ChartShapeProperties),
    #[xml(tag = "c:pictureOptions")]
    CPictureOptions(PictureOptions),
    #[xml(tag = "c:marker")]
    CMarker(Marker),
    #[xml(tag = "c:dPt")]
    CDPt(DataPoint),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:cat")]
    CCat(CategoryAxisData),
    #[xml(tag = "c:val")]
    CVal(Values),
    #[xml(tag = "c:extLst")]
    CExtLst(RadarSerExtensionList),
}
/// Defines the RadarChartExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct RadarChartExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<RadarChartExtension>,
}
/// Defines the Overlap Class.
/// When the object is serialized out as xml, it's qualified name is c:overlap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:overlap")]
pub struct Overlap {
    /// Overlap Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<u8>,
}
/// Defines the BarChartExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct BarChartExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<BarChartExtension>,
}
/// Defines the Shape Class.
/// When the object is serialized out as xml, it's qualified name is c:shape.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:shape")]
pub struct Shape {
    /// Shape Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<ShapeValues>,
}
/// Defines the Bar3DChartExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct Bar3DChartExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<Bar3DChartExtension>,
}
/// Defines the AreaChartExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct AreaChartExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<AreaChartExtension>,
}
/// Defines the Area3DChartExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct Area3DChartExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<Area3DChartExtension>,
}
/// Defines the BubbleChartSeries Class.
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ser")]
pub struct BubbleChartSeries {
    #[xml(
        child = "c:idx",
        child = "c:order",
        child = "c:tx",
        child = "c:spPr",
        child = "c:pictureOptions",
        child = "c:invertIfNegative",
        child = "c:dPt",
        child = "c:dLbls",
        child = "c:trendline",
        child = "c:errBars",
        child = "c:xVal",
        child = "c:yVal",
        child = "c:bubbleSize",
        child = "c:bubble3D",
        child = "c:extLst",
    )]
    pub children: Vec<BubbleChartSeriesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BubbleChartSeriesChildChoice {
    #[xml(tag = "c:idx")]
    CIdx(Index),
    #[xml(tag = "c:order")]
    COrder(Order),
    #[xml(tag = "c:tx")]
    CTx(SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(ChartShapeProperties),
    #[xml(tag = "c:pictureOptions")]
    CPictureOptions(PictureOptions),
    #[xml(tag = "c:invertIfNegative")]
    CInvertIfNegative(InvertIfNegative),
    #[xml(tag = "c:dPt")]
    CDPt(DataPoint),
    #[xml(tag = "c:dLbls")]
    CDLbls(DataLabels),
    #[xml(tag = "c:trendline")]
    CTrendline(Trendline),
    #[xml(tag = "c:errBars")]
    CErrBars(ErrorBars),
    #[xml(tag = "c:xVal")]
    CXVal(XValues),
    #[xml(tag = "c:yVal")]
    CYVal(YValues),
    #[xml(tag = "c:bubbleSize")]
    CBubbleSize(BubbleSize),
    #[xml(tag = "c:bubble3D")]
    CBubble3D(Bubble3D),
    #[xml(tag = "c:extLst")]
    CExtLst(BubbleSerExtensionList),
}
/// Defines the BubbleScale Class.
/// When the object is serialized out as xml, it's qualified name is c:bubbleScale.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:bubbleScale")]
pub struct BubbleScale {
    /// Bubble Scale Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<i32>,
}
/// Defines the SizeRepresents Class.
/// When the object is serialized out as xml, it's qualified name is c:sizeRepresents.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:sizeRepresents")]
pub struct SizeRepresents {
    /// Size Represents Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<SizeRepresentsValues>,
}
/// Defines the BubbleChartExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct BubbleChartExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<BubbleChartExtension>,
}
/// Defines the SurfaceChartExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct SurfaceChartExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<SurfaceChartExtension>,
}
/// Defines the Surface3DChartExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct Surface3DChartExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<Surface3DChartExtension>,
}
/// Defines the LabelAlignment Class.
/// When the object is serialized out as xml, it's qualified name is c:lblAlgn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:lblAlgn")]
pub struct LabelAlignment {
    /// Label Alignment Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: LabelAlignmentValues,
}
/// Defines the LabelOffset Class.
/// When the object is serialized out as xml, it's qualified name is c:lblOffset.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:lblOffset")]
pub struct LabelOffset {
    /// Label Offset Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<i16>,
}
/// Defines the TickLabelSkip Class.
/// When the object is serialized out as xml, it's qualified name is c:tickLblSkip.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:tickLblSkip")]
pub struct TickLabelSkip {
    /// Tick Skip Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the TickMarkSkip Class.
/// When the object is serialized out as xml, it's qualified name is c:tickMarkSkip.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:tickMarkSkip")]
pub struct TickMarkSkip {
    /// Tick Skip Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the SkipType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct SkipType {
    /// Tick Skip Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the CatAxExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct CatAxExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<CatAxExtension>,
}
/// Defines the BaseTimeUnit Class.
/// When the object is serialized out as xml, it's qualified name is c:baseTimeUnit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:baseTimeUnit")]
pub struct BaseTimeUnit {
    /// Time Unit Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<TimeUnitValues>,
}
/// Defines the MajorTimeUnit Class.
/// When the object is serialized out as xml, it's qualified name is c:majorTimeUnit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:majorTimeUnit")]
pub struct MajorTimeUnit {
    /// Time Unit Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<TimeUnitValues>,
}
/// Defines the MinorTimeUnit Class.
/// When the object is serialized out as xml, it's qualified name is c:minorTimeUnit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:minorTimeUnit")]
pub struct MinorTimeUnit {
    /// Time Unit Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<TimeUnitValues>,
}
/// Defines the TimeUnitType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TimeUnitType {
    /// Time Unit Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<TimeUnitValues>,
}
/// Defines the MajorUnit Class.
/// When the object is serialized out as xml, it's qualified name is c:majorUnit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:majorUnit")]
pub struct MajorUnit {
    /// Major Unit Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Defines the MinorUnit Class.
/// When the object is serialized out as xml, it's qualified name is c:minorUnit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:minorUnit")]
pub struct MinorUnit {
    /// Major Unit Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Defines the AxisUnitType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct AxisUnitType {
    /// Major Unit Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Defines the DateAxExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct DateAxExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<DateAxExtension>,
}
/// Defines the SerAxExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct SerAxExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<SerAxExtension>,
}
/// Defines the CrossBetween Class.
/// When the object is serialized out as xml, it's qualified name is c:crossBetween.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:crossBetween")]
pub struct CrossBetween {
    /// Cross Between Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: CrossBetweenValues,
}
/// Defines the DisplayUnits Class.
/// When the object is serialized out as xml, it's qualified name is c:dispUnits.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:dispUnits")]
pub struct DisplayUnits {
    #[xml(
        child = "c:custUnit",
        child = "c:builtInUnit",
        child = "c:dispUnitsLbl",
        child = "c:extLst",
    )]
    pub children: Vec<DisplayUnitsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DisplayUnitsChildChoice {
    #[xml(tag = "c:custUnit")]
    CCustUnit(CustomDisplayUnit),
    #[xml(tag = "c:builtInUnit")]
    CBuiltInUnit(BuiltInUnit),
    #[xml(tag = "c:dispUnitsLbl")]
    CDispUnitsLbl(DisplayUnitsLabel),
    #[xml(tag = "c:extLst")]
    CExtLst(ExtensionList),
}
/// Defines the ValAxExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct ValAxExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<ValAxExtension>,
}
/// Defines the EditingLanguage Class.
/// When the object is serialized out as xml, it's qualified name is c:lang.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:lang")]
pub struct EditingLanguage {
    /// Language Code
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Defines the Style Class.
/// When the object is serialized out as xml, it's qualified name is c:style.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:style")]
pub struct Style {
    /// Style Type
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<u8>,
}
/// Defines the ColorMapOverride Class.
/// When the object is serialized out as xml, it's qualified name is c:clrMapOvr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:clrMapOvr")]
pub struct ColorMapOverride {
    /// Background 1
    /// Represents the following attribute in the schema: :bg1
    #[xml(attr = "bg1")]
    pub background1: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Text 1
    /// Represents the following attribute in the schema: :tx1
    #[xml(attr = "tx1")]
    pub text1: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Background 2
    /// Represents the following attribute in the schema: :bg2
    #[xml(attr = "bg2")]
    pub background2: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Text 2
    /// Represents the following attribute in the schema: :tx2
    #[xml(attr = "tx2")]
    pub text2: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Accent 1
    /// Represents the following attribute in the schema: :accent1
    #[xml(attr = "accent1")]
    pub accent1: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Accent 2
    /// Represents the following attribute in the schema: :accent2
    #[xml(attr = "accent2")]
    pub accent2: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Accent 3
    /// Represents the following attribute in the schema: :accent3
    #[xml(attr = "accent3")]
    pub accent3: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Accent 4
    /// Represents the following attribute in the schema: :accent4
    #[xml(attr = "accent4")]
    pub accent4: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Accent 5
    /// Represents the following attribute in the schema: :accent5
    #[xml(attr = "accent5")]
    pub accent5: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Accent 6
    /// Represents the following attribute in the schema: :accent6
    #[xml(attr = "accent6")]
    pub accent6: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Hyperlink
    /// Represents the following attribute in the schema: :hlink
    #[xml(attr = "hlink")]
    pub hyperlink: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Followed Hyperlink
    /// Represents the following attribute in the schema: :folHlink
    #[xml(attr = "folHlink")]
    pub followed_hyperlink: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Defines the PivotSource Class.
/// When the object is serialized out as xml, it's qualified name is c:pivotSource.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:pivotSource")]
pub struct PivotSource {
    ///Pivot Name
    #[xml(child = "c:name")]
    pub pivot_table_name: PivotTableName,
    ///Format ID
    #[xml(child = "c:fmtId")]
    pub format_id: FormatId,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the Protection Class.
/// When the object is serialized out as xml, it's qualified name is c:protection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:protection")]
pub struct Protection {
    ///Chart Object
    #[xml(child = "c:chartObject")]
    pub chart_object: Option<ChartObject>,
    ///Data Cannot Be Changed
    #[xml(child = "c:data")]
    pub data: Option<Data>,
    ///Formatting
    #[xml(child = "c:formatting")]
    pub formatting: Option<Formatting>,
    ///Selection
    #[xml(child = "c:selection")]
    pub selection: Option<Selection>,
    ///User Interface
    #[xml(child = "c:userInterface")]
    pub user_interface: Option<UserInterface>,
}
/// Defines the Chart Class.
/// When the object is serialized out as xml, it's qualified name is c:chart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:chart")]
pub struct Chart {
    ///Title data and formatting
    #[xml(child = "c:title")]
    pub title: Option<Title>,
    ///True if the chart automatic title has been deleted.
    #[xml(child = "c:autoTitleDeleted")]
    pub auto_title_deleted: Option<AutoTitleDeleted>,
    ///pivot chart format persistence data
    #[xml(child = "c:pivotFmts")]
    pub pivot_formats: Option<PivotFormats>,
    ///3D view settings
    #[xml(child = "c:view3D")]
    pub view3_d: Option<View3D>,
    ///3D floor formatting
    #[xml(child = "c:floor")]
    pub floor: Option<Floor>,
    ///3D side wall formatting
    #[xml(child = "c:sideWall")]
    pub side_wall: Option<SideWall>,
    ///3D back wall formatting
    #[xml(child = "c:backWall")]
    pub back_wall: Option<BackWall>,
    ///Plot data and formatting
    #[xml(child = "c:plotArea")]
    pub plot_area: PlotArea,
    ///Legend data and formatting
    #[xml(child = "c:legend")]
    pub legend: Option<Legend>,
    ///True if only visible cells are plotted.
    #[xml(child = "c:plotVisOnly")]
    pub plot_visible_only: Option<PlotVisibleOnly>,
    ///The way that blank cells are plotted on a chart.
    #[xml(child = "c:dispBlanksAs")]
    pub display_blanks_as: Option<DisplayBlanksAs>,
    ///True if we should render datalabels over the maximum scale
    #[xml(child = "c:showDLblsOverMax")]
    pub show_data_labels_over_maximum: Option<ShowDataLabelsOverMaximum>,
    ///Extensibility container
    #[xml(child = "c:extLst")]
    pub chart_extension_list: Option<ChartExtensionList>,
}
/// Defines the ExternalData Class.
/// When the object is serialized out as xml, it's qualified name is c:externalData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:externalData")]
pub struct ExternalData {
    /// Relationship Reference
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
    ///Update Automatically
    #[xml(child = "c:autoUpdate")]
    pub auto_update: Option<AutoUpdate>,
}
/// Defines the PrintSettings Class.
/// When the object is serialized out as xml, it's qualified name is c:printSettings.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:printSettings")]
pub struct PrintSettings {
    ///Header and Footer
    #[xml(child = "c:headerFooter")]
    pub header_footer: Option<HeaderFooter>,
    ///Page Margins
    #[xml(child = "c:pageMargins")]
    pub page_margins: Option<PageMargins>,
    ///Page Setup
    #[xml(child = "c:pageSetup")]
    pub page_setup: Option<PageSetup>,
    ///Legacy Drawing for Headers and Footers
    #[xml(child = "c:legacyDrawingHF")]
    pub legacy_drawing_header_footer: Option<LegacyDrawingHeaderFooter>,
}
/// Defines the ChartSpaceExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct ChartSpaceExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<ChartSpaceExtension>,
}
/// Defines the ChartSpaceExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct ChartSpaceExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "c14:pivotOptions",
        child = "c14:sketchOptions",
        child = "c15:pivotSource",
    )]
    pub children: Vec<ChartSpaceExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ChartSpaceExtensionChildChoice {
    #[xml(tag = "c14:pivotOptions")]
    C14PivotOptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::PivotOptions,
    ),
    #[xml(tag = "c14:sketchOptions")]
    C14SketchOptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::SketchOptions,
    ),
    #[xml(tag = "c15:pivotSource")]
    C15PivotSource(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::PivotSource,
    ),
}
/// Defines the DLblExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct DLblExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<DLblExtension>,
}
/// Defines the DLblExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct DLblExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "c15:dlblFieldTable",
        child = "c15:xForSave",
        child = "c15:showDataLabelsRange",
        child = "c15:spPr",
        child = "c15:layout",
        child = "c16:uniqueId",
    )]
    pub children: Vec<DLblExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DLblExtensionChildChoice {
    #[xml(tag = "c15:dlblFieldTable")]
    C15DlblFieldTable(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelFieldTable,
    ),
    #[xml(tag = "c15:xForSave")]
    C15XForSave(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ExceptionForSave,
    ),
    #[xml(tag = "c15:showDataLabelsRange")]
    C15ShowDataLabelsRange(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShowDataLabelsRange,
    ),
    #[xml(tag = "c15:spPr")]
    C15SpPr(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShapeProperties,
    ),
    #[xml(tag = "c15:layout")]
    C15Layout(crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::Layout),
    #[xml(tag = "c16:uniqueId")]
    C16UniqueId(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    ),
}
/// Defines the DataPoint Class.
/// When the object is serialized out as xml, it's qualified name is c:dPt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:dPt")]
pub struct DataPoint {
    ///Index
    #[xml(child = "c:idx")]
    pub index: Index,
    ///Invert if Negative
    #[xml(child = "c:invertIfNegative")]
    pub invert_if_negative: Option<InvertIfNegative>,
    ///Marker
    #[xml(child = "c:marker")]
    pub marker: Option<Marker>,
    ///3D Bubble
    #[xml(child = "c:bubble3D")]
    pub bubble3_d: Option<Bubble3D>,
    ///Explosion
    #[xml(child = "c:explosion")]
    pub explosion: Option<Explosion>,
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
    /// _
    #[xml(child = "c:pictureOptions")]
    pub picture_options: Option<PictureOptions>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the Trendline Class.
/// When the object is serialized out as xml, it's qualified name is c:trendline.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:trendline")]
pub struct Trendline {
    ///Trendline Name
    #[xml(child = "c:name")]
    pub trendline_name: Option<TrendlineName>,
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
    ///Trendline Type
    #[xml(child = "c:trendlineType")]
    pub trendline_type: TrendlineType,
    ///Polynomial Trendline Order
    #[xml(child = "c:order")]
    pub polynomial_order: Option<PolynomialOrder>,
    ///Period
    #[xml(child = "c:period")]
    pub period: Option<Period>,
    ///Forward
    #[xml(child = "c:forward")]
    pub forward: Option<Forward>,
    ///Backward
    #[xml(child = "c:backward")]
    pub backward: Option<Backward>,
    ///Intercept
    #[xml(child = "c:intercept")]
    pub intercept: Option<Intercept>,
    ///Display R Squared Value
    #[xml(child = "c:dispRSqr")]
    pub display_r_squared_value: Option<DisplayRSquaredValue>,
    ///Display Equation
    #[xml(child = "c:dispEq")]
    pub display_equation: Option<DisplayEquation>,
    ///Trendline Label
    #[xml(child = "c:trendlineLbl")]
    pub trendline_label: Option<TrendlineLabel>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the ErrorBars Class.
/// When the object is serialized out as xml, it's qualified name is c:errBars.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:errBars")]
pub struct ErrorBars {
    ///Error Bar Direction
    #[xml(child = "c:errDir")]
    pub error_direction: Option<ErrorDirection>,
    ///Error Bar Type
    #[xml(child = "c:errBarType")]
    pub error_bar_type: ErrorBarType,
    ///Error Bar Value Type
    #[xml(child = "c:errValType")]
    pub error_bar_value_type: ErrorBarValueType,
    ///No End Cap
    #[xml(child = "c:noEndCap")]
    pub no_end_cap: Option<NoEndCap>,
    ///Plus
    #[xml(child = "c:plus")]
    pub plus: Option<Plus>,
    ///Minus
    #[xml(child = "c:minus")]
    pub minus: Option<Minus>,
    ///Error Bar Value
    #[xml(child = "c:val")]
    pub error_bar_value: Option<ErrorBarValue>,
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<ChartShapeProperties>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the CategoryAxisData Class.
/// When the object is serialized out as xml, it's qualified name is c:cat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:cat")]
pub struct CategoryAxisData {
    #[xml(
        child = "c:multiLvlStrRef",
        child = "c:numRef",
        child = "c:numLit",
        child = "c:strRef",
        child = "c:strLit",
    )]
    pub children: Vec<CategoryAxisDataChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CategoryAxisDataChildChoice {
    #[xml(tag = "c:multiLvlStrRef")]
    CMultiLvlStrRef(MultiLevelStringReference),
    #[xml(tag = "c:numRef")]
    CNumRef(NumberReference),
    #[xml(tag = "c:numLit")]
    CNumLit(NumberLiteral),
    #[xml(tag = "c:strRef")]
    CStrRef(StringReference),
    #[xml(tag = "c:strLit")]
    CStrLit(StringLiteral),
}
/// Defines the XValues Class.
/// When the object is serialized out as xml, it's qualified name is c:xVal.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:xVal")]
pub struct XValues {
    #[xml(
        child = "c:multiLvlStrRef",
        child = "c:numRef",
        child = "c:numLit",
        child = "c:strRef",
        child = "c:strLit",
    )]
    pub children: Vec<XValuesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum XValuesChildChoice {
    #[xml(tag = "c:multiLvlStrRef")]
    CMultiLvlStrRef(MultiLevelStringReference),
    #[xml(tag = "c:numRef")]
    CNumRef(NumberReference),
    #[xml(tag = "c:numLit")]
    CNumLit(NumberLiteral),
    #[xml(tag = "c:strRef")]
    CStrRef(StringReference),
    #[xml(tag = "c:strLit")]
    CStrLit(StringLiteral),
}
/// Defines the AxisDataSourceType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct AxisDataSourceType {
    #[xml(
        child = "c:multiLvlStrRef",
        child = "c:numRef",
        child = "c:numLit",
        child = "c:strRef",
        child = "c:strLit",
    )]
    pub children: Vec<AxisDataSourceTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum AxisDataSourceTypeChildChoice {
    #[xml(tag = "c:multiLvlStrRef")]
    CMultiLvlStrRef(MultiLevelStringReference),
    #[xml(tag = "c:numRef")]
    CNumRef(NumberReference),
    #[xml(tag = "c:numLit")]
    CNumLit(NumberLiteral),
    #[xml(tag = "c:strRef")]
    CStrRef(StringReference),
    #[xml(tag = "c:strLit")]
    CStrLit(StringLiteral),
}
/// Defines the LineSerExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct LineSerExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<LineSerExtension>,
}
/// Defines the LineSerExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct LineSerExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "c15:filteredSeriesTitle",
        child = "c15:filteredCategoryTitle",
        child = "c15:datalabelsRange",
        child = "c15:categoryFilterExceptions",
        child = "c16:categoryFilterExceptions",
        child = "c16:datapointuniqueidmap",
        child = "c16:uniqueId",
    )]
    pub children: Vec<LineSerExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LineSerExtensionChildChoice {
    #[xml(tag = "c15:filteredSeriesTitle")]
    C15FilteredSeriesTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    ),
    #[xml(tag = "c15:filteredCategoryTitle")]
    C15FilteredCategoryTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    ),
    #[xml(tag = "c15:datalabelsRange")]
    C15DatalabelsRange(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    ),
    #[xml(tag = "c15:categoryFilterExceptions")]
    C15CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:categoryFilterExceptions")]
    C16CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:datapointuniqueidmap")]
    C16Datapointuniqueidmap(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    ),
    #[xml(tag = "c16:uniqueId")]
    C16UniqueId(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    ),
}
/// Defines the ScatterSerExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct ScatterSerExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<ScatterSerExtension>,
}
/// Defines the ScatterSerExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct ScatterSerExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "c15:filteredSeriesTitle",
        child = "c15:filteredCategoryTitle",
        child = "c15:datalabelsRange",
        child = "c15:categoryFilterExceptions",
        child = "c16:categoryFilterExceptions",
        child = "c16:datapointuniqueidmap",
        child = "c16:uniqueId",
    )]
    pub children: Vec<ScatterSerExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ScatterSerExtensionChildChoice {
    #[xml(tag = "c15:filteredSeriesTitle")]
    C15FilteredSeriesTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    ),
    #[xml(tag = "c15:filteredCategoryTitle")]
    C15FilteredCategoryTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    ),
    #[xml(tag = "c15:datalabelsRange")]
    C15DatalabelsRange(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    ),
    #[xml(tag = "c15:categoryFilterExceptions")]
    C15CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:categoryFilterExceptions")]
    C16CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:datapointuniqueidmap")]
    C16Datapointuniqueidmap(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    ),
    #[xml(tag = "c16:uniqueId")]
    C16UniqueId(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    ),
}
/// Defines the RadarSerExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct RadarSerExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<RadarSerExtension>,
}
/// Defines the RadarSerExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct RadarSerExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "c15:filteredSeriesTitle",
        child = "c15:filteredCategoryTitle",
        child = "c15:datalabelsRange",
        child = "c15:categoryFilterExceptions",
        child = "c16:categoryFilterExceptions",
        child = "c16:datapointuniqueidmap",
        child = "c16:uniqueId",
    )]
    pub children: Vec<RadarSerExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RadarSerExtensionChildChoice {
    #[xml(tag = "c15:filteredSeriesTitle")]
    C15FilteredSeriesTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    ),
    #[xml(tag = "c15:filteredCategoryTitle")]
    C15FilteredCategoryTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    ),
    #[xml(tag = "c15:datalabelsRange")]
    C15DatalabelsRange(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    ),
    #[xml(tag = "c15:categoryFilterExceptions")]
    C15CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:categoryFilterExceptions")]
    C16CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:datapointuniqueidmap")]
    C16Datapointuniqueidmap(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    ),
    #[xml(tag = "c16:uniqueId")]
    C16UniqueId(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    ),
}
/// Defines the BarSerExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct BarSerExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<BarSerExtension>,
}
/// Defines the BarSerExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct BarSerExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "c14:invertSolidFillFmt",
        child = "c15:filteredSeriesTitle",
        child = "c15:filteredCategoryTitle",
        child = "c15:datalabelsRange",
        child = "c15:categoryFilterExceptions",
        child = "c16:categoryFilterExceptions",
        child = "c16:datapointuniqueidmap",
        child = "c16:uniqueId",
    )]
    pub children: Vec<BarSerExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BarSerExtensionChildChoice {
    #[xml(tag = "c14:invertSolidFillFmt")]
    C14InvertSolidFillFmt(
        crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::InvertSolidFillFormat,
    ),
    #[xml(tag = "c15:filteredSeriesTitle")]
    C15FilteredSeriesTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    ),
    #[xml(tag = "c15:filteredCategoryTitle")]
    C15FilteredCategoryTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    ),
    #[xml(tag = "c15:datalabelsRange")]
    C15DatalabelsRange(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    ),
    #[xml(tag = "c15:categoryFilterExceptions")]
    C15CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:categoryFilterExceptions")]
    C16CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:datapointuniqueidmap")]
    C16Datapointuniqueidmap(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    ),
    #[xml(tag = "c16:uniqueId")]
    C16UniqueId(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    ),
}
/// Defines the AreaSerExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct AreaSerExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<AreaSerExtension>,
}
/// Defines the AreaSerExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct AreaSerExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "c15:filteredSeriesTitle",
        child = "c15:filteredCategoryTitle",
        child = "c15:datalabelsRange",
        child = "c15:categoryFilterExceptions",
        child = "c16:categoryFilterExceptions",
        child = "c16:datapointuniqueidmap",
        child = "c16:uniqueId",
    )]
    pub children: Vec<AreaSerExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum AreaSerExtensionChildChoice {
    #[xml(tag = "c15:filteredSeriesTitle")]
    C15FilteredSeriesTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    ),
    #[xml(tag = "c15:filteredCategoryTitle")]
    C15FilteredCategoryTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    ),
    #[xml(tag = "c15:datalabelsRange")]
    C15DatalabelsRange(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    ),
    #[xml(tag = "c15:categoryFilterExceptions")]
    C15CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:categoryFilterExceptions")]
    C16CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:datapointuniqueidmap")]
    C16Datapointuniqueidmap(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    ),
    #[xml(tag = "c16:uniqueId")]
    C16UniqueId(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    ),
}
/// Defines the PieSerExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct PieSerExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<PieSerExtension>,
}
/// Defines the PieSerExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct PieSerExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "c15:filteredSeriesTitle",
        child = "c15:filteredCategoryTitle",
        child = "c15:datalabelsRange",
        child = "c15:categoryFilterExceptions",
        child = "c16:categoryFilterExceptions",
        child = "c16:datapointuniqueidmap",
        child = "c16:uniqueId",
    )]
    pub children: Vec<PieSerExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PieSerExtensionChildChoice {
    #[xml(tag = "c15:filteredSeriesTitle")]
    C15FilteredSeriesTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    ),
    #[xml(tag = "c15:filteredCategoryTitle")]
    C15FilteredCategoryTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    ),
    #[xml(tag = "c15:datalabelsRange")]
    C15DatalabelsRange(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    ),
    #[xml(tag = "c15:categoryFilterExceptions")]
    C15CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:categoryFilterExceptions")]
    C16CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:datapointuniqueidmap")]
    C16Datapointuniqueidmap(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    ),
    #[xml(tag = "c16:uniqueId")]
    C16UniqueId(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    ),
}
/// Defines the BubbleSerExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct BubbleSerExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<BubbleSerExtension>,
}
/// Defines the BubbleSerExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct BubbleSerExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "c14:invertSolidFillFmt",
        child = "c15:filteredCategoryTitle",
        child = "c15:datalabelsRange",
        child = "c15:categoryFilterExceptions",
        child = "c16:categoryFilterExceptions",
        child = "c16:datapointuniqueidmap",
        child = "c16:uniqueId",
    )]
    pub children: Vec<BubbleSerExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BubbleSerExtensionChildChoice {
    #[xml(tag = "c14:invertSolidFillFmt")]
    C14InvertSolidFillFmt(
        crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::InvertSolidFillFormat,
    ),
    #[xml(tag = "c15:filteredCategoryTitle")]
    C15FilteredCategoryTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    ),
    #[xml(tag = "c15:datalabelsRange")]
    C15DatalabelsRange(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    ),
    #[xml(tag = "c15:categoryFilterExceptions")]
    C15CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:categoryFilterExceptions")]
    C16CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:datapointuniqueidmap")]
    C16Datapointuniqueidmap(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    ),
    #[xml(tag = "c16:uniqueId")]
    C16UniqueId(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    ),
}
/// Defines the SurfaceSerExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct SurfaceSerExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<SurfaceSerExtension>,
}
/// Defines the SurfaceSerExtension Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct SurfaceSerExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "c15:filteredSeriesTitle",
        child = "c15:filteredCategoryTitle",
        child = "c15:categoryFilterExceptions",
        child = "c16:categoryFilterExceptions",
        child = "c16:datapointuniqueidmap",
        child = "c16:uniqueId",
    )]
    pub children: Vec<SurfaceSerExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SurfaceSerExtensionChildChoice {
    #[xml(tag = "c15:filteredSeriesTitle")]
    C15FilteredSeriesTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    ),
    #[xml(tag = "c15:filteredCategoryTitle")]
    C15FilteredCategoryTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    ),
    #[xml(tag = "c15:categoryFilterExceptions")]
    C15CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:categoryFilterExceptions")]
    C16CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:datapointuniqueidmap")]
    C16Datapointuniqueidmap(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    ),
    #[xml(tag = "c16:uniqueId")]
    C16UniqueId(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    ),
}
/// Defines the DataDisplayOptions16 Class.
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:ext")]
pub struct DataDisplayOptions16 {
    /// _
    #[xml(child = "c16r3:dispNaAsBlank")]
    pub boolean_false: Option<
        crate::schemas::schemas_microsoft_com_office_drawing_2017_03_chart::BooleanFalse,
    >,
}
/// pivot chart format persistence data.
/// When the object is serialized out as xml, it's qualified name is c:pivotFmts.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:pivotFmts")]
pub struct PivotFormats {
    /// _
    #[xml(child = "c:pivotFmt")]
    pub c_pivot_fmt: Vec<PivotFormat>,
}
/// 3D view settings.
/// When the object is serialized out as xml, it's qualified name is c:view3D.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:view3D")]
pub struct View3D {
    ///X Rotation
    #[xml(child = "c:rotX")]
    pub rotate_x: Option<RotateX>,
    ///Height Percent
    #[xml(child = "c:hPercent")]
    pub height_percent: Option<HeightPercent>,
    ///Y Rotation
    #[xml(child = "c:rotY")]
    pub rotate_y: Option<RotateY>,
    ///Depth Percent
    #[xml(child = "c:depthPercent")]
    pub depth_percent: Option<DepthPercent>,
    ///Right Angle Axes
    #[xml(child = "c:rAngAx")]
    pub right_angle_axes: Option<RightAngleAxes>,
    ///Perspective
    #[xml(child = "c:perspective")]
    pub perspective: Option<Perspective>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// 3D floor formatting.
/// When the object is serialized out as xml, it's qualified name is c:floor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:floor")]
pub struct Floor {
    ///Thickness
    #[xml(child = "c:thickness")]
    pub thickness: Option<Thickness>,
    /// _
    #[xml(child = "c:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    ///Picture Options
    #[xml(child = "c:pictureOptions")]
    pub picture_options: Option<PictureOptions>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// 3D side wall formatting.
/// When the object is serialized out as xml, it's qualified name is c:sideWall.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:sideWall")]
pub struct SideWall {
    ///Thickness
    #[xml(child = "c:thickness")]
    pub thickness: Option<Thickness>,
    /// _
    #[xml(child = "c:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    ///Picture Options
    #[xml(child = "c:pictureOptions")]
    pub picture_options: Option<PictureOptions>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// 3D back wall formatting.
/// When the object is serialized out as xml, it's qualified name is c:backWall.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:backWall")]
pub struct BackWall {
    ///Thickness
    #[xml(child = "c:thickness")]
    pub thickness: Option<Thickness>,
    /// _
    #[xml(child = "c:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    ///Picture Options
    #[xml(child = "c:pictureOptions")]
    pub picture_options: Option<PictureOptions>,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the SurfaceType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct SurfaceType {}
/// Plot data and formatting.
/// When the object is serialized out as xml, it's qualified name is c:plotArea.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:plotArea")]
pub struct PlotArea {
    #[xml(
        child = "c:layout",
        child = "c:areaChart",
        child = "c:area3DChart",
        child = "c:lineChart",
        child = "c:line3DChart",
        child = "c:stockChart",
        child = "c:radarChart",
        child = "c:scatterChart",
        child = "c:pieChart",
        child = "c:pie3DChart",
        child = "c:doughnutChart",
        child = "c:barChart",
        child = "c:bar3DChart",
        child = "c:ofPieChart",
        child = "c:surfaceChart",
        child = "c:surface3DChart",
        child = "c:bubbleChart",
        child = "c:valAx",
        child = "c:catAx",
        child = "c:dateAx",
        child = "c:serAx",
        child = "c:dTable",
        child = "c:spPr",
        child = "c:extLst",
    )]
    pub children: Vec<PlotAreaChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PlotAreaChildChoice {
    #[xml(tag = "c:layout")]
    CLayout(Layout),
    #[xml(tag = "c:areaChart")]
    CAreaChart(AreaChart),
    #[xml(tag = "c:area3DChart")]
    CArea3DChart(Area3DChart),
    #[xml(tag = "c:lineChart")]
    CLineChart(LineChart),
    #[xml(tag = "c:line3DChart")]
    CLine3DChart(Line3DChart),
    #[xml(tag = "c:stockChart")]
    CStockChart(StockChart),
    #[xml(tag = "c:radarChart")]
    CRadarChart(RadarChart),
    #[xml(tag = "c:scatterChart")]
    CScatterChart(ScatterChart),
    #[xml(tag = "c:pieChart")]
    CPieChart(PieChart),
    #[xml(tag = "c:pie3DChart")]
    CPie3DChart(Pie3DChart),
    #[xml(tag = "c:doughnutChart")]
    CDoughnutChart(DoughnutChart),
    #[xml(tag = "c:barChart")]
    CBarChart(BarChart),
    #[xml(tag = "c:bar3DChart")]
    CBar3DChart(Bar3DChart),
    #[xml(tag = "c:ofPieChart")]
    COfPieChart(OfPieChart),
    #[xml(tag = "c:surfaceChart")]
    CSurfaceChart(SurfaceChart),
    #[xml(tag = "c:surface3DChart")]
    CSurface3DChart(Surface3DChart),
    #[xml(tag = "c:bubbleChart")]
    CBubbleChart(BubbleChart),
    #[xml(tag = "c:valAx")]
    CValAx(ValueAxis),
    #[xml(tag = "c:catAx")]
    CCatAx(CategoryAxis),
    #[xml(tag = "c:dateAx")]
    CDateAx(DateAxis),
    #[xml(tag = "c:serAx")]
    CSerAx(SeriesAxis),
    #[xml(tag = "c:dTable")]
    CDTable(DataTable),
    #[xml(tag = "c:spPr")]
    CSpPr(ShapeProperties),
    #[xml(tag = "c:extLst")]
    CExtLst(ExtensionList),
}
/// Legend data and formatting.
/// When the object is serialized out as xml, it's qualified name is c:legend.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:legend")]
pub struct Legend {
    ///Legend Position
    #[xml(child = "c:legendPos")]
    pub legend_position: Option<LegendPosition>,
    /// _
    #[xml(child = "c:legendEntry")]
    pub c_legend_entry: Vec<LegendEntry>,
    /// _
    #[xml(child = "c:layout")]
    pub c_layout: Option<Layout>,
    /// _
    #[xml(child = "c:overlay")]
    pub c_overlay: Option<Overlay>,
    /// _
    #[xml(child = "c:spPr")]
    pub c_sp_pr: Option<ChartShapeProperties>,
    /// _
    #[xml(child = "c:txPr")]
    pub c_tx_pr: Option<TextProperties>,
    /// _
    #[xml(child = "c:extLst")]
    pub c_ext_lst: Option<ExtensionList>,
}
/// The way that blank cells are plotted on a chart..
/// When the object is serialized out as xml, it's qualified name is c:dispBlanksAs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:dispBlanksAs")]
pub struct DisplayBlanksAs {
    /// Display Blanks As Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<DisplayBlanksAsValues>,
}
/// Extensibility container.
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c:extLst")]
pub struct ChartExtensionList {
    /// _
    #[xml(child = "c:ext")]
    pub c_ext: Vec<DataDisplayOptions16>,
}
