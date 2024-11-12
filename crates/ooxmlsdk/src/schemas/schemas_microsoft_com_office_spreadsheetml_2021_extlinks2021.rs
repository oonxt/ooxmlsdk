/// Defines the ExternalBookAlternateUrls Class.
/// When the object is serialized out as xml, it's qualified name is xxl21:alternateUrls.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxl21:alternateUrls")]
pub struct ExternalBookAlternateUrls {
    /// driveId
    /// Represents the following attribute in the schema: :driveId
    #[xml(attr = "driveId")]
    pub drive_id: Option<String>,
    /// itemId
    /// Represents the following attribute in the schema: :itemId
    #[xml(attr = "itemId")]
    pub item_id: Option<String>,
    /// _
    #[xml(child = "xxl21:absoluteUrl")]
    pub absolute_url_alternate_url: Option<AbsoluteUrlAlternateUrl>,
    /// _
    #[xml(child = "xxl21:relativeUrl")]
    pub relative_url_alternate_url: Option<RelativeUrlAlternateUrl>,
}
/// Defines the AbsoluteUrlAlternateUrl Class.
/// When the object is serialized out as xml, it's qualified name is xxl21:absoluteUrl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxl21:absoluteUrl")]
pub struct AbsoluteUrlAlternateUrl {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the RelativeUrlAlternateUrl Class.
/// When the object is serialized out as xml, it's qualified name is xxl21:relativeUrl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxl21:relativeUrl")]
pub struct RelativeUrlAlternateUrl {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the OpenXmlAlternateUrlElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlAlternateUrlElement {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
