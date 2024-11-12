#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FormulaDirection {
    #[default]
    Col,
    Row,
}
crate::__string_enum! {
    FormulaDirection { Col = "col", Row = "row", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StringDimensionType {
    #[default]
    Cat,
    ColorStr,
    EntityId,
}
crate::__string_enum! {
    StringDimensionType { Cat = "cat", ColorStr = "colorStr", EntityId = "entityId", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum NumericDimensionType {
    #[default]
    Val,
    X,
    Y,
    Size,
    ColorVal,
}
crate::__string_enum! {
    NumericDimensionType { Val = "val", X = "x", Y = "y", Size = "size", ColorVal =
    "colorVal", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SidePos {
    #[default]
    L,
    T,
    R,
    B,
}
crate::__string_enum! {
    SidePos { L = "l", T = "t", R = "r", B = "b", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PosAlign {
    #[default]
    Min,
    Ctr,
    Max,
}
crate::__string_enum! {
    PosAlign { Min = "min", Ctr = "ctr", Max = "max", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AxisUnit {
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
    Percentage,
}
crate::__string_enum! {
    AxisUnit { Hundreds = "hundreds", Thousands = "thousands", TenThousands =
    "tenThousands", HundredThousands = "hundredThousands", Millions = "millions",
    TenMillions = "tenMillions", HundredMillions = "hundredMillions", Billions =
    "billions", Trillions = "trillions", Percentage = "percentage", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TickMarksType {
    #[default]
    In,
    Out,
    Cross,
    None,
}
crate::__string_enum! {
    TickMarksType { In = "in", Out = "out", Cross = "cross", None = "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SeriesLayout {
    #[default]
    BoxWhisker,
    ClusteredColumn,
    Funnel,
    ParetoLine,
    RegionMap,
    Sunburst,
    Treemap,
    Waterfall,
}
crate::__string_enum! {
    SeriesLayout { BoxWhisker = "boxWhisker", ClusteredColumn = "clusteredColumn", Funnel
    = "funnel", ParetoLine = "paretoLine", RegionMap = "regionMap", Sunburst =
    "sunburst", Treemap = "treemap", Waterfall = "waterfall", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ParentLabelLayoutVal {
    #[default]
    None,
    Banner,
    Overlapping,
}
crate::__string_enum! {
    ParentLabelLayoutVal { None = "none", Banner = "banner", Overlapping = "overlapping",
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RegionLabelLayoutEnum {
    #[default]
    None,
    BestFitOnly,
    ShowAll,
}
crate::__string_enum! {
    RegionLabelLayoutEnum { None = "none", BestFitOnly = "bestFitOnly", ShowAll =
    "showAll", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum IntervalClosedSide {
    #[default]
    L,
    R,
}
crate::__string_enum! {
    IntervalClosedSide { L = "l", R = "r", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EntityTypeEnum {
    #[default]
    Address,
    AdminDistrict,
    AdminDistrict2,
    AdminDistrict3,
    Continent,
    CountryRegion,
    Locality,
    Ocean,
    Planet,
    PostalCode,
    Region,
    Unsupported,
}
crate::__string_enum! {
    EntityTypeEnum { Address = "address", AdminDistrict = "adminDistrict", AdminDistrict2
    = "adminDistrict2", AdminDistrict3 = "adminDistrict3", Continent = "continent",
    CountryRegion = "countryRegion", Locality = "locality", Ocean = "ocean", Planet =
    "planet", PostalCode = "postalCode", Region = "region", Unsupported = "unsupported",
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GeoProjectionType {
    #[default]
    Mercator,
    Miller,
    Robinson,
    Albers,
}
crate::__string_enum! {
    GeoProjectionType { Mercator = "mercator", Miller = "miller", Robinson = "robinson",
    Albers = "albers", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GeoMappingLevel {
    #[default]
    DataOnly,
    PostalCode,
    County,
    State,
    CountryRegion,
    CountryRegionList,
    World,
}
crate::__string_enum! {
    GeoMappingLevel { DataOnly = "dataOnly", PostalCode = "postalCode", County =
    "county", State = "state", CountryRegion = "countryRegion", CountryRegionList =
    "countryRegionList", World = "world", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum QuartileMethod {
    #[default]
    Inclusive,
    Exclusive,
}
crate::__string_enum! {
    QuartileMethod { Inclusive = "inclusive", Exclusive = "exclusive", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataLabelPos {
    #[default]
    BestFit,
    B,
    Ctr,
    InBase,
    InEnd,
    L,
    OutEnd,
    R,
    T,
}
crate::__string_enum! {
    DataLabelPos { BestFit = "bestFit", B = "b", Ctr = "ctr", InBase = "inBase", InEnd =
    "inEnd", L = "l", OutEnd = "outEnd", R = "r", T = "t", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PageOrientation {
    #[default]
    Default,
    Portrait,
    Landscape,
}
crate::__string_enum! {
    PageOrientation { Default = "default", Portrait = "portrait", Landscape =
    "landscape", }
}
/// Defines the ChartSpace Class.
/// When the object is serialized out as xml, it's qualified name is cx:chartSpace.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:chartSpace")]
pub struct ChartSpace {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "cx:chartData")]
    pub chart_data: ChartData,
    /// _
    #[xml(child = "cx:chart")]
    pub chart: Chart,
    /// _
    #[xml(child = "cx:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cx:txPr")]
    pub tx_pr_text_body: Option<TxPrTextBody>,
    /// _
    #[xml(child = "cx:clrMapOvr")]
    pub color_mapping_type: Option<ColorMappingType>,
    /// _
    #[xml(child = "cx:fmtOvrs")]
    pub format_overrides: Option<FormatOverrides>,
    /// _
    #[xml(child = "cx:printSettings")]
    pub print_settings: Option<PrintSettings>,
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the RelId Class.
/// When the object is serialized out as xml, it's qualified name is cx:chart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:chart")]
pub struct RelId {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the Openxmlsdk_49BECFFA_3B03_4D13_8272_D6CCB22579E3XsdunsignedInt Class.
/// When the object is serialized out as xml, it's qualified name is cx:openxmlsdk_49BECFFA_3B03_4D13_8272_D6CCB22579E3.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:openxmlsdk_49BECFFA_3B03_4D13_8272_D6CCB22579E3")]
pub struct Openxmlsdk49becffa3b034d138272D6ccb22579e3XsdunsignedInt {
    #[xml(text)]
    pub child: i32,
}
/// Defines the BinCountXsdunsignedInt Class.
/// When the object is serialized out as xml, it's qualified name is cx:binCount.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:binCount")]
pub struct BinCountXsdunsignedInt {
    #[xml(text)]
    pub child: i32,
}
/// Defines the Extension2 Class.
/// When the object is serialized out as xml, it's qualified name is cx:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:ext")]
pub struct Extension2 {
    /// uri
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: Option<String>,
}
/// Defines the MinColorSolidColorFillProperties Class.
/// When the object is serialized out as xml, it's qualified name is cx:minColor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:minColor")]
pub struct MinColorSolidColorFillProperties {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<MinColorSolidColorFillPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MinColorSolidColorFillPropertiesChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    ),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    ),
    #[xml(tag = "a:hslClr")]
    AHslClr(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor,
    ),
    #[xml(tag = "a:prstClr")]
    APrstClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor,
    ),
}
/// Defines the MidColorSolidColorFillProperties Class.
/// When the object is serialized out as xml, it's qualified name is cx:midColor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:midColor")]
pub struct MidColorSolidColorFillProperties {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<MidColorSolidColorFillPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MidColorSolidColorFillPropertiesChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    ),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    ),
    #[xml(tag = "a:hslClr")]
    AHslClr(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor,
    ),
    #[xml(tag = "a:prstClr")]
    APrstClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor,
    ),
}
/// Defines the MaxColorSolidColorFillProperties Class.
/// When the object is serialized out as xml, it's qualified name is cx:maxColor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:maxColor")]
pub struct MaxColorSolidColorFillProperties {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<MaxColorSolidColorFillPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MaxColorSolidColorFillPropertiesChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    ),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    ),
    #[xml(tag = "a:hslClr")]
    AHslClr(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor,
    ),
    #[xml(tag = "a:prstClr")]
    APrstClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor,
    ),
}
/// Defines the OpenXmlSolidColorFillPropertiesElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlSolidColorFillPropertiesElement {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<OpenXmlSolidColorFillPropertiesElementChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OpenXmlSolidColorFillPropertiesElementChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    ),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    ),
    #[xml(tag = "a:hslClr")]
    AHslClr(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor,
    ),
    #[xml(tag = "a:prstClr")]
    APrstClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor,
    ),
}
/// Defines the ChartStringValue Class.
/// When the object is serialized out as xml, it's qualified name is cx:pt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:pt")]
pub struct ChartStringValue {
    /// idx
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
    #[xml(text)]
    pub child: String,
}
/// Defines the Formula Class.
/// When the object is serialized out as xml, it's qualified name is cx:f.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:f")]
pub struct Formula {
    /// dir
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub dir: Option<FormulaDirection>,
    #[xml(text)]
    pub child: String,
}
/// Defines the NfFormula Class.
/// When the object is serialized out as xml, it's qualified name is cx:nf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:nf")]
pub struct NfFormula {
    /// dir
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub dir: Option<FormulaDirection>,
    #[xml(text)]
    pub child: String,
}
/// Defines the OpenXmlFormulaElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlFormulaElement {
    /// dir
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub dir: Option<FormulaDirection>,
    #[xml(text)]
    pub child: String,
}
/// Defines the StringLevel Class.
/// When the object is serialized out as xml, it's qualified name is cx:lvl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:lvl")]
pub struct StringLevel {
    /// ptCount
    /// Represents the following attribute in the schema: :ptCount
    #[xml(attr = "ptCount")]
    pub pt_count: i32,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// _
    #[xml(child = "cx:pt")]
    pub cx_pt: Vec<ChartStringValue>,
}
/// Defines the NumericValue Class.
/// When the object is serialized out as xml, it's qualified name is cx:pt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:pt")]
pub struct NumericValue {
    /// idx
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub idx: i32,
    #[xml(text)]
    pub child: f64,
}
/// Defines the NumericLevel Class.
/// When the object is serialized out as xml, it's qualified name is cx:lvl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:lvl")]
pub struct NumericLevel {
    /// ptCount
    /// Represents the following attribute in the schema: :ptCount
    #[xml(attr = "ptCount")]
    pub pt_count: i32,
    /// formatCode
    /// Represents the following attribute in the schema: :formatCode
    #[xml(attr = "formatCode")]
    pub format_code: Option<String>,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// _
    #[xml(child = "cx:pt")]
    pub cx_pt: Vec<NumericValue>,
}
/// Defines the NumericDimension Class.
/// When the object is serialized out as xml, it's qualified name is cx:numDim.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:numDim")]
pub struct NumericDimension {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: NumericDimensionType,
    #[xml(child = "cx:f", child = "cx:nf", child = "cx:lvl")]
    pub children: Vec<NumericDimensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NumericDimensionChildChoice {
    #[xml(tag = "cx:f")]
    CxF(Formula),
    #[xml(tag = "cx:nf")]
    CxNf(NfFormula),
    #[xml(tag = "cx:lvl")]
    CxLvl(NumericLevel),
}
/// Defines the StringDimension Class.
/// When the object is serialized out as xml, it's qualified name is cx:strDim.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:strDim")]
pub struct StringDimension {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: StringDimensionType,
    #[xml(child = "cx:f", child = "cx:nf", child = "cx:lvl")]
    pub children: Vec<StringDimensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum StringDimensionChildChoice {
    #[xml(tag = "cx:f")]
    CxF(Formula),
    #[xml(tag = "cx:nf")]
    CxNf(NfFormula),
    #[xml(tag = "cx:lvl")]
    CxLvl(StringLevel),
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is cx:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:extLst")]
pub struct ExtensionList {
    /// _
    #[xml(child = "cx:ext")]
    pub cx_ext: Vec<Extension2>,
}
/// Defines the ExternalData Class.
/// When the object is serialized out as xml, it's qualified name is cx:externalData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:externalData")]
pub struct ExternalData {
    /// RelId of the relationship for the external data
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
    /// True if the external link should automatically update
    /// Represents the following attribute in the schema: cx:autoUpdate
    #[xml(attr = "cx:autoUpdate")]
    pub cx_auto_update: Option<bool>,
}
/// Defines the Data Class.
/// When the object is serialized out as xml, it's qualified name is cx:data.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:data")]
pub struct Data {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    #[xml(child = "cx:numDim", child = "cx:strDim", child = "cx:extLst")]
    pub children: Vec<DataChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DataChildChoice {
    #[xml(tag = "cx:numDim")]
    CxNumDim(NumericDimension),
    #[xml(tag = "cx:strDim")]
    CxStrDim(StringDimension),
    #[xml(tag = "cx:extLst")]
    CxExtLst(ExtensionList),
}
/// Defines the VXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is cx:v.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:v")]
pub struct VXsdstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the CopyrightXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is cx:copyright.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:copyright")]
pub struct CopyrightXsdstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the SeparatorXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is cx:separator.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:separator")]
pub struct SeparatorXsdstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the OddHeaderXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is cx:oddHeader.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:oddHeader")]
pub struct OddHeaderXsdstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the OddFooterXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is cx:oddFooter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:oddFooter")]
pub struct OddFooterXsdstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the EvenHeaderXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is cx:evenHeader.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:evenHeader")]
pub struct EvenHeaderXsdstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the EvenFooterXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is cx:evenFooter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:evenFooter")]
pub struct EvenFooterXsdstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the FirstHeaderXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is cx:firstHeader.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:firstHeader")]
pub struct FirstHeaderXsdstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the FirstFooterXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is cx:firstFooter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:firstFooter")]
pub struct FirstFooterXsdstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the TextData Class.
/// When the object is serialized out as xml, it's qualified name is cx:txData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:txData")]
pub struct TextData {
    #[xml(child = "cx:f", child = "cx:v")]
    pub children: Vec<TextDataChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextDataChildChoice {
    #[xml(tag = "cx:f")]
    CxF(Formula),
    #[xml(tag = "cx:v")]
    CxV(VXsdstring),
}
/// Defines the RichTextBody Class.
/// When the object is serialized out as xml, it's qualified name is cx:rich.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:rich")]
pub struct RichTextBody {
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
/// Defines the TxPrTextBody Class.
/// When the object is serialized out as xml, it's qualified name is cx:txPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:txPr")]
pub struct TxPrTextBody {
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
/// Defines the Text Class.
/// When the object is serialized out as xml, it's qualified name is cx:tx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:tx")]
pub struct Text {
    #[xml(child = "cx:txData", child = "cx:rich")]
    pub children: Vec<TextChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextChildChoice {
    #[xml(tag = "cx:txData")]
    CxTxData(TextData),
    #[xml(tag = "cx:rich")]
    CxRich(RichTextBody),
}
/// Defines the ShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is cx:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:spPr")]
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
/// Defines the AxisUnitsLabel Class.
/// When the object is serialized out as xml, it's qualified name is cx:unitsLabel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:unitsLabel")]
pub struct AxisUnitsLabel {
    /// _
    #[xml(child = "cx:tx")]
    pub text: Option<Text>,
    /// _
    #[xml(child = "cx:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cx:txPr")]
    pub tx_pr_text_body: Option<TxPrTextBody>,
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the CategoryAxisScaling Class.
/// When the object is serialized out as xml, it's qualified name is cx:catScaling.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:catScaling")]
pub struct CategoryAxisScaling {
    /// gapWidth
    /// Represents the following attribute in the schema: :gapWidth
    #[xml(attr = "gapWidth")]
    pub gap_width: Option<String>,
}
/// Defines the ValueAxisScaling Class.
/// When the object is serialized out as xml, it's qualified name is cx:valScaling.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:valScaling")]
pub struct ValueAxisScaling {
    /// max
    /// Represents the following attribute in the schema: :max
    #[xml(attr = "max")]
    pub max: Option<String>,
    /// min
    /// Represents the following attribute in the schema: :min
    #[xml(attr = "min")]
    pub min: Option<String>,
    /// majorUnit
    /// Represents the following attribute in the schema: :majorUnit
    #[xml(attr = "majorUnit")]
    pub major_unit: Option<String>,
    /// minorUnit
    /// Represents the following attribute in the schema: :minorUnit
    #[xml(attr = "minorUnit")]
    pub minor_unit: Option<String>,
}
/// Defines the AxisTitle Class.
/// When the object is serialized out as xml, it's qualified name is cx:title.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:title")]
pub struct AxisTitle {
    /// _
    #[xml(child = "cx:tx")]
    pub text: Option<Text>,
    /// _
    #[xml(child = "cx:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cx:txPr")]
    pub tx_pr_text_body: Option<TxPrTextBody>,
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the AxisUnits Class.
/// When the object is serialized out as xml, it's qualified name is cx:units.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:units")]
pub struct AxisUnits {
    /// unit
    /// Represents the following attribute in the schema: :unit
    #[xml(attr = "unit")]
    pub unit: Option<AxisUnit>,
    /// _
    #[xml(child = "cx:unitsLabel")]
    pub axis_units_label: Option<AxisUnitsLabel>,
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the MajorGridlinesGridlines Class.
/// When the object is serialized out as xml, it's qualified name is cx:majorGridlines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:majorGridlines")]
pub struct MajorGridlinesGridlines {
    /// _
    #[xml(child = "cx:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the MinorGridlinesGridlines Class.
/// When the object is serialized out as xml, it's qualified name is cx:minorGridlines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:minorGridlines")]
pub struct MinorGridlinesGridlines {
    /// _
    #[xml(child = "cx:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the OpenXmlGridlinesElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlGridlinesElement {}
/// Defines the MajorTickMarksTickMarks Class.
/// When the object is serialized out as xml, it's qualified name is cx:majorTickMarks.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:majorTickMarks")]
pub struct MajorTickMarksTickMarks {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<TickMarksType>,
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the MinorTickMarksTickMarks Class.
/// When the object is serialized out as xml, it's qualified name is cx:minorTickMarks.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:minorTickMarks")]
pub struct MinorTickMarksTickMarks {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<TickMarksType>,
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the OpenXmlTickMarksElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlTickMarksElement {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<TickMarksType>,
}
/// Defines the TickLabels Class.
/// When the object is serialized out as xml, it's qualified name is cx:tickLabels.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:tickLabels")]
pub struct TickLabels {
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the NumberFormat Class.
/// When the object is serialized out as xml, it's qualified name is cx:numFmt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:numFmt")]
pub struct NumberFormat {
    /// formatCode
    /// Represents the following attribute in the schema: :formatCode
    #[xml(attr = "formatCode")]
    pub format_code: String,
    /// sourceLinked
    /// Represents the following attribute in the schema: :sourceLinked
    #[xml(attr = "sourceLinked")]
    pub source_linked: Option<bool>,
}
/// Defines the Xsddouble Class.
/// When the object is serialized out as xml, it's qualified name is cx:binSize.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:binSize")]
pub struct Xsddouble {
    #[xml(text)]
    pub child: f64,
}
/// Defines the Address Class.
/// When the object is serialized out as xml, it's qualified name is cx:address.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:address")]
pub struct Address {
    /// address1
    /// Represents the following attribute in the schema: :address1
    #[xml(attr = "address1")]
    pub address1: Option<String>,
    /// countryRegion
    /// Represents the following attribute in the schema: :countryRegion
    #[xml(attr = "countryRegion")]
    pub country_region: Option<String>,
    /// adminDistrict1
    /// Represents the following attribute in the schema: :adminDistrict1
    #[xml(attr = "adminDistrict1")]
    pub admin_district1: Option<String>,
    /// adminDistrict2
    /// Represents the following attribute in the schema: :adminDistrict2
    #[xml(attr = "adminDistrict2")]
    pub admin_district2: Option<String>,
    /// postalCode
    /// Represents the following attribute in the schema: :postalCode
    #[xml(attr = "postalCode")]
    pub postal_code: Option<String>,
    /// locality
    /// Represents the following attribute in the schema: :locality
    #[xml(attr = "locality")]
    pub locality: Option<String>,
    /// isoCountryCode
    /// Represents the following attribute in the schema: :isoCountryCode
    #[xml(attr = "isoCountryCode")]
    pub iso_country_code: Option<String>,
}
/// Defines the GeoLocation Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoLocation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoLocation")]
pub struct GeoLocation {
    /// latitude
    /// Represents the following attribute in the schema: :latitude
    #[xml(attr = "latitude")]
    pub latitude: Option<f64>,
    /// longitude
    /// Represents the following attribute in the schema: :longitude
    #[xml(attr = "longitude")]
    pub longitude: Option<f64>,
    /// entityName
    /// Represents the following attribute in the schema: :entityName
    #[xml(attr = "entityName")]
    pub entity_name: String,
    /// entityType
    /// Represents the following attribute in the schema: :entityType
    #[xml(attr = "entityType")]
    pub entity_type: EntityTypeEnum,
    /// _
    #[xml(child = "cx:address")]
    pub address: Option<Address>,
}
/// Defines the GeoLocationQuery Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoLocationQuery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoLocationQuery")]
pub struct GeoLocationQuery {
    /// countryRegion
    /// Represents the following attribute in the schema: :countryRegion
    #[xml(attr = "countryRegion")]
    pub country_region: Option<String>,
    /// adminDistrict1
    /// Represents the following attribute in the schema: :adminDistrict1
    #[xml(attr = "adminDistrict1")]
    pub admin_district1: Option<String>,
    /// adminDistrict2
    /// Represents the following attribute in the schema: :adminDistrict2
    #[xml(attr = "adminDistrict2")]
    pub admin_district2: Option<String>,
    /// postalCode
    /// Represents the following attribute in the schema: :postalCode
    #[xml(attr = "postalCode")]
    pub postal_code: Option<String>,
    /// entityType
    /// Represents the following attribute in the schema: :entityType
    #[xml(attr = "entityType")]
    pub entity_type: EntityTypeEnum,
}
/// Defines the GeoLocations Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoLocations.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoLocations")]
pub struct GeoLocations {
    /// _
    #[xml(child = "cx:geoLocation")]
    pub geo_location: Option<GeoLocation>,
}
/// Defines the GeoLocationQueryResult Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoLocationQueryResult.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoLocationQueryResult")]
pub struct GeoLocationQueryResult {
    /// _
    #[xml(child = "cx:geoLocationQuery")]
    pub geo_location_query: Option<GeoLocationQuery>,
    /// _
    #[xml(child = "cx:geoLocations")]
    pub geo_locations: Option<GeoLocations>,
}
/// Defines the GeoPolygon Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoPolygon.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoPolygon")]
pub struct GeoPolygon {
    /// polygonId
    /// Represents the following attribute in the schema: :polygonId
    #[xml(attr = "polygonId")]
    pub polygon_id: String,
    /// numPoints
    /// Represents the following attribute in the schema: :numPoints
    #[xml(attr = "numPoints")]
    pub num_points: i32,
    /// pcaRings
    /// Represents the following attribute in the schema: :pcaRings
    #[xml(attr = "pcaRings")]
    pub pca_rings: String,
}
/// Defines the GeoPolygons Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoPolygons.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoPolygons")]
pub struct GeoPolygons {
    /// _
    #[xml(child = "cx:geoPolygon")]
    pub cx_geo_polygon: Vec<GeoPolygon>,
}
/// Defines the Copyrights Class.
/// When the object is serialized out as xml, it's qualified name is cx:copyrights.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:copyrights")]
pub struct Copyrights {
    /// _
    #[xml(child = "cx:copyright")]
    pub cx_copyright: Vec<CopyrightXsdstring>,
}
/// Defines the GeoDataEntityQuery Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoDataEntityQuery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoDataEntityQuery")]
pub struct GeoDataEntityQuery {
    /// entityType
    /// Represents the following attribute in the schema: :entityType
    #[xml(attr = "entityType")]
    pub entity_type: EntityTypeEnum,
    /// entityId
    /// Represents the following attribute in the schema: :entityId
    #[xml(attr = "entityId")]
    pub entity_id: String,
}
/// Defines the GeoData Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoData")]
pub struct GeoData {
    /// entityName
    /// Represents the following attribute in the schema: :entityName
    #[xml(attr = "entityName")]
    pub entity_name: String,
    /// entityId
    /// Represents the following attribute in the schema: :entityId
    #[xml(attr = "entityId")]
    pub entity_id: String,
    /// east
    /// Represents the following attribute in the schema: :east
    #[xml(attr = "east")]
    pub east: f64,
    /// west
    /// Represents the following attribute in the schema: :west
    #[xml(attr = "west")]
    pub west: f64,
    /// north
    /// Represents the following attribute in the schema: :north
    #[xml(attr = "north")]
    pub north: f64,
    /// south
    /// Represents the following attribute in the schema: :south
    #[xml(attr = "south")]
    pub south: f64,
    /// _
    #[xml(child = "cx:geoPolygons")]
    pub geo_polygons: Option<GeoPolygons>,
    /// _
    #[xml(child = "cx:copyrights")]
    pub copyrights: Option<Copyrights>,
}
/// Defines the GeoDataEntityQueryResult Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoDataEntityQueryResult.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoDataEntityQueryResult")]
pub struct GeoDataEntityQueryResult {
    /// _
    #[xml(child = "cx:geoDataEntityQuery")]
    pub geo_data_entity_query: Option<GeoDataEntityQuery>,
    /// _
    #[xml(child = "cx:geoData")]
    pub geo_data: Option<GeoData>,
}
/// Defines the GeoDataPointQuery Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoDataPointQuery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoDataPointQuery")]
pub struct GeoDataPointQuery {
    /// entityType
    /// Represents the following attribute in the schema: :entityType
    #[xml(attr = "entityType")]
    pub entity_type: EntityTypeEnum,
    /// latitude
    /// Represents the following attribute in the schema: :latitude
    #[xml(attr = "latitude")]
    pub latitude: f64,
    /// longitude
    /// Represents the following attribute in the schema: :longitude
    #[xml(attr = "longitude")]
    pub longitude: f64,
}
/// Defines the GeoDataPointToEntityQuery Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoDataPointToEntityQuery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoDataPointToEntityQuery")]
pub struct GeoDataPointToEntityQuery {
    /// entityType
    /// Represents the following attribute in the schema: :entityType
    #[xml(attr = "entityType")]
    pub entity_type: EntityTypeEnum,
    /// entityId
    /// Represents the following attribute in the schema: :entityId
    #[xml(attr = "entityId")]
    pub entity_id: String,
}
/// Defines the GeoDataPointToEntityQueryResult Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoDataPointToEntityQueryResult.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoDataPointToEntityQueryResult")]
pub struct GeoDataPointToEntityQueryResult {
    /// _
    #[xml(child = "cx:geoDataPointQuery")]
    pub geo_data_point_query: Option<GeoDataPointQuery>,
    /// _
    #[xml(child = "cx:geoDataPointToEntityQuery")]
    pub geo_data_point_to_entity_query: Option<GeoDataPointToEntityQuery>,
}
/// Defines the EntityType Class.
/// When the object is serialized out as xml, it's qualified name is cx:entityType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:entityType")]
pub struct EntityType {
    #[xml(attr = "cx:entityType")]
    pub child: Option<EntityTypeEnum>,
}
/// Defines the GeoChildTypes Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoChildTypes.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoChildTypes")]
pub struct GeoChildTypes {
    /// _
    #[xml(child = "cx:entityType")]
    pub cx_entity_type: Vec<EntityType>,
}
/// Defines the GeoHierarchyEntity Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoHierarchyEntity.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoHierarchyEntity")]
pub struct GeoHierarchyEntity {
    /// entityName
    /// Represents the following attribute in the schema: :entityName
    #[xml(attr = "entityName")]
    pub entity_name: String,
    /// entityId
    /// Represents the following attribute in the schema: :entityId
    #[xml(attr = "entityId")]
    pub entity_id: String,
    /// entityType
    /// Represents the following attribute in the schema: :entityType
    #[xml(attr = "entityType")]
    pub entity_type: EntityTypeEnum,
}
/// Defines the GeoChildEntitiesQuery Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoChildEntitiesQuery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoChildEntitiesQuery")]
pub struct GeoChildEntitiesQuery {
    /// entityId
    /// Represents the following attribute in the schema: :entityId
    #[xml(attr = "entityId")]
    pub entity_id: String,
    /// _
    #[xml(child = "cx:geoChildTypes")]
    pub geo_child_types: Option<GeoChildTypes>,
}
/// Defines the GeoChildEntities Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoChildEntities.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoChildEntities")]
pub struct GeoChildEntities {
    /// _
    #[xml(child = "cx:geoHierarchyEntity")]
    pub cx_geo_hierarchy_entity: Vec<GeoHierarchyEntity>,
}
/// Defines the GeoChildEntitiesQueryResult Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoChildEntitiesQueryResult.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoChildEntitiesQueryResult")]
pub struct GeoChildEntitiesQueryResult {
    /// _
    #[xml(child = "cx:geoChildEntitiesQuery")]
    pub geo_child_entities_query: Option<GeoChildEntitiesQuery>,
    /// _
    #[xml(child = "cx:geoChildEntities")]
    pub geo_child_entities: Option<GeoChildEntities>,
}
/// Defines the GeoParentEntitiesQuery Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoParentEntitiesQuery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoParentEntitiesQuery")]
pub struct GeoParentEntitiesQuery {
    /// entityId
    /// Represents the following attribute in the schema: :entityId
    #[xml(attr = "entityId")]
    pub entity_id: String,
}
/// Defines the GeoEntity Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoEntity.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoEntity")]
pub struct GeoEntity {
    /// entityName
    /// Represents the following attribute in the schema: :entityName
    #[xml(attr = "entityName")]
    pub entity_name: String,
    /// entityType
    /// Represents the following attribute in the schema: :entityType
    #[xml(attr = "entityType")]
    pub entity_type: EntityTypeEnum,
}
/// Defines the GeoParentEntity Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoParentEntity.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoParentEntity")]
pub struct GeoParentEntity {
    /// entityId
    /// Represents the following attribute in the schema: :entityId
    #[xml(attr = "entityId")]
    pub entity_id: String,
}
/// Defines the GeoParentEntitiesQueryResult Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoParentEntitiesQueryResult.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoParentEntitiesQueryResult")]
pub struct GeoParentEntitiesQueryResult {
    /// _
    #[xml(child = "cx:geoParentEntitiesQuery")]
    pub geo_parent_entities_query: GeoParentEntitiesQuery,
    /// _
    #[xml(child = "cx:geoEntity")]
    pub geo_entity: Option<GeoEntity>,
    /// _
    #[xml(child = "cx:geoParentEntity")]
    pub geo_parent_entity: Option<GeoParentEntity>,
}
/// Defines the GeoLocationQueryResults Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoLocationQueryResults.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoLocationQueryResults")]
pub struct GeoLocationQueryResults {
    /// _
    #[xml(child = "cx:geoLocationQueryResult")]
    pub cx_geo_location_query_result: Vec<GeoLocationQueryResult>,
}
/// Defines the GeoDataEntityQueryResults Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoDataEntityQueryResults.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoDataEntityQueryResults")]
pub struct GeoDataEntityQueryResults {
    /// _
    #[xml(child = "cx:geoDataEntityQueryResult")]
    pub cx_geo_data_entity_query_result: Vec<GeoDataEntityQueryResult>,
}
/// Defines the GeoDataPointToEntityQueryResults Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoDataPointToEntityQueryResults.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoDataPointToEntityQueryResults")]
pub struct GeoDataPointToEntityQueryResults {
    /// _
    #[xml(child = "cx:geoDataPointToEntityQueryResult")]
    pub cx_geo_data_point_to_entity_query_result: Vec<GeoDataPointToEntityQueryResult>,
}
/// Defines the GeoChildEntitiesQueryResults Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoChildEntitiesQueryResults.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoChildEntitiesQueryResults")]
pub struct GeoChildEntitiesQueryResults {
    /// _
    #[xml(child = "cx:geoChildEntitiesQueryResult")]
    pub cx_geo_child_entities_query_result: Vec<GeoChildEntitiesQueryResult>,
}
/// Defines the GeoParentEntitiesQueryResults Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoParentEntitiesQueryResults.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoParentEntitiesQueryResults")]
pub struct GeoParentEntitiesQueryResults {
    /// _
    #[xml(child = "cx:geoParentEntitiesQueryResult")]
    pub cx_geo_parent_entities_query_result: Vec<GeoParentEntitiesQueryResult>,
}
/// Defines the Xsdbase64Binary Class.
/// When the object is serialized out as xml, it's qualified name is cx:binary.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:binary")]
pub struct Xsdbase64Binary {
    #[xml(text)]
    pub child: String,
}
/// Defines the Clear Class.
/// When the object is serialized out as xml, it's qualified name is cx:clear.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:clear")]
pub struct Clear {
    /// _
    #[xml(child = "cx:geoLocationQueryResults")]
    pub geo_location_query_results: Option<GeoLocationQueryResults>,
    /// _
    #[xml(child = "cx:geoDataEntityQueryResults")]
    pub geo_data_entity_query_results: Option<GeoDataEntityQueryResults>,
    /// _
    #[xml(child = "cx:geoDataPointToEntityQueryResults")]
    pub geo_data_point_to_entity_query_results: Option<GeoDataPointToEntityQueryResults>,
    /// _
    #[xml(child = "cx:geoChildEntitiesQueryResults")]
    pub geo_child_entities_query_results: Option<GeoChildEntitiesQueryResults>,
    /// _
    #[xml(child = "cx:geoParentEntitiesQueryResults")]
    pub geo_parent_entities_query_results: Option<GeoParentEntitiesQueryResults>,
}
/// Defines the GeoCache Class.
/// When the object is serialized out as xml, it's qualified name is cx:geoCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geoCache")]
pub struct GeoCache {
    /// provider
    /// Represents the following attribute in the schema: :provider
    #[xml(attr = "provider")]
    pub provider: String,
    #[xml(child = "cx:binary", child = "cx:clear")]
    pub children: Vec<GeoCacheChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GeoCacheChildChoice {
    #[xml(tag = "cx:binary")]
    CxBinary(Xsdbase64Binary),
    #[xml(tag = "cx:clear")]
    CxClear(Clear),
}
/// Defines the ParentLabelLayout Class.
/// When the object is serialized out as xml, it's qualified name is cx:parentLabelLayout.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:parentLabelLayout")]
pub struct ParentLabelLayout {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub parent_label_layout_val: ParentLabelLayoutVal,
}
/// Defines the RegionLabelLayout Class.
/// When the object is serialized out as xml, it's qualified name is cx:regionLabelLayout.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:regionLabelLayout")]
pub struct RegionLabelLayout {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: RegionLabelLayoutEnum,
}
/// Defines the SeriesElementVisibilities Class.
/// When the object is serialized out as xml, it's qualified name is cx:visibility.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:visibility")]
pub struct SeriesElementVisibilities {
    /// connectorLines
    /// Represents the following attribute in the schema: :connectorLines
    #[xml(attr = "connectorLines")]
    pub connector_lines: Option<bool>,
    /// meanLine
    /// Represents the following attribute in the schema: :meanLine
    #[xml(attr = "meanLine")]
    pub mean_line: Option<bool>,
    /// meanMarker
    /// Represents the following attribute in the schema: :meanMarker
    #[xml(attr = "meanMarker")]
    pub mean_marker: Option<bool>,
    /// nonoutliers
    /// Represents the following attribute in the schema: :nonoutliers
    #[xml(attr = "nonoutliers")]
    pub nonoutliers: Option<bool>,
    /// outliers
    /// Represents the following attribute in the schema: :outliers
    #[xml(attr = "outliers")]
    pub outliers: Option<bool>,
}
/// Defines the Aggregation Class.
/// When the object is serialized out as xml, it's qualified name is cx:aggregation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:aggregation")]
pub struct Aggregation {}
/// Defines the Binning Class.
/// When the object is serialized out as xml, it's qualified name is cx:binning.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:binning")]
pub struct Binning {
    /// intervalClosed
    /// Represents the following attribute in the schema: :intervalClosed
    #[xml(attr = "intervalClosed")]
    pub interval_closed: Option<IntervalClosedSide>,
    /// underflow
    /// Represents the following attribute in the schema: :underflow
    #[xml(attr = "underflow")]
    pub underflow: Option<String>,
    /// overflow
    /// Represents the following attribute in the schema: :overflow
    #[xml(attr = "overflow")]
    pub overflow: Option<String>,
    #[xml(child = "cx:binSize", child = "cx:binCount")]
    pub children: Vec<BinningChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BinningChildChoice {
    #[xml(tag = "cx:binSize")]
    CxBinSize(Xsddouble),
    #[xml(tag = "cx:binCount")]
    CxBinCount(BinCountXsdunsignedInt),
}
/// Defines the Geography Class.
/// When the object is serialized out as xml, it's qualified name is cx:geography.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:geography")]
pub struct Geography {
    /// projectionType
    /// Represents the following attribute in the schema: :projectionType
    #[xml(attr = "projectionType")]
    pub projection_type: Option<GeoProjectionType>,
    /// viewedRegionType
    /// Represents the following attribute in the schema: :viewedRegionType
    #[xml(attr = "viewedRegionType")]
    pub viewed_region_type: Option<GeoMappingLevel>,
    /// cultureLanguage
    /// Represents the following attribute in the schema: :cultureLanguage
    #[xml(attr = "cultureLanguage")]
    pub culture_language: String,
    /// cultureRegion
    /// Represents the following attribute in the schema: :cultureRegion
    #[xml(attr = "cultureRegion")]
    pub culture_region: String,
    /// attribution
    /// Represents the following attribute in the schema: :attribution
    #[xml(attr = "attribution")]
    pub attribution: String,
    /// _
    #[xml(child = "cx:geoCache")]
    pub geo_cache: Option<GeoCache>,
}
/// Defines the Statistics Class.
/// When the object is serialized out as xml, it's qualified name is cx:statistics.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:statistics")]
pub struct Statistics {
    /// quartileMethod
    /// Represents the following attribute in the schema: :quartileMethod
    #[xml(attr = "quartileMethod")]
    pub quartile_method: Option<QuartileMethod>,
}
/// Defines the Subtotals Class.
/// When the object is serialized out as xml, it's qualified name is cx:subtotals.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:subtotals")]
pub struct Subtotals {
    /// _
    #[xml(child = "cx:idx")]
    pub cx_idx: Vec<UnsignedIntegerType>,
}
/// Defines the ExtremeValueColorPosition Class.
/// When the object is serialized out as xml, it's qualified name is cx:extremeValue.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:extremeValue")]
pub struct ExtremeValueColorPosition {}
/// Defines the NumberColorPosition Class.
/// When the object is serialized out as xml, it's qualified name is cx:number.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:number")]
pub struct NumberColorPosition {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Defines the PercentageColorPosition Class.
/// When the object is serialized out as xml, it's qualified name is cx:percent.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:percent")]
pub struct PercentageColorPosition {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Defines the MinValueColorEndPosition Class.
/// When the object is serialized out as xml, it's qualified name is cx:min.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:min")]
pub struct MinValueColorEndPosition {
    #[xml(child = "cx:extremeValue", child = "cx:number", child = "cx:percent")]
    pub children: Vec<MinValueColorEndPositionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MinValueColorEndPositionChildChoice {
    #[xml(tag = "cx:extremeValue")]
    CxExtremeValue(ExtremeValueColorPosition),
    #[xml(tag = "cx:number")]
    CxNumber(NumberColorPosition),
    #[xml(tag = "cx:percent")]
    CxPercent(PercentageColorPosition),
}
/// Defines the MaxValueColorEndPosition Class.
/// When the object is serialized out as xml, it's qualified name is cx:max.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:max")]
pub struct MaxValueColorEndPosition {
    #[xml(child = "cx:extremeValue", child = "cx:number", child = "cx:percent")]
    pub children: Vec<MaxValueColorEndPositionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MaxValueColorEndPositionChildChoice {
    #[xml(tag = "cx:extremeValue")]
    CxExtremeValue(ExtremeValueColorPosition),
    #[xml(tag = "cx:number")]
    CxNumber(NumberColorPosition),
    #[xml(tag = "cx:percent")]
    CxPercent(PercentageColorPosition),
}
/// Defines the OpenXmlValueColorEndPositionElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlValueColorEndPositionElement {
    #[xml(child = "cx:extremeValue", child = "cx:number", child = "cx:percent")]
    pub children: Vec<OpenXmlValueColorEndPositionElementChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OpenXmlValueColorEndPositionElementChildChoice {
    #[xml(tag = "cx:extremeValue")]
    CxExtremeValue(ExtremeValueColorPosition),
    #[xml(tag = "cx:number")]
    CxNumber(NumberColorPosition),
    #[xml(tag = "cx:percent")]
    CxPercent(PercentageColorPosition),
}
/// Defines the ValueColorMiddlePosition Class.
/// When the object is serialized out as xml, it's qualified name is cx:mid.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:mid")]
pub struct ValueColorMiddlePosition {
    #[xml(child = "cx:number", child = "cx:percent")]
    pub children: Vec<ValueColorMiddlePositionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ValueColorMiddlePositionChildChoice {
    #[xml(tag = "cx:number")]
    CxNumber(NumberColorPosition),
    #[xml(tag = "cx:percent")]
    CxPercent(PercentageColorPosition),
}
/// Defines the DataLabelVisibilities Class.
/// When the object is serialized out as xml, it's qualified name is cx:visibility.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:visibility")]
pub struct DataLabelVisibilities {
    /// seriesName
    /// Represents the following attribute in the schema: :seriesName
    #[xml(attr = "seriesName")]
    pub series_name: Option<bool>,
    /// categoryName
    /// Represents the following attribute in the schema: :categoryName
    #[xml(attr = "categoryName")]
    pub category_name: Option<bool>,
    /// value
    /// Represents the following attribute in the schema: :value
    #[xml(attr = "value")]
    pub value: Option<bool>,
}
/// Defines the DataLabel Class.
/// When the object is serialized out as xml, it's qualified name is cx:dataLabel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:dataLabel")]
pub struct DataLabel {
    /// idx
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub idx: i32,
    /// pos
    /// Represents the following attribute in the schema: :pos
    #[xml(attr = "pos")]
    pub pos: Option<DataLabelPos>,
    /// _
    #[xml(child = "cx:numFmt")]
    pub number_format: Option<NumberFormat>,
    /// _
    #[xml(child = "cx:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cx:txPr")]
    pub tx_pr_text_body: Option<TxPrTextBody>,
    /// _
    #[xml(child = "cx:visibility")]
    pub data_label_visibilities: Option<DataLabelVisibilities>,
    /// _
    #[xml(child = "cx:separator")]
    pub separator_xsdstring: Option<SeparatorXsdstring>,
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the DataLabelHidden Class.
/// When the object is serialized out as xml, it's qualified name is cx:dataLabelHidden.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:dataLabelHidden")]
pub struct DataLabelHidden {
    /// idx
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub idx: i32,
}
/// Defines the ValueColors Class.
/// When the object is serialized out as xml, it's qualified name is cx:valueColors.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:valueColors")]
pub struct ValueColors {
    /// _
    #[xml(child = "cx:minColor")]
    pub min_color_solid_color_fill_properties: Option<MinColorSolidColorFillProperties>,
    /// _
    #[xml(child = "cx:midColor")]
    pub mid_color_solid_color_fill_properties: Option<MidColorSolidColorFillProperties>,
    /// _
    #[xml(child = "cx:maxColor")]
    pub max_color_solid_color_fill_properties: Option<MaxColorSolidColorFillProperties>,
}
/// Defines the ValueColorPositions Class.
/// When the object is serialized out as xml, it's qualified name is cx:valueColorPositions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:valueColorPositions")]
pub struct ValueColorPositions {
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "cx:min")]
    pub min_value_color_end_position: Option<MinValueColorEndPosition>,
    /// _
    #[xml(child = "cx:mid")]
    pub value_color_middle_position: Option<ValueColorMiddlePosition>,
    /// _
    #[xml(child = "cx:max")]
    pub max_value_color_end_position: Option<MaxValueColorEndPosition>,
}
/// Defines the DataPoint Class.
/// When the object is serialized out as xml, it's qualified name is cx:dataPt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:dataPt")]
pub struct DataPoint {
    /// idx
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub idx: i32,
    /// _
    #[xml(child = "cx:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the DataLabels Class.
/// When the object is serialized out as xml, it's qualified name is cx:dataLabels.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:dataLabels")]
pub struct DataLabels {
    /// pos
    /// Represents the following attribute in the schema: :pos
    #[xml(attr = "pos")]
    pub pos: Option<DataLabelPos>,
    /// _
    #[xml(child = "cx:numFmt")]
    pub number_format: Option<NumberFormat>,
    /// _
    #[xml(child = "cx:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cx:txPr")]
    pub tx_pr_text_body: Option<TxPrTextBody>,
    /// _
    #[xml(child = "cx:visibility")]
    pub data_label_visibilities: Option<DataLabelVisibilities>,
    /// _
    #[xml(child = "cx:separator")]
    pub separator_xsdstring: Option<SeparatorXsdstring>,
    /// _
    #[xml(child = "cx:dataLabel")]
    pub cx_data_label: Vec<DataLabel>,
    /// _
    #[xml(child = "cx:dataLabelHidden")]
    pub cx_data_label_hidden: Vec<DataLabelHidden>,
    /// _
    #[xml(child = "cx:extLst")]
    pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the DataId Class.
/// When the object is serialized out as xml, it's qualified name is cx:dataId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:dataId")]
pub struct DataId {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the SeriesLayoutProperties Class.
/// When the object is serialized out as xml, it's qualified name is cx:layoutPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:layoutPr")]
pub struct SeriesLayoutProperties {
    #[xml(
        child = "cx:parentLabelLayout",
        child = "cx:regionLabelLayout",
        child = "cx:visibility",
        child = "cx:aggregation",
        child = "cx:binning",
        child = "cx:geography",
        child = "cx:statistics",
        child = "cx:subtotals",
        child = "cx:extLst",
    )]
    pub children: Vec<SeriesLayoutPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SeriesLayoutPropertiesChildChoice {
    #[xml(tag = "cx:parentLabelLayout")]
    CxParentLabelLayout(ParentLabelLayout),
    #[xml(tag = "cx:regionLabelLayout")]
    CxRegionLabelLayout(RegionLabelLayout),
    #[xml(tag = "cx:visibility")]
    CxVisibility(SeriesElementVisibilities),
    #[xml(tag = "cx:aggregation")]
    CxAggregation(Aggregation),
    #[xml(tag = "cx:binning")]
    CxBinning(Binning),
    #[xml(tag = "cx:geography")]
    CxGeography(Geography),
    #[xml(tag = "cx:statistics")]
    CxStatistics(Statistics),
    #[xml(tag = "cx:subtotals")]
    CxSubtotals(Subtotals),
    #[xml(tag = "cx:extLst")]
    CxExtLst(ExtensionList),
}
/// Defines the AxisId Class.
/// When the object is serialized out as xml, it's qualified name is cx:axisId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:axisId")]
pub struct AxisId {
    #[xml(text)]
    pub child: i32,
}
/// Defines the PlotSurface Class.
/// When the object is serialized out as xml, it's qualified name is cx:plotSurface.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:plotSurface")]
pub struct PlotSurface {
    /// _
    #[xml(child = "cx:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the Series Class.
/// When the object is serialized out as xml, it's qualified name is cx:series.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:series")]
pub struct Series {
    /// layoutId
    /// Represents the following attribute in the schema: :layoutId
    #[xml(attr = "layoutId")]
    pub layout_id: SeriesLayout,
    /// hidden
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    /// ownerIdx
    /// Represents the following attribute in the schema: :ownerIdx
    #[xml(attr = "ownerIdx")]
    pub owner_idx: Option<i32>,
    /// uniqueId
    /// Represents the following attribute in the schema: :uniqueId
    #[xml(attr = "uniqueId")]
    pub unique_id: Option<String>,
    /// formatIdx
    /// Represents the following attribute in the schema: :formatIdx
    #[xml(attr = "formatIdx")]
    pub format_idx: Option<i32>,
    /// _
    #[xml(child = "cx:tx")]
    pub text: Option<Text>,
    /// _
    #[xml(child = "cx:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cx:valueColors")]
    pub value_colors: Option<ValueColors>,
    /// _
    #[xml(child = "cx:valueColorPositions")]
    pub value_color_positions: Option<ValueColorPositions>,
    /// _
    #[xml(child = "cx:dataPt")]
    pub cx_data_pt: Vec<DataPoint>,
    /// _
    #[xml(child = "cx:dataLabels")]
    pub cx_data_labels: Option<DataLabels>,
    /// _
    #[xml(child = "cx:dataId")]
    pub cx_data_id: Option<DataId>,
    /// _
    #[xml(child = "cx:layoutPr")]
    pub cx_layout_pr: Option<SeriesLayoutProperties>,
    /// _
    #[xml(child = "cx:axisId")]
    pub cx_axis_id: Vec<AxisId>,
    /// _
    #[xml(child = "cx:extLst")]
    pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the PlotAreaRegion Class.
/// When the object is serialized out as xml, it's qualified name is cx:plotAreaRegion.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:plotAreaRegion")]
pub struct PlotAreaRegion {
    /// _
    #[xml(child = "cx:plotSurface")]
    pub plot_surface: Option<PlotSurface>,
    /// _
    #[xml(child = "cx:series")]
    pub cx_series: Vec<Series>,
    /// _
    #[xml(child = "cx:extLst")]
    pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the Axis Class.
/// When the object is serialized out as xml, it's qualified name is cx:axis.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:axis")]
pub struct Axis {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// hidden
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    #[xml(
        child = "cx:catScaling",
        child = "cx:valScaling",
        child = "cx:title",
        child = "cx:units",
        child = "cx:majorGridlines",
        child = "cx:minorGridlines",
        child = "cx:majorTickMarks",
        child = "cx:minorTickMarks",
        child = "cx:tickLabels",
        child = "cx:numFmt",
        child = "cx:spPr",
        child = "cx:txPr",
        child = "cx:extLst",
    )]
    pub children: Vec<AxisChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum AxisChildChoice {
    #[xml(tag = "cx:catScaling")]
    CxCatScaling(CategoryAxisScaling),
    #[xml(tag = "cx:valScaling")]
    CxValScaling(ValueAxisScaling),
    #[xml(tag = "cx:title")]
    CxTitle(AxisTitle),
    #[xml(tag = "cx:units")]
    CxUnits(AxisUnits),
    #[xml(tag = "cx:majorGridlines")]
    CxMajorGridlines(MajorGridlinesGridlines),
    #[xml(tag = "cx:minorGridlines")]
    CxMinorGridlines(MinorGridlinesGridlines),
    #[xml(tag = "cx:majorTickMarks")]
    CxMajorTickMarks(MajorTickMarksTickMarks),
    #[xml(tag = "cx:minorTickMarks")]
    CxMinorTickMarks(MinorTickMarksTickMarks),
    #[xml(tag = "cx:tickLabels")]
    CxTickLabels(TickLabels),
    #[xml(tag = "cx:numFmt")]
    CxNumFmt(NumberFormat),
    #[xml(tag = "cx:spPr")]
    CxSpPr(ShapeProperties),
    #[xml(tag = "cx:txPr")]
    CxTxPr(TxPrTextBody),
    #[xml(tag = "cx:extLst")]
    CxExtLst(ExtensionList),
}
/// Defines the ChartTitle Class.
/// When the object is serialized out as xml, it's qualified name is cx:title.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:title")]
pub struct ChartTitle {
    /// pos
    /// Represents the following attribute in the schema: :pos
    #[xml(attr = "pos")]
    pub pos: Option<SidePos>,
    /// align
    /// Represents the following attribute in the schema: :align
    #[xml(attr = "align")]
    pub align: Option<PosAlign>,
    /// overlay
    /// Represents the following attribute in the schema: :overlay
    #[xml(attr = "overlay")]
    pub overlay: Option<bool>,
    /// _
    #[xml(child = "cx:tx")]
    pub text: Option<Text>,
    /// _
    #[xml(child = "cx:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cx:txPr")]
    pub tx_pr_text_body: Option<TxPrTextBody>,
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the PlotArea Class.
/// When the object is serialized out as xml, it's qualified name is cx:plotArea.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:plotArea")]
pub struct PlotArea {
    /// _
    #[xml(child = "cx:plotAreaRegion")]
    pub plot_area_region: PlotAreaRegion,
    /// _
    #[xml(child = "cx:axis")]
    pub cx_axis: Vec<Axis>,
    /// _
    #[xml(child = "cx:spPr")]
    pub cx_sp_pr: Option<ShapeProperties>,
    /// _
    #[xml(child = "cx:extLst")]
    pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the Legend Class.
/// When the object is serialized out as xml, it's qualified name is cx:legend.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:legend")]
pub struct Legend {
    /// pos
    /// Represents the following attribute in the schema: :pos
    #[xml(attr = "pos")]
    pub pos: Option<SidePos>,
    /// align
    /// Represents the following attribute in the schema: :align
    #[xml(attr = "align")]
    pub align: Option<PosAlign>,
    /// overlay
    /// Represents the following attribute in the schema: :overlay
    #[xml(attr = "overlay")]
    pub overlay: Option<bool>,
    /// _
    #[xml(child = "cx:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cx:txPr")]
    pub tx_pr_text_body: Option<TxPrTextBody>,
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the FormatOverride Class.
/// When the object is serialized out as xml, it's qualified name is cx:fmtOvr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:fmtOvr")]
pub struct FormatOverride {
    /// idx
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub idx: i32,
    /// _
    #[xml(child = "cx:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the HeaderFooter Class.
/// When the object is serialized out as xml, it's qualified name is cx:headerFooter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:headerFooter")]
pub struct HeaderFooter {
    /// alignWithMargins
    /// Represents the following attribute in the schema: :alignWithMargins
    #[xml(attr = "alignWithMargins")]
    pub align_with_margins: Option<bool>,
    /// differentOddEven
    /// Represents the following attribute in the schema: :differentOddEven
    #[xml(attr = "differentOddEven")]
    pub different_odd_even: Option<bool>,
    /// differentFirst
    /// Represents the following attribute in the schema: :differentFirst
    #[xml(attr = "differentFirst")]
    pub different_first: Option<bool>,
    /// _
    #[xml(child = "cx:oddHeader")]
    pub odd_header_xsdstring: Option<OddHeaderXsdstring>,
    /// _
    #[xml(child = "cx:oddFooter")]
    pub odd_footer_xsdstring: Option<OddFooterXsdstring>,
    /// _
    #[xml(child = "cx:evenHeader")]
    pub even_header_xsdstring: Option<EvenHeaderXsdstring>,
    /// _
    #[xml(child = "cx:evenFooter")]
    pub even_footer_xsdstring: Option<EvenFooterXsdstring>,
    /// _
    #[xml(child = "cx:firstHeader")]
    pub first_header_xsdstring: Option<FirstHeaderXsdstring>,
    /// _
    #[xml(child = "cx:firstFooter")]
    pub first_footer_xsdstring: Option<FirstFooterXsdstring>,
}
/// Defines the PageMargins Class.
/// When the object is serialized out as xml, it's qualified name is cx:pageMargins.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:pageMargins")]
pub struct PageMargins {
    /// l
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub l: f64,
    /// r
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub r: f64,
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub t: f64,
    /// b
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub b: f64,
    /// header
    /// Represents the following attribute in the schema: :header
    #[xml(attr = "header")]
    pub header: f64,
    /// footer
    /// Represents the following attribute in the schema: :footer
    #[xml(attr = "footer")]
    pub footer: f64,
}
/// Defines the PageSetup Class.
/// When the object is serialized out as xml, it's qualified name is cx:pageSetup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:pageSetup")]
pub struct PageSetup {
    /// paperSize
    /// Represents the following attribute in the schema: :paperSize
    #[xml(attr = "paperSize")]
    pub paper_size: Option<i32>,
    /// firstPageNumber
    /// Represents the following attribute in the schema: :firstPageNumber
    #[xml(attr = "firstPageNumber")]
    pub first_page_number: Option<i32>,
    /// orientation
    /// Represents the following attribute in the schema: :orientation
    #[xml(attr = "orientation")]
    pub orientation: Option<PageOrientation>,
    /// blackAndWhite
    /// Represents the following attribute in the schema: :blackAndWhite
    #[xml(attr = "blackAndWhite")]
    pub black_and_white: Option<bool>,
    /// draft
    /// Represents the following attribute in the schema: :draft
    #[xml(attr = "draft")]
    pub draft: Option<bool>,
    /// useFirstPageNumber
    /// Represents the following attribute in the schema: :useFirstPageNumber
    #[xml(attr = "useFirstPageNumber")]
    pub use_first_page_number: Option<bool>,
    /// horizontalDpi
    /// Represents the following attribute in the schema: :horizontalDpi
    #[xml(attr = "horizontalDpi")]
    pub horizontal_dpi: Option<i32>,
    /// verticalDpi
    /// Represents the following attribute in the schema: :verticalDpi
    #[xml(attr = "verticalDpi")]
    pub vertical_dpi: Option<i32>,
    /// copies
    /// Represents the following attribute in the schema: :copies
    #[xml(attr = "copies")]
    pub copies: Option<i32>,
}
/// Defines the ChartData Class.
/// When the object is serialized out as xml, it's qualified name is cx:chartData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:chartData")]
pub struct ChartData {
    /// _
    #[xml(child = "cx:externalData")]
    pub external_data: Option<ExternalData>,
    /// _
    #[xml(child = "cx:data")]
    pub cx_data: Vec<Data>,
    /// _
    #[xml(child = "cx:extLst")]
    pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the Chart Class.
/// When the object is serialized out as xml, it's qualified name is cx:chart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:chart")]
pub struct Chart {
    /// _
    #[xml(child = "cx:title")]
    pub chart_title: Option<ChartTitle>,
    /// _
    #[xml(child = "cx:plotArea")]
    pub plot_area: PlotArea,
    /// _
    #[xml(child = "cx:legend")]
    pub legend: Option<Legend>,
    /// _
    #[xml(child = "cx:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the ColorMappingType Class.
/// When the object is serialized out as xml, it's qualified name is cx:clrMapOvr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:clrMapOvr")]
pub struct ColorMappingType {
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
/// Defines the FormatOverrides Class.
/// When the object is serialized out as xml, it's qualified name is cx:fmtOvrs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:fmtOvrs")]
pub struct FormatOverrides {
    /// _
    #[xml(child = "cx:fmtOvr")]
    pub cx_fmt_ovr: Vec<FormatOverride>,
}
/// Defines the PrintSettings Class.
/// When the object is serialized out as xml, it's qualified name is cx:printSettings.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:printSettings")]
pub struct PrintSettings {
    /// _
    #[xml(child = "cx:headerFooter")]
    pub header_footer: Option<HeaderFooter>,
    /// _
    #[xml(child = "cx:pageMargins")]
    pub page_margins: Option<PageMargins>,
    /// _
    #[xml(child = "cx:pageSetup")]
    pub page_setup: Option<PageSetup>,
}
/// Index of subtotal data point.
/// When the object is serialized out as xml, it's qualified name is cx:idx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cx:idx")]
pub struct UnsignedIntegerType {
    /// Integer Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
