/// Defines the Macrosheet Class.
/// When the object is serialized out as xml, it's qualified name is xne:macrosheet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xne:macrosheet")]
pub struct Macrosheet {
    #[xml(attr = "xmlns", with = "macrosheet_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Sheet Properties
    #[xml(child = "x:sheetPr")]
    pub sheet_properties: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SheetProperties,
    >,
    ///Macro Sheet Dimensions
    #[xml(child = "x:dimension")]
    pub sheet_dimension: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SheetDimension,
    >,
    ///Macro Sheet Views
    #[xml(child = "x:sheetViews")]
    pub sheet_views: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SheetViews,
    >,
    ///Sheet Format Properties
    #[xml(child = "x:sheetFormatPr")]
    pub sheet_format_properties: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SheetFormatProperties,
    >,
    /// _
    #[xml(child = "x:cols")]
    pub x_cols: Vec<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Columns,
    >,
    /// _
    #[xml(child = "x:sheetData")]
    pub x_sheet_data: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SheetData,
    /// _
    #[xml(child = "x:sheetProtection")]
    pub x_sheet_protection: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SheetProtection,
    >,
    /// _
    #[xml(child = "x:autoFilter")]
    pub x_auto_filter: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::AutoFilter,
    >,
    /// _
    #[xml(child = "x:sortState")]
    pub x_sort_state: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SortState,
    >,
    /// _
    #[xml(child = "x:dataConsolidate")]
    pub x_data_consolidate: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataConsolidate,
    >,
    /// _
    #[xml(child = "x:customSheetViews")]
    pub x_custom_sheet_views: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CustomSheetViews,
    >,
    /// _
    #[xml(child = "x:phoneticPr")]
    pub x_phonetic_pr: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PhoneticProperties,
    >,
    /// _
    #[xml(child = "x:conditionalFormatting")]
    pub x_conditional_formatting: Vec<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ConditionalFormatting,
    >,
    /// _
    #[xml(child = "x:printOptions")]
    pub x_print_options: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PrintOptions,
    >,
    /// _
    #[xml(child = "x:pageMargins")]
    pub x_page_margins: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PageMargins,
    >,
    /// _
    #[xml(child = "x:pageSetup")]
    pub x_page_setup: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PageSetup,
    >,
    /// _
    #[xml(child = "x:headerFooter")]
    pub x_header_footer: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::HeaderFooter,
    >,
    /// _
    #[xml(child = "x:rowBreaks")]
    pub x_row_breaks: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::RowBreaks,
    >,
    /// _
    #[xml(child = "x:colBreaks")]
    pub x_col_breaks: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ColumnBreaks,
    >,
    /// _
    #[xml(child = "x:customProperties")]
    pub x_custom_properties: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CustomProperties,
    >,
    /// _
    #[xml(child = "x:drawing")]
    pub x_drawing: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Drawing,
    >,
    /// _
    #[xml(child = "x:legacyDrawing")]
    pub x_legacy_drawing: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::LegacyDrawing,
    >,
    /// _
    #[xml(child = "x:legacyDrawingHF")]
    pub x_legacy_drawing_hf: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::LegacyDrawingHeaderFooter,
    >,
    /// _
    #[xml(child = "x:picture")]
    pub x_picture: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Picture,
    >,
    /// _
    #[xml(child = "x:oleObjects")]
    pub x_ole_objects: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::OleObjects,
    >,
    /// _
    #[xml(child = "x:drawingHF")]
    pub x_drawing_hf: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DrawingHeaderFooter,
    >,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList,
    >,
}
mod macrosheet_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/excel/2006/main")
    }
}
/// Worksheet Sort Map.
/// When the object is serialized out as xml, it's qualified name is xne:worksheetSortMap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xne:worksheetSortMap")]
pub struct WorksheetSortMap {
    #[xml(attr = "xmlns", with = "worksheet_sort_map_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Row Sort Map
    #[xml(child = "xne:rowSortMap")]
    pub row_sort_map: Option<RowSortMap>,
    ///Column Sort Map
    #[xml(child = "xne:colSortMap")]
    pub column_sort_map: Option<ColumnSortMap>,
}
mod worksheet_sort_map_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/excel/2006/main")
    }
}
/// Defines the ReferenceSequence Class.
/// When the object is serialized out as xml, it's qualified name is xne:sqref.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xne:sqref")]
pub struct ReferenceSequence {
    #[xml(text)]
    pub child: String,
}
/// Defines the Formula Class.
/// When the object is serialized out as xml, it's qualified name is xne:f.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xne:f")]
pub struct Formula {
    #[xml(text)]
    pub child: String,
}
/// Row Sort Map.
/// When the object is serialized out as xml, it's qualified name is xne:rowSortMap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xne:rowSortMap")]
pub struct RowSortMap {
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub r#ref: String,
    /// Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<u32>,
    /// _
    #[xml(child = "xne:row")]
    pub xne_row: Vec<RowSortMapItem>,
}
/// Column Sort Map.
/// When the object is serialized out as xml, it's qualified name is xne:colSortMap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xne:colSortMap")]
pub struct ColumnSortMap {
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub r#ref: String,
    /// Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<u32>,
    /// _
    #[xml(child = "xne:col")]
    pub xne_col: Vec<ColumnSortMapItem>,
}
/// Row.
/// When the object is serialized out as xml, it's qualified name is xne:row.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xne:row")]
pub struct RowSortMapItem {
    /// New Value
    /// Represents the following attribute in the schema: :newVal
    #[xml(attr = "newVal")]
    pub new_val: u32,
    /// Old Value
    /// Represents the following attribute in the schema: :oldVal
    #[xml(attr = "oldVal")]
    pub old_val: u32,
}
/// Column.
/// When the object is serialized out as xml, it's qualified name is xne:col.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xne:col")]
pub struct ColumnSortMapItem {
    /// New Value
    /// Represents the following attribute in the schema: :newVal
    #[xml(attr = "newVal")]
    pub new_val: u32,
    /// Old Value
    /// Represents the following attribute in the schema: :oldVal
    #[xml(attr = "oldVal")]
    pub old_val: u32,
}
/// Defines the SortMapItemType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct SortMapItemType {
    /// New Value
    /// Represents the following attribute in the schema: :newVal
    #[xml(attr = "newVal")]
    pub new_val: u32,
    /// Old Value
    /// Represents the following attribute in the schema: :oldVal
    #[xml(attr = "oldVal")]
    pub old_val: u32,
}
