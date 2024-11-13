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
pub enum SparklineAxisMinMaxValues {
    #[default]
    Individual,
    Group,
    Custom,
}
crate::__string_enum! {
    SparklineAxisMinMaxValues { Individual = "individual", Group = "group", Custom =
    "custom", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SparklineTypeValues {
    #[default]
    Line,
    Column,
    Stacked,
}
crate::__string_enum! {
    SparklineTypeValues { Line = "line", Column = "column", Stacked = "stacked", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PivotShowAsValues {
    #[default]
    PercentOfParent,
    PercentOfParentRow,
    PercentOfParentColumn,
    PercentOfRunningTotal,
    RankAscending,
    RankDescending,
}
crate::__string_enum! {
    PivotShowAsValues { PercentOfParent = "percentOfParent", PercentOfParentRow =
    "percentOfParentRow", PercentOfParentColumn = "percentOfParentCol",
    PercentOfRunningTotal = "percentOfRunningTotal", RankAscending = "rankAscending",
    RankDescending = "rankDescending", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataBarDirectionValues {
    #[default]
    Context,
    LeftToRight,
    RightToLeft,
}
crate::__string_enum! {
    DataBarDirectionValues { Context = "context", LeftToRight = "leftToRight",
    RightToLeft = "rightToLeft", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataBarAxisPositionValues {
    #[default]
    Automatic,
    Middle,
    None,
}
crate::__string_enum! {
    DataBarAxisPositionValues { Automatic = "automatic", Middle = "middle", None =
    "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConditionalFormattingValueObjectTypeValues {
    #[default]
    Numeric,
    Percent,
    Max,
    Min,
    Formula,
    Percentile,
    AutoMin,
    AutoMax,
}
crate::__string_enum! {
    ConditionalFormattingValueObjectTypeValues { Numeric = "num", Percent = "percent",
    Max = "max", Min = "min", Formula = "formula", Percentile = "percentile", AutoMin =
    "autoMin", AutoMax = "autoMax", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum IconSetTypeValues {
    #[default]
    ThreeArrows,
    ThreeArrowsGray,
    ThreeFlags,
    ThreeTrafficLights1,
    ThreeTrafficLights2,
    ThreeSigns,
    ThreeSymbols,
    ThreeSymbols2,
    FourArrows,
    FourArrowsGray,
    FourRedToBlack,
    FourRating,
    FourTrafficLights,
    FiveArrows,
    FiveArrowsGray,
    FiveRating,
    FiveQuarters,
    ThreeStars,
    ThreeTriangles,
    FiveBoxes,
    NoIcons,
}
crate::__string_enum! {
    IconSetTypeValues { ThreeArrows = "3arrows", ThreeArrowsGray = "3arrowsGray",
    ThreeFlags = "3flags", ThreeTrafficLights1 = "3trafficLights1", ThreeTrafficLights2 =
    "3trafficLights2", ThreeSigns = "3signs", ThreeSymbols = "3symbols", ThreeSymbols2 =
    "3symbols2", FourArrows = "4arrows", FourArrowsGray = "4arrowsGray", FourRedToBlack =
    "4redToBlack", FourRating = "4rating", FourTrafficLights = "4trafficLights",
    FiveArrows = "5arrows", FiveArrowsGray = "5arrowsGray", FiveRating = "5rating",
    FiveQuarters = "5quarters", ThreeStars = "3stars", ThreeTriangles = "3triangles",
    FiveBoxes = "5boxes", NoIcons = "noIcons", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PivotEditValueTypeValues {
    #[default]
    Number,
    DateTime,
    String,
    Boolean,
    Error,
}
crate::__string_enum! {
    PivotEditValueTypeValues { Number = "number", DateTime = "dateTime", String =
    "string", Boolean = "boolean", Error = "error", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AllocationMethodValues {
    #[default]
    EqualAllocation,
    EqualIncrement,
    WeightedAllocation,
    WeightedIncrement,
}
crate::__string_enum! {
    AllocationMethodValues { EqualAllocation = "equalAllocation", EqualIncrement =
    "equalIncrement", WeightedAllocation = "weightedAllocation", WeightedIncrement =
    "weightedIncrement", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SlicerStyleTypeValues {
    #[default]
    UnselectedItemWithData,
    SelectedItemWithData,
    UnselectedItemWithNoData,
    SelectedItemWithNoData,
    HoveredUnselectedItemWithData,
    HoveredSelectedItemWithData,
    HoveredUnselectedItemWithNoData,
    HoveredSelectedItemWithNoData,
}
crate::__string_enum! {
    SlicerStyleTypeValues { UnselectedItemWithData = "unselectedItemWithData",
    SelectedItemWithData = "selectedItemWithData", UnselectedItemWithNoData =
    "unselectedItemWithNoData", SelectedItemWithNoData = "selectedItemWithNoData",
    HoveredUnselectedItemWithData = "hoveredUnselectedItemWithData",
    HoveredSelectedItemWithData = "hoveredSelectedItemWithData",
    HoveredUnselectedItemWithNoData = "hoveredUnselectedItemWithNoData",
    HoveredSelectedItemWithNoData = "hoveredSelectedItemWithNoData", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CheckedValues {
    #[default]
    Unchecked,
    Checked,
    Mixed,
}
crate::__string_enum! {
    CheckedValues { Unchecked = "unchecked", Checked = "checked", Mixed = "mixed", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DropStyleValues {
    #[default]
    Combo,
    ComboEdit,
    Simple,
}
crate::__string_enum! {
    DropStyleValues { Combo = "combo", ComboEdit = "comboedit", Simple = "simple", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SelectionTypeValues {
    #[default]
    Single,
    Multiple,
    Extended,
}
crate::__string_enum! {
    SelectionTypeValues { Single = "single", Multiple = "multi", Extended = "extended", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextHorizontalAlignmentValues {
    #[default]
    Left,
    Center,
    Right,
    Justify,
    Distributed,
}
crate::__string_enum! {
    TextHorizontalAlignmentValues { Left = "left", Center = "center", Right = "right",
    Justify = "justify", Distributed = "distributed", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextVerticalAlignmentValues {
    #[default]
    Top,
    Center,
    Bottom,
    Justify,
    Distributed,
}
crate::__string_enum! {
    TextVerticalAlignmentValues { Top = "top", Center = "center", Bottom = "bottom",
    Justify = "justify", Distributed = "distributed", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EditValidationValues {
    #[default]
    Text,
    Integer,
    Number,
    Reference,
    Formula,
}
crate::__string_enum! {
    EditValidationValues { Text = "text", Integer = "integer", Number = "number",
    Reference = "reference", Formula = "formula", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OlapSlicerCacheSortOrderValues {
    #[default]
    Natural,
    Ascending,
    Descending,
}
crate::__string_enum! {
    OlapSlicerCacheSortOrderValues { Natural = "natural", Ascending = "ascending",
    Descending = "descending", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TabularSlicerCacheSortOrderValues {
    #[default]
    Ascending,
    Descending,
}
crate::__string_enum! {
    TabularSlicerCacheSortOrderValues { Ascending = "ascending", Descending =
    "descending", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SlicerCacheCrossFilterValues {
    #[default]
    None,
    ShowItemsWithDataAtTop,
    ShowItemsWithNoData,
}
crate::__string_enum! {
    SlicerCacheCrossFilterValues { None = "none", ShowItemsWithDataAtTop =
    "showItemsWithDataAtTop", ShowItemsWithNoData = "showItemsWithNoData", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ObjectTypeValues {
    #[default]
    Button,
    CheckBox,
    Drop,
    GroupBox,
    Label,
    List,
    Radio,
    Scroll,
    Spin,
    EditBox,
    Dialog,
}
crate::__string_enum! {
    ObjectTypeValues { Button = "button", CheckBox = "checkBox", Drop = "drop", GroupBox
    = "gBox", Label = "label", List = "list", Radio = "radio", Scroll = "scroll", Spin =
    "spin", EditBox = "editBox", Dialog = "dialog", }
}
/// Defines the ConditionalFormattings Class.
/// When the object is serialized out as xml, it's qualified name is x14:conditionalFormattings.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:conditionalFormattings")]
pub struct ConditionalFormattings {
    /// _
    #[xml(child = "x14:conditionalFormatting")]
    pub x14_conditional_formatting: Vec<ConditionalFormatting>,
}
/// Defines the DataValidations Class.
/// When the object is serialized out as xml, it's qualified name is x14:dataValidations.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:dataValidations")]
pub struct DataValidations {
    /// disablePrompts
    /// Represents the following attribute in the schema: :disablePrompts
    #[xml(attr = "disablePrompts")]
    pub disable_prompts: Option<bool>,
    /// xWindow
    /// Represents the following attribute in the schema: :xWindow
    #[xml(attr = "xWindow")]
    pub x_window: Option<i32>,
    /// yWindow
    /// Represents the following attribute in the schema: :yWindow
    #[xml(attr = "yWindow")]
    pub y_window: Option<i32>,
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x14:dataValidation")]
    pub x14_data_validation: Vec<DataValidation>,
}
/// Defines the SparklineGroups Class.
/// When the object is serialized out as xml, it's qualified name is x14:sparklineGroups.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:sparklineGroups")]
pub struct SparklineGroups {
    /// _
    #[xml(child = "x14:sparklineGroup")]
    pub x14_sparkline_group: Vec<SparklineGroup>,
}
/// Defines the SlicerList Class.
/// When the object is serialized out as xml, it's qualified name is x14:slicerList.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:slicerList")]
pub struct SlicerList {
    /// _
    #[xml(child = "x14:slicer")]
    pub x14_slicer: Vec<SlicerRef>,
}
/// Defines the ProtectedRanges Class.
/// When the object is serialized out as xml, it's qualified name is x14:protectedRanges.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:protectedRanges")]
pub struct ProtectedRanges {
    /// _
    #[xml(child = "x14:protectedRange")]
    pub x14_protected_range: Vec<ProtectedRange>,
}
/// Defines the IgnoredErrors Class.
/// When the object is serialized out as xml, it's qualified name is x14:ignoredErrors.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:ignoredErrors")]
pub struct IgnoredErrors {
    /// _
    #[xml(child = "x14:ignoredError")]
    pub x14_ignored_error: Vec<IgnoredError>,
    /// _
    #[xml(child = "x14:extLst")]
    pub x14_ext_lst: Option<ExtensionList>,
}
/// Defines the DefinedNames Class.
/// When the object is serialized out as xml, it's qualified name is x14:definedNames.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:definedNames")]
pub struct DefinedNames {
    /// _
    #[xml(child = "x14:definedName")]
    pub x14_defined_name: Vec<DefinedName>,
}
/// Defines the PivotCaches Class.
/// When the object is serialized out as xml, it's qualified name is x14:pivotCaches.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:pivotCaches")]
pub struct PivotCaches {
    /// _
    #[xml(child = "x:pivotCache")]
    pub x_pivot_cache: Vec<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCache,
    >,
}
/// Defines the SlicerCaches Class.
/// When the object is serialized out as xml, it's qualified name is x14:slicerCaches.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:slicerCaches")]
pub struct SlicerCaches {
    /// _
    #[xml(child = "x14:slicerCache")]
    pub x14_slicer_cache: Vec<SlicerCache>,
}
/// Defines the WorkbookProperties Class.
/// When the object is serialized out as xml, it's qualified name is x14:workbookPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:workbookPr")]
pub struct WorkbookProperties {
    /// defaultImageDpi
    /// Represents the following attribute in the schema: :defaultImageDpi
    #[xml(attr = "defaultImageDpi")]
    pub default_image_dpi: Option<i32>,
    /// discardImageEditData
    /// Represents the following attribute in the schema: :discardImageEditData
    #[xml(attr = "discardImageEditData")]
    pub discard_image_edit_data: Option<bool>,
    /// accuracyVersion
    /// Represents the following attribute in the schema: :accuracyVersion
    #[xml(attr = "accuracyVersion")]
    pub accuracy_version: Option<i32>,
}
/// Defines the CalculatedMember Class.
/// When the object is serialized out as xml, it's qualified name is x14:calculatedMember.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:calculatedMember")]
pub struct CalculatedMember {
    /// displayFolder
    /// Represents the following attribute in the schema: :displayFolder
    #[xml(attr = "displayFolder")]
    pub display_folder: Option<String>,
    /// flattenHierarchies
    /// Represents the following attribute in the schema: :flattenHierarchies
    #[xml(attr = "flattenHierarchies")]
    pub flatten_hierarchies: Option<bool>,
    /// dynamicSet
    /// Represents the following attribute in the schema: :dynamicSet
    #[xml(attr = "dynamicSet")]
    pub dynamic_set: Option<bool>,
    /// hierarchizeDistinct
    /// Represents the following attribute in the schema: :hierarchizeDistinct
    #[xml(attr = "hierarchizeDistinct")]
    pub hierarchize_distinct: Option<bool>,
    /// mdxLong
    /// Represents the following attribute in the schema: :mdxLong
    #[xml(attr = "mdxLong")]
    pub mdx_long: Option<String>,
    /// _
    #[xml(child = "x14:tupleSet")]
    pub tuple_set: Option<TupleSet>,
}
/// Defines the CacheHierarchy Class.
/// When the object is serialized out as xml, it's qualified name is x14:cacheHierarchy.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:cacheHierarchy")]
pub struct CacheHierarchy {
    /// flattenHierarchies
    /// Represents the following attribute in the schema: :flattenHierarchies
    #[xml(attr = "flattenHierarchies")]
    pub flatten_hierarchies: Option<bool>,
    /// measuresSet
    /// Represents the following attribute in the schema: :measuresSet
    #[xml(attr = "measuresSet")]
    pub measures_set: Option<bool>,
    /// hierarchizeDistinct
    /// Represents the following attribute in the schema: :hierarchizeDistinct
    #[xml(attr = "hierarchizeDistinct")]
    pub hierarchize_distinct: Option<bool>,
    /// ignore
    /// Represents the following attribute in the schema: :ignore
    #[xml(attr = "ignore")]
    pub ignore: Option<bool>,
    /// _
    #[xml(child = "x14:setLevels")]
    pub set_levels: Option<SetLevels>,
}
/// Defines the DataField Class.
/// When the object is serialized out as xml, it's qualified name is x14:dataField.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:dataField")]
pub struct DataField {
    /// pivotShowAs
    /// Represents the following attribute in the schema: :pivotShowAs
    #[xml(attr = "pivotShowAs")]
    pub pivot_show_as: Option<PivotShowAsValues>,
    /// sourceField
    /// Represents the following attribute in the schema: :sourceField
    #[xml(attr = "sourceField")]
    pub source_field: Option<i32>,
    /// uniqueName
    /// Represents the following attribute in the schema: :uniqueName
    #[xml(attr = "uniqueName")]
    pub unique_name: Option<String>,
}
/// Defines the PivotField Class.
/// When the object is serialized out as xml, it's qualified name is x14:pivotField.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:pivotField")]
pub struct PivotField {
    /// fillDownLabels
    /// Represents the following attribute in the schema: :fillDownLabels
    #[xml(attr = "fillDownLabels")]
    pub fill_down_labels: Option<bool>,
    /// ignore
    /// Represents the following attribute in the schema: :ignore
    #[xml(attr = "ignore")]
    pub ignore: Option<bool>,
}
/// Defines the PivotTableDefinition Class.
/// When the object is serialized out as xml, it's qualified name is x14:pivotTableDefinition.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:pivotTableDefinition")]
pub struct PivotTableDefinition {
    /// fillDownLabelsDefault
    /// Represents the following attribute in the schema: :fillDownLabelsDefault
    #[xml(attr = "fillDownLabelsDefault")]
    pub fill_down_labels_default: Option<bool>,
    /// visualTotalsForSets
    /// Represents the following attribute in the schema: :visualTotalsForSets
    #[xml(attr = "visualTotalsForSets")]
    pub visual_totals_for_sets: Option<bool>,
    /// calculatedMembersInFilters
    /// Represents the following attribute in the schema: :calculatedMembersInFilters
    #[xml(attr = "calculatedMembersInFilters")]
    pub calculated_members_in_filters: Option<bool>,
    /// altText
    /// Represents the following attribute in the schema: :altText
    #[xml(attr = "altText")]
    pub alt_text: Option<String>,
    /// altTextSummary
    /// Represents the following attribute in the schema: :altTextSummary
    #[xml(attr = "altTextSummary")]
    pub alt_text_summary: Option<String>,
    /// enableEdit
    /// Represents the following attribute in the schema: :enableEdit
    #[xml(attr = "enableEdit")]
    pub enable_edit: Option<bool>,
    /// autoApply
    /// Represents the following attribute in the schema: :autoApply
    #[xml(attr = "autoApply")]
    pub auto_apply: Option<bool>,
    /// allocationMethod
    /// Represents the following attribute in the schema: :allocationMethod
    #[xml(attr = "allocationMethod")]
    pub allocation_method: Option<AllocationMethodValues>,
    /// weightExpression
    /// Represents the following attribute in the schema: :weightExpression
    #[xml(attr = "weightExpression")]
    pub weight_expression: Option<String>,
    /// hideValuesRow
    /// Represents the following attribute in the schema: :hideValuesRow
    #[xml(attr = "hideValuesRow")]
    pub hide_values_row: Option<bool>,
    /// _
    #[xml(child = "x14:pivotEdits")]
    pub pivot_edits: Option<PivotEdits>,
    /// _
    #[xml(child = "x14:pivotChanges")]
    pub pivot_changes: Option<PivotChanges>,
    /// _
    #[xml(child = "x14:conditionalFormats")]
    pub conditional_formats: Option<ConditionalFormats>,
}
/// Defines the PivotCacheDefinition Class.
/// When the object is serialized out as xml, it's qualified name is x14:pivotCacheDefinition.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:pivotCacheDefinition")]
pub struct PivotCacheDefinition {
    /// slicerData
    /// Represents the following attribute in the schema: :slicerData
    #[xml(attr = "slicerData")]
    pub slicer_data: Option<bool>,
    /// pivotCacheId
    /// Represents the following attribute in the schema: :pivotCacheId
    #[xml(attr = "pivotCacheId")]
    pub pivot_cache_id: Option<i32>,
    /// supportSubqueryNonVisual
    /// Represents the following attribute in the schema: :supportSubqueryNonVisual
    #[xml(attr = "supportSubqueryNonVisual")]
    pub support_subquery_non_visual: Option<bool>,
    /// supportSubqueryCalcMem
    /// Represents the following attribute in the schema: :supportSubqueryCalcMem
    #[xml(attr = "supportSubqueryCalcMem")]
    pub support_subquery_calc_mem: Option<bool>,
    /// supportAddCalcMems
    /// Represents the following attribute in the schema: :supportAddCalcMems
    #[xml(attr = "supportAddCalcMems")]
    pub support_add_calc_mems: Option<bool>,
}
/// Defines the Connection Class.
/// When the object is serialized out as xml, it's qualified name is x14:connection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:connection")]
pub struct Connection {
    /// culture
    /// Represents the following attribute in the schema: :culture
    #[xml(attr = "culture")]
    pub culture: Option<String>,
    /// embeddedDataId
    /// Represents the following attribute in the schema: :embeddedDataId
    #[xml(attr = "embeddedDataId")]
    pub embedded_data_id: Option<String>,
    /// _
    #[xml(child = "x14:calculatedMembers")]
    pub calculated_members: Option<CalculatedMembers>,
}
/// Defines the Table Class.
/// When the object is serialized out as xml, it's qualified name is x14:table.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:table")]
pub struct Table {
    /// altText
    /// Represents the following attribute in the schema: :altText
    #[xml(attr = "altText")]
    pub alt_text: Option<String>,
    /// altTextSummary
    /// Represents the following attribute in the schema: :altTextSummary
    #[xml(attr = "altTextSummary")]
    pub alt_text_summary: Option<String>,
}
/// Defines the SlicerStyles Class.
/// When the object is serialized out as xml, it's qualified name is x14:slicerStyles.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:slicerStyles")]
pub struct SlicerStyles {
    /// defaultSlicerStyle
    /// Represents the following attribute in the schema: :defaultSlicerStyle
    #[xml(attr = "defaultSlicerStyle")]
    pub default_slicer_style: String,
    /// _
    #[xml(child = "x14:slicerStyle")]
    pub x14_slicer_style: Vec<SlicerStyle>,
}
/// Defines the DifferentialFormats Class.
/// When the object is serialized out as xml, it's qualified name is x14:dxfs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:dxfs")]
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
/// Defines the OleItem Class.
/// When the object is serialized out as xml, it's qualified name is x14:oleItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:oleItem")]
pub struct OleItem {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// icon
    /// Represents the following attribute in the schema: :icon
    #[xml(attr = "icon")]
    pub icon: Option<bool>,
    /// advise
    /// Represents the following attribute in the schema: :advise
    #[xml(attr = "advise")]
    pub advise: Option<bool>,
    /// preferPic
    /// Represents the following attribute in the schema: :preferPic
    #[xml(attr = "preferPic")]
    pub prefer_picture: Option<bool>,
    /// _
    #[xml(child = "x14:values")]
    pub dde_values: Option<DdeValues>,
}
/// Defines the PivotHierarchy Class.
/// When the object is serialized out as xml, it's qualified name is x14:pivotHierarchy.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:pivotHierarchy")]
pub struct PivotHierarchy {
    /// ignore
    /// Represents the following attribute in the schema: :ignore
    #[xml(attr = "ignore")]
    pub ignore: Option<bool>,
}
/// Defines the CacheField Class.
/// When the object is serialized out as xml, it's qualified name is x14:cacheField.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:cacheField")]
pub struct CacheField {
    /// ignore
    /// Represents the following attribute in the schema: :ignore
    #[xml(attr = "ignore")]
    pub ignore: Option<bool>,
}
/// Defines the Id Class.
/// When the object is serialized out as xml, it's qualified name is x14:id.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:id")]
pub struct Id {
    #[xml(text)]
    pub child: String,
}
/// Defines the IconFilter Class.
/// When the object is serialized out as xml, it's qualified name is x14:iconFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:iconFilter")]
pub struct IconFilter {
    /// iconSet
    /// Represents the following attribute in the schema: :iconSet
    #[xml(attr = "iconSet")]
    pub icon_set: IconSetTypeValues,
    /// iconId
    /// Represents the following attribute in the schema: :iconId
    #[xml(attr = "iconId")]
    pub icon_id: i32,
}
/// Defines the Filter Class.
/// When the object is serialized out as xml, it's qualified name is x14:filter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:filter")]
pub struct Filter {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<String>,
}
/// Defines the CustomFilters Class.
/// When the object is serialized out as xml, it's qualified name is x14:customFilters.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:customFilters")]
pub struct CustomFilters {
    /// and
    /// Represents the following attribute in the schema: :and
    #[xml(attr = "and")]
    pub and: Option<bool>,
    /// _
    #[xml(child = "x14:customFilter")]
    pub x14_custom_filter: Vec<CustomFilter>,
}
/// Defines the SortCondition Class.
/// When the object is serialized out as xml, it's qualified name is x14:sortCondition.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:sortCondition")]
pub struct SortCondition {
    /// descending
    /// Represents the following attribute in the schema: :descending
    #[xml(attr = "descending")]
    pub descending: Option<bool>,
    /// sortBy
    /// Represents the following attribute in the schema: :sortBy
    #[xml(attr = "sortBy")]
    pub sort_by: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SortByValues,
    >,
    /// ref
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: String,
    /// customList
    /// Represents the following attribute in the schema: :customList
    #[xml(attr = "customList")]
    pub custom_list: Option<String>,
    /// dxfId
    /// Represents the following attribute in the schema: :dxfId
    #[xml(attr = "dxfId")]
    pub format_id: Option<i32>,
    /// iconSet
    /// Represents the following attribute in the schema: :iconSet
    #[xml(attr = "iconSet")]
    pub icon_set: Option<IconSetTypeValues>,
    /// iconId
    /// Represents the following attribute in the schema: :iconId
    #[xml(attr = "iconId")]
    pub icon_id: Option<i32>,
}
/// Defines the SourceConnection Class.
/// When the object is serialized out as xml, it's qualified name is x14:sourceConnection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:sourceConnection")]
pub struct SourceConnection {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
}
/// Defines the DatastoreItem Class.
/// When the object is serialized out as xml, it's qualified name is x14:datastoreItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:datastoreItem")]
pub struct DatastoreItem {
    #[xml(attr = "xmlns", with = "datastore_item_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// _
    #[xml(child = "x14:extLst")]
    pub extension_list: Option<ExtensionList>,
}
mod datastore_item_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/spreadsheetml/2009/9/main")
    }
}
/// Defines the FormControlProperties Class.
/// When the object is serialized out as xml, it's qualified name is x14:formControlPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:formControlPr")]
pub struct FormControlProperties {
    #[xml(attr = "xmlns", with = "form_control_properties_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// objectType
    /// Represents the following attribute in the schema: :objectType
    #[xml(attr = "objectType")]
    pub object_type: Option<ObjectTypeValues>,
    /// checked
    /// Represents the following attribute in the schema: :checked
    #[xml(attr = "checked")]
    pub checked: Option<CheckedValues>,
    /// colored
    /// Represents the following attribute in the schema: :colored
    #[xml(attr = "colored")]
    pub colored: Option<bool>,
    /// dropLines
    /// Represents the following attribute in the schema: :dropLines
    #[xml(attr = "dropLines")]
    pub drop_lines: Option<i32>,
    /// dropStyle
    /// Represents the following attribute in the schema: :dropStyle
    #[xml(attr = "dropStyle")]
    pub drop_style: Option<DropStyleValues>,
    /// dx
    /// Represents the following attribute in the schema: :dx
    #[xml(attr = "dx")]
    pub scroll_bar_width: Option<i32>,
    /// firstButton
    /// Represents the following attribute in the schema: :firstButton
    #[xml(attr = "firstButton")]
    pub first_button: Option<bool>,
    /// fmlaGroup
    /// Represents the following attribute in the schema: :fmlaGroup
    #[xml(attr = "fmlaGroup")]
    pub fmla_group: Option<String>,
    /// fmlaLink
    /// Represents the following attribute in the schema: :fmlaLink
    #[xml(attr = "fmlaLink")]
    pub fmla_link: Option<String>,
    /// fmlaRange
    /// Represents the following attribute in the schema: :fmlaRange
    #[xml(attr = "fmlaRange")]
    pub fmla_range: Option<String>,
    /// fmlaTxbx
    /// Represents the following attribute in the schema: :fmlaTxbx
    #[xml(attr = "fmlaTxbx")]
    pub fmla_textbox: Option<String>,
    /// horiz
    /// Represents the following attribute in the schema: :horiz
    #[xml(attr = "horiz")]
    pub horizontal: Option<bool>,
    /// inc
    /// Represents the following attribute in the schema: :inc
    #[xml(attr = "inc")]
    pub incremental: Option<i32>,
    /// justLastX
    /// Represents the following attribute in the schema: :justLastX
    #[xml(attr = "justLastX")]
    pub just_last_x: Option<bool>,
    /// lockText
    /// Represents the following attribute in the schema: :lockText
    #[xml(attr = "lockText")]
    pub lock_text: Option<bool>,
    /// max
    /// Represents the following attribute in the schema: :max
    #[xml(attr = "max")]
    pub max: Option<i32>,
    /// min
    /// Represents the following attribute in the schema: :min
    #[xml(attr = "min")]
    pub min: Option<i32>,
    /// multiSel
    /// Represents the following attribute in the schema: :multiSel
    #[xml(attr = "multiSel")]
    pub multiple_selection: Option<String>,
    /// noThreeD
    /// Represents the following attribute in the schema: :noThreeD
    #[xml(attr = "noThreeD")]
    pub no_three_d: Option<bool>,
    /// noThreeD2
    /// Represents the following attribute in the schema: :noThreeD2
    #[xml(attr = "noThreeD2")]
    pub no_three_d2: Option<bool>,
    /// page
    /// Represents the following attribute in the schema: :page
    #[xml(attr = "page")]
    pub page: Option<i32>,
    /// sel
    /// Represents the following attribute in the schema: :sel
    #[xml(attr = "sel")]
    pub selected: Option<i32>,
    /// seltype
    /// Represents the following attribute in the schema: :seltype
    #[xml(attr = "seltype")]
    pub selection_type: Option<SelectionTypeValues>,
    /// textHAlign
    /// Represents the following attribute in the schema: :textHAlign
    #[xml(attr = "textHAlign")]
    pub text_horizontal_align: Option<TextHorizontalAlignmentValues>,
    /// textVAlign
    /// Represents the following attribute in the schema: :textVAlign
    #[xml(attr = "textVAlign")]
    pub text_vertical_align: Option<TextVerticalAlignmentValues>,
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<i32>,
    /// widthMin
    /// Represents the following attribute in the schema: :widthMin
    #[xml(attr = "widthMin")]
    pub minimum_width: Option<i32>,
    /// editVal
    /// Represents the following attribute in the schema: :editVal
    #[xml(attr = "editVal")]
    pub edit_val: Option<EditValidationValues>,
    /// multiLine
    /// Represents the following attribute in the schema: :multiLine
    #[xml(attr = "multiLine")]
    pub multiple_lines: Option<bool>,
    /// verticalBar
    /// Represents the following attribute in the schema: :verticalBar
    #[xml(attr = "verticalBar")]
    pub vertical_bar: Option<bool>,
    /// passwordEdit
    /// Represents the following attribute in the schema: :passwordEdit
    #[xml(attr = "passwordEdit")]
    pub password_edit: Option<bool>,
    /// _
    #[xml(child = "x14:itemLst")]
    pub list_items: Option<ListItems>,
    /// _
    #[xml(child = "x14:extLst")]
    pub extension_list: Option<ExtensionList>,
}
mod form_control_properties_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/spreadsheetml/2009/9/main")
    }
}
/// Defines the Slicers Class.
/// When the object is serialized out as xml, it's qualified name is x14:slicers.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:slicers")]
pub struct Slicers {
    #[xml(attr = "xmlns", with = "slicers_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "x14:slicer")]
    pub x14_slicer: Vec<Slicer>,
}
mod slicers_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/spreadsheetml/2009/9/main")
    }
}
/// Defines the SlicerCacheDefinition Class.
/// When the object is serialized out as xml, it's qualified name is x14:slicerCacheDefinition.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:slicerCacheDefinition")]
pub struct SlicerCacheDefinition {
    #[xml(attr = "xmlns", with = "slicer_cache_definition_xmlns_derive")]
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
    #[xml(child = "x14:pivotTables")]
    pub slicer_cache_pivot_tables: Option<SlicerCachePivotTables>,
    /// _
    #[xml(child = "x14:data")]
    pub slicer_cache_data: Option<SlicerCacheData>,
    /// _
    #[xml(child = "x14:extLst")]
    pub slicer_cache_definition_extension_list: Option<
        SlicerCacheDefinitionExtensionList,
    >,
}
mod slicer_cache_definition_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/spreadsheetml/2009/9/main")
    }
}
/// Defines the ConditionalFormatting Class.
/// When the object is serialized out as xml, it's qualified name is x14:conditionalFormatting.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:conditionalFormatting")]
pub struct ConditionalFormatting {
    /// pivot
    /// Represents the following attribute in the schema: :pivot
    #[xml(attr = "pivot")]
    pub pivot: Option<bool>,
    /// _
    #[xml(child = "x14:cfRule")]
    pub x14_cf_rule: Vec<ConditionalFormattingRule>,
    /// _
    #[xml(child = "xne:sqref")]
    pub xne_sqref: Option<
        crate::schemas::schemas_microsoft_com_office_excel_2006_main::ReferenceSequence,
    >,
    /// _
    #[xml(child = "x14:extLst")]
    pub x14_ext_lst: Option<ExtensionList>,
}
/// Defines the ConditionalFormattingRule Class.
/// When the object is serialized out as xml, it's qualified name is x14:cfRule.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:cfRule")]
pub struct ConditionalFormattingRule {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ConditionalFormatValues,
    >,
    /// priority
    /// Represents the following attribute in the schema: :priority
    #[xml(attr = "priority")]
    pub priority: Option<i32>,
    /// stopIfTrue
    /// Represents the following attribute in the schema: :stopIfTrue
    #[xml(attr = "stopIfTrue")]
    pub stop_if_true: Option<bool>,
    /// aboveAverage
    /// Represents the following attribute in the schema: :aboveAverage
    #[xml(attr = "aboveAverage")]
    pub above_average: Option<bool>,
    /// percent
    /// Represents the following attribute in the schema: :percent
    #[xml(attr = "percent")]
    pub percent: Option<bool>,
    /// bottom
    /// Represents the following attribute in the schema: :bottom
    #[xml(attr = "bottom")]
    pub bottom: Option<bool>,
    /// operator
    /// Represents the following attribute in the schema: :operator
    #[xml(attr = "operator")]
    pub operator: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ConditionalFormattingOperatorValues,
    >,
    /// text
    /// Represents the following attribute in the schema: :text
    #[xml(attr = "text")]
    pub text: Option<String>,
    /// timePeriod
    /// Represents the following attribute in the schema: :timePeriod
    #[xml(attr = "timePeriod")]
    pub time_period: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::TimePeriodValues,
    >,
    /// rank
    /// Represents the following attribute in the schema: :rank
    #[xml(attr = "rank")]
    pub rank: Option<i32>,
    /// stdDev
    /// Represents the following attribute in the schema: :stdDev
    #[xml(attr = "stdDev")]
    pub standard_deviation: Option<i32>,
    /// equalAverage
    /// Represents the following attribute in the schema: :equalAverage
    #[xml(attr = "equalAverage")]
    pub equal_average: Option<bool>,
    /// activePresent
    /// Represents the following attribute in the schema: :activePresent
    #[xml(attr = "activePresent")]
    pub active_present: Option<bool>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// _
    #[xml(child = "xne:f")]
    pub xne_f: Vec<
        crate::schemas::schemas_microsoft_com_office_excel_2006_main::Formula,
    >,
    /// _
    #[xml(child = "x14:colorScale")]
    pub x14_color_scale: Option<ColorScale>,
    /// _
    #[xml(child = "x14:dataBar")]
    pub x14_data_bar: Option<DataBar>,
    /// _
    #[xml(child = "x14:iconSet")]
    pub x14_icon_set: Option<IconSet>,
    /// _
    #[xml(child = "x14:dxf")]
    pub x14_dxf: Option<DifferentialType>,
    /// _
    #[xml(child = "x14:extLst")]
    pub x14_ext_lst: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x14:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:extLst")]
pub struct ExtensionList {
    #[xml(child = "x:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "x:ext")]
    XExt(crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension),
}
/// Defines the DataValidation Class.
/// When the object is serialized out as xml, it's qualified name is x14:dataValidation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:dataValidation")]
pub struct DataValidation {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationValues,
    >,
    /// errorStyle
    /// Represents the following attribute in the schema: :errorStyle
    #[xml(attr = "errorStyle")]
    pub error_style: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationErrorStyleValues,
    >,
    /// imeMode
    /// Represents the following attribute in the schema: :imeMode
    #[xml(attr = "imeMode")]
    pub ime_mode: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationImeModeValues,
    >,
    /// operator
    /// Represents the following attribute in the schema: :operator
    #[xml(attr = "operator")]
    pub operator: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationOperatorValues,
    >,
    /// allowBlank
    /// Represents the following attribute in the schema: :allowBlank
    #[xml(attr = "allowBlank")]
    pub allow_blank: Option<bool>,
    /// showDropDown
    /// Represents the following attribute in the schema: :showDropDown
    #[xml(attr = "showDropDown")]
    pub show_drop_down: Option<bool>,
    /// showInputMessage
    /// Represents the following attribute in the schema: :showInputMessage
    #[xml(attr = "showInputMessage")]
    pub show_input_message: Option<bool>,
    /// showErrorMessage
    /// Represents the following attribute in the schema: :showErrorMessage
    #[xml(attr = "showErrorMessage")]
    pub show_error_message: Option<bool>,
    /// errorTitle
    /// Represents the following attribute in the schema: :errorTitle
    #[xml(attr = "errorTitle")]
    pub error_title: Option<String>,
    /// error
    /// Represents the following attribute in the schema: :error
    #[xml(attr = "error")]
    pub error: Option<String>,
    /// promptTitle
    /// Represents the following attribute in the schema: :promptTitle
    #[xml(attr = "promptTitle")]
    pub prompt_title: Option<String>,
    /// prompt
    /// Represents the following attribute in the schema: :prompt
    #[xml(attr = "prompt")]
    pub prompt: Option<String>,
    /// _
    #[xml(child = "x14:formula1")]
    pub data_validation_forumla1: Option<DataValidationForumla1>,
    /// _
    #[xml(child = "x14:formula2")]
    pub data_validation_forumla2: Option<DataValidationForumla2>,
    /// _
    #[xml(child = "xne:sqref")]
    pub reference_sequence: crate::schemas::schemas_microsoft_com_office_excel_2006_main::ReferenceSequence,
}
/// Defines the DataValidationForumla1 Class.
/// When the object is serialized out as xml, it's qualified name is x14:formula1.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:formula1")]
pub struct DataValidationForumla1 {
    /// _
    #[xml(child = "xne:f")]
    pub formula: crate::schemas::schemas_microsoft_com_office_excel_2006_main::Formula,
}
/// Defines the DataValidationForumla2 Class.
/// When the object is serialized out as xml, it's qualified name is x14:formula2.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:formula2")]
pub struct DataValidationForumla2 {
    /// _
    #[xml(child = "xne:f")]
    pub formula: crate::schemas::schemas_microsoft_com_office_excel_2006_main::Formula,
}
/// Defines the DataValidationFormulaType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct DataValidationFormulaType {}
/// Defines the SparklineGroup Class.
/// When the object is serialized out as xml, it's qualified name is x14:sparklineGroup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:sparklineGroup")]
pub struct SparklineGroup {
    /// manualMax
    /// Represents the following attribute in the schema: :manualMax
    #[xml(attr = "manualMax")]
    pub manual_max: Option<f64>,
    /// manualMin
    /// Represents the following attribute in the schema: :manualMin
    #[xml(attr = "manualMin")]
    pub manual_min: Option<f64>,
    /// lineWeight
    /// Represents the following attribute in the schema: :lineWeight
    #[xml(attr = "lineWeight")]
    pub line_weight: Option<f64>,
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<SparklineTypeValues>,
    /// dateAxis
    /// Represents the following attribute in the schema: :dateAxis
    #[xml(attr = "dateAxis")]
    pub date_axis: Option<bool>,
    /// displayEmptyCellsAs
    /// Represents the following attribute in the schema: :displayEmptyCellsAs
    #[xml(attr = "displayEmptyCellsAs")]
    pub display_empty_cells_as: Option<DisplayBlanksAsValues>,
    /// markers
    /// Represents the following attribute in the schema: :markers
    #[xml(attr = "markers")]
    pub markers: Option<bool>,
    /// high
    /// Represents the following attribute in the schema: :high
    #[xml(attr = "high")]
    pub high: Option<bool>,
    /// low
    /// Represents the following attribute in the schema: :low
    #[xml(attr = "low")]
    pub low: Option<bool>,
    /// first
    /// Represents the following attribute in the schema: :first
    #[xml(attr = "first")]
    pub first: Option<bool>,
    /// last
    /// Represents the following attribute in the schema: :last
    #[xml(attr = "last")]
    pub last: Option<bool>,
    /// negative
    /// Represents the following attribute in the schema: :negative
    #[xml(attr = "negative")]
    pub negative: Option<bool>,
    /// displayXAxis
    /// Represents the following attribute in the schema: :displayXAxis
    #[xml(attr = "displayXAxis")]
    pub display_x_axis: Option<bool>,
    /// displayHidden
    /// Represents the following attribute in the schema: :displayHidden
    #[xml(attr = "displayHidden")]
    pub display_hidden: Option<bool>,
    /// minAxisType
    /// Represents the following attribute in the schema: :minAxisType
    #[xml(attr = "minAxisType")]
    pub min_axis_type: Option<SparklineAxisMinMaxValues>,
    /// maxAxisType
    /// Represents the following attribute in the schema: :maxAxisType
    #[xml(attr = "maxAxisType")]
    pub max_axis_type: Option<SparklineAxisMinMaxValues>,
    /// rightToLeft
    /// Represents the following attribute in the schema: :rightToLeft
    #[xml(attr = "rightToLeft")]
    pub right_to_left: Option<bool>,
    /// _
    #[xml(child = "x14:colorSeries")]
    pub series_color: Option<SeriesColor>,
    /// _
    #[xml(child = "x14:colorNegative")]
    pub negative_color: Option<NegativeColor>,
    /// _
    #[xml(child = "x14:colorAxis")]
    pub axis_color: Option<AxisColor>,
    /// _
    #[xml(child = "x14:colorMarkers")]
    pub markers_color: Option<MarkersColor>,
    /// _
    #[xml(child = "x14:colorFirst")]
    pub first_marker_color: Option<FirstMarkerColor>,
    /// _
    #[xml(child = "x14:colorLast")]
    pub last_marker_color: Option<LastMarkerColor>,
    /// _
    #[xml(child = "x14:colorHigh")]
    pub high_marker_color: Option<HighMarkerColor>,
    /// _
    #[xml(child = "x14:colorLow")]
    pub low_marker_color: Option<LowMarkerColor>,
    /// _
    #[xml(child = "xne:f")]
    pub formula: Option<
        crate::schemas::schemas_microsoft_com_office_excel_2006_main::Formula,
    >,
    /// _
    #[xml(child = "x14:sparklines")]
    pub sparklines: Sparklines,
}
/// Defines the SeriesColor Class.
/// When the object is serialized out as xml, it's qualified name is x14:colorSeries.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:colorSeries")]
pub struct SeriesColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the NegativeColor Class.
/// When the object is serialized out as xml, it's qualified name is x14:colorNegative.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:colorNegative")]
pub struct NegativeColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the AxisColor Class.
/// When the object is serialized out as xml, it's qualified name is x14:colorAxis.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:colorAxis")]
pub struct AxisColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the MarkersColor Class.
/// When the object is serialized out as xml, it's qualified name is x14:colorMarkers.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:colorMarkers")]
pub struct MarkersColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the FirstMarkerColor Class.
/// When the object is serialized out as xml, it's qualified name is x14:colorFirst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:colorFirst")]
pub struct FirstMarkerColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the LastMarkerColor Class.
/// When the object is serialized out as xml, it's qualified name is x14:colorLast.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:colorLast")]
pub struct LastMarkerColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the HighMarkerColor Class.
/// When the object is serialized out as xml, it's qualified name is x14:colorHigh.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:colorHigh")]
pub struct HighMarkerColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the LowMarkerColor Class.
/// When the object is serialized out as xml, it's qualified name is x14:colorLow.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:colorLow")]
pub struct LowMarkerColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the Color Class.
/// When the object is serialized out as xml, it's qualified name is x14:color.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:color")]
pub struct Color {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the FillColor Class.
/// When the object is serialized out as xml, it's qualified name is x14:fillColor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:fillColor")]
pub struct FillColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the BorderColor Class.
/// When the object is serialized out as xml, it's qualified name is x14:borderColor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:borderColor")]
pub struct BorderColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the NegativeFillColor Class.
/// When the object is serialized out as xml, it's qualified name is x14:negativeFillColor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:negativeFillColor")]
pub struct NegativeFillColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the NegativeBorderColor Class.
/// When the object is serialized out as xml, it's qualified name is x14:negativeBorderColor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:negativeBorderColor")]
pub struct NegativeBorderColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the BarAxisColor Class.
/// When the object is serialized out as xml, it's qualified name is x14:axisColor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:axisColor")]
pub struct BarAxisColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the ColorType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct ColorType {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the Sparklines Class.
/// When the object is serialized out as xml, it's qualified name is x14:sparklines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:sparklines")]
pub struct Sparklines {
    /// _
    #[xml(child = "x14:sparkline")]
    pub x14_sparkline: Vec<Sparkline>,
}
/// Defines the Sparkline Class.
/// When the object is serialized out as xml, it's qualified name is x14:sparkline.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:sparkline")]
pub struct Sparkline {
    /// _
    #[xml(child = "xne:f")]
    pub formula: Option<
        crate::schemas::schemas_microsoft_com_office_excel_2006_main::Formula,
    >,
    /// _
    #[xml(child = "xne:sqref")]
    pub reference_sequence: crate::schemas::schemas_microsoft_com_office_excel_2006_main::ReferenceSequence,
}
/// Defines the SlicerRef Class.
/// When the object is serialized out as xml, it's qualified name is x14:slicer.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:slicer")]
pub struct SlicerRef {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the SlicerCache Class.
/// When the object is serialized out as xml, it's qualified name is x14:slicerCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:slicerCache")]
pub struct SlicerCache {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the DefinedName Class.
/// When the object is serialized out as xml, it's qualified name is x14:definedName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:definedName")]
pub struct DefinedName {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// _
    #[xml(child = "x14:argumentDescriptions")]
    pub argument_descriptions: Option<ArgumentDescriptions>,
}
/// Defines the ArgumentDescriptions Class.
/// When the object is serialized out as xml, it's qualified name is x14:argumentDescriptions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:argumentDescriptions")]
pub struct ArgumentDescriptions {
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x14:argumentDescription")]
    pub x14_argument_description: Vec<ArgumentDescription>,
}
/// Defines the ArgumentDescription Class.
/// When the object is serialized out as xml, it's qualified name is x14:argumentDescription.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:argumentDescription")]
pub struct ArgumentDescription {
    /// index
    /// Represents the following attribute in the schema: :index
    #[xml(attr = "index")]
    pub index: i32,
    #[xml(text)]
    pub child: String,
}
/// Defines the TupleSet Class.
/// When the object is serialized out as xml, it's qualified name is x14:tupleSet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:tupleSet")]
pub struct TupleSet {
    /// rowCount
    /// Represents the following attribute in the schema: :rowCount
    #[xml(attr = "rowCount")]
    pub row_count: Option<i32>,
    /// columnCount
    /// Represents the following attribute in the schema: :columnCount
    #[xml(attr = "columnCount")]
    pub column_count: Option<i32>,
    /// _
    #[xml(child = "x14:headers")]
    pub tuple_set_headers: TupleSetHeaders,
    /// _
    #[xml(child = "x14:rows")]
    pub tuple_set_rows: TupleSetRows,
}
/// Defines the TupleSetHeaders Class.
/// When the object is serialized out as xml, it's qualified name is x14:headers.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:headers")]
pub struct TupleSetHeaders {
    /// _
    #[xml(child = "x14:header")]
    pub x14_header: Vec<TupleSetHeader>,
}
/// Defines the TupleSetRows Class.
/// When the object is serialized out as xml, it's qualified name is x14:rows.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:rows")]
pub struct TupleSetRows {
    /// _
    #[xml(child = "x14:row")]
    pub x14_row: Vec<TupleSetRow>,
}
/// Defines the TupleSetHeader Class.
/// When the object is serialized out as xml, it's qualified name is x14:header.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:header")]
pub struct TupleSetHeader {
    /// uniqueName
    /// Represents the following attribute in the schema: :uniqueName
    #[xml(attr = "uniqueName")]
    pub unique_name: Option<String>,
    /// hierarchyName
    /// Represents the following attribute in the schema: :hierarchyName
    #[xml(attr = "hierarchyName")]
    pub hierarchy_name: Option<String>,
}
/// Defines the TupleSetRow Class.
/// When the object is serialized out as xml, it's qualified name is x14:row.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:row")]
pub struct TupleSetRow {
    /// _
    #[xml(child = "x14:rowItem")]
    pub x14_row_item: Vec<TupleSetRowItem>,
}
/// Defines the TupleSetRowItem Class.
/// When the object is serialized out as xml, it's qualified name is x14:rowItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:rowItem")]
pub struct TupleSetRowItem {
    /// u
    /// Represents the following attribute in the schema: :u
    #[xml(attr = "u")]
    pub unique_name: Option<String>,
    /// d
    /// Represents the following attribute in the schema: :d
    #[xml(attr = "d")]
    pub display_name: Option<String>,
}
/// Defines the SetLevel Class.
/// When the object is serialized out as xml, it's qualified name is x14:setLevel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:setLevel")]
pub struct SetLevel {
    /// hierarchy
    /// Represents the following attribute in the schema: :hierarchy
    #[xml(attr = "hierarchy")]
    pub hierarchy: i32,
}
/// Defines the SetLevels Class.
/// When the object is serialized out as xml, it's qualified name is x14:setLevels.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:setLevels")]
pub struct SetLevels {
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x14:setLevel")]
    pub x14_set_level: Vec<SetLevel>,
}
/// Defines the ColorScale Class.
/// When the object is serialized out as xml, it's qualified name is x14:colorScale.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:colorScale")]
pub struct ColorScale {
    /// _
    #[xml(child = "x14:cfvo")]
    pub x14_cfvo: Vec<ConditionalFormattingValueObject>,
    /// _
    #[xml(child = "x14:color")]
    pub x14_color: Vec<Color>,
}
/// Defines the DataBar Class.
/// When the object is serialized out as xml, it's qualified name is x14:dataBar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:dataBar")]
pub struct DataBar {
    /// minLength
    /// Represents the following attribute in the schema: :minLength
    #[xml(attr = "minLength")]
    pub min_length: Option<i32>,
    /// maxLength
    /// Represents the following attribute in the schema: :maxLength
    #[xml(attr = "maxLength")]
    pub max_length: Option<i32>,
    /// showValue
    /// Represents the following attribute in the schema: :showValue
    #[xml(attr = "showValue")]
    pub show_value: Option<bool>,
    /// border
    /// Represents the following attribute in the schema: :border
    #[xml(attr = "border")]
    pub border: Option<bool>,
    /// gradient
    /// Represents the following attribute in the schema: :gradient
    #[xml(attr = "gradient")]
    pub gradient: Option<bool>,
    /// direction
    /// Represents the following attribute in the schema: :direction
    #[xml(attr = "direction")]
    pub direction: Option<DataBarDirectionValues>,
    /// negativeBarColorSameAsPositive
    /// Represents the following attribute in the schema: :negativeBarColorSameAsPositive
    #[xml(attr = "negativeBarColorSameAsPositive")]
    pub negative_bar_color_same_as_positive: Option<bool>,
    /// negativeBarBorderColorSameAsPositive
    /// Represents the following attribute in the schema: :negativeBarBorderColorSameAsPositive
    #[xml(attr = "negativeBarBorderColorSameAsPositive")]
    pub negative_bar_border_color_same_as_positive: Option<bool>,
    /// axisPosition
    /// Represents the following attribute in the schema: :axisPosition
    #[xml(attr = "axisPosition")]
    pub axis_position: Option<DataBarAxisPositionValues>,
    /// _
    #[xml(child = "x14:cfvo")]
    pub x14_cfvo: Vec<ConditionalFormattingValueObject>,
    /// _
    #[xml(child = "x14:fillColor")]
    pub x14_fill_color: Option<FillColor>,
    /// _
    #[xml(child = "x14:borderColor")]
    pub x14_border_color: Option<BorderColor>,
    /// _
    #[xml(child = "x14:negativeFillColor")]
    pub x14_negative_fill_color: Option<NegativeFillColor>,
    /// _
    #[xml(child = "x14:negativeBorderColor")]
    pub x14_negative_border_color: Option<NegativeBorderColor>,
    /// _
    #[xml(child = "x14:axisColor")]
    pub x14_axis_color: Option<BarAxisColor>,
}
/// Defines the IconSet Class.
/// When the object is serialized out as xml, it's qualified name is x14:iconSet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:iconSet")]
pub struct IconSet {
    /// iconSet
    /// Represents the following attribute in the schema: :iconSet
    #[xml(attr = "iconSet")]
    pub icon_set_types: Option<IconSetTypeValues>,
    /// showValue
    /// Represents the following attribute in the schema: :showValue
    #[xml(attr = "showValue")]
    pub show_value: Option<bool>,
    /// percent
    /// Represents the following attribute in the schema: :percent
    #[xml(attr = "percent")]
    pub percent: Option<bool>,
    /// reverse
    /// Represents the following attribute in the schema: :reverse
    #[xml(attr = "reverse")]
    pub reverse: Option<bool>,
    /// custom
    /// Represents the following attribute in the schema: :custom
    #[xml(attr = "custom")]
    pub custom: Option<bool>,
    /// _
    #[xml(child = "x14:cfvo")]
    pub x14_cfvo: Vec<ConditionalFormattingValueObject>,
    /// _
    #[xml(child = "x14:cfIcon")]
    pub x14_cf_icon: Vec<ConditionalFormattingIcon>,
}
/// Defines the DifferentialType Class.
/// When the object is serialized out as xml, it's qualified name is x14:dxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:dxf")]
pub struct DifferentialType {
    ///Font Properties
    #[xml(child = "x:font")]
    pub font: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Font,
    >,
    ///Number Format
    #[xml(child = "x:numFmt")]
    pub numbering_format: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::NumberingFormat,
    >,
    ///Fill
    #[xml(child = "x:fill")]
    pub fill: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Fill,
    >,
    ///Alignment
    #[xml(child = "x:alignment")]
    pub alignment: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Alignment,
    >,
    ///Border Properties
    #[xml(child = "x:border")]
    pub border: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Border,
    >,
    ///Protection Properties
    #[xml(child = "x:protection")]
    pub protection: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Protection,
    >,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList,
    >,
}
/// Defines the ConditionalFormattingValueObject Class.
/// When the object is serialized out as xml, it's qualified name is x14:cfvo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:cfvo")]
pub struct ConditionalFormattingValueObject {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: ConditionalFormattingValueObjectTypeValues,
    /// gte
    /// Represents the following attribute in the schema: :gte
    #[xml(attr = "gte")]
    pub greater_than_or_equal: Option<bool>,
    /// _
    #[xml(child = "xne:f")]
    pub formula: Option<
        crate::schemas::schemas_microsoft_com_office_excel_2006_main::Formula,
    >,
    /// _
    #[xml(child = "x14:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the ConditionalFormattingIcon Class.
/// When the object is serialized out as xml, it's qualified name is x14:cfIcon.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:cfIcon")]
pub struct ConditionalFormattingIcon {
    /// iconSet
    /// Represents the following attribute in the schema: :iconSet
    #[xml(attr = "iconSet")]
    pub icon_set: IconSetTypeValues,
    /// iconId
    /// Represents the following attribute in the schema: :iconId
    #[xml(attr = "iconId")]
    pub icon_id: i32,
}
/// Defines the PivotEdits Class.
/// When the object is serialized out as xml, it's qualified name is x14:pivotEdits.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:pivotEdits")]
pub struct PivotEdits {
    /// _
    #[xml(child = "x14:pivotEdit")]
    pub x14_pivot_edit: Vec<PivotEdit>,
}
/// Defines the PivotChanges Class.
/// When the object is serialized out as xml, it's qualified name is x14:pivotChanges.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:pivotChanges")]
pub struct PivotChanges {
    /// _
    #[xml(child = "x14:pivotChange")]
    pub x14_pivot_change: Vec<PivotChange>,
}
/// Defines the ConditionalFormats Class.
/// When the object is serialized out as xml, it's qualified name is x14:conditionalFormats.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:conditionalFormats")]
pub struct ConditionalFormats {
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x14:conditionalFormat")]
    pub x14_conditional_format: Vec<ConditionalFormat>,
}
/// Defines the CalculatedMembers Class.
/// When the object is serialized out as xml, it's qualified name is x14:calculatedMembers.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:calculatedMembers")]
pub struct CalculatedMembers {
    /// Calculated Members Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:calculatedMember")]
    pub x_calculated_member: Vec<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CalculatedMember,
    >,
}
/// Defines the PivotEdit Class.
/// When the object is serialized out as xml, it's qualified name is x14:pivotEdit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:pivotEdit")]
pub struct PivotEdit {
    /// _
    #[xml(child = "x14:userEdit")]
    pub pivot_user_edit: PivotUserEdit,
    /// _
    #[xml(child = "x14:tupleItems")]
    pub tuple_items: TupleItems,
    /// _
    #[xml(child = "x14:pivotArea")]
    pub pivot_area: PivotArea,
    /// _
    #[xml(child = "x14:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotUserEdit Class.
/// When the object is serialized out as xml, it's qualified name is x14:userEdit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:userEdit")]
pub struct PivotUserEdit {
    #[xml(child = "xne:f", child = "x14:editValue")]
    pub children: Vec<PivotUserEditChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PivotUserEditChildChoice {
    #[xml(tag = "xne:f")]
    XneF(crate::schemas::schemas_microsoft_com_office_excel_2006_main::Formula),
    #[xml(tag = "x14:editValue")]
    X14EditValue(PivotEditValue),
}
/// Defines the TupleItems Class.
/// When the object is serialized out as xml, it's qualified name is x14:tupleItems.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:tupleItems")]
pub struct TupleItems {
    /// _
    #[xml(child = "x14:tupleItem")]
    pub x14_tuple_item: Vec<Xstring>,
}
/// Defines the PivotArea Class.
/// When the object is serialized out as xml, it's qualified name is x14:pivotArea.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:pivotArea")]
pub struct PivotArea {
    /// Field Index
    /// Represents the following attribute in the schema: :field
    #[xml(attr = "field")]
    pub field: Option<i32>,
    /// Rule Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotAreaValues,
    >,
    /// Data Only
    /// Represents the following attribute in the schema: :dataOnly
    #[xml(attr = "dataOnly")]
    pub data_only: Option<bool>,
    /// Labels Only
    /// Represents the following attribute in the schema: :labelOnly
    #[xml(attr = "labelOnly")]
    pub label_only: Option<bool>,
    /// Include Row Grand Total
    /// Represents the following attribute in the schema: :grandRow
    #[xml(attr = "grandRow")]
    pub grand_row: Option<bool>,
    /// Include Column Grand Total
    /// Represents the following attribute in the schema: :grandCol
    #[xml(attr = "grandCol")]
    pub grand_column: Option<bool>,
    /// Cache Index
    /// Represents the following attribute in the schema: :cacheIndex
    #[xml(attr = "cacheIndex")]
    pub cache_index: Option<bool>,
    /// Outline
    /// Represents the following attribute in the schema: :outline
    #[xml(attr = "outline")]
    pub outline: Option<bool>,
    /// Offset Reference
    /// Represents the following attribute in the schema: :offset
    #[xml(attr = "offset")]
    pub offset: Option<String>,
    /// Collapsed Levels Are Subtotals
    /// Represents the following attribute in the schema: :collapsedLevelsAreSubtotals
    #[xml(attr = "collapsedLevelsAreSubtotals")]
    pub collapsed_levels_are_subtotals: Option<bool>,
    /// Axis
    /// Represents the following attribute in the schema: :axis
    #[xml(attr = "axis")]
    pub axis: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableAxisValues,
    >,
    /// Field Position
    /// Represents the following attribute in the schema: :fieldPosition
    #[xml(attr = "fieldPosition")]
    pub field_position: Option<i32>,
    ///References
    #[xml(child = "x:references")]
    pub pivot_area_references: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotAreaReferences,
    >,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList,
    >,
}
/// Defines the PivotChange Class.
/// When the object is serialized out as xml, it's qualified name is x14:pivotChange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:pivotChange")]
pub struct PivotChange {
    /// allocationMethod
    /// Represents the following attribute in the schema: :allocationMethod
    #[xml(attr = "allocationMethod")]
    pub allocation_method: Option<AllocationMethodValues>,
    /// weightExpression
    /// Represents the following attribute in the schema: :weightExpression
    #[xml(attr = "weightExpression")]
    pub weight_expression: Option<String>,
    /// _
    #[xml(child = "x14:editValue")]
    pub pivot_edit_value: PivotEditValue,
    /// _
    #[xml(child = "x14:tupleItems")]
    pub tuple_items: TupleItems,
    /// _
    #[xml(child = "x14:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotEditValue Class.
/// When the object is serialized out as xml, it's qualified name is x14:editValue.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:editValue")]
pub struct PivotEditValue {
    /// valueType
    /// Represents the following attribute in the schema: :valueType
    #[xml(attr = "valueType")]
    pub value_type: PivotEditValueTypeValues,
    #[xml(text)]
    pub child: String,
}
/// Defines the Xstring Class.
/// When the object is serialized out as xml, it's qualified name is x14:tupleItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:tupleItem")]
pub struct Xstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the SlicerStyleElements Class.
/// When the object is serialized out as xml, it's qualified name is x14:slicerStyleElements.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:slicerStyleElements")]
pub struct SlicerStyleElements {
    /// _
    #[xml(child = "x14:slicerStyleElement")]
    pub x14_slicer_style_element: Vec<SlicerStyleElement>,
}
/// Defines the DdeValues Class.
/// When the object is serialized out as xml, it's qualified name is x14:values.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:values")]
pub struct DdeValues {
    /// Rows
    /// Represents the following attribute in the schema: :rows
    #[xml(attr = "rows")]
    pub rows: Option<i32>,
    /// Columns
    /// Represents the following attribute in the schema: :cols
    #[xml(attr = "cols")]
    pub columns: Option<i32>,
    /// _
    #[xml(child = "x:value")]
    pub x_value: Vec<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Value,
    >,
}
/// Defines the ConditionalFormat Class.
/// When the object is serialized out as xml, it's qualified name is x14:conditionalFormat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:conditionalFormat")]
pub struct ConditionalFormat {
    /// scope
    /// Represents the following attribute in the schema: :scope
    #[xml(attr = "scope")]
    pub scope: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ScopeValues,
    >,
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::RuleValues,
    >,
    /// priority
    /// Represents the following attribute in the schema: :priority
    #[xml(attr = "priority")]
    pub priority: Option<i32>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// _
    #[xml(child = "x14:pivotAreas")]
    pub pivot_areas: Option<PivotAreas>,
    /// _
    #[xml(child = "x14:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotAreas Class.
/// When the object is serialized out as xml, it's qualified name is x14:pivotAreas.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:pivotAreas")]
pub struct PivotAreas {
    /// Pivot Area Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:pivotArea")]
    pub x_pivot_area: Vec<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotArea,
    >,
}
/// Defines the SlicerStyle Class.
/// When the object is serialized out as xml, it's qualified name is x14:slicerStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:slicerStyle")]
pub struct SlicerStyle {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// _
    #[xml(child = "x14:slicerStyleElements")]
    pub slicer_style_elements: Option<SlicerStyleElements>,
}
/// Defines the SlicerStyleElement Class.
/// When the object is serialized out as xml, it's qualified name is x14:slicerStyleElement.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:slicerStyleElement")]
pub struct SlicerStyleElement {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: SlicerStyleTypeValues,
    /// dxfId
    /// Represents the following attribute in the schema: :dxfId
    #[xml(attr = "dxfId")]
    pub format_id: Option<i32>,
}
/// Defines the IgnoredError Class.
/// When the object is serialized out as xml, it's qualified name is x14:ignoredError.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:ignoredError")]
pub struct IgnoredError {
    /// evalError
    /// Represents the following attribute in the schema: :evalError
    #[xml(attr = "evalError")]
    pub eval_error: Option<bool>,
    /// twoDigitTextYear
    /// Represents the following attribute in the schema: :twoDigitTextYear
    #[xml(attr = "twoDigitTextYear")]
    pub two_digit_text_year: Option<bool>,
    /// numberStoredAsText
    /// Represents the following attribute in the schema: :numberStoredAsText
    #[xml(attr = "numberStoredAsText")]
    pub number_stored_as_text: Option<bool>,
    /// formula
    /// Represents the following attribute in the schema: :formula
    #[xml(attr = "formula")]
    pub formula: Option<bool>,
    /// formulaRange
    /// Represents the following attribute in the schema: :formulaRange
    #[xml(attr = "formulaRange")]
    pub formula_range: Option<bool>,
    /// unlockedFormula
    /// Represents the following attribute in the schema: :unlockedFormula
    #[xml(attr = "unlockedFormula")]
    pub unlocked_formula: Option<bool>,
    /// emptyCellReference
    /// Represents the following attribute in the schema: :emptyCellReference
    #[xml(attr = "emptyCellReference")]
    pub empty_cell_reference: Option<bool>,
    /// listDataValidation
    /// Represents the following attribute in the schema: :listDataValidation
    #[xml(attr = "listDataValidation")]
    pub list_data_validation: Option<bool>,
    /// calculatedColumn
    /// Represents the following attribute in the schema: :calculatedColumn
    #[xml(attr = "calculatedColumn")]
    pub calculated_column: Option<bool>,
    /// _
    #[xml(child = "xne:sqref")]
    pub reference_sequence: crate::schemas::schemas_microsoft_com_office_excel_2006_main::ReferenceSequence,
}
/// Defines the ProtectedRange Class.
/// When the object is serialized out as xml, it's qualified name is x14:protectedRange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:protectedRange")]
pub struct ProtectedRange {
    /// password
    /// Represents the following attribute in the schema: :password
    #[xml(attr = "password")]
    pub password: Option<String>,
    /// algorithmName
    /// Represents the following attribute in the schema: :algorithmName
    #[xml(attr = "algorithmName")]
    pub algorithm_name: Option<String>,
    /// hashValue
    /// Represents the following attribute in the schema: :hashValue
    #[xml(attr = "hashValue")]
    pub hash_value: Option<String>,
    /// saltValue
    /// Represents the following attribute in the schema: :saltValue
    #[xml(attr = "saltValue")]
    pub salt_value: Option<String>,
    /// spinCount
    /// Represents the following attribute in the schema: :spinCount
    #[xml(attr = "spinCount")]
    pub spin_count: Option<i32>,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// securityDescriptor
    /// Represents the following attribute in the schema: :securityDescriptor
    #[xml(attr = "securityDescriptor")]
    pub security_descriptor: Option<String>,
    /// _
    #[xml(child = "xne:sqref")]
    pub reference_sequence: crate::schemas::schemas_microsoft_com_office_excel_2006_main::ReferenceSequence,
}
/// Defines the CustomFilter Class.
/// When the object is serialized out as xml, it's qualified name is x14:customFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:customFilter")]
pub struct CustomFilter {
    /// operator
    /// Represents the following attribute in the schema: :operator
    #[xml(attr = "operator")]
    pub operator: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::FilterOperatorValues,
    >,
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<String>,
}
/// Defines the ListItem Class.
/// When the object is serialized out as xml, it's qualified name is x14:item.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:item")]
pub struct ListItem {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Defines the ListItems Class.
/// When the object is serialized out as xml, it's qualified name is x14:itemLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:itemLst")]
pub struct ListItems {
    /// _
    #[xml(child = "x14:item")]
    pub x14_item: Vec<ListItem>,
    /// _
    #[xml(child = "x14:extLst")]
    pub x14_ext_lst: Option<ExtensionList>,
}
/// Defines the Slicer Class.
/// When the object is serialized out as xml, it's qualified name is x14:slicer.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:slicer")]
pub struct Slicer {
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
    /// startItem
    /// Represents the following attribute in the schema: :startItem
    #[xml(attr = "startItem")]
    pub start_item: Option<i32>,
    /// columnCount
    /// Represents the following attribute in the schema: :columnCount
    #[xml(attr = "columnCount")]
    pub column_count: Option<i32>,
    /// showCaption
    /// Represents the following attribute in the schema: :showCaption
    #[xml(attr = "showCaption")]
    pub show_caption: Option<bool>,
    /// level
    /// Represents the following attribute in the schema: :level
    #[xml(attr = "level")]
    pub level: Option<i32>,
    /// style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// lockedPosition
    /// Represents the following attribute in the schema: :lockedPosition
    #[xml(attr = "lockedPosition")]
    pub locked_position: Option<bool>,
    /// rowHeight
    /// Represents the following attribute in the schema: :rowHeight
    #[xml(attr = "rowHeight")]
    pub row_height: i32,
    /// _
    #[xml(child = "x14:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the OlapSlicerCache Class.
/// When the object is serialized out as xml, it's qualified name is x14:olap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:olap")]
pub struct OlapSlicerCache {
    /// pivotCacheId
    /// Represents the following attribute in the schema: :pivotCacheId
    #[xml(attr = "pivotCacheId")]
    pub pivot_cache_id: i32,
    /// _
    #[xml(child = "x14:levels")]
    pub olap_slicer_cache_levels_data: OlapSlicerCacheLevelsData,
    /// _
    #[xml(child = "x14:selections")]
    pub olap_slicer_cache_selections: OlapSlicerCacheSelections,
    /// _
    #[xml(child = "x14:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the TabularSlicerCache Class.
/// When the object is serialized out as xml, it's qualified name is x14:tabular.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:tabular")]
pub struct TabularSlicerCache {
    /// pivotCacheId
    /// Represents the following attribute in the schema: :pivotCacheId
    #[xml(attr = "pivotCacheId")]
    pub pivot_cache_id: i32,
    /// sortOrder
    /// Represents the following attribute in the schema: :sortOrder
    #[xml(attr = "sortOrder")]
    pub sort_order: Option<TabularSlicerCacheSortOrderValues>,
    /// customListSort
    /// Represents the following attribute in the schema: :customListSort
    #[xml(attr = "customListSort")]
    pub custom_list_sort: Option<bool>,
    /// showMissing
    /// Represents the following attribute in the schema: :showMissing
    #[xml(attr = "showMissing")]
    pub show_missing: Option<bool>,
    /// crossFilter
    /// Represents the following attribute in the schema: :crossFilter
    #[xml(attr = "crossFilter")]
    pub cross_filter: Option<SlicerCacheCrossFilterValues>,
    /// _
    #[xml(child = "x14:items")]
    pub tabular_slicer_cache_items: TabularSlicerCacheItems,
    /// _
    #[xml(child = "x14:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the SlicerCachePivotTable Class.
/// When the object is serialized out as xml, it's qualified name is x14:pivotTable.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:pivotTable")]
pub struct SlicerCachePivotTable {
    /// tabId
    /// Represents the following attribute in the schema: :tabId
    #[xml(attr = "tabId")]
    pub tab_id: i32,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
}
/// Defines the OlapSlicerCacheItemParent Class.
/// When the object is serialized out as xml, it's qualified name is x14:p.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:p")]
pub struct OlapSlicerCacheItemParent {
    /// n
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub name: String,
}
/// Defines the OlapSlicerCacheItem Class.
/// When the object is serialized out as xml, it's qualified name is x14:i.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:i")]
pub struct OlapSlicerCacheItem {
    /// n
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub name: String,
    /// c
    /// Represents the following attribute in the schema: :c
    #[xml(attr = "c")]
    pub display_name: Option<String>,
    /// nd
    /// Represents the following attribute in the schema: :nd
    #[xml(attr = "nd")]
    pub non_display: Option<bool>,
    /// _
    #[xml(child = "x14:p")]
    pub x14_p: Vec<OlapSlicerCacheItemParent>,
}
/// Defines the OlapSlicerCacheRange Class.
/// When the object is serialized out as xml, it's qualified name is x14:range.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:range")]
pub struct OlapSlicerCacheRange {
    /// startItem
    /// Represents the following attribute in the schema: :startItem
    #[xml(attr = "startItem")]
    pub start_item: i32,
    /// _
    #[xml(child = "x14:i")]
    pub x14_i: Vec<OlapSlicerCacheItem>,
}
/// Defines the OlapSlicerCacheRanges Class.
/// When the object is serialized out as xml, it's qualified name is x14:ranges.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:ranges")]
pub struct OlapSlicerCacheRanges {
    /// _
    #[xml(child = "x14:range")]
    pub x14_range: Vec<OlapSlicerCacheRange>,
}
/// Defines the OlapSlicerCacheLevelData Class.
/// When the object is serialized out as xml, it's qualified name is x14:level.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:level")]
pub struct OlapSlicerCacheLevelData {
    /// uniqueName
    /// Represents the following attribute in the schema: :uniqueName
    #[xml(attr = "uniqueName")]
    pub unique_name: String,
    /// sourceCaption
    /// Represents the following attribute in the schema: :sourceCaption
    #[xml(attr = "sourceCaption")]
    pub source_caption: Option<String>,
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: i32,
    /// sortOrder
    /// Represents the following attribute in the schema: :sortOrder
    #[xml(attr = "sortOrder")]
    pub sort_order: Option<OlapSlicerCacheSortOrderValues>,
    /// crossFilter
    /// Represents the following attribute in the schema: :crossFilter
    #[xml(attr = "crossFilter")]
    pub cross_filter: Option<SlicerCacheCrossFilterValues>,
    /// _
    #[xml(child = "x14:ranges")]
    pub olap_slicer_cache_ranges: Option<OlapSlicerCacheRanges>,
}
/// Defines the OlapSlicerCacheLevelsData Class.
/// When the object is serialized out as xml, it's qualified name is x14:levels.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:levels")]
pub struct OlapSlicerCacheLevelsData {
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x14:level")]
    pub x14_level: Vec<OlapSlicerCacheLevelData>,
}
/// Defines the OlapSlicerCacheSelections Class.
/// When the object is serialized out as xml, it's qualified name is x14:selections.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:selections")]
pub struct OlapSlicerCacheSelections {
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x14:selection")]
    pub x14_selection: Vec<OlapSlicerCacheSelection>,
}
/// Defines the OlapSlicerCacheSelection Class.
/// When the object is serialized out as xml, it's qualified name is x14:selection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:selection")]
pub struct OlapSlicerCacheSelection {
    /// n
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub name: String,
    /// _
    #[xml(child = "x14:p")]
    pub x14_p: Vec<OlapSlicerCacheItemParent>,
}
/// Defines the TabularSlicerCacheItems Class.
/// When the object is serialized out as xml, it's qualified name is x14:items.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:items")]
pub struct TabularSlicerCacheItems {
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x14:i")]
    pub x14_i: Vec<TabularSlicerCacheItem>,
}
/// Defines the TabularSlicerCacheItem Class.
/// When the object is serialized out as xml, it's qualified name is x14:i.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:i")]
pub struct TabularSlicerCacheItem {
    /// x
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub atom: i32,
    /// s
    /// Represents the following attribute in the schema: :s
    #[xml(attr = "s")]
    pub is_selected: Option<bool>,
    /// nd
    /// Represents the following attribute in the schema: :nd
    #[xml(attr = "nd")]
    pub non_display: Option<bool>,
}
/// Defines the SlicerCachePivotTables Class.
/// When the object is serialized out as xml, it's qualified name is x14:pivotTables.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:pivotTables")]
pub struct SlicerCachePivotTables {
    /// _
    #[xml(child = "x14:pivotTable")]
    pub x14_pivot_table: Vec<SlicerCachePivotTable>,
}
/// Defines the SlicerCacheData Class.
/// When the object is serialized out as xml, it's qualified name is x14:data.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:data")]
pub struct SlicerCacheData {
    #[xml(child = "x14:olap", child = "x14:tabular")]
    pub children: Vec<SlicerCacheDataChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SlicerCacheDataChildChoice {
    #[xml(tag = "x14:olap")]
    X14Olap(OlapSlicerCache),
    #[xml(tag = "x14:tabular")]
    X14Tabular(TabularSlicerCache),
}
/// Defines the SlicerCacheDefinitionExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x14:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x14:extLst")]
pub struct SlicerCacheDefinitionExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SlicerCacheDefinitionExtension,
    >,
}
