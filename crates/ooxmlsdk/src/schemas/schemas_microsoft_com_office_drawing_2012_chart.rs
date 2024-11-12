/// Defines the PivotSource Class.
/// When the object is serialized out as xml, it's qualified name is c15:pivotSource.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:pivotSource")]
pub struct PivotSource {
    ///Pivot Name
    #[xml(child = "c:name")]
    pub pivot_table_name: crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PivotTableName,
    ///Format ID
    #[xml(child = "c:fmtId")]
    pub format_id: crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::FormatId,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ExtensionList,
    >,
}
/// Defines the NumberingFormat Class.
/// When the object is serialized out as xml, it's qualified name is c15:numFmt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:numFmt")]
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
/// Defines the ShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is c15:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:spPr")]
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
/// Defines the Layout Class.
/// When the object is serialized out as xml, it's qualified name is c15:layout.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:layout")]
pub struct Layout {
    ///Manual Layout
    #[xml(child = "c:manualLayout")]
    pub manual_layout: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ManualLayout,
    >,
    ///Chart Extensibility
    #[xml(child = "c:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ExtensionList,
    >,
}
/// Defines the FullReference Class.
/// When the object is serialized out as xml, it's qualified name is c15:fullRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:fullRef")]
pub struct FullReference {
    /// _
    #[xml(child = "c15:sqref")]
    pub sequence_of_references: SequenceOfReferences,
}
/// Defines the LevelReference Class.
/// When the object is serialized out as xml, it's qualified name is c15:levelRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:levelRef")]
pub struct LevelReference {
    /// _
    #[xml(child = "c15:sqref")]
    pub sequence_of_references: SequenceOfReferences,
}
/// Defines the FormulaReference Class.
/// When the object is serialized out as xml, it's qualified name is c15:formulaRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:formulaRef")]
pub struct FormulaReference {
    /// _
    #[xml(child = "c15:sqref")]
    pub sequence_of_references: SequenceOfReferences,
}
/// Defines the FilteredSeriesTitle Class.
/// When the object is serialized out as xml, it's qualified name is c15:filteredSeriesTitle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:filteredSeriesTitle")]
pub struct FilteredSeriesTitle {
    /// _
    #[xml(child = "c15:tx")]
    pub chart_text: ChartText,
}
/// Defines the FilteredCategoryTitle Class.
/// When the object is serialized out as xml, it's qualified name is c15:filteredCategoryTitle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:filteredCategoryTitle")]
pub struct FilteredCategoryTitle {
    /// _
    #[xml(child = "c15:cat")]
    pub axis_data_source_type: AxisDataSourceType,
}
/// Defines the FilteredAreaSeries Class.
/// When the object is serialized out as xml, it's qualified name is c15:filteredAreaSeries.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:filteredAreaSeries")]
pub struct FilteredAreaSeries {
    /// _
    #[xml(child = "c15:ser")]
    pub area_chart_series: AreaChartSeries,
}
/// Defines the FilteredBarSeries Class.
/// When the object is serialized out as xml, it's qualified name is c15:filteredBarSeries.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:filteredBarSeries")]
pub struct FilteredBarSeries {
    /// _
    #[xml(child = "c15:ser")]
    pub bar_chart_series: BarChartSeries,
}
/// Defines the FilteredBubbleSeries Class.
/// When the object is serialized out as xml, it's qualified name is c15:filteredBubbleSeries.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:filteredBubbleSeries")]
pub struct FilteredBubbleSeries {
    /// _
    #[xml(child = "c15:ser")]
    pub bubble_chart_series: BubbleChartSeries,
}
/// Defines the FilteredLineSeriesExtension Class.
/// When the object is serialized out as xml, it's qualified name is c15:filteredLineSeries.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:filteredLineSeries")]
pub struct FilteredLineSeriesExtension {
    /// _
    #[xml(child = "c15:ser")]
    pub line_chart_series: LineChartSeries,
}
/// Defines the FilteredPieSeries Class.
/// When the object is serialized out as xml, it's qualified name is c15:filteredPieSeries.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:filteredPieSeries")]
pub struct FilteredPieSeries {
    /// _
    #[xml(child = "c15:ser")]
    pub pie_chart_series: PieChartSeries,
}
/// Defines the FilteredRadarSeries Class.
/// When the object is serialized out as xml, it's qualified name is c15:filteredRadarSeries.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:filteredRadarSeries")]
pub struct FilteredRadarSeries {
    /// _
    #[xml(child = "c15:ser")]
    pub radar_chart_series: RadarChartSeries,
}
/// Defines the FilteredScatterSeries Class.
/// When the object is serialized out as xml, it's qualified name is c15:filteredScatterSeries.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:filteredScatterSeries")]
pub struct FilteredScatterSeries {
    /// _
    #[xml(child = "c15:ser")]
    pub scatter_chart_series: ScatterChartSeries,
}
/// Defines the FilteredSurfaceSeries Class.
/// When the object is serialized out as xml, it's qualified name is c15:filteredSurfaceSeries.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:filteredSurfaceSeries")]
pub struct FilteredSurfaceSeries {
    /// _
    #[xml(child = "c15:ser")]
    pub surface_chart_series: SurfaceChartSeries,
}
/// Defines the DataLabelsRange Class.
/// When the object is serialized out as xml, it's qualified name is c15:datalabelsRange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:datalabelsRange")]
pub struct DataLabelsRange {
    /// _
    #[xml(child = "c15:f")]
    pub formula: Formula,
    /// _
    #[xml(child = "c15:dlblRangeCache")]
    pub data_labels_range_chache: Option<DataLabelsRangeChache>,
}
/// Defines the CategoryFilterExceptions Class.
/// When the object is serialized out as xml, it's qualified name is c15:categoryFilterExceptions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:categoryFilterExceptions")]
pub struct CategoryFilterExceptions {
    /// _
    #[xml(child = "c15:categoryFilterException")]
    pub c15_category_filter_exception: Vec<CategoryFilterException>,
}
/// Defines the DataLabelFieldTable Class.
/// When the object is serialized out as xml, it's qualified name is c15:dlblFieldTable.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:dlblFieldTable")]
pub struct DataLabelFieldTable {
    /// _
    #[xml(child = "c15:dlblFTEntry")]
    pub c15_dlbl_ft_entry: Vec<DataLabelFieldTableEntry>,
}
/// Defines the ExceptionForSave Class.
/// When the object is serialized out as xml, it's qualified name is c15:xForSave.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:xForSave")]
pub struct ExceptionForSave {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the ShowDataLabelsRange Class.
/// When the object is serialized out as xml, it's qualified name is c15:showDataLabelsRange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:showDataLabelsRange")]
pub struct ShowDataLabelsRange {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the ShowLeaderLines Class.
/// When the object is serialized out as xml, it's qualified name is c15:showLeaderLines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:showLeaderLines")]
pub struct ShowLeaderLines {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the AutoGeneneratedCategories Class.
/// When the object is serialized out as xml, it's qualified name is c15:autoCat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:autoCat")]
pub struct AutoGeneneratedCategories {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the InvertIfNegativeBoolean Class.
/// When the object is serialized out as xml, it's qualified name is c15:invertIfNegative.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:invertIfNegative")]
pub struct InvertIfNegativeBoolean {
    /// Boolean Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the Bubble3D Class.
/// When the object is serialized out as xml, it's qualified name is c15:bubble3D.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:bubble3D")]
pub struct Bubble3D {
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
/// Defines the ChartText Class.
/// When the object is serialized out as xml, it's qualified name is c15:tx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:tx")]
pub struct ChartText {
    #[xml(child = "c:strRef", child = "c:rich", child = "c:strLit")]
    pub children: Vec<ChartTextChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ChartTextChildChoice {
    #[xml(tag = "c:strRef")]
    CStrRef(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::StringReference,
    ),
    #[xml(tag = "c:rich")]
    CRich(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::RichText),
    #[xml(tag = "c:strLit")]
    CStrLit(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::StringLiteral,
    ),
}
/// Defines the LeaderLines Class.
/// When the object is serialized out as xml, it's qualified name is c15:leaderLines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:leaderLines")]
pub struct LeaderLines {
    /// _
    #[xml(child = "c:spPr")]
    pub chart_shape_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    >,
}
/// Defines the SequenceOfReferences Class.
/// When the object is serialized out as xml, it's qualified name is c15:sqref.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:sqref")]
pub struct SequenceOfReferences {
    #[xml(text)]
    pub child: String,
}
/// Defines the Formula Class.
/// When the object is serialized out as xml, it's qualified name is c15:f.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:f")]
pub struct Formula {
    #[xml(text)]
    pub child: String,
}
/// Defines the TextFieldGuid Class.
/// When the object is serialized out as xml, it's qualified name is c15:txfldGUID.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:txfldGUID")]
pub struct TextFieldGuid {
    #[xml(text)]
    pub child: String,
}
/// Defines the AxisDataSourceType Class.
/// When the object is serialized out as xml, it's qualified name is c15:cat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:cat")]
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
    CMultiLvlStrRef(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::MultiLevelStringReference,
    ),
    #[xml(tag = "c:numRef")]
    CNumRef(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::NumberReference,
    ),
    #[xml(tag = "c:numLit")]
    CNumLit(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::NumberLiteral,
    ),
    #[xml(tag = "c:strRef")]
    CStrRef(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::StringReference,
    ),
    #[xml(tag = "c:strLit")]
    CStrLit(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::StringLiteral,
    ),
}
/// Defines the BarChartSeries Class.
/// When the object is serialized out as xml, it's qualified name is c15:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:ser")]
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
    CIdx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index),
    #[xml(tag = "c:order")]
    COrder(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order),
    #[xml(tag = "c:tx")]
    CTx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    ),
    #[xml(tag = "c:invertIfNegative")]
    CInvertIfNegative(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::InvertIfNegative,
    ),
    #[xml(tag = "c:pictureOptions")]
    CPictureOptions(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PictureOptions,
    ),
    #[xml(tag = "c:dPt")]
    CDPt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataPoint),
    #[xml(tag = "c:dLbls")]
    CDLbls(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabels),
    #[xml(tag = "c:trendline")]
    CTrendline(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Trendline,
    ),
    #[xml(tag = "c:errBars")]
    CErrBars(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ErrorBars),
    #[xml(tag = "c:cat")]
    CCat(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::CategoryAxisData,
    ),
    #[xml(tag = "c:val")]
    CVal(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Values),
    #[xml(tag = "c:shape")]
    CShape(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Shape),
    #[xml(tag = "c:extLst")]
    CExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::BarSerExtensionList,
    ),
}
/// Defines the LineChartSeries Class.
/// When the object is serialized out as xml, it's qualified name is c15:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:ser")]
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
    CIdx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index),
    #[xml(tag = "c:order")]
    COrder(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order),
    #[xml(tag = "c:tx")]
    CTx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    ),
    #[xml(tag = "c:marker")]
    CMarker(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Marker),
    #[xml(tag = "c:pictureOptions")]
    CPictureOptions(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PictureOptions,
    ),
    #[xml(tag = "c:dPt")]
    CDPt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataPoint),
    #[xml(tag = "c:dLbls")]
    CDLbls(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabels),
    #[xml(tag = "c:trendline")]
    CTrendline(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Trendline,
    ),
    #[xml(tag = "c:errBars")]
    CErrBars(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ErrorBars),
    #[xml(tag = "c:cat")]
    CCat(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::CategoryAxisData,
    ),
    #[xml(tag = "c:val")]
    CVal(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Values),
    #[xml(tag = "c:smooth")]
    CSmooth(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Smooth),
    #[xml(tag = "c:extLst")]
    CExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::LineSerExtensionList,
    ),
}
/// Defines the ScatterChartSeries Class.
/// When the object is serialized out as xml, it's qualified name is c15:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:ser")]
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
    CIdx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index),
    #[xml(tag = "c:order")]
    COrder(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order),
    #[xml(tag = "c:tx")]
    CTx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    ),
    #[xml(tag = "c:marker")]
    CMarker(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Marker),
    #[xml(tag = "c:dPt")]
    CDPt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataPoint),
    #[xml(tag = "c:dLbls")]
    CDLbls(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabels),
    #[xml(tag = "c:trendline")]
    CTrendline(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Trendline,
    ),
    #[xml(tag = "c:errBars")]
    CErrBars(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ErrorBars),
    #[xml(tag = "c:xVal")]
    CXVal(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::XValues),
    #[xml(tag = "c:yVal")]
    CYVal(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::YValues),
    #[xml(tag = "c:smooth")]
    CSmooth(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Smooth),
    #[xml(tag = "c:extLst")]
    CExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ScatterSerExtensionList,
    ),
}
/// Defines the AreaChartSeries Class.
/// When the object is serialized out as xml, it's qualified name is c15:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:ser")]
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
    CIdx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index),
    #[xml(tag = "c:order")]
    COrder(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order),
    #[xml(tag = "c:tx")]
    CTx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    ),
    #[xml(tag = "c:pictureOptions")]
    CPictureOptions(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PictureOptions,
    ),
    #[xml(tag = "c:dPt")]
    CDPt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataPoint),
    #[xml(tag = "c:dLbls")]
    CDLbls(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabels),
    #[xml(tag = "c:trendline")]
    CTrendline(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Trendline,
    ),
    #[xml(tag = "c:errBars")]
    CErrBars(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ErrorBars),
    #[xml(tag = "c:cat")]
    CCat(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::CategoryAxisData,
    ),
    #[xml(tag = "c:val")]
    CVal(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Values),
    #[xml(tag = "c:extLst")]
    CExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::AreaSerExtensionList,
    ),
}
/// Defines the PieChartSeries Class.
/// When the object is serialized out as xml, it's qualified name is c15:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:ser")]
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
    CIdx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index),
    #[xml(tag = "c:order")]
    COrder(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order),
    #[xml(tag = "c:tx")]
    CTx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    ),
    #[xml(tag = "c:pictureOptions")]
    CPictureOptions(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PictureOptions,
    ),
    #[xml(tag = "c:explosion")]
    CExplosion(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Explosion,
    ),
    #[xml(tag = "c:dPt")]
    CDPt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataPoint),
    #[xml(tag = "c:dLbls")]
    CDLbls(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabels),
    #[xml(tag = "c:cat")]
    CCat(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::CategoryAxisData,
    ),
    #[xml(tag = "c:val")]
    CVal(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Values),
    #[xml(tag = "c:extLst")]
    CExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PieSerExtensionList,
    ),
}
/// Defines the BubbleChartSeries Class.
/// When the object is serialized out as xml, it's qualified name is c15:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:ser")]
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
    CIdx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index),
    #[xml(tag = "c:order")]
    COrder(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order),
    #[xml(tag = "c:tx")]
    CTx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    ),
    #[xml(tag = "c:pictureOptions")]
    CPictureOptions(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PictureOptions,
    ),
    #[xml(tag = "c:invertIfNegative")]
    CInvertIfNegative(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::InvertIfNegative,
    ),
    #[xml(tag = "c:dPt")]
    CDPt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataPoint),
    #[xml(tag = "c:dLbls")]
    CDLbls(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabels),
    #[xml(tag = "c:trendline")]
    CTrendline(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Trendline,
    ),
    #[xml(tag = "c:errBars")]
    CErrBars(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ErrorBars),
    #[xml(tag = "c:xVal")]
    CXVal(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::XValues),
    #[xml(tag = "c:yVal")]
    CYVal(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::YValues),
    #[xml(tag = "c:bubbleSize")]
    CBubbleSize(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::BubbleSize,
    ),
    #[xml(tag = "c:bubble3D")]
    CBubble3D(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Bubble3D),
    #[xml(tag = "c:extLst")]
    CExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::BubbleSerExtensionList,
    ),
}
/// Defines the RadarChartSeries Class.
/// When the object is serialized out as xml, it's qualified name is c15:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:ser")]
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
    CIdx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index),
    #[xml(tag = "c:order")]
    COrder(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order),
    #[xml(tag = "c:tx")]
    CTx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    ),
    #[xml(tag = "c:pictureOptions")]
    CPictureOptions(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PictureOptions,
    ),
    #[xml(tag = "c:marker")]
    CMarker(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Marker),
    #[xml(tag = "c:dPt")]
    CDPt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataPoint),
    #[xml(tag = "c:dLbls")]
    CDLbls(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabels),
    #[xml(tag = "c:cat")]
    CCat(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::CategoryAxisData,
    ),
    #[xml(tag = "c:val")]
    CVal(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Values),
    #[xml(tag = "c:extLst")]
    CExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::RadarSerExtensionList,
    ),
}
/// Defines the SurfaceChartSeries Class.
/// When the object is serialized out as xml, it's qualified name is c15:ser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:ser")]
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
    CIdx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index),
    #[xml(tag = "c:order")]
    COrder(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order),
    #[xml(tag = "c:tx")]
    CTx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText),
    #[xml(tag = "c:spPr")]
    CSpPr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    ),
    #[xml(tag = "c:pictureOptions")]
    CPictureOptions(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PictureOptions,
    ),
    #[xml(tag = "c:cat")]
    CCat(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::CategoryAxisData,
    ),
    #[xml(tag = "c:val")]
    CVal(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Values),
    #[xml(tag = "c:bubble3D")]
    CBubble3D(crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Bubble3D),
    #[xml(tag = "c:extLst")]
    CExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SurfaceSerExtensionList,
    ),
}
/// Defines the DataLabelsRangeChache Class.
/// When the object is serialized out as xml, it's qualified name is c15:dlblRangeCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:dlblRangeCache")]
pub struct DataLabelsRangeChache {
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
/// Defines the DataLabelFieldTableCache Class.
/// When the object is serialized out as xml, it's qualified name is c15:dlblFieldTableCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:dlblFieldTableCache")]
pub struct DataLabelFieldTableCache {
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
/// Defines the StringDataType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct StringDataType {}
/// Defines the Explosion Class.
/// When the object is serialized out as xml, it's qualified name is c15:explosion.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:explosion")]
pub struct Explosion {
    /// Integer Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the Marker Class.
/// When the object is serialized out as xml, it's qualified name is c15:marker.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:marker")]
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
/// Defines the DataLabel Class.
/// When the object is serialized out as xml, it's qualified name is c15:dLbl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:dLbl")]
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
/// Defines the CategoryFilterException Class.
/// When the object is serialized out as xml, it's qualified name is c15:categoryFilterException.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:categoryFilterException")]
pub struct CategoryFilterException {
    /// _
    #[xml(child = "c15:sqref")]
    pub sequence_of_references: SequenceOfReferences,
    /// _
    #[xml(child = "c15:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "c15:explosion")]
    pub explosion: Option<Explosion>,
    /// _
    #[xml(child = "c15:invertIfNegative")]
    pub invert_if_negative_boolean: Option<InvertIfNegativeBoolean>,
    /// _
    #[xml(child = "c15:bubble3D")]
    pub bubble3_d: Option<Bubble3D>,
    /// _
    #[xml(child = "c15:marker")]
    pub marker: Option<Marker>,
    /// _
    #[xml(child = "c15:dLbl")]
    pub data_label: Option<DataLabel>,
}
/// Defines the DataLabelFieldTableEntry Class.
/// When the object is serialized out as xml, it's qualified name is c15:dlblFTEntry.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c15:dlblFTEntry")]
pub struct DataLabelFieldTableEntry {
    /// _
    #[xml(child = "c15:txfldGUID")]
    pub text_field_guid: TextFieldGuid,
    /// _
    #[xml(child = "c15:f")]
    pub formula: Formula,
    /// _
    #[xml(child = "c15:dlblFieldTableCache")]
    pub data_label_field_table_cache: Option<DataLabelFieldTableCache>,
}
