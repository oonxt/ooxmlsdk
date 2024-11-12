#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TimelineStyleType {
    #[default]
    SelectionLabel,
    TimeLevel,
    PeriodLabel1,
    PeriodLabel2,
    SelectedTimeBlock,
    UnselectedTimeBlock,
    SelectedTimeBlockSpace,
}
crate::__string_enum! {
    TimelineStyleType { SelectionLabel = "selectionLabel", TimeLevel = "timeLevel",
    PeriodLabel1 = "periodLabel1", PeriodLabel2 = "periodLabel2", SelectedTimeBlock =
    "selectedTimeBlock", UnselectedTimeBlock = "unselectedTimeBlock",
    SelectedTimeBlockSpace = "selectedTimeBlockSpace", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CalculatedMemberNumberFormat {
    #[default]
    Default,
    Number,
    Percent,
}
crate::__string_enum! {
    CalculatedMemberNumberFormat { Default = "default", Number = "number", Percent =
    "percent", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SxvCellType {
    #[default]
    Boolean,
    Number,
    Error,
    String,
    Date,
    Blank,
}
crate::__string_enum! {
    SxvCellType { Boolean = "b", Number = "n", Error = "e", String = "str", Date = "d",
    Blank = "bl", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MovingPeriodStep {
    #[default]
    Year,
    Quarter,
    Month,
    Week,
    Day,
}
crate::__string_enum! {
    MovingPeriodStep { Year = "year", Quarter = "quarter", Month = "month", Week =
    "week", Day = "day", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum QuestionType {
    #[default]
    CheckBox,
    Choice,
    Date,
    Time,
    MultipleLinesOfText,
    Number,
    SingleLineOfText,
}
crate::__string_enum! {
    QuestionType { CheckBox = "checkBox", Choice = "choice", Date = "date", Time =
    "time", MultipleLinesOfText = "multipleLinesOfText", Number = "number",
    SingleLineOfText = "singleLineOfText", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum QuestionFormat {
    #[default]
    GeneralDate,
    LongDate,
    ShortDate,
    LongTime,
    ShortTime,
    GeneralNumber,
    Standard,
    Fixed,
    Percent,
    Currency,
}
crate::__string_enum! {
    QuestionFormat { GeneralDate = "generalDate", LongDate = "longDate", ShortDate =
    "shortDate", LongTime = "longTime", ShortTime = "shortTime", GeneralNumber =
    "generalNumber", Standard = "standard", Fixed = "fixed", Percent = "percent",
    Currency = "currency", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SurveyPosition {
    #[default]
    Absolute,
    Fixed,
    Relative,
    Static,
    Inherit,
}
crate::__string_enum! {
    SurveyPosition { Absolute = "absolute", Fixed = "fixed", Relative = "relative",
    Static = "static", Inherit = "inherit", }
}
/// Defines the PivotCaches Class.
/// When the object is serialized out as xml, it's qualified name is x15:pivotCaches.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:pivotCaches")]
pub struct PivotCaches {
    #[xml(child = "x:pivotCache")]
    pub children: Vec<PivotCachesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PivotCachesChildChoice {
    #[xml(tag = "x:pivotCache")]
    XPivotCache(
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCache,
    ),
}
/// Defines the TimelineCachePivotCaches Class.
/// When the object is serialized out as xml, it's qualified name is x15:timelineCachePivotCaches.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:timelineCachePivotCaches")]
pub struct TimelineCachePivotCaches {
    #[xml(child = "x:pivotCache")]
    pub children: Vec<TimelineCachePivotCachesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TimelineCachePivotCachesChildChoice {
    #[xml(tag = "x:pivotCache")]
    XPivotCache(
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCache,
    ),
}
/// Defines the OpenXmlPivotCachesElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlPivotCachesElement {
    #[xml(child = "x:pivotCache")]
    pub children: Vec<OpenXmlPivotCachesElementChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OpenXmlPivotCachesElementChildChoice {
    #[xml(tag = "x:pivotCache")]
    XPivotCache(
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCache,
    ),
}
/// Defines the PivotTableReferences Class.
/// When the object is serialized out as xml, it's qualified name is x15:pivotTableReferences.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:pivotTableReferences")]
pub struct PivotTableReferences {
    /// _
    #[xml(child = "x15:pivotTableReference")]
    pub x15_pivot_table_reference: Vec<PivotTableReference>,
}
/// Defines the QueryTable Class.
/// When the object is serialized out as xml, it's qualified name is x15:queryTable.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:queryTable")]
pub struct QueryTable {
    /// clipped
    /// Represents the following attribute in the schema: :clipped
    #[xml(attr = "clipped")]
    pub clipped: Option<bool>,
    /// sourceDataName
    /// Represents the following attribute in the schema: :sourceDataName
    #[xml(attr = "sourceDataName")]
    pub source_data_name: Option<String>,
    /// drillThrough
    /// Represents the following attribute in the schema: :drillThrough
    #[xml(attr = "drillThrough")]
    pub drill_through: Option<bool>,
}
/// Defines the WebExtensions Class.
/// When the object is serialized out as xml, it's qualified name is x15:webExtensions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:webExtensions")]
pub struct WebExtensions {
    /// _
    #[xml(child = "x15:webExtension")]
    pub x15_web_extension: Vec<WebExtension>,
}
/// Defines the TimelineCacheReferences Class.
/// When the object is serialized out as xml, it's qualified name is x15:timelineCacheRefs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:timelineCacheRefs")]
pub struct TimelineCacheReferences {
    /// _
    #[xml(child = "x15:timelineCacheRef")]
    pub x15_timeline_cache_ref: Vec<TimelineCacheReference>,
}
/// Defines the TimelineReferences Class.
/// When the object is serialized out as xml, it's qualified name is x15:timelineRefs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:timelineRefs")]
pub struct TimelineReferences {
    /// _
    #[xml(child = "x15:timelineRef")]
    pub x15_timeline_ref: Vec<TimelineReference>,
}
/// Defines the WorkbookProperties Class.
/// When the object is serialized out as xml, it's qualified name is x15:workbookPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:workbookPr")]
pub struct WorkbookProperties {
    /// chartTrackingRefBase
    /// Represents the following attribute in the schema: :chartTrackingRefBase
    #[xml(attr = "chartTrackingRefBase")]
    pub chart_tracking_reference_base: Option<bool>,
}
/// Defines the TimelineStyles Class.
/// When the object is serialized out as xml, it's qualified name is x15:timelineStyles.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:timelineStyles")]
pub struct TimelineStyles {
    /// defaultTimelineStyle
    /// Represents the following attribute in the schema: :defaultTimelineStyle
    #[xml(attr = "defaultTimelineStyle")]
    pub default_timeline_style: String,
    /// _
    #[xml(child = "x15:timelineStyle")]
    pub x15_timeline_style: Vec<TimelineStyle>,
}
/// Defines the DifferentialFormats Class.
/// When the object is serialized out as xml, it's qualified name is x15:dxfs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:dxfs")]
pub struct DifferentialFormats {
    /// Format Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:dxf")]
    pub x_dxf: Vec<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DifferentialFormat,
    >,
}
/// Defines the Connection Class.
/// When the object is serialized out as xml, it's qualified name is x15:connection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:connection")]
pub struct Connection {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// model
    /// Represents the following attribute in the schema: :model
    #[xml(attr = "model")]
    pub model: Option<bool>,
    /// excludeFromRefreshAll
    /// Represents the following attribute in the schema: :excludeFromRefreshAll
    #[xml(attr = "excludeFromRefreshAll")]
    pub exclude_from_refresh_all: Option<bool>,
    /// autoDelete
    /// Represents the following attribute in the schema: :autoDelete
    #[xml(attr = "autoDelete")]
    pub auto_delete: Option<bool>,
    /// usedByAddin
    /// Represents the following attribute in the schema: :usedByAddin
    #[xml(attr = "usedByAddin")]
    pub used_by_addin: Option<bool>,
    /// _
    #[xml(child = "x15:textPr")]
    pub text_properties: Option<TextProperties>,
    /// _
    #[xml(child = "x15:modelTextPr")]
    pub model_text_properties: Option<ModelTextProperties>,
    /// _
    #[xml(child = "x15:rangePr")]
    pub range_properties: Option<RangeProperties>,
    /// _
    #[xml(child = "x15:oledbPr")]
    pub ole_db_prpoperties: Option<OleDbPrpoperties>,
    /// _
    #[xml(child = "x15:dataFeedPr")]
    pub data_feed_properties: Option<DataFeedProperties>,
}
/// Defines the CalculatedMember Class.
/// When the object is serialized out as xml, it's qualified name is x15:calculatedMember.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:calculatedMember")]
pub struct CalculatedMember {
    /// measureGroup
    /// Represents the following attribute in the schema: :measureGroup
    #[xml(attr = "measureGroup")]
    pub measure_group: Option<String>,
    /// numberFormat
    /// Represents the following attribute in the schema: :numberFormat
    #[xml(attr = "numberFormat")]
    pub number_format: Option<CalculatedMemberNumberFormat>,
    /// measure
    /// Represents the following attribute in the schema: :measure
    #[xml(attr = "measure")]
    pub measure: Option<bool>,
}
/// Defines the PivotTableUISettings Class.
/// When the object is serialized out as xml, it's qualified name is x15:pivotTableUISettings.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:pivotTableUISettings")]
pub struct PivotTableUiSettings {
    /// sourceDataName
    /// Represents the following attribute in the schema: :sourceDataName
    #[xml(attr = "sourceDataName")]
    pub source_data_name: Option<String>,
    /// relNeededHidden
    /// Represents the following attribute in the schema: :relNeededHidden
    #[xml(attr = "relNeededHidden")]
    pub rel_needed_hidden: Option<bool>,
    /// _
    #[xml(child = "x15:activeTabTopLevelEntity")]
    pub x15_active_tab_top_level_entity: Vec<FieldListActiveTabTopLevelEntity>,
    /// _
    #[xml(child = "x15:extLst")]
    pub x15_ext_lst: Option<ExtensionList>,
}
/// Defines the PivotFilter Class.
/// When the object is serialized out as xml, it's qualified name is x15:pivotFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:pivotFilter")]
pub struct PivotFilter {
    /// useWholeDay
    /// Represents the following attribute in the schema: :useWholeDay
    #[xml(attr = "useWholeDay")]
    pub use_whole_day: bool,
}
/// Defines the CachedUniqueNames Class.
/// When the object is serialized out as xml, it's qualified name is x15:cachedUniqueNames.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:cachedUniqueNames")]
pub struct CachedUniqueNames {
    /// _
    #[xml(child = "x15:cachedUniqueName")]
    pub x15_cached_unique_name: Vec<CachedUniqueName>,
}
/// Defines the CacheHierarchy Class.
/// When the object is serialized out as xml, it's qualified name is x15:cacheHierarchy.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:cacheHierarchy")]
pub struct CacheHierarchy {
    /// aggregatedColumn
    /// Represents the following attribute in the schema: :aggregatedColumn
    #[xml(attr = "aggregatedColumn")]
    pub aggregated_column: i32,
}
/// Defines the TimelinePivotCacheDefinition Class.
/// When the object is serialized out as xml, it's qualified name is x15:timelinePivotCacheDefinition.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:timelinePivotCacheDefinition")]
pub struct TimelinePivotCacheDefinition {
    /// timelineData
    /// Represents the following attribute in the schema: :timelineData
    #[xml(attr = "timelineData")]
    pub timeline_data: Option<bool>,
}
/// Defines the PivotCacheIdVersion Class.
/// When the object is serialized out as xml, it's qualified name is x15:pivotCacheIdVersion.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:pivotCacheIdVersion")]
pub struct PivotCacheIdVersion {
    /// cacheIdSupportedVersion
    /// Represents the following attribute in the schema: :cacheIdSupportedVersion
    #[xml(attr = "cacheIdSupportedVersion")]
    pub cache_id_supported_version: u8,
    /// cacheIdCreatedVersion
    /// Represents the following attribute in the schema: :cacheIdCreatedVersion
    #[xml(attr = "cacheIdCreatedVersion")]
    pub cache_id_created_version: u8,
}
/// Defines the DataModel Class.
/// When the object is serialized out as xml, it's qualified name is x15:dataModel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:dataModel")]
pub struct DataModel {
    /// minVersionLoad
    /// Represents the following attribute in the schema: :minVersionLoad
    #[xml(attr = "minVersionLoad")]
    pub min_version_load: Option<u8>,
    /// _
    #[xml(child = "x15:modelTables")]
    pub model_tables: Option<ModelTables>,
    /// _
    #[xml(child = "x15:modelRelationships")]
    pub model_relationships: Option<ModelRelationships>,
    /// _
    #[xml(child = "x15:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotTableData Class.
/// When the object is serialized out as xml, it's qualified name is x15:pivotTableData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:pivotTableData")]
pub struct PivotTableData {
    /// rowCount
    /// Represents the following attribute in the schema: :rowCount
    #[xml(attr = "rowCount")]
    pub row_count: Option<i32>,
    /// columnCount
    /// Represents the following attribute in the schema: :columnCount
    #[xml(attr = "columnCount")]
    pub column_count: Option<i32>,
    /// cacheId
    /// Represents the following attribute in the schema: :cacheId
    #[xml(attr = "cacheId")]
    pub cache_id: i32,
    #[xml(child = "x15:pivotRow")]
    pub children: Vec<PivotTableDataChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PivotTableDataChildChoice {
    #[xml(tag = "x15:pivotRow")]
    X15PivotRow(PivotRow),
}
/// Defines the PivotCacheDecoupled Class.
/// When the object is serialized out as xml, it's qualified name is x15:pivotCacheDecoupled.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:pivotCacheDecoupled")]
pub struct PivotCacheDecoupled {
    /// decoupled
    /// Represents the following attribute in the schema: :decoupled
    #[xml(attr = "decoupled")]
    pub decoupled: Option<bool>,
}
/// Defines the DataField Class.
/// When the object is serialized out as xml, it's qualified name is x15:dataField.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:dataField")]
pub struct DataField {
    /// isCountDistinct
    /// Represents the following attribute in the schema: :isCountDistinct
    #[xml(attr = "isCountDistinct")]
    pub is_count_distinct: Option<bool>,
}
/// Defines the MovingPeriodState Class.
/// When the object is serialized out as xml, it's qualified name is x15:movingPeriodState.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:movingPeriodState")]
pub struct MovingPeriodState {
    /// referenceDateBegin
    /// Represents the following attribute in the schema: :referenceDateBegin
    #[xml(attr = "referenceDateBegin")]
    pub reference_date_begin: String,
    /// referencePeriod
    /// Represents the following attribute in the schema: :referencePeriod
    #[xml(attr = "referencePeriod")]
    pub reference_period: MovingPeriodStep,
    /// referenceMultiple
    /// Represents the following attribute in the schema: :referenceMultiple
    #[xml(attr = "referenceMultiple")]
    pub reference_multiple: i32,
    /// movingPeriod
    /// Represents the following attribute in the schema: :movingPeriod
    #[xml(attr = "movingPeriod")]
    pub moving_period: MovingPeriodStep,
    /// movingMultiple
    /// Represents the following attribute in the schema: :movingMultiple
    #[xml(attr = "movingMultiple")]
    pub moving_multiple: i32,
}
/// Defines the SlicerCaches Class.
/// When the object is serialized out as xml, it's qualified name is x15:slicerCaches.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:slicerCaches")]
pub struct SlicerCaches {
    /// _
    #[xml(child = "x14:slicerCache")]
    pub x14_slicer_cache: Vec<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCache,
    >,
}
/// Defines the TableSlicerCache Class.
/// When the object is serialized out as xml, it's qualified name is x15:tableSlicerCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:tableSlicerCache")]
pub struct TableSlicerCache {
    /// tableId
    /// Represents the following attribute in the schema: :tableId
    #[xml(attr = "tableId")]
    pub table_id: i32,
    /// column
    /// Represents the following attribute in the schema: :column
    #[xml(attr = "column")]
    pub column: i32,
    /// sortOrder
    /// Represents the following attribute in the schema: :sortOrder
    #[xml(attr = "sortOrder")]
    pub sort_order: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::TabularSlicerCacheSortOrderValues,
    >,
    /// customListSort
    /// Represents the following attribute in the schema: :customListSort
    #[xml(attr = "customListSort")]
    pub custom_list_sort: Option<bool>,
    /// crossFilter
    /// Represents the following attribute in the schema: :crossFilter
    #[xml(attr = "crossFilter")]
    pub cross_filter: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCacheCrossFilterValues,
    >,
    /// _
    #[xml(child = "x15:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the SlicerCacheHideItemsWithNoData Class.
/// When the object is serialized out as xml, it's qualified name is x15:slicerCacheHideItemsWithNoData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:slicerCacheHideItemsWithNoData")]
pub struct SlicerCacheHideItemsWithNoData {
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x15:slicerCacheOlapLevelName")]
    pub x15_slicer_cache_olap_level_name: Vec<SlicerCacheOlapLevelName>,
}
/// Defines the SlicerCachePivotTables Class.
/// When the object is serialized out as xml, it's qualified name is x15:slicerCachePivotTables.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:slicerCachePivotTables")]
pub struct SlicerCachePivotTables {
    /// _
    #[xml(child = "x14:pivotTable")]
    pub x14_pivot_table: Vec<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCachePivotTable,
    >,
}
/// Defines the Survey Class.
/// When the object is serialized out as xml, it's qualified name is x15:survey.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:survey")]
pub struct Survey {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// guid
    /// Represents the following attribute in the schema: :guid
    #[xml(attr = "guid")]
    pub guid: String,
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// _
    #[xml(child = "x15:surveyPr")]
    pub survey_pr_survey_element_pr: Option<SurveyPrSurveyElementPr>,
    /// _
    #[xml(child = "x15:titlePr")]
    pub title_pr_survey_element_pr: Option<TitlePrSurveyElementPr>,
    /// _
    #[xml(child = "x15:descriptionPr")]
    pub description_pr_survey_element_pr: Option<DescriptionPrSurveyElementPr>,
    /// _
    #[xml(child = "x15:questions")]
    pub survey_questions: SurveyQuestions,
    /// _
    #[xml(child = "x15:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the Timelines Class.
/// When the object is serialized out as xml, it's qualified name is x15:timelines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:timelines")]
pub struct Timelines {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "x15:timeline")]
    pub x15_timeline: Vec<Timeline>,
}
/// Defines the TimelineCacheDefinition Class.
/// When the object is serialized out as xml, it's qualified name is x15:timelineCacheDefinition.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:timelineCacheDefinition")]
pub struct TimelineCacheDefinition {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// sourceName
    /// Represents the following attribute in the schema: :sourceName
    #[xml(attr = "sourceName")]
    pub source_name: String,
    /// _
    #[xml(child = "x15:pivotTables")]
    pub timeline_cache_pivot_tables: Option<TimelineCachePivotTables>,
    /// _
    #[xml(child = "x15:state")]
    pub timeline_state: TimelineState,
    /// _
    #[xml(child = "x15:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotTableReference Class.
/// When the object is serialized out as xml, it's qualified name is x15:pivotTableReference.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:pivotTableReference")]
pub struct PivotTableReference {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the WebExtension Class.
/// When the object is serialized out as xml, it's qualified name is x15:webExtension.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:webExtension")]
pub struct WebExtension {
    /// appRef
    /// Represents the following attribute in the schema: :appRef
    #[xml(attr = "appRef")]
    pub application_reference: String,
    /// _
    #[xml(child = "xne:f")]
    pub formula: crate::schemas::schemas_microsoft_com_office_excel_2006_main::Formula,
}
/// Defines the TimelineCacheReference Class.
/// When the object is serialized out as xml, it's qualified name is x15:timelineCacheRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:timelineCacheRef")]
pub struct TimelineCacheReference {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the TimelineReference Class.
/// When the object is serialized out as xml, it's qualified name is x15:timelineRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:timelineRef")]
pub struct TimelineReference {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the TimelineStyle Class.
/// When the object is serialized out as xml, it's qualified name is x15:timelineStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:timelineStyle")]
pub struct TimelineStyle {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// _
    #[xml(child = "x15:timelineStyleElements")]
    pub timeline_style_elements: Option<TimelineStyleElements>,
}
/// Defines the TimelineStyleElement Class.
/// When the object is serialized out as xml, it's qualified name is x15:timelineStyleElement.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:timelineStyleElement")]
pub struct TimelineStyleElement {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: TimelineStyleType,
    /// dxfId
    /// Represents the following attribute in the schema: :dxfId
    #[xml(attr = "dxfId")]
    pub format_id: Option<i32>,
}
/// Defines the TimelineStyleElements Class.
/// When the object is serialized out as xml, it's qualified name is x15:timelineStyleElements.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:timelineStyleElements")]
pub struct TimelineStyleElements {
    /// _
    #[xml(child = "x15:timelineStyleElement")]
    pub x15_timeline_style_element: Vec<TimelineStyleElement>,
}
/// Defines the DbTable Class.
/// When the object is serialized out as xml, it's qualified name is x15:dbTable.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:dbTable")]
pub struct DbTable {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
}
/// Defines the DbTables Class.
/// When the object is serialized out as xml, it's qualified name is x15:dbTables.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:dbTables")]
pub struct DbTables {
    /// _
    #[xml(child = "x15:dbTable")]
    pub x15_db_table: Vec<DbTable>,
}
/// Defines the DbCommand Class.
/// When the object is serialized out as xml, it's qualified name is x15:dbCommand.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:dbCommand")]
pub struct DbCommand {
    /// text
    /// Represents the following attribute in the schema: :text
    #[xml(attr = "text")]
    pub text: String,
}
/// Defines the TextProperties Class.
/// When the object is serialized out as xml, it's qualified name is x15:textPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:textPr")]
pub struct TextProperties {
    /// prompt
    /// Represents the following attribute in the schema: :prompt
    #[xml(attr = "prompt")]
    pub prompt: Option<bool>,
    /// fileType
    /// Represents the following attribute in the schema: :fileType
    #[xml(attr = "fileType")]
    pub file_type: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::FileTypeValues,
    >,
    /// codePage
    /// Represents the following attribute in the schema: :codePage
    #[xml(attr = "codePage")]
    pub code_page: Option<i32>,
    /// characterSet
    /// Represents the following attribute in the schema: :characterSet
    #[xml(attr = "characterSet")]
    pub text_character_set: Option<String>,
    /// firstRow
    /// Represents the following attribute in the schema: :firstRow
    #[xml(attr = "firstRow")]
    pub first_row: Option<i32>,
    /// sourceFile
    /// Represents the following attribute in the schema: :sourceFile
    #[xml(attr = "sourceFile")]
    pub source_file: Option<String>,
    /// delimited
    /// Represents the following attribute in the schema: :delimited
    #[xml(attr = "delimited")]
    pub delimited: Option<bool>,
    /// decimal
    /// Represents the following attribute in the schema: :decimal
    #[xml(attr = "decimal")]
    pub decimal: Option<String>,
    /// thousands
    /// Represents the following attribute in the schema: :thousands
    #[xml(attr = "thousands")]
    pub thousands: Option<String>,
    /// tab
    /// Represents the following attribute in the schema: :tab
    #[xml(attr = "tab")]
    pub tab_as_delimiter: Option<bool>,
    /// space
    /// Represents the following attribute in the schema: :space
    #[xml(attr = "space")]
    pub space: Option<bool>,
    /// comma
    /// Represents the following attribute in the schema: :comma
    #[xml(attr = "comma")]
    pub comma: Option<bool>,
    /// semicolon
    /// Represents the following attribute in the schema: :semicolon
    #[xml(attr = "semicolon")]
    pub semicolon: Option<bool>,
    /// consecutive
    /// Represents the following attribute in the schema: :consecutive
    #[xml(attr = "consecutive")]
    pub consecutive: Option<bool>,
    /// qualifier
    /// Represents the following attribute in the schema: :qualifier
    #[xml(attr = "qualifier")]
    pub qualifier: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::QualifierValues,
    >,
    /// delimiter
    /// Represents the following attribute in the schema: :delimiter
    #[xml(attr = "delimiter")]
    pub delimiter: Option<String>,
    /// _
    #[xml(child = "x:textFields")]
    pub text_fields: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::TextFields,
    >,
}
/// Defines the ModelTextProperties Class.
/// When the object is serialized out as xml, it's qualified name is x15:modelTextPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:modelTextPr")]
pub struct ModelTextProperties {
    /// headers
    /// Represents the following attribute in the schema: :headers
    #[xml(attr = "headers")]
    pub headers: Option<bool>,
}
/// Defines the RangeProperties Class.
/// When the object is serialized out as xml, it's qualified name is x15:rangePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:rangePr")]
pub struct RangeProperties {
    /// sourceName
    /// Represents the following attribute in the schema: :sourceName
    #[xml(attr = "sourceName")]
    pub source_name: String,
}
/// Defines the OleDbPrpoperties Class.
/// When the object is serialized out as xml, it's qualified name is x15:oledbPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:oledbPr")]
pub struct OleDbPrpoperties {
    /// connection
    /// Represents the following attribute in the schema: :connection
    #[xml(attr = "connection")]
    pub connection: Option<String>,
    #[xml(child = "x15:dbTables", child = "x15:dbCommand")]
    pub children: Vec<OleDbPrpopertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OleDbPrpopertiesChildChoice {
    #[xml(tag = "x15:dbTables")]
    X15DbTables(DbTables),
    #[xml(tag = "x15:dbCommand")]
    X15DbCommand(DbCommand),
}
/// Defines the DataFeedProperties Class.
/// When the object is serialized out as xml, it's qualified name is x15:dataFeedPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:dataFeedPr")]
pub struct DataFeedProperties {
    /// connection
    /// Represents the following attribute in the schema: :connection
    #[xml(attr = "connection")]
    pub connection: String,
    /// _
    #[xml(child = "x15:dbTables")]
    pub db_tables: DbTables,
}
/// Defines the FieldListActiveTabTopLevelEntity Class.
/// When the object is serialized out as xml, it's qualified name is x15:activeTabTopLevelEntity.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:activeTabTopLevelEntity")]
pub struct FieldListActiveTabTopLevelEntity {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<i32>,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x15:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:extLst")]
pub struct ExtensionList {
    #[xml(child = "x:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "x:ext")]
    XExt(crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension),
}
/// Defines the CachedUniqueName Class.
/// When the object is serialized out as xml, it's qualified name is x15:cachedUniqueName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:cachedUniqueName")]
pub struct CachedUniqueName {
    /// index
    /// Represents the following attribute in the schema: :index
    #[xml(attr = "index")]
    pub index: i32,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
}
/// Defines the ModelTable Class.
/// When the object is serialized out as xml, it's qualified name is x15:modelTable.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:modelTable")]
pub struct ModelTable {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// connection
    /// Represents the following attribute in the schema: :connection
    #[xml(attr = "connection")]
    pub connection: String,
}
/// Defines the ModelRelationship Class.
/// When the object is serialized out as xml, it's qualified name is x15:modelRelationship.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:modelRelationship")]
pub struct ModelRelationship {
    /// fromTable
    /// Represents the following attribute in the schema: :fromTable
    #[xml(attr = "fromTable")]
    pub from_table: String,
    /// fromColumn
    /// Represents the following attribute in the schema: :fromColumn
    #[xml(attr = "fromColumn")]
    pub from_column: String,
    /// toTable
    /// Represents the following attribute in the schema: :toTable
    #[xml(attr = "toTable")]
    pub to_table: String,
    /// toColumn
    /// Represents the following attribute in the schema: :toColumn
    #[xml(attr = "toColumn")]
    pub to_column: String,
}
/// Defines the ModelTables Class.
/// When the object is serialized out as xml, it's qualified name is x15:modelTables.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:modelTables")]
pub struct ModelTables {
    /// _
    #[xml(child = "x15:modelTable")]
    pub x15_model_table: Vec<ModelTable>,
}
/// Defines the ModelRelationships Class.
/// When the object is serialized out as xml, it's qualified name is x15:modelRelationships.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:modelRelationships")]
pub struct ModelRelationships {
    /// _
    #[xml(child = "x15:modelRelationship")]
    pub x15_model_relationship: Vec<ModelRelationship>,
}
/// Defines the PivotValueCell Class.
/// When the object is serialized out as xml, it's qualified name is x15:c.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:c")]
pub struct PivotValueCell {
    /// i
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub item: Option<i32>,
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub text: Option<SxvCellType>,
    /// _
    #[xml(child = "x15:v")]
    pub xstring: Xstring,
    /// _
    #[xml(child = "x15:x")]
    pub pivot_value_cell_extra: Option<PivotValueCellExtra>,
}
/// Defines the Xstring Class.
/// When the object is serialized out as xml, it's qualified name is x15:v.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:v")]
pub struct Xstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the PivotValueCellExtra Class.
/// When the object is serialized out as xml, it's qualified name is x15:x.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:x")]
pub struct PivotValueCellExtra {
    /// in
    /// Represents the following attribute in the schema: :in
    #[xml(attr = "in")]
    pub format_index: Option<i32>,
    /// bc
    /// Represents the following attribute in the schema: :bc
    #[xml(attr = "bc")]
    pub background_color: Option<String>,
    /// fc
    /// Represents the following attribute in the schema: :fc
    #[xml(attr = "fc")]
    pub foreground_color: Option<String>,
    /// i
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub italic: Option<bool>,
    /// un
    /// Represents the following attribute in the schema: :un
    #[xml(attr = "un")]
    pub underline: Option<bool>,
    /// st
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub strikethrough: Option<bool>,
    /// b
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bold: Option<bool>,
}
/// Defines the PivotTableServerFormats Class.
/// When the object is serialized out as xml, it's qualified name is x15:pivotTableServerFormats.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:pivotTableServerFormats")]
pub struct PivotTableServerFormats {
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: i32,
    /// _
    #[xml(child = "x15:serverFormat")]
    pub x15_server_format: Vec<ServerFormat>,
}
/// Defines the ServerFormat Class.
/// When the object is serialized out as xml, it's qualified name is x15:serverFormat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:serverFormat")]
pub struct ServerFormat {
    /// Culture
    /// Represents the following attribute in the schema: :culture
    #[xml(attr = "culture")]
    pub culture: Option<String>,
    /// Format
    /// Represents the following attribute in the schema: :format
    #[xml(attr = "format")]
    pub format: Option<String>,
}
/// Defines the SlicerCacheOlapLevelName Class.
/// When the object is serialized out as xml, it's qualified name is x15:slicerCacheOlapLevelName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:slicerCacheOlapLevelName")]
pub struct SlicerCacheOlapLevelName {
    /// uniqueName
    /// Represents the following attribute in the schema: :uniqueName
    #[xml(attr = "uniqueName")]
    pub unique_name: String,
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: i32,
}
/// Defines the SurveyPrSurveyElementPr Class.
/// When the object is serialized out as xml, it's qualified name is x15:surveyPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:surveyPr")]
pub struct SurveyPrSurveyElementPr {
    /// cssClass
    /// Represents the following attribute in the schema: :cssClass
    #[xml(attr = "cssClass")]
    pub css_class: Option<String>,
    /// bottom
    /// Represents the following attribute in the schema: :bottom
    #[xml(attr = "bottom")]
    pub bottom: Option<i32>,
    /// top
    /// Represents the following attribute in the schema: :top
    #[xml(attr = "top")]
    pub top: Option<i32>,
    /// left
    /// Represents the following attribute in the schema: :left
    #[xml(attr = "left")]
    pub left: Option<i32>,
    /// right
    /// Represents the following attribute in the schema: :right
    #[xml(attr = "right")]
    pub right: Option<i32>,
    /// width
    /// Represents the following attribute in the schema: :width
    #[xml(attr = "width")]
    pub width: Option<i32>,
    /// height
    /// Represents the following attribute in the schema: :height
    #[xml(attr = "height")]
    pub height: Option<i32>,
    /// position
    /// Represents the following attribute in the schema: :position
    #[xml(attr = "position")]
    pub position: Option<SurveyPosition>,
    /// _
    #[xml(child = "x15:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the TitlePrSurveyElementPr Class.
/// When the object is serialized out as xml, it's qualified name is x15:titlePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:titlePr")]
pub struct TitlePrSurveyElementPr {
    /// cssClass
    /// Represents the following attribute in the schema: :cssClass
    #[xml(attr = "cssClass")]
    pub css_class: Option<String>,
    /// bottom
    /// Represents the following attribute in the schema: :bottom
    #[xml(attr = "bottom")]
    pub bottom: Option<i32>,
    /// top
    /// Represents the following attribute in the schema: :top
    #[xml(attr = "top")]
    pub top: Option<i32>,
    /// left
    /// Represents the following attribute in the schema: :left
    #[xml(attr = "left")]
    pub left: Option<i32>,
    /// right
    /// Represents the following attribute in the schema: :right
    #[xml(attr = "right")]
    pub right: Option<i32>,
    /// width
    /// Represents the following attribute in the schema: :width
    #[xml(attr = "width")]
    pub width: Option<i32>,
    /// height
    /// Represents the following attribute in the schema: :height
    #[xml(attr = "height")]
    pub height: Option<i32>,
    /// position
    /// Represents the following attribute in the schema: :position
    #[xml(attr = "position")]
    pub position: Option<SurveyPosition>,
    /// _
    #[xml(child = "x15:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the DescriptionPrSurveyElementPr Class.
/// When the object is serialized out as xml, it's qualified name is x15:descriptionPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:descriptionPr")]
pub struct DescriptionPrSurveyElementPr {
    /// cssClass
    /// Represents the following attribute in the schema: :cssClass
    #[xml(attr = "cssClass")]
    pub css_class: Option<String>,
    /// bottom
    /// Represents the following attribute in the schema: :bottom
    #[xml(attr = "bottom")]
    pub bottom: Option<i32>,
    /// top
    /// Represents the following attribute in the schema: :top
    #[xml(attr = "top")]
    pub top: Option<i32>,
    /// left
    /// Represents the following attribute in the schema: :left
    #[xml(attr = "left")]
    pub left: Option<i32>,
    /// right
    /// Represents the following attribute in the schema: :right
    #[xml(attr = "right")]
    pub right: Option<i32>,
    /// width
    /// Represents the following attribute in the schema: :width
    #[xml(attr = "width")]
    pub width: Option<i32>,
    /// height
    /// Represents the following attribute in the schema: :height
    #[xml(attr = "height")]
    pub height: Option<i32>,
    /// position
    /// Represents the following attribute in the schema: :position
    #[xml(attr = "position")]
    pub position: Option<SurveyPosition>,
    /// _
    #[xml(child = "x15:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the QuestionsPrSurveyElementPr Class.
/// When the object is serialized out as xml, it's qualified name is x15:questionsPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:questionsPr")]
pub struct QuestionsPrSurveyElementPr {
    /// cssClass
    /// Represents the following attribute in the schema: :cssClass
    #[xml(attr = "cssClass")]
    pub css_class: Option<String>,
    /// bottom
    /// Represents the following attribute in the schema: :bottom
    #[xml(attr = "bottom")]
    pub bottom: Option<i32>,
    /// top
    /// Represents the following attribute in the schema: :top
    #[xml(attr = "top")]
    pub top: Option<i32>,
    /// left
    /// Represents the following attribute in the schema: :left
    #[xml(attr = "left")]
    pub left: Option<i32>,
    /// right
    /// Represents the following attribute in the schema: :right
    #[xml(attr = "right")]
    pub right: Option<i32>,
    /// width
    /// Represents the following attribute in the schema: :width
    #[xml(attr = "width")]
    pub width: Option<i32>,
    /// height
    /// Represents the following attribute in the schema: :height
    #[xml(attr = "height")]
    pub height: Option<i32>,
    /// position
    /// Represents the following attribute in the schema: :position
    #[xml(attr = "position")]
    pub position: Option<SurveyPosition>,
    /// _
    #[xml(child = "x15:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the QuestionPrSurveyElementPr Class.
/// When the object is serialized out as xml, it's qualified name is x15:questionPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:questionPr")]
pub struct QuestionPrSurveyElementPr {
    /// cssClass
    /// Represents the following attribute in the schema: :cssClass
    #[xml(attr = "cssClass")]
    pub css_class: Option<String>,
    /// bottom
    /// Represents the following attribute in the schema: :bottom
    #[xml(attr = "bottom")]
    pub bottom: Option<i32>,
    /// top
    /// Represents the following attribute in the schema: :top
    #[xml(attr = "top")]
    pub top: Option<i32>,
    /// left
    /// Represents the following attribute in the schema: :left
    #[xml(attr = "left")]
    pub left: Option<i32>,
    /// right
    /// Represents the following attribute in the schema: :right
    #[xml(attr = "right")]
    pub right: Option<i32>,
    /// width
    /// Represents the following attribute in the schema: :width
    #[xml(attr = "width")]
    pub width: Option<i32>,
    /// height
    /// Represents the following attribute in the schema: :height
    #[xml(attr = "height")]
    pub height: Option<i32>,
    /// position
    /// Represents the following attribute in the schema: :position
    #[xml(attr = "position")]
    pub position: Option<SurveyPosition>,
    /// _
    #[xml(child = "x15:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the OpenXmlSurveyElementPrElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlSurveyElementPrElement {
    /// cssClass
    /// Represents the following attribute in the schema: :cssClass
    #[xml(attr = "cssClass")]
    pub css_class: Option<String>,
    /// bottom
    /// Represents the following attribute in the schema: :bottom
    #[xml(attr = "bottom")]
    pub bottom: Option<i32>,
    /// top
    /// Represents the following attribute in the schema: :top
    #[xml(attr = "top")]
    pub top: Option<i32>,
    /// left
    /// Represents the following attribute in the schema: :left
    #[xml(attr = "left")]
    pub left: Option<i32>,
    /// right
    /// Represents the following attribute in the schema: :right
    #[xml(attr = "right")]
    pub right: Option<i32>,
    /// width
    /// Represents the following attribute in the schema: :width
    #[xml(attr = "width")]
    pub width: Option<i32>,
    /// height
    /// Represents the following attribute in the schema: :height
    #[xml(attr = "height")]
    pub height: Option<i32>,
    /// position
    /// Represents the following attribute in the schema: :position
    #[xml(attr = "position")]
    pub position: Option<SurveyPosition>,
}
/// Defines the SurveyQuestions Class.
/// When the object is serialized out as xml, it's qualified name is x15:questions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:questions")]
pub struct SurveyQuestions {
    /// _
    #[xml(child = "x15:questionsPr")]
    pub questions_pr_survey_element_pr: Option<QuestionsPrSurveyElementPr>,
    /// _
    #[xml(child = "x15:question")]
    pub x15_question: Vec<SurveyQuestion>,
}
/// Defines the SurveyQuestion Class.
/// When the object is serialized out as xml, it's qualified name is x15:question.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:question")]
pub struct SurveyQuestion {
    /// binding
    /// Represents the following attribute in the schema: :binding
    #[xml(attr = "binding")]
    pub binding: i32,
    /// text
    /// Represents the following attribute in the schema: :text
    #[xml(attr = "text")]
    pub text: Option<String>,
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<QuestionType>,
    /// format
    /// Represents the following attribute in the schema: :format
    #[xml(attr = "format")]
    pub format: Option<QuestionFormat>,
    /// helpText
    /// Represents the following attribute in the schema: :helpText
    #[xml(attr = "helpText")]
    pub help_text: Option<String>,
    /// required
    /// Represents the following attribute in the schema: :required
    #[xml(attr = "required")]
    pub required: Option<bool>,
    /// defaultValue
    /// Represents the following attribute in the schema: :defaultValue
    #[xml(attr = "defaultValue")]
    pub default_value: Option<String>,
    /// decimalPlaces
    /// Represents the following attribute in the schema: :decimalPlaces
    #[xml(attr = "decimalPlaces")]
    pub decimal_places: Option<i32>,
    /// rowSource
    /// Represents the following attribute in the schema: :rowSource
    #[xml(attr = "rowSource")]
    pub row_source: Option<String>,
    /// _
    #[xml(child = "x15:questionPr")]
    pub question_pr_survey_element_pr: Option<QuestionPrSurveyElementPr>,
    /// _
    #[xml(child = "x15:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the Timeline Class.
/// When the object is serialized out as xml, it's qualified name is x15:timeline.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:timeline")]
pub struct Timeline {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// cache
    /// Represents the following attribute in the schema: :cache
    #[xml(attr = "cache")]
    pub cache: String,
    /// caption
    /// Represents the following attribute in the schema: :caption
    #[xml(attr = "caption")]
    pub caption: Option<String>,
    /// showHeader
    /// Represents the following attribute in the schema: :showHeader
    #[xml(attr = "showHeader")]
    pub show_header: Option<bool>,
    /// showSelectionLabel
    /// Represents the following attribute in the schema: :showSelectionLabel
    #[xml(attr = "showSelectionLabel")]
    pub show_selection_label: Option<bool>,
    /// showTimeLevel
    /// Represents the following attribute in the schema: :showTimeLevel
    #[xml(attr = "showTimeLevel")]
    pub show_time_level: Option<bool>,
    /// showHorizontalScrollbar
    /// Represents the following attribute in the schema: :showHorizontalScrollbar
    #[xml(attr = "showHorizontalScrollbar")]
    pub show_horizontal_scrollbar: Option<bool>,
    /// level
    /// Represents the following attribute in the schema: :level
    #[xml(attr = "level")]
    pub level: i32,
    /// selectionLevel
    /// Represents the following attribute in the schema: :selectionLevel
    #[xml(attr = "selectionLevel")]
    pub selection_level: i32,
    /// scrollPosition
    /// Represents the following attribute in the schema: :scrollPosition
    #[xml(attr = "scrollPosition")]
    pub scroll_position: Option<String>,
    /// style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// _
    #[xml(child = "x15:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the TimelineCachePivotTable Class.
/// When the object is serialized out as xml, it's qualified name is x15:pivotTable.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:pivotTable")]
pub struct TimelineCachePivotTable {
    /// tabId
    /// Represents the following attribute in the schema: :tabId
    #[xml(attr = "tabId")]
    pub tab_id: i32,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
}
/// Defines the SelectionTimelineRange Class.
/// When the object is serialized out as xml, it's qualified name is x15:selection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:selection")]
pub struct SelectionTimelineRange {
    /// startDate
    /// Represents the following attribute in the schema: :startDate
    #[xml(attr = "startDate")]
    pub start_date: String,
    /// endDate
    /// Represents the following attribute in the schema: :endDate
    #[xml(attr = "endDate")]
    pub end_date: String,
}
/// Defines the BoundsTimelineRange Class.
/// When the object is serialized out as xml, it's qualified name is x15:bounds.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:bounds")]
pub struct BoundsTimelineRange {
    /// startDate
    /// Represents the following attribute in the schema: :startDate
    #[xml(attr = "startDate")]
    pub start_date: String,
    /// endDate
    /// Represents the following attribute in the schema: :endDate
    #[xml(attr = "endDate")]
    pub end_date: String,
}
/// Defines the TimelineRange Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TimelineRange {
    /// startDate
    /// Represents the following attribute in the schema: :startDate
    #[xml(attr = "startDate")]
    pub start_date: String,
    /// endDate
    /// Represents the following attribute in the schema: :endDate
    #[xml(attr = "endDate")]
    pub end_date: String,
}
/// Defines the AutoFilter Class.
/// When the object is serialized out as xml, it's qualified name is x15:autoFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:autoFilter")]
pub struct AutoFilter {
    /// Cell or Range Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: Option<String>,
    /// _
    #[xml(child = "x:filterColumn")]
    pub x_filter_column: Vec<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::FilterColumn,
    >,
    /// _
    #[xml(child = "x:sortState")]
    pub x_sort_state: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SortState,
    >,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList,
    >,
}
/// Defines the TimelineCachePivotTables Class.
/// When the object is serialized out as xml, it's qualified name is x15:pivotTables.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:pivotTables")]
pub struct TimelineCachePivotTables {
    /// _
    #[xml(child = "x15:pivotTable")]
    pub x15_pivot_table: Vec<TimelineCachePivotTable>,
}
/// Defines the TimelineState Class.
/// When the object is serialized out as xml, it's qualified name is x15:state.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:state")]
pub struct TimelineState {
    /// singleRangeFilterState
    /// Represents the following attribute in the schema: :singleRangeFilterState
    #[xml(attr = "singleRangeFilterState")]
    pub single_range_filter_state: Option<bool>,
    /// minimalRefreshVersion
    /// Represents the following attribute in the schema: :minimalRefreshVersion
    #[xml(attr = "minimalRefreshVersion")]
    pub minimal_refresh_version: i32,
    /// lastRefreshVersion
    /// Represents the following attribute in the schema: :lastRefreshVersion
    #[xml(attr = "lastRefreshVersion")]
    pub last_refresh_version: i32,
    /// pivotCacheId
    /// Represents the following attribute in the schema: :pivotCacheId
    #[xml(attr = "pivotCacheId")]
    pub pivot_cache_id: i32,
    /// filterType
    /// Represents the following attribute in the schema: :filterType
    #[xml(attr = "filterType")]
    pub filter_type: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotFilterValues,
    /// filterId
    /// Represents the following attribute in the schema: :filterId
    #[xml(attr = "filterId")]
    pub filter_id: Option<i32>,
    /// filterTabId
    /// Represents the following attribute in the schema: :filterTabId
    #[xml(attr = "filterTabId")]
    pub filter_tab_id: Option<i32>,
    /// filterPivotName
    /// Represents the following attribute in the schema: :filterPivotName
    #[xml(attr = "filterPivotName")]
    pub filter_pivot_name: Option<String>,
    /// _
    #[xml(child = "x15:selection")]
    pub selection_timeline_range: Option<SelectionTimelineRange>,
    /// _
    #[xml(child = "x15:bounds")]
    pub bounds_timeline_range: BoundsTimelineRange,
    /// _
    #[xml(child = "x15:movingPeriodState")]
    pub moving_period_state: Option<MovingPeriodState>,
    /// _
    #[xml(child = "x15:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotRow Class.
/// When the object is serialized out as xml, it's qualified name is x15:pivotRow.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15:pivotRow")]
pub struct PivotRow {
    /// r
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub reference: Option<i32>,
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: i32,
    /// _
    #[xml(child = "x15:c")]
    pub x15_c: Vec<PivotValueCell>,
}
