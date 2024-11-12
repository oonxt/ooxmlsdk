/// Defines the RichValueRefreshIntervals Class.
/// When the object is serialized out as xml, it's qualified name is xlrvr:refreshIntervals.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrvr:refreshIntervals")]
pub struct RichValueRefreshIntervals {
    /// _
    #[xml(child = "xlrvr:refreshInterval")]
    pub xlrvr_refresh_interval: Vec<RichValueRefreshInterval>,
}
/// Defines the RichValueRefreshInterval Class.
/// When the object is serialized out as xml, it's qualified name is xlrvr:refreshInterval.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrvr:refreshInterval")]
pub struct RichValueRefreshInterval {
    /// resourceIdInt
    /// Represents the following attribute in the schema: :resourceIdInt
    #[xml(attr = "resourceIdInt")]
    pub resource_id_int: Option<i32>,
    /// resourceIdStr
    /// Represents the following attribute in the schema: :resourceIdStr
    #[xml(attr = "resourceIdStr")]
    pub resource_id_str: Option<String>,
    /// interval
    /// Represents the following attribute in the schema: :interval
    #[xml(attr = "interval")]
    pub interval: i32,
}
