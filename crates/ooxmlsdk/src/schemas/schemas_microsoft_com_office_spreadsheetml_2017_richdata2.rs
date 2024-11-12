#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SupportingPropertyBagValueType {
    #[default]
    D,
    I,
    B,
    S,
    Spb,
    Spba,
}
crate::__string_enum! {
    SupportingPropertyBagValueType { D = "d", I = "i", B = "b", S = "s", Spb = "spb",
    Spba = "spba", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SupportingPropertyBagArrayValueType {
    #[default]
    D,
    I,
    B,
    S,
    Spb,
}
crate::__string_enum! {
    SupportingPropertyBagArrayValueType { D = "d", I = "i", B = "b", S = "s", Spb =
    "spb", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ArrayValueType {
    #[default]
    D,
    I,
    B,
    E,
    S,
    R,
    A,
}
crate::__string_enum! {
    ArrayValueType { D = "d", I = "i", B = "b", E = "e", S = "s", R = "r", A = "a", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RichFormatPropertyType {
    #[default]
    B,
    N,
    I,
    S,
}
crate::__string_enum! {
    RichFormatPropertyType { B = "b", N = "n", I = "i", S = "s", }
}
/// Defines the RichFilterColumn Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:filterColumn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:filterColumn")]
pub struct RichFilterColumn {
    #[xml(
        child = "xlrd2:filters",
        child = "xlrd2:top10",
        child = "xlrd2:customFilters",
        child = "xlrd2:dynamicFilter",
        child = "xlrd2:extLst",
    )]
    pub children: Vec<RichFilterColumnChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RichFilterColumnChildChoice {
    #[xml(tag = "xlrd2:filters")]
    Xlrd2Filters(RichFilters),
    #[xml(tag = "xlrd2:top10")]
    Xlrd2Top10(RichTop10),
    #[xml(tag = "xlrd2:customFilters")]
    Xlrd2CustomFilters(CustomRichFilters),
    #[xml(tag = "xlrd2:dynamicFilter")]
    Xlrd2DynamicFilter(DynamicRichFilter),
    #[xml(tag = "xlrd2:extLst")]
    Xlrd2ExtLst(ExtensionList),
}
/// Defines the RichSortCondition Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:richSortCondition.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:richSortCondition")]
pub struct RichSortCondition {
    /// richSortKey
    /// Represents the following attribute in the schema: :richSortKey
    #[xml(attr = "richSortKey")]
    pub rich_sort_key: Option<String>,
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
    pub icon_set: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::IconSetTypeValues,
    >,
    /// iconId
    /// Represents the following attribute in the schema: :iconId
    #[xml(attr = "iconId")]
    pub icon_id: Option<i32>,
}
/// Defines the SupportingPropertyBags Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:supportingPropertyBags.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:supportingPropertyBags")]
pub struct SupportingPropertyBags {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "xlrd2:spbArrays")]
    pub supporting_property_bag_array_data: Option<SupportingPropertyBagArrayData>,
    /// _
    #[xml(child = "xlrd2:spbData")]
    pub supporting_property_bag_data: SupportingPropertyBagData,
}
/// Defines the SupportingPropertyBagStructures Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:spbStructures.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:spbStructures")]
pub struct SupportingPropertyBagStructures {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: i32,
    /// _
    #[xml(child = "xlrd2:s")]
    pub xlrd2_s: Vec<SupportingPropertyBagStructure>,
    /// _
    #[xml(child = "xlrd2:extLst")]
    pub xlrd2_ext_lst: Option<ExtensionList>,
}
/// Defines the ArrayData Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:arrayData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:arrayData")]
pub struct ArrayData {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: i32,
    /// _
    #[xml(child = "xlrd2:a")]
    pub xlrd2_a: Vec<Array>,
    /// _
    #[xml(child = "xlrd2:extLst")]
    pub xlrd2_ext_lst: Option<ExtensionList>,
}
/// Defines the RichStylesheet Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:richStyleSheet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:richStyleSheet")]
pub struct RichStylesheet {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "xlrd2:dxfs")]
    pub dxfs: Option<Dxfs>,
    /// _
    #[xml(child = "xlrd2:richProperties")]
    pub rich_format_properties: Option<RichFormatProperties>,
    /// _
    #[xml(child = "xlrd2:richStyles")]
    pub rich_styles: Option<RichStyles>,
    /// _
    #[xml(child = "xlrd2:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the RichValueTypesInfo Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:rvTypesInfo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:rvTypesInfo")]
pub struct RichValueTypesInfo {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "xlrd2:global")]
    pub rich_value_global_type: Option<RichValueGlobalType>,
    /// _
    #[xml(child = "xlrd2:types")]
    pub rich_value_types: Option<RichValueTypes>,
    /// _
    #[xml(child = "xlrd2:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the RichFilters Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:filters.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:filters")]
pub struct RichFilters {
    /// _
    #[xml(child = "xlrd2:filter")]
    pub xlrd2_filter: Vec<RichFilter>,
    /// _
    #[xml(child = "xlrd2:dateGroupItem")]
    pub xlrd2_date_group_item: Vec<RichDateGroupItem>,
    /// _
    #[xml(child = "xlrd2:extLst")]
    pub xlrd2_ext_lst: Option<ExtensionList>,
}
/// Defines the RichTop10 Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:top10.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:top10")]
pub struct RichTop10 {
    /// key
    /// Represents the following attribute in the schema: :key
    #[xml(attr = "key")]
    pub key: Option<String>,
    /// Top
    /// Represents the following attribute in the schema: :top
    #[xml(attr = "top")]
    pub top: Option<bool>,
    /// Filter by Percent
    /// Represents the following attribute in the schema: :percent
    #[xml(attr = "percent")]
    pub percent: Option<bool>,
    /// Top or Bottom Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
    /// Filter Value
    /// Represents the following attribute in the schema: :filterVal
    #[xml(attr = "filterVal")]
    pub filter_value: Option<f64>,
}
/// Defines the CustomRichFilters Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:customFilters.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:customFilters")]
pub struct CustomRichFilters {
    /// and
    /// Represents the following attribute in the schema: :and
    #[xml(attr = "and")]
    pub and: Option<bool>,
    #[xml(child = "xlrd2:customFilter", child = "xlrd2:extLst")]
    pub children: Vec<CustomRichFiltersChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CustomRichFiltersChildChoice {
    #[xml(tag = "xlrd2:customFilter")]
    Xlrd2CustomFilter(CustomRichFilter),
    #[xml(tag = "xlrd2:extLst")]
    Xlrd2ExtLst(ExtensionList),
}
/// Defines the DynamicRichFilter Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:dynamicFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:dynamicFilter")]
pub struct DynamicRichFilter {
    /// key
    /// Represents the following attribute in the schema: :key
    #[xml(attr = "key")]
    pub key: Option<String>,
    /// Dynamic filter type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DynamicFilterValues,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<f64>,
    /// Max Value
    /// Represents the following attribute in the schema: :maxVal
    #[xml(attr = "maxVal")]
    pub max_val: Option<f64>,
    /// valIso
    /// Represents the following attribute in the schema: :valIso
    #[xml(attr = "valIso")]
    pub val_iso: Option<String>,
    /// maxValIso
    /// Represents the following attribute in the schema: :maxValIso
    #[xml(attr = "maxValIso")]
    pub max_val_iso: Option<String>,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:extLst")]
pub struct ExtensionList {
    #[xml(child = "x:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "x:ext")]
    XExt(crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension),
}
/// Defines the RichFilter Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:filter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:filter")]
pub struct RichFilter {
    /// key
    /// Represents the following attribute in the schema: :key
    #[xml(attr = "key")]
    pub key: Option<String>,
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<String>,
    /// blank
    /// Represents the following attribute in the schema: :blank
    #[xml(attr = "blank")]
    pub blank: Option<bool>,
    /// nodata
    /// Represents the following attribute in the schema: :nodata
    #[xml(attr = "nodata")]
    pub nodata: Option<bool>,
}
/// Defines the RichDateGroupItem Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:dateGroupItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:dateGroupItem")]
pub struct RichDateGroupItem {
    /// key
    /// Represents the following attribute in the schema: :key
    #[xml(attr = "key")]
    pub key: Option<String>,
    /// Year
    /// Represents the following attribute in the schema: :year
    #[xml(attr = "year")]
    pub year: i16,
    /// Month
    /// Represents the following attribute in the schema: :month
    #[xml(attr = "month")]
    pub month: Option<i16>,
    /// Day
    /// Represents the following attribute in the schema: :day
    #[xml(attr = "day")]
    pub day: Option<i16>,
    /// Hour
    /// Represents the following attribute in the schema: :hour
    #[xml(attr = "hour")]
    pub hour: Option<i16>,
    /// Minute
    /// Represents the following attribute in the schema: :minute
    #[xml(attr = "minute")]
    pub minute: Option<i16>,
    /// Second
    /// Represents the following attribute in the schema: :second
    #[xml(attr = "second")]
    pub second: Option<i16>,
    /// Date Time Grouping
    /// Represents the following attribute in the schema: :dateTimeGrouping
    #[xml(attr = "dateTimeGrouping")]
    pub date_time_grouping: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DateTimeGroupingValues,
}
/// Defines the CustomRichFilter Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:customFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:customFilter")]
pub struct CustomRichFilter {
    /// key
    /// Represents the following attribute in the schema: :key
    #[xml(attr = "key")]
    pub key: Option<String>,
    /// Filter Comparison Operator
    /// Represents the following attribute in the schema: :operator
    #[xml(attr = "operator")]
    pub operator: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::FilterOperatorValues,
    >,
    /// Top or Bottom Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<String>,
}
/// Defines the SupportingPropertyBagArrayData Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:spbArrays.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:spbArrays")]
pub struct SupportingPropertyBagArrayData {
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: i32,
    /// _
    #[xml(child = "xlrd2:a")]
    pub xlrd2_a: Vec<SupportingPropertyBagArray>,
    /// _
    #[xml(child = "xlrd2:extLst")]
    pub xlrd2_ext_lst: Option<ExtensionList>,
}
/// Defines the SupportingPropertyBagData Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:spbData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:spbData")]
pub struct SupportingPropertyBagData {
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: i32,
    /// _
    #[xml(child = "xlrd2:spb")]
    pub xlrd2_spb: Vec<SupportingPropertyBag>,
    /// _
    #[xml(child = "xlrd2:extLst")]
    pub xlrd2_ext_lst: Option<ExtensionList>,
}
/// Defines the SupportingPropertyBag Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:spb.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:spb")]
pub struct SupportingPropertyBag {
    /// s
    /// Represents the following attribute in the schema: :s
    #[xml(attr = "s")]
    pub s: i32,
    /// _
    #[xml(child = "xlrd2:v")]
    pub xlrd2_v: Vec<SupportingPropertyBagValue>,
}
/// Defines the SupportingPropertyBagValue Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:v.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:v")]
pub struct SupportingPropertyBagValue {
    #[xml(text)]
    pub child: String,
}
/// Defines the SupportingPropertyBagStructure Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:s.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:s")]
pub struct SupportingPropertyBagStructure {
    /// _
    #[xml(child = "xlrd2:k")]
    pub xlrd2_k: Vec<SupportingPropertyBagKey>,
}
/// Defines the SupportingPropertyBagKey Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:k.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:k")]
pub struct SupportingPropertyBagKey {
    /// n
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub n: String,
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub t: Option<SupportingPropertyBagValueType>,
}
/// Defines the SupportingPropertyBagArray Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:a.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:a")]
pub struct SupportingPropertyBagArray {
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: i32,
    /// _
    #[xml(child = "xlrd2:v")]
    pub xlrd2_v: Vec<SupportingPropertyBagArrayValue>,
}
/// Defines the SupportingPropertyBagArrayValue Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:v.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:v")]
pub struct SupportingPropertyBagArrayValue {
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub t: Option<SupportingPropertyBagArrayValueType>,
    #[xml(text)]
    pub child: String,
}
/// Defines the Array Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:a.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:a")]
pub struct Array {
    /// r
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub r: i32,
    /// c
    /// Represents the following attribute in the schema: :c
    #[xml(attr = "c")]
    pub c: Option<i32>,
    /// _
    #[xml(child = "xlrd2:v")]
    pub xlrd2_v: Vec<ArrayValue>,
}
/// Defines the ArrayValue Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:v.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:v")]
pub struct ArrayValue {
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub t: Option<ArrayValueType>,
    #[xml(text)]
    pub child: String,
}
/// Defines the Dxfs Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:dxfs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:dxfs")]
pub struct Dxfs {
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
/// Defines the RichFormatProperties Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:richProperties.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:richProperties")]
pub struct RichFormatProperties {
    /// _
    #[xml(child = "xlrd2:rPr")]
    pub xlrd2_r_pr: Vec<RichFormatProperty>,
}
/// Defines the RichStyles Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:richStyles.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:richStyles")]
pub struct RichStyles {
    /// _
    #[xml(child = "xlrd2:rSty")]
    pub xlrd2_r_sty: Vec<RichStyle>,
}
/// Defines the RichFormatProperty Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:rPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:rPr")]
pub struct RichFormatProperty {
    /// n
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub n: String,
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub t: RichFormatPropertyType,
}
/// Defines the RichStyle Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:rSty.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:rSty")]
pub struct RichStyle {
    /// dxfid
    /// Represents the following attribute in the schema: :dxfid
    #[xml(attr = "dxfid")]
    pub dxfid: Option<i32>,
    /// _
    #[xml(child = "xlrd2:rpv")]
    pub xlrd2_rpv: Vec<RichStylePropertyValue>,
}
/// Defines the RichStylePropertyValue Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:rpv.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:rpv")]
pub struct RichStylePropertyValue {
    /// i
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub i: i32,
    #[xml(text)]
    pub child: String,
}
/// Defines the RichValueGlobalType Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:global.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:global")]
pub struct RichValueGlobalType {
    /// _
    #[xml(child = "xlrd2:keyFlags")]
    pub rich_value_type_key_flags: Option<RichValueTypeKeyFlags>,
    /// _
    #[xml(child = "xlrd2:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the RichValueTypes Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:types.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:types")]
pub struct RichValueTypes {
    /// _
    #[xml(child = "xlrd2:type")]
    pub xlrd2_type: Vec<RichValueType>,
}
/// Defines the RichValueType Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:type.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:type")]
pub struct RichValueType {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// _
    #[xml(child = "xlrd2:keyFlags")]
    pub rich_value_type_key_flags: Option<RichValueTypeKeyFlags>,
    /// _
    #[xml(child = "xlrd2:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the RichValueTypeKeyFlags Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:keyFlags.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:keyFlags")]
pub struct RichValueTypeKeyFlags {
    /// _
    #[xml(child = "xlrd2:key")]
    pub xlrd2_key: Vec<RichValueTypeReservedKey>,
}
/// Defines the RichValueTypeReservedKey Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:key.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:key")]
pub struct RichValueTypeReservedKey {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// _
    #[xml(child = "xlrd2:flag")]
    pub xlrd2_flag: Vec<RichValueTypeReservedKeyFlag>,
}
/// Defines the RichValueTypeReservedKeyFlag Class.
/// When the object is serialized out as xml, it's qualified name is xlrd2:flag.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd2:flag")]
pub struct RichValueTypeReservedKeyFlag {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// value
    /// Represents the following attribute in the schema: :value
    #[xml(attr = "value")]
    pub value: bool,
}
