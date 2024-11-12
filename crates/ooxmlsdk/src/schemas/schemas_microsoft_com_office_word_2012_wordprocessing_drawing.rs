/// Defines the WebVideoProperty Class.
/// When the object is serialized out as xml, it's qualified name is wp15:webVideoPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp15:webVideoPr")]
pub struct WebVideoProperty {
    /// embeddedHtml
    /// Represents the following attribute in the schema: :embeddedHtml
    #[xml(attr = "embeddedHtml")]
    pub embedded_html: Option<String>,
    /// h
    /// Represents the following attribute in the schema: :h
    #[xml(attr = "h")]
    pub height: Option<i32>,
    /// w
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i32>,
}
