#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum KnownContextNodeTypeValues {
    #[default]
    Root,
    UnclassifiedInk,
    WritingRegion,
    AnalysisHint,
    Object,
    InkDrawing,
    Image,
    Paragraph,
    Line,
    InkBullet,
    InkWord,
    TextWord,
    CustomRecognizer,
    MathRegion,
    MathEquation,
    MathStruct,
    MathSymbol,
    MathIdentifier,
    MathOperator,
    MathNumber,
    NonInkDrawing,
    GroupNode,
    MixedDrawing,
}
crate::__string_enum! {
    KnownContextNodeTypeValues { Root = "root", UnclassifiedInk = "unclassifiedInk",
    WritingRegion = "writingRegion", AnalysisHint = "analysisHint", Object = "object",
    InkDrawing = "inkDrawing", Image = "image", Paragraph = "paragraph", Line = "line",
    InkBullet = "inkBullet", InkWord = "inkWord", TextWord = "textWord", CustomRecognizer
    = "customRecognizer", MathRegion = "mathRegion", MathEquation = "mathEquation",
    MathStruct = "mathStruct", MathSymbol = "mathSymbol", MathIdentifier =
    "mathIdentifier", MathOperator = "mathOperator", MathNumber = "mathNumber",
    NonInkDrawing = "nonInkDrawing", GroupNode = "groupNode", MixedDrawing =
    "mixedDrawing", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LinkDirectionValues {
    #[default]
    To,
    From,
    With,
}
crate::__string_enum! {
    LinkDirectionValues { To = "to", From = "from", With = "with", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum KnownSemanticTypeValues {
    #[default]
    None,
    Underline,
    Strikethrough,
    Highlight,
    ScratchOut,
    VerticalRange,
    Callout,
    Enclosure,
    Comment,
    Container,
    Connector,
}
crate::__string_enum! {
    KnownSemanticTypeValues { None = "none", Underline = "underline", Strikethrough =
    "strikethrough", Highlight = "highlight", ScratchOut = "scratchOut", VerticalRange =
    "verticalRange", Callout = "callout", Enclosure = "enclosure", Comment = "comment",
    Container = "container", Connector = "connector", }
}
/// Defines the ContextNode Class.
/// When the object is serialized out as xml, it's qualified name is msink:context.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "msink:context")]
pub struct ContextNode {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: String,
    /// rotatedBoundingBox
    /// Represents the following attribute in the schema: :rotatedBoundingBox
    #[xml(attr = "rotatedBoundingBox")]
    pub rotated_bounding_box: Option<String>,
    /// alignmentLevel
    /// Represents the following attribute in the schema: :alignmentLevel
    #[xml(attr = "alignmentLevel")]
    pub alignment_level: Option<i32>,
    /// contentType
    /// Represents the following attribute in the schema: :contentType
    #[xml(attr = "contentType")]
    pub content_type: Option<i32>,
    /// ascender
    /// Represents the following attribute in the schema: :ascender
    #[xml(attr = "ascender")]
    pub ascender: Option<String>,
    /// descender
    /// Represents the following attribute in the schema: :descender
    #[xml(attr = "descender")]
    pub descender: Option<String>,
    /// baseline
    /// Represents the following attribute in the schema: :baseline
    #[xml(attr = "baseline")]
    pub baseline: Option<String>,
    /// midline
    /// Represents the following attribute in the schema: :midline
    #[xml(attr = "midline")]
    pub midline: Option<String>,
    /// customRecognizerId
    /// Represents the following attribute in the schema: :customRecognizerId
    #[xml(attr = "customRecognizerId")]
    pub custom_recognizer_id: Option<String>,
    /// mathML
    /// Represents the following attribute in the schema: :mathML
    #[xml(attr = "mathML")]
    pub math_ml: Option<String>,
    /// mathStruct
    /// Represents the following attribute in the schema: :mathStruct
    #[xml(attr = "mathStruct")]
    pub math_struct: Option<String>,
    /// mathSymbol
    /// Represents the following attribute in the schema: :mathSymbol
    #[xml(attr = "mathSymbol")]
    pub math_symbol: Option<String>,
    /// beginModifierType
    /// Represents the following attribute in the schema: :beginModifierType
    #[xml(attr = "beginModifierType")]
    pub begin_modifier_type: Option<String>,
    /// endModifierType
    /// Represents the following attribute in the schema: :endModifierType
    #[xml(attr = "endModifierType")]
    pub end_modifier_type: Option<String>,
    /// rotationAngle
    /// Represents the following attribute in the schema: :rotationAngle
    #[xml(attr = "rotationAngle")]
    pub rotation_angle: Option<i32>,
    /// hotPoints
    /// Represents the following attribute in the schema: :hotPoints
    #[xml(attr = "hotPoints")]
    pub hot_points: Option<String>,
    /// centroid
    /// Represents the following attribute in the schema: :centroid
    #[xml(attr = "centroid")]
    pub centroid: Option<String>,
    /// semanticType
    /// Represents the following attribute in the schema: :semanticType
    #[xml(attr = "semanticType")]
    pub semantic_type: Option<String>,
    /// shapeName
    /// Represents the following attribute in the schema: :shapeName
    #[xml(attr = "shapeName")]
    pub shape_name: Option<String>,
    /// shapeGeometry
    /// Represents the following attribute in the schema: :shapeGeometry
    #[xml(attr = "shapeGeometry")]
    pub shape_geometry: Option<String>,
    /// _
    #[xml(child = "msink:property")]
    pub msink_property: Vec<ContextNodeProperty>,
    /// _
    #[xml(child = "msink:sourceLink")]
    pub msink_source_link: Vec<SourceLink>,
    /// _
    #[xml(child = "msink:destinationLink")]
    pub msink_destination_link: Vec<DestinationLink>,
}
/// Defines the ContextNodeProperty Class.
/// When the object is serialized out as xml, it's qualified name is msink:property.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "msink:property")]
pub struct ContextNodeProperty {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<String>,
    #[xml(text)]
    pub child: String,
}
/// Defines the SourceLink Class.
/// When the object is serialized out as xml, it's qualified name is msink:sourceLink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "msink:sourceLink")]
pub struct SourceLink {
    /// direction
    /// Represents the following attribute in the schema: :direction
    #[xml(attr = "direction")]
    pub direction: Option<LinkDirectionValues>,
    /// ref
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: Option<String>,
}
/// Defines the DestinationLink Class.
/// When the object is serialized out as xml, it's qualified name is msink:destinationLink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "msink:destinationLink")]
pub struct DestinationLink {
    /// direction
    /// Represents the following attribute in the schema: :direction
    #[xml(attr = "direction")]
    pub direction: Option<LinkDirectionValues>,
    /// ref
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: Option<String>,
}
/// Defines the ContextLinkType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct ContextLinkType {
    /// direction
    /// Represents the following attribute in the schema: :direction
    #[xml(attr = "direction")]
    pub direction: Option<LinkDirectionValues>,
    /// ref
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: Option<String>,
}
