/// Defines the PivotTableDefinition16 Class.
/// When the object is serialized out as xml, it's qualified name is xpdl:pivotTableDefinition16.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xpdl:pivotTableDefinition16")]
pub struct PivotTableDefinition16 {
    /// EnabledSubtotalsDefault
    /// Represents the following attribute in the schema: :EnabledSubtotalsDefault
    #[xml(attr = "EnabledSubtotalsDefault")]
    pub enabled_subtotals_default: Option<bool>,
    /// SubtotalsOnTopDefault
    /// Represents the following attribute in the schema: :SubtotalsOnTopDefault
    #[xml(attr = "SubtotalsOnTopDefault")]
    pub subtotals_on_top_default: Option<bool>,
    /// InsertBlankRowDefault
    /// Represents the following attribute in the schema: :InsertBlankRowDefault
    #[xml(attr = "InsertBlankRowDefault")]
    pub insert_blank_row_default: Option<bool>,
}
