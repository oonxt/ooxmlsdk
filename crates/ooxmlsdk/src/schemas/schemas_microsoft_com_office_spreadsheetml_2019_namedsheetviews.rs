/// Defines the NamedSheetViews Class.
/// When the object is serialized out as xml, it's qualified name is xnsv:namedSheetViews.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xnsv:namedSheetViews")]
pub struct NamedSheetViews {
    #[xml(attr = "xmlns", with = "named_sheet_views_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "xnsv:namedSheetView")]
    pub xnsv_named_sheet_view: Vec<NamedSheetView>,
    /// _
    #[xml(child = "xnsv:extLst")]
    pub xnsv_ext_lst: Option<ExtensionList>,
}
mod named_sheet_views_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/spreadsheetml/2019/namedsheetviews")
    }
}
/// Defines the NamedSheetView Class.
/// When the object is serialized out as xml, it's qualified name is xnsv:namedSheetView.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xnsv:namedSheetView")]
pub struct NamedSheetView {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// _
    #[xml(child = "xnsv:nsvFilter")]
    pub xnsv_nsv_filter: Vec<NsvFilter>,
    /// _
    #[xml(child = "xnsv:extLst")]
    pub xnsv_ext_lst: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is xnsv:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xnsv:extLst")]
pub struct ExtensionList {
    #[xml(child = "x:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "x:ext")]
    XExt(crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension),
}
/// Defines the NsvFilter Class.
/// When the object is serialized out as xml, it's qualified name is xnsv:nsvFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xnsv:nsvFilter")]
pub struct NsvFilter {
    /// filterId
    /// Represents the following attribute in the schema: :filterId
    #[xml(attr = "filterId")]
    pub filter_id: String,
    /// ref
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub r#ref: Option<String>,
    /// tableId
    /// Represents the following attribute in the schema: :tableId
    #[xml(attr = "tableId")]
    pub table_id: Option<i32>,
    /// _
    #[xml(child = "xnsv:columnFilter")]
    pub xnsv_column_filter: Vec<ColumnFilter>,
    /// _
    #[xml(child = "xnsv:sortRules")]
    pub xnsv_sort_rules: Option<SortRules>,
    /// _
    #[xml(child = "xnsv:extLst")]
    pub xnsv_ext_lst: Option<ExtensionList>,
}
/// Defines the ColumnFilter Class.
/// When the object is serialized out as xml, it's qualified name is xnsv:columnFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xnsv:columnFilter")]
pub struct ColumnFilter {
    /// colId
    /// Represents the following attribute in the schema: :colId
    #[xml(attr = "colId")]
    pub col_id: i32,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// _
    #[xml(child = "xnsv:dxf")]
    pub differential_format_type: Option<DifferentialFormatType>,
    /// _
    #[xml(child = "xnsv:filter")]
    pub xnsv_filter: Vec<FilterColumn>,
    /// _
    #[xml(child = "xnsv:extLst")]
    pub xnsv_ext_lst: Option<ExtensionList>,
}
/// Defines the SortRules Class.
/// When the object is serialized out as xml, it's qualified name is xnsv:sortRules.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xnsv:sortRules")]
pub struct SortRules {
    /// sortMethod
    /// Represents the following attribute in the schema: :sortMethod
    #[xml(attr = "sortMethod")]
    pub sort_method: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SortMethodValues,
    >,
    /// caseSensitive
    /// Represents the following attribute in the schema: :caseSensitive
    #[xml(attr = "caseSensitive")]
    pub case_sensitive: Option<bool>,
    /// _
    #[xml(child = "xnsv:sortRule")]
    pub xnsv_sort_rule: Vec<SortRule>,
    /// _
    #[xml(child = "xnsv:extLst")]
    pub xnsv_ext_lst: Option<ExtensionList>,
}
/// Defines the DifferentialFormatType Class.
/// When the object is serialized out as xml, it's qualified name is xnsv:dxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xnsv:dxf")]
pub struct DifferentialFormatType {
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
/// Defines the FilterColumn Class.
/// When the object is serialized out as xml, it's qualified name is xnsv:filter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xnsv:filter")]
pub struct FilterColumn {
    /// Filter Column Data
    /// Represents the following attribute in the schema: :colId
    #[xml(attr = "colId")]
    pub column_id: i32,
    /// Hidden AutoFilter Button
    /// Represents the following attribute in the schema: :hiddenButton
    #[xml(attr = "hiddenButton")]
    pub hidden_button: Option<bool>,
    /// Show Filter Button
    /// Represents the following attribute in the schema: :showButton
    #[xml(attr = "showButton")]
    pub show_button: Option<bool>,
    #[xml(
        child = "x:filters",
        child = "x:top10",
        child = "x14:customFilters",
        child = "x:customFilters",
        child = "x:dynamicFilter",
        child = "x:colorFilter",
        child = "x14:iconFilter",
        child = "x:iconFilter",
        child = "x:extLst",
    )]
    pub children: Vec<FilterColumnChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FilterColumnChildChoice {
    #[xml(tag = "x:filters")]
    XFilters(
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Filters,
    ),
    #[xml(tag = "x:top10")]
    XTop10(crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Top10),
    #[xml(tag = "x14:customFilters")]
    X14CustomFilters(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::CustomFilters,
    ),
    #[xml(tag = "x:customFilters")]
    XCustomFilters(
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CustomFilters,
    ),
    #[xml(tag = "x:dynamicFilter")]
    XDynamicFilter(
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DynamicFilter,
    ),
    #[xml(tag = "x:colorFilter")]
    XColorFilter(
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ColorFilter,
    ),
    #[xml(tag = "x14:iconFilter")]
    X14IconFilter(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::IconFilter,
    ),
    #[xml(tag = "x:iconFilter")]
    XIconFilter(
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::IconFilter,
    ),
    #[xml(tag = "x:extLst")]
    XExtLst(
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList,
    ),
}
/// Defines the SortRule Class.
/// When the object is serialized out as xml, it's qualified name is xnsv:sortRule.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xnsv:sortRule")]
pub struct SortRule {
    /// colId
    /// Represents the following attribute in the schema: :colId
    #[xml(attr = "colId")]
    pub col_id: i32,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    #[xml(
        child = "xnsv:dxf",
        child = "xnsv:sortCondition",
        child = "xnsv:richSortCondition",
    )]
    pub children: Vec<SortRuleChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SortRuleChildChoice {
    #[xml(tag = "xnsv:dxf")]
    XnsvDxf(DifferentialFormatType),
    #[xml(tag = "xnsv:sortCondition")]
    XnsvSortCondition(SortCondition),
    #[xml(tag = "xnsv:richSortCondition")]
    XnsvRichSortCondition(RichSortCondition),
}
/// Defines the SortCondition Class.
/// When the object is serialized out as xml, it's qualified name is xnsv:sortCondition.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xnsv:sortCondition")]
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
    pub icon_set: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::IconSetTypeValues,
    >,
    /// iconId
    /// Represents the following attribute in the schema: :iconId
    #[xml(attr = "iconId")]
    pub icon_id: Option<i32>,
}
/// Defines the RichSortCondition Class.
/// When the object is serialized out as xml, it's qualified name is xnsv:richSortCondition.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xnsv:richSortCondition")]
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
