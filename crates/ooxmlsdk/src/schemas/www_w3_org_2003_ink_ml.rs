#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ChannelDataTypeValues {
    #[default]
    Integer,
    Decimal,
    Boolean,
}
crate::__string_enum! {
    ChannelDataTypeValues { Integer = "integer", Decimal = "decimal", Boolean =
    "boolean", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ChannelValueOrientationValues {
    #[default]
    PlusVe,
    MinusVe,
}
crate::__string_enum! {
    ChannelValueOrientationValues { PlusVe = "+ve", MinusVe = "-ve", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StandardChannelPropertyNameValues {
    #[default]
    Threshold,
    Resolution,
    Quantization,
    Noise,
    Accuracy,
    CrossCoupling,
    Skew,
    MinBandwidth,
    PeakRate,
    Distortion,
}
crate::__string_enum! {
    StandardChannelPropertyNameValues { Threshold = "threshold", Resolution =
    "resolution", Quantization = "quantization", Noise = "noise", Accuracy = "accuracy",
    CrossCoupling = "crossCoupling", Skew = "skew", MinBandwidth = "minBandwidth",
    PeakRate = "peakRate", Distortion = "distortion", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StandardBrushPropertyNameValues {
    #[default]
    Width,
    Height,
    Color,
    Transparency,
    Tip,
    RasterOp,
    AntiAliased,
    FitToCurve,
    IgnorePressure,
}
crate::__string_enum! {
    StandardBrushPropertyNameValues { Width = "width", Height = "height", Color =
    "color", Transparency = "transparency", Tip = "tip", RasterOp = "rasterOp",
    AntiAliased = "antiAliased", FitToCurve = "fitToCurve", IgnorePressure =
    "ignorePressure", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StandardChannelNameValues {
    #[default]
    XCoordinate,
    YCoordinate,
    ZCoordinate,
    PenTipForce,
    TangentPressure,
    ButtonPressure,
    TipSwitchState,
    SideButton1State,
    SideButton2State,
    SideButton3State,
    SideButton4State,
    PenInverted,
    TiltAlongXAxis,
    TiltAlongYAxis,
    PenAzimuthAngle,
    PenElevationAngle,
    PexAxisRotation,
    PitchRotation,
    RollRotation,
    YawRotation,
    ColorValue,
    RedColorValue,
    GreenColorValue,
    BlueColorValue,
    CyanColorValue,
    MegentaColorValue,
    YellowColorValue,
    BlackColorValue,
    StrokesWidth,
    Time,
    SerialNumber,
    TouchWidth,
    TouchHeight,
    FingerTouch,
}
crate::__string_enum! {
    StandardChannelNameValues { XCoordinate = "x", YCoordinate = "y", ZCoordinate = "z",
    PenTipForce = "f", TangentPressure = "tp", ButtonPressure = "bp", TipSwitchState =
    "s", SideButton1State = "b1", SideButton2State = "b2", SideButton3State = "b3",
    SideButton4State = "b4", PenInverted = "e", TiltAlongXAxis = "oTx", TiltAlongYAxis =
    "oTy", PenAzimuthAngle = "oa", PenElevationAngle = "oe", PexAxisRotation = "or",
    PitchRotation = "rp", RollRotation = "rr", YawRotation = "ry", ColorValue = "c",
    RedColorValue = "cr", GreenColorValue = "cg", BlueColorValue = "cb", CyanColorValue =
    "cc", MegentaColorValue = "cm", YellowColorValue = "cy", BlackColorValue = "ck",
    StrokesWidth = "w", Time = "t", SerialNumber = "sn", TouchWidth = "tw", TouchHeight =
    "th", FingerTouch = "tc", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StandardLengthUnitsValues {
    #[default]
    Meter,
    Centimeter,
    Millimeter,
    Inche,
    Point,
    Pica,
    Em,
    Ex,
}
crate::__string_enum! {
    StandardLengthUnitsValues { Meter = "m", Centimeter = "cm", Millimeter = "mm", Inche
    = "in", Point = "pt", Pica = "pc", Em = "em", Ex = "ex", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StandardPerLengthUnitsValues {
    #[default]
    PerMeter,
    PerCentimeter,
    PerMillimeter,
    PerInche,
    PerPoint,
    PerPica,
    PerEm,
    PerEx,
}
crate::__string_enum! {
    StandardPerLengthUnitsValues { PerMeter = "1M", PerCentimeter = "1Cm", PerMillimeter
    = "1Mm", PerInche = "1In", PerPoint = "1Pt", PerPica = "1Pc", PerEm = "1Em", PerEx =
    "1Ex", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StandardTimeUnitsValues {
    #[default]
    Second,
    Millisecond,
}
crate::__string_enum! {
    StandardTimeUnitsValues { Second = "s", Millisecond = "ms", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StandardPerTimeUnitsValues {
    #[default]
    PerSecond,
    PerMillisecond,
}
crate::__string_enum! {
    StandardPerTimeUnitsValues { PerSecond = "1S", PerMillisecond = "1Ms", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StandardMassForceUnitsValues {
    #[default]
    Kilogram,
    Gram,
    Milligram,
    Newton,
    Pond,
}
crate::__string_enum! {
    StandardMassForceUnitsValues { Kilogram = "kg", Gram = "g", Milligram = "mg", Newton
    = "n", Pond = "lb", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StandardPerMassForceUnitsValues {
    #[default]
    PerKilogram,
    PerGram,
    PerMilligram,
    PerNewton,
    PerPond,
}
crate::__string_enum! {
    StandardPerMassForceUnitsValues { PerKilogram = "1Kg", PerGram = "1G", PerMilligram =
    "1Mg", PerNewton = "1N", PerPond = "1Lb", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StandardAngleUnitsValues {
    #[default]
    Degree,
    Radian,
}
crate::__string_enum! {
    StandardAngleUnitsValues { Degree = "deg", Radian = "rad", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StandardPerAngleUnitsValues {
    #[default]
    PerDegree,
    PerRadian,
}
crate::__string_enum! {
    StandardPerAngleUnitsValues { PerDegree = "1Deg", PerRadian = "1Rad", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StandardOtherUnitsValues {
    #[default]
    Percentage,
    DeviceResolution,
    None,
}
crate::__string_enum! {
    StandardOtherUnitsValues { Percentage = "", DeviceResolution = "dev", None = "none",
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StandardPerOtherUnitsValues {
    #[default]
    PerPercentage,
    PerDeviceResolution,
}
crate::__string_enum! {
    StandardPerOtherUnitsValues { PerPercentage = "1", PerDeviceResolution = "1Dev", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TraceTypeValues {
    #[default]
    PenDown,
    PenUp,
    Indeterminate,
}
crate::__string_enum! {
    TraceTypeValues { PenDown = "penDown", PenUp = "penUp", Indeterminate =
    "indeterminate", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TraceContinuationValues {
    #[default]
    Begin,
    End,
    Middle,
}
crate::__string_enum! {
    TraceContinuationValues { Begin = "begin", End = "end", Middle = "middle", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RasterOperationValues {
    #[default]
    Black,
    NotMergePen,
    MaskNotPen,
    NotCopyPen,
    MaskPenNot,
    Not,
    XOrPen,
    NotMaskPen,
    MaskPen,
    NotXOrPen,
    NoOperation,
    MergeNotPen,
    CopyPen,
    MergePenNot,
    MergePen,
    White,
}
crate::__string_enum! {
    RasterOperationValues { Black = "black", NotMergePen = "notMergePen", MaskNotPen =
    "maskNotPen", NotCopyPen = "notCopyPen", MaskPenNot = "maskPenNot", Not = "not",
    XOrPen = "xOrPen", NotMaskPen = "notMaskPen", MaskPen = "maskPen", NotXOrPen =
    "notXOrPen", NoOperation = "noOperation", MergeNotPen = "mergeNotPen", CopyPen =
    "copyPen", MergePenNot = "mergePenNot", MergePen = "mergePen", White = "white", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PenTipShapeValues {
    #[default]
    Ellipse,
    Rectangle,
    Drop,
}
crate::__string_enum! {
    PenTipShapeValues { Ellipse = "ellipse", Rectangle = "rectangle", Drop = "drop", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MappingTypeValues {
    #[default]
    Identity,
    Lookup,
    Affine,
    MathMl,
    Product,
    Unknown,
}
crate::__string_enum! {
    MappingTypeValues { Identity = "identity", Lookup = "lookup", Affine = "affine",
    MathMl = "mathml", Product = "product", Unknown = "unknown", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TableApplyValues {
    #[default]
    Absolute,
    Relative,
}
crate::__string_enum! {
    TableApplyValues { Absolute = "absolute", Relative = "relative", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TableInterpolationValues {
    #[default]
    Floor,
    Middle,
    Ceiling,
    Linear,
    Cubic,
}
crate::__string_enum! {
    TableInterpolationValues { Floor = "floor", Middle = "middle", Ceiling = "ceiling",
    Linear = "linear", Cubic = "cubic", }
}
/// Defines the Ink Class.
/// When the object is serialized out as xml, it's qualified name is inkml:ink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:ink")]
pub struct Ink {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// documentID
    /// Represents the following attribute in the schema: :documentID
    #[xml(attr = "documentID")]
    pub document_id: Option<String>,
    #[xml(
        child = "inkml:annotation",
        child = "inkml:annotationXML",
        child = "inkml:definitions",
        child = "inkml:context",
        child = "inkml:trace",
        child = "inkml:traceGroup",
        child = "inkml:traceView",
    )]
    pub children: Vec<InkChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum InkChildChoice {
    #[xml(tag = "inkml:annotation")]
    InkmlAnnotation(Annotation),
    #[xml(tag = "inkml:annotationXML")]
    InkmlAnnotationXml(AnnotationXml),
    #[xml(tag = "inkml:definitions")]
    InkmlDefinitions(Definitions),
    #[xml(tag = "inkml:context")]
    InkmlContext(Context),
    #[xml(tag = "inkml:trace")]
    InkmlTrace(Trace),
    #[xml(tag = "inkml:traceGroup")]
    InkmlTraceGroup(TraceGroup),
    #[xml(tag = "inkml:traceView")]
    InkmlTraceView(TraceView),
}
/// Defines the Bind Class.
/// When the object is serialized out as xml, it's qualified name is inkml:bind.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:bind")]
pub struct Bind {
    /// source
    /// Represents the following attribute in the schema: :source
    #[xml(attr = "source")]
    pub source: Option<String>,
    /// target
    /// Represents the following attribute in the schema: :target
    #[xml(attr = "target")]
    pub target: Option<String>,
    /// column
    /// Represents the following attribute in the schema: :column
    #[xml(attr = "column")]
    pub column: Option<String>,
    /// variable
    /// Represents the following attribute in the schema: :variable
    #[xml(attr = "variable")]
    pub variable: Option<String>,
}
/// Defines the Table Class.
/// When the object is serialized out as xml, it's qualified name is inkml:table.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:table")]
pub struct Table {
    /// id
    /// Represents the following attribute in the schema: xml:id
    #[xml(attr = "xml:id")]
    pub id: Option<String>,
    /// apply
    /// Represents the following attribute in the schema: :apply
    #[xml(attr = "apply")]
    pub apply: Option<TableApplyValues>,
    /// interpolation
    /// Represents the following attribute in the schema: :interpolation
    #[xml(attr = "interpolation")]
    pub interpolation: Option<TableInterpolationValues>,
    #[xml(text)]
    pub child: String,
}
/// Defines the Matrix Class.
/// When the object is serialized out as xml, it's qualified name is inkml:matrix.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:matrix")]
pub struct Matrix {
    /// id
    /// Represents the following attribute in the schema: xml:id
    #[xml(attr = "xml:id")]
    pub id: Option<String>,
    #[xml(text)]
    pub child: String,
}
/// Defines the Mapping Class.
/// When the object is serialized out as xml, it's qualified name is inkml:mapping.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:mapping")]
pub struct Mapping {
    /// id
    /// Represents the following attribute in the schema: xml:id
    #[xml(attr = "xml:id")]
    pub id: Option<String>,
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<MappingTypeValues>,
    /// mappingRef
    /// Represents the following attribute in the schema: :mappingRef
    #[xml(attr = "mappingRef")]
    pub mapping_ref: Option<String>,
    #[xml(
        child = "inkml:bind",
        child = "inkml:table",
        child = "inkml:matrix",
        child = "inkml:mapping",
    )]
    pub children: Vec<MappingChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MappingChildChoice {
    #[xml(tag = "inkml:bind")]
    InkmlBind(Bind),
    #[xml(tag = "inkml:table")]
    InkmlTable(Table),
    #[xml(tag = "inkml:matrix")]
    InkmlMatrix(Matrix),
    #[xml(tag = "inkml:mapping")]
    InkmlMapping(Mapping),
}
/// Defines the Channel Class.
/// When the object is serialized out as xml, it's qualified name is inkml:channel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:channel")]
pub struct Channel {
    /// id
    /// Represents the following attribute in the schema: xml:id
    #[xml(attr = "xml:id")]
    pub id: Option<String>,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<ChannelDataTypeValues>,
    /// default
    /// Represents the following attribute in the schema: :default
    #[xml(attr = "default")]
    pub default: Option<String>,
    /// min
    /// Represents the following attribute in the schema: :min
    #[xml(attr = "min")]
    pub min: Option<String>,
    /// max
    /// Represents the following attribute in the schema: :max
    #[xml(attr = "max")]
    pub max: Option<String>,
    /// orientation
    /// Represents the following attribute in the schema: :orientation
    #[xml(attr = "orientation")]
    pub orientation: Option<ChannelValueOrientationValues>,
    /// respectTo
    /// Represents the following attribute in the schema: :respectTo
    #[xml(attr = "respectTo")]
    pub respect_to: Option<String>,
    /// units
    /// Represents the following attribute in the schema: :units
    #[xml(attr = "units")]
    pub units: Option<String>,
    /// _
    #[xml(child = "inkml:mapping")]
    pub inkml_mapping: Vec<Mapping>,
}
/// Defines the IntermittentChannels Class.
/// When the object is serialized out as xml, it's qualified name is inkml:intermittentChannels.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:intermittentChannels")]
pub struct IntermittentChannels {
    /// _
    #[xml(child = "inkml:channel")]
    pub inkml_channel: Vec<Channel>,
}
/// Defines the ChannelProperty Class.
/// When the object is serialized out as xml, it's qualified name is inkml:channelProperty.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:channelProperty")]
pub struct ChannelProperty {
    /// channel
    /// Represents the following attribute in the schema: :channel
    #[xml(attr = "channel")]
    pub channel: String,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// value
    /// Represents the following attribute in the schema: :value
    #[xml(attr = "value")]
    pub value: String,
    /// units
    /// Represents the following attribute in the schema: :units
    #[xml(attr = "units")]
    pub units: Option<String>,
}
/// Defines the TraceFormat Class.
/// When the object is serialized out as xml, it's qualified name is inkml:traceFormat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:traceFormat")]
pub struct TraceFormat {
    /// id
    /// Represents the following attribute in the schema: xml:id
    #[xml(attr = "xml:id")]
    pub id: Option<String>,
    /// _
    #[xml(child = "inkml:channel")]
    pub inkml_channel: Vec<Channel>,
    /// _
    #[xml(child = "inkml:intermittentChannels")]
    pub inkml_intermittent_channels: Option<IntermittentChannels>,
}
/// Defines the SampleRate Class.
/// When the object is serialized out as xml, it's qualified name is inkml:sampleRate.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:sampleRate")]
pub struct SampleRate {
    /// uniform
    /// Represents the following attribute in the schema: :uniform
    #[xml(attr = "uniform")]
    pub uniform: Option<bool>,
    /// value
    /// Represents the following attribute in the schema: :value
    #[xml(attr = "value")]
    pub value: String,
}
/// Defines the Latency Class.
/// When the object is serialized out as xml, it's qualified name is inkml:latency.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:latency")]
pub struct Latency {
    /// value
    /// Represents the following attribute in the schema: :value
    #[xml(attr = "value")]
    pub value: String,
}
/// Defines the ActiveArea Class.
/// When the object is serialized out as xml, it's qualified name is inkml:activeArea.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:activeArea")]
pub struct ActiveArea {
    /// size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: Option<String>,
    /// height
    /// Represents the following attribute in the schema: :height
    #[xml(attr = "height")]
    pub height: Option<String>,
    /// width
    /// Represents the following attribute in the schema: :width
    #[xml(attr = "width")]
    pub width: Option<String>,
    /// units
    /// Represents the following attribute in the schema: :units
    #[xml(attr = "units")]
    pub units: Option<String>,
}
/// Defines the SourceProperty Class.
/// When the object is serialized out as xml, it's qualified name is inkml:srcProperty.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:srcProperty")]
pub struct SourceProperty {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// value
    /// Represents the following attribute in the schema: :value
    #[xml(attr = "value")]
    pub value: String,
    /// units
    /// Represents the following attribute in the schema: :units
    #[xml(attr = "units")]
    pub units: Option<String>,
}
/// Defines the ChannelProperties Class.
/// When the object is serialized out as xml, it's qualified name is inkml:channelProperties.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:channelProperties")]
pub struct ChannelProperties {
    /// _
    #[xml(child = "inkml:channelProperty")]
    pub inkml_channel_property: Vec<ChannelProperty>,
}
/// Defines the Annotation Class.
/// When the object is serialized out as xml, it's qualified name is inkml:annotation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:annotation")]
pub struct Annotation {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<String>,
    /// encoding
    /// Represents the following attribute in the schema: :encoding
    #[xml(attr = "encoding")]
    pub encoding: Option<String>,
    #[xml(text)]
    pub child: String,
}
/// Defines the AnnotationXml Class.
/// When the object is serialized out as xml, it's qualified name is inkml:annotationXML.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:annotationXML")]
pub struct AnnotationXml {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<String>,
    /// encoding
    /// Represents the following attribute in the schema: :encoding
    #[xml(attr = "encoding")]
    pub encoding: Option<String>,
    /// href
    /// Represents the following attribute in the schema: :href
    #[xml(attr = "href")]
    pub href: Option<String>,
    /// _
    #[xml(child = "emma:emma")]
    pub emma: Option<crate::schemas::www_w3_org_2003_04_emma::Emma>,
}
/// Defines the BrushProperty Class.
/// When the object is serialized out as xml, it's qualified name is inkml:brushProperty.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:brushProperty")]
pub struct BrushProperty {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// value
    /// Represents the following attribute in the schema: :value
    #[xml(attr = "value")]
    pub value: String,
    /// units
    /// Represents the following attribute in the schema: :units
    #[xml(attr = "units")]
    pub units: Option<String>,
    /// _
    #[xml(child = "inkml:annotation")]
    pub inkml_annotation: Vec<Annotation>,
    /// _
    #[xml(child = "inkml:annotationXML")]
    pub inkml_annotation_xml: Vec<AnnotationXml>,
}
/// Defines the Canvas Class.
/// When the object is serialized out as xml, it's qualified name is inkml:canvas.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:canvas")]
pub struct Canvas {
    /// id
    /// Represents the following attribute in the schema: xml:id
    #[xml(attr = "xml:id")]
    pub id: Option<String>,
    /// traceFormatRef
    /// Represents the following attribute in the schema: :traceFormatRef
    #[xml(attr = "traceFormatRef")]
    pub trace_format_ref: Option<String>,
    /// _
    #[xml(child = "inkml:traceFormat")]
    pub trace_format: Option<TraceFormat>,
}
/// Defines the CanvasTransform Class.
/// When the object is serialized out as xml, it's qualified name is inkml:canvasTransform.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:canvasTransform")]
pub struct CanvasTransform {
    /// id
    /// Represents the following attribute in the schema: xml:id
    #[xml(attr = "xml:id")]
    pub id: Option<String>,
    /// invertible
    /// Represents the following attribute in the schema: :invertible
    #[xml(attr = "invertible")]
    pub invertible: Option<bool>,
    /// _
    #[xml(child = "inkml:mapping")]
    pub inkml_mapping: Vec<Mapping>,
}
/// Defines the InkSource Class.
/// When the object is serialized out as xml, it's qualified name is inkml:inkSource.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:inkSource")]
pub struct InkSource {
    /// id
    /// Represents the following attribute in the schema: xml:id
    #[xml(attr = "xml:id")]
    pub id: String,
    /// manufacturer
    /// Represents the following attribute in the schema: :manufacturer
    #[xml(attr = "manufacturer")]
    pub manufacturer: Option<String>,
    /// model
    /// Represents the following attribute in the schema: :model
    #[xml(attr = "model")]
    pub model: Option<String>,
    /// serialNo
    /// Represents the following attribute in the schema: :serialNo
    #[xml(attr = "serialNo")]
    pub serial_no: Option<String>,
    /// specificationRef
    /// Represents the following attribute in the schema: :specificationRef
    #[xml(attr = "specificationRef")]
    pub specification_ref: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// _
    #[xml(child = "inkml:traceFormat")]
    pub trace_format: TraceFormat,
    /// _
    #[xml(child = "inkml:sampleRate")]
    pub sample_rate: Option<SampleRate>,
    /// _
    #[xml(child = "inkml:latency")]
    pub latency: Option<Latency>,
    /// _
    #[xml(child = "inkml:activeArea")]
    pub active_area: Option<ActiveArea>,
    /// _
    #[xml(child = "inkml:srcProperty")]
    pub inkml_src_property: Vec<SourceProperty>,
    /// _
    #[xml(child = "inkml:channelProperties")]
    pub inkml_channel_properties: Option<ChannelProperties>,
}
/// Defines the Brush Class.
/// When the object is serialized out as xml, it's qualified name is inkml:brush.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:brush")]
pub struct Brush {
    /// id
    /// Represents the following attribute in the schema: xml:id
    #[xml(attr = "xml:id")]
    pub id: Option<String>,
    /// brushRef
    /// Represents the following attribute in the schema: :brushRef
    #[xml(attr = "brushRef")]
    pub brush_ref: Option<String>,
    /// _
    #[xml(child = "inkml:annotation")]
    pub inkml_annotation: Vec<Annotation>,
    /// _
    #[xml(child = "inkml:annotationXML")]
    pub inkml_annotation_xml: Vec<AnnotationXml>,
    /// _
    #[xml(child = "inkml:brushProperty")]
    pub inkml_brush_property: Vec<BrushProperty>,
}
/// Defines the Timestamp Class.
/// When the object is serialized out as xml, it's qualified name is inkml:timestamp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:timestamp")]
pub struct Timestamp {
    /// id
    /// Represents the following attribute in the schema: xml:id
    #[xml(attr = "xml:id")]
    pub id: String,
    /// time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: Option<String>,
    /// timestampRef
    /// Represents the following attribute in the schema: :timestampRef
    #[xml(attr = "timestampRef")]
    pub timestamp_ref: Option<String>,
    /// timeString
    /// Represents the following attribute in the schema: :timeString
    #[xml(attr = "timeString")]
    pub time_string: Option<String>,
    /// timeOffset
    /// Represents the following attribute in the schema: :timeOffset
    #[xml(attr = "timeOffset")]
    pub time_offset: Option<String>,
}
/// Defines the Trace Class.
/// When the object is serialized out as xml, it's qualified name is inkml:trace.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:trace")]
pub struct Trace {
    /// id
    /// Represents the following attribute in the schema: xml:id
    #[xml(attr = "xml:id")]
    pub id: Option<String>,
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<TraceTypeValues>,
    /// continuation
    /// Represents the following attribute in the schema: :continuation
    #[xml(attr = "continuation")]
    pub continuation: Option<TraceContinuationValues>,
    /// priorRef
    /// Represents the following attribute in the schema: :priorRef
    #[xml(attr = "priorRef")]
    pub prior_ref: Option<String>,
    /// contextRef
    /// Represents the following attribute in the schema: :contextRef
    #[xml(attr = "contextRef")]
    pub context_ref: Option<String>,
    /// brushRef
    /// Represents the following attribute in the schema: :brushRef
    #[xml(attr = "brushRef")]
    pub brush_ref: Option<String>,
    /// duration
    /// Represents the following attribute in the schema: :duration
    #[xml(attr = "duration")]
    pub duration: Option<String>,
    /// timeOffset
    /// Represents the following attribute in the schema: :timeOffset
    #[xml(attr = "timeOffset")]
    pub time_offset: Option<String>,
    #[xml(text)]
    pub child: String,
}
/// Defines the TraceGroup Class.
/// When the object is serialized out as xml, it's qualified name is inkml:traceGroup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:traceGroup")]
pub struct TraceGroup {
    /// id
    /// Represents the following attribute in the schema: xml:id
    #[xml(attr = "xml:id")]
    pub id: Option<String>,
    /// contextRef
    /// Represents the following attribute in the schema: :contextRef
    #[xml(attr = "contextRef")]
    pub context_ref: Option<String>,
    /// brushRef
    /// Represents the following attribute in the schema: :brushRef
    #[xml(attr = "brushRef")]
    pub brush_ref: Option<String>,
    #[xml(
        child = "inkml:annotation",
        child = "inkml:annotationXML",
        child = "inkml:trace",
        child = "inkml:traceGroup",
    )]
    pub children: Vec<TraceGroupChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TraceGroupChildChoice {
    #[xml(tag = "inkml:annotation")]
    InkmlAnnotation(Annotation),
    #[xml(tag = "inkml:annotationXML")]
    InkmlAnnotationXml(AnnotationXml),
    #[xml(tag = "inkml:trace")]
    InkmlTrace(Trace),
    #[xml(tag = "inkml:traceGroup")]
    InkmlTraceGroup(TraceGroup),
}
/// Defines the TraceView Class.
/// When the object is serialized out as xml, it's qualified name is inkml:traceView.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:traceView")]
pub struct TraceView {
    /// id
    /// Represents the following attribute in the schema: xml:id
    #[xml(attr = "xml:id")]
    pub id: Option<String>,
    /// contextRef
    /// Represents the following attribute in the schema: :contextRef
    #[xml(attr = "contextRef")]
    pub context_ref: Option<String>,
    /// traceDataRef
    /// Represents the following attribute in the schema: :traceDataRef
    #[xml(attr = "traceDataRef")]
    pub trace_data_ref: Option<String>,
    /// from
    /// Represents the following attribute in the schema: :from
    #[xml(attr = "from")]
    pub from: Option<String>,
    /// to
    /// Represents the following attribute in the schema: :to
    #[xml(attr = "to")]
    pub to: Option<String>,
    #[xml(
        child = "inkml:annotation",
        child = "inkml:annotationXML",
        child = "inkml:traceView",
    )]
    pub children: Vec<TraceViewChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TraceViewChildChoice {
    #[xml(tag = "inkml:annotation")]
    InkmlAnnotation(Annotation),
    #[xml(tag = "inkml:annotationXML")]
    InkmlAnnotationXml(AnnotationXml),
    #[xml(tag = "inkml:traceView")]
    InkmlTraceView(TraceView),
}
/// Defines the Context Class.
/// When the object is serialized out as xml, it's qualified name is inkml:context.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:context")]
pub struct Context {
    /// id
    /// Represents the following attribute in the schema: xml:id
    #[xml(attr = "xml:id")]
    pub id: Option<String>,
    /// contextRef
    /// Represents the following attribute in the schema: :contextRef
    #[xml(attr = "contextRef")]
    pub context_ref: Option<String>,
    /// canvasRef
    /// Represents the following attribute in the schema: :canvasRef
    #[xml(attr = "canvasRef")]
    pub canvas_ref: Option<String>,
    /// canvasTransformRef
    /// Represents the following attribute in the schema: :canvasTransformRef
    #[xml(attr = "canvasTransformRef")]
    pub canvas_transform_ref: Option<String>,
    /// traceFormatRef
    /// Represents the following attribute in the schema: :traceFormatRef
    #[xml(attr = "traceFormatRef")]
    pub trace_fromat_ref: Option<String>,
    /// inkSourceRef
    /// Represents the following attribute in the schema: :inkSourceRef
    #[xml(attr = "inkSourceRef")]
    pub ink_source_ref: Option<String>,
    /// brushRef
    /// Represents the following attribute in the schema: :brushRef
    #[xml(attr = "brushRef")]
    pub brush_ref: Option<String>,
    /// timestampRef
    /// Represents the following attribute in the schema: :timestampRef
    #[xml(attr = "timestampRef")]
    pub timestamp_ref: Option<String>,
    /// _
    #[xml(child = "inkml:canvas")]
    pub canvas: Option<Canvas>,
    /// _
    #[xml(child = "inkml:canvasTransform")]
    pub canvas_transform: Option<CanvasTransform>,
    /// _
    #[xml(child = "inkml:traceFormat")]
    pub trace_format: Option<TraceFormat>,
    /// _
    #[xml(child = "inkml:inkSource")]
    pub ink_source: Option<InkSource>,
    /// _
    #[xml(child = "inkml:brush")]
    pub brush: Option<Brush>,
    /// _
    #[xml(child = "inkml:timestamp")]
    pub timestamp: Option<Timestamp>,
}
/// Defines the Definitions Class.
/// When the object is serialized out as xml, it's qualified name is inkml:definitions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "inkml:definitions")]
pub struct Definitions {
    #[xml(
        child = "inkml:brush",
        child = "inkml:canvas",
        child = "inkml:canvasTransform",
        child = "inkml:context",
        child = "inkml:inkSource",
        child = "inkml:mapping",
        child = "inkml:timestamp",
        child = "inkml:trace",
        child = "inkml:traceFormat",
        child = "inkml:traceGroup",
        child = "inkml:traceView",
    )]
    pub children: Vec<DefinitionsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DefinitionsChildChoice {
    #[xml(tag = "inkml:brush")]
    InkmlBrush(Brush),
    #[xml(tag = "inkml:canvas")]
    InkmlCanvas(Canvas),
    #[xml(tag = "inkml:canvasTransform")]
    InkmlCanvasTransform(CanvasTransform),
    #[xml(tag = "inkml:context")]
    InkmlContext(Context),
    #[xml(tag = "inkml:inkSource")]
    InkmlInkSource(InkSource),
    #[xml(tag = "inkml:mapping")]
    InkmlMapping(Mapping),
    #[xml(tag = "inkml:timestamp")]
    InkmlTimestamp(Timestamp),
    #[xml(tag = "inkml:trace")]
    InkmlTrace(Trace),
    #[xml(tag = "inkml:traceFormat")]
    InkmlTraceFormat(TraceFormat),
    #[xml(tag = "inkml:traceGroup")]
    InkmlTraceGroup(TraceGroup),
    #[xml(tag = "inkml:traceView")]
    InkmlTraceView(TraceView),
}
