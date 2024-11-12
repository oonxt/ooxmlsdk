/// Defines the ShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is c16:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:spPr")]
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
/// Defines the UnsignedIntegerType Class.
/// When the object is serialized out as xml, it's qualified name is c16:explosion.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:explosion")]
pub struct UnsignedIntegerType {
    /// Integer Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the InvertIfNegativeBoolean Class.
/// When the object is serialized out as xml, it's qualified name is c16:invertIfNegative.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:invertIfNegative")]
pub struct InvertIfNegativeBoolean {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the Bubble3DBoolean Class.
/// When the object is serialized out as xml, it's qualified name is c16:bubble3D.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:bubble3D")]
pub struct Bubble3DBoolean {
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
/// Defines the Marker Class.
/// When the object is serialized out as xml, it's qualified name is c16:marker.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:marker")]
pub struct Marker {
    ///Symbol
    #[xml(child = "c:symbol")]
    pub symbol: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Symbol,
    >,
    ///Size
    #[xml(child = "c:size")]
    pub size: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Size,
    >,
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    >,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ExtensionList,
    >,
}
/// Defines the DLbl Class.
/// When the object is serialized out as xml, it's qualified name is c16:dLbl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:dLbl")]
pub struct DLbl {
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
    pub children: Vec<DLblChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DLblChildChoice {
    #[xml(tag = "c:idx")]
    CIdx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index),
    #[xml(tag = "c:delete")]
    CDelete(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Delete),
    #[xml(tag = "c:layout")]
    CLayout(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Layout),
    #[xml(tag = "c:tx")]
    CTx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartText),
    #[xml(tag = "c:numFmt")]
    CNumFmt(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::NumberingFormat,
    ),
    #[xml(tag = "c:spPr")]
    CSpPr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    ),
    #[xml(tag = "c:txPr")]
    CTxPr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::TextProperties,
    ),
    #[xml(tag = "c:dLblPos")]
    CDLblPos(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabelPosition,
    ),
    #[xml(tag = "c:showLegendKey")]
    CShowLegendKey(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ShowLegendKey,
    ),
    #[xml(tag = "c:showVal")]
    CShowVal(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ShowValue),
    #[xml(tag = "c:showCatName")]
    CShowCatName(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ShowCategoryName,
    ),
    #[xml(tag = "c:showSerName")]
    CShowSerName(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ShowSeriesName,
    ),
    #[xml(tag = "c:showPercent")]
    CShowPercent(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ShowPercent,
    ),
    #[xml(tag = "c:showBubbleSize")]
    CShowBubbleSize(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ShowBubbleSize,
    ),
    #[xml(tag = "c:separator")]
    CSeparator(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Separator,
    ),
    #[xml(tag = "c:extLst")]
    CExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DLblExtensionList,
    ),
}
/// Defines the CategoryFilterExceptions Class.
/// When the object is serialized out as xml, it's qualified name is c16:categoryFilterExceptions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:categoryFilterExceptions")]
pub struct CategoryFilterExceptions {
    /// _
    #[xml(child = "c16:categoryFilterException")]
    pub c16_category_filter_exception: Vec<CategoryFilterException>,
}
/// Defines the PivotOptions16 Class.
/// When the object is serialized out as xml, it's qualified name is c16:pivotOptions16.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:pivotOptions16")]
pub struct PivotOptions16 {
    /// _
    #[xml(child = "c16:showExpandCollapseFieldButtons")]
    pub boolean_false: Option<BooleanFalse>,
}
/// Defines the ChartDataPointUniqueIDMap Class.
/// When the object is serialized out as xml, it's qualified name is c16:datapointuniqueidmap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:datapointuniqueidmap")]
pub struct ChartDataPointUniqueIdMap {
    /// _
    #[xml(child = "c16:ptentry")]
    pub c16_ptentry: Vec<ChartDataPointUniqueIdMapEntry>,
}
/// Defines the UniqueIdChartUniqueID Class.
/// When the object is serialized out as xml, it's qualified name is c16:uniqueId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:uniqueId")]
pub struct UniqueIdChartUniqueId {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Defines the UniqueID Class.
/// When the object is serialized out as xml, it's qualified name is c16:uniqueID.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:uniqueID")]
pub struct UniqueId {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Defines the UniqueIDChart Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct UniqueIdChart {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Defines the CategoryFilterException Class.
/// When the object is serialized out as xml, it's qualified name is c16:categoryFilterException.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:categoryFilterException")]
pub struct CategoryFilterException {
    /// _
    #[xml(child = "c16:uniqueId")]
    pub unique_id_chart_unique_id: UniqueIdChartUniqueId,
    /// _
    #[xml(child = "c16:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "c16:explosion")]
    pub unsigned_integer_type: Option<UnsignedIntegerType>,
    /// _
    #[xml(child = "c16:invertIfNegative")]
    pub invert_if_negative_boolean: Option<InvertIfNegativeBoolean>,
    /// _
    #[xml(child = "c16:bubble3D")]
    pub bubble3_d_boolean: Option<Bubble3DBoolean>,
    /// _
    #[xml(child = "c16:marker")]
    pub marker: Option<Marker>,
    /// _
    #[xml(child = "c16:dLbl")]
    pub d_lbl: Option<DLbl>,
}
/// Defines the NumberDataType Class.
/// When the object is serialized out as xml, it's qualified name is c16:numCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:numCache")]
pub struct NumberDataType {
    ///Format Code
    #[xml(child = "c:formatCode")]
    pub format_code: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::FormatCode,
    >,
    ///Point Count
    #[xml(child = "c:ptCount")]
    pub point_count: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PointCount,
    >,
    /// _
    #[xml(child = "c:pt")]
    pub c_pt: Vec<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::NumericPoint,
    >,
    /// _
    #[xml(child = "c:extLst")]
    pub c_ext_lst: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ExtensionList,
    >,
}
/// Defines the NumFilteredLiteralCache Class.
/// When the object is serialized out as xml, it's qualified name is c16:filteredLitCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:filteredLitCache")]
pub struct NumFilteredLiteralCache {
    /// _
    #[xml(child = "c16:numCache")]
    pub number_data_type: NumberDataType,
}
/// Defines the StringDataType Class.
/// When the object is serialized out as xml, it's qualified name is c16:strCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:strCache")]
pub struct StringDataType {
    /// _
    #[xml(child = "c:ptCount")]
    pub point_count: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PointCount,
    >,
    /// _
    #[xml(child = "c:pt")]
    pub c_pt: Vec<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::StringPoint,
    >,
    /// _
    #[xml(child = "c:extLst")]
    pub c_ext_lst: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::StrDataExtensionList,
    >,
}
/// Defines the StrFilteredLiteralCache Class.
/// When the object is serialized out as xml, it's qualified name is c16:filteredLitCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:filteredLitCache")]
pub struct StrFilteredLiteralCache {
    /// _
    #[xml(child = "c16:strCache")]
    pub string_data_type: StringDataType,
}
/// Defines the MultiLvlStrData Class.
/// When the object is serialized out as xml, it's qualified name is c16:multiLvlStrCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:multiLvlStrCache")]
pub struct MultiLvlStrData {
    /// _
    #[xml(child = "c:ptCount")]
    pub point_count: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PointCount,
    >,
    /// _
    #[xml(child = "c:lvl")]
    pub c_lvl: Vec<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Level,
    >,
    /// _
    #[xml(child = "c:extLst")]
    pub c_ext_lst: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ExtensionList,
    >,
}
/// Defines the MultiLvlStrFilteredLiteralCache Class.
/// When the object is serialized out as xml, it's qualified name is c16:filteredLitCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:filteredLitCache")]
pub struct MultiLvlStrFilteredLiteralCache {
    /// _
    #[xml(child = "c16:multiLvlStrCache")]
    pub multi_lvl_str_data: MultiLvlStrData,
}
/// Defines the LiteralDataChart Class.
/// When the object is serialized out as xml, it's qualified name is c16:literalDataChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:literalDataChart")]
pub struct LiteralDataChart {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: bool,
}
/// Defines the BooleanFalse Class.
/// When the object is serialized out as xml, it's qualified name is c16:showExpandCollapseFieldButtons.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:showExpandCollapseFieldButtons")]
pub struct BooleanFalse {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the XsdunsignedInt Class.
/// When the object is serialized out as xml, it's qualified name is c16:ptidx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:ptidx")]
pub struct XsdunsignedInt {
    #[xml(text)]
    pub child: i32,
}
/// Defines the ChartDataPointUniqueIDMapEntry Class.
/// When the object is serialized out as xml, it's qualified name is c16:ptentry.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16:ptentry")]
pub struct ChartDataPointUniqueIdMapEntry {
    /// _
    #[xml(child = "c16:ptidx")]
    pub xsdunsigned_int: XsdunsignedInt,
    /// _
    #[xml(child = "c16:uniqueID")]
    pub unique_id: UniqueId,
}
