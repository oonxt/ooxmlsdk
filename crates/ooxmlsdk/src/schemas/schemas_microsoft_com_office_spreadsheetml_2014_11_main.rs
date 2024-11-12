#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ModelTimeGroupingContentType {
    #[default]
    Years,
    Quarters,
    Monthsindex,
    Months,
    Daysindex,
    Days,
    Hours,
    Minutes,
    Seconds,
}
crate::__string_enum! {
    ModelTimeGroupingContentType { Years = "years", Quarters = "quarters", Monthsindex =
    "monthsindex", Months = "months", Daysindex = "daysindex", Days = "days", Hours =
    "hours", Minutes = "minutes", Seconds = "seconds", }
}
/// Defines the ModelTimeGroupings Class.
/// When the object is serialized out as xml, it's qualified name is x16:modelTimeGroupings.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x16:modelTimeGroupings")]
pub struct ModelTimeGroupings {
    /// _
    #[xml(child = "x16:modelTimeGrouping")]
    pub x16_model_time_grouping: Vec<ModelTimeGrouping>,
}
/// Defines the ModelTimeGrouping Class.
/// When the object is serialized out as xml, it's qualified name is x16:modelTimeGrouping.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x16:modelTimeGrouping")]
pub struct ModelTimeGrouping {
    /// tableName
    /// Represents the following attribute in the schema: :tableName
    #[xml(attr = "tableName")]
    pub table_name: String,
    /// columnName
    /// Represents the following attribute in the schema: :columnName
    #[xml(attr = "columnName")]
    pub column_name: String,
    /// columnId
    /// Represents the following attribute in the schema: :columnId
    #[xml(attr = "columnId")]
    pub column_id: String,
    /// _
    #[xml(child = "x16:calculatedTimeColumn")]
    pub x16_calculated_time_column: Vec<CalculatedTimeColumn>,
}
/// Defines the CalculatedTimeColumn Class.
/// When the object is serialized out as xml, it's qualified name is x16:calculatedTimeColumn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x16:calculatedTimeColumn")]
pub struct CalculatedTimeColumn {
    /// columnName
    /// Represents the following attribute in the schema: :columnName
    #[xml(attr = "columnName")]
    pub column_name: String,
    /// columnId
    /// Represents the following attribute in the schema: :columnId
    #[xml(attr = "columnId")]
    pub column_id: String,
    /// contentType
    /// Represents the following attribute in the schema: :contentType
    #[xml(attr = "contentType")]
    pub content_type: ModelTimeGroupingContentType,
    /// isSelected
    /// Represents the following attribute in the schema: :isSelected
    #[xml(attr = "isSelected")]
    pub is_selected: bool,
}
