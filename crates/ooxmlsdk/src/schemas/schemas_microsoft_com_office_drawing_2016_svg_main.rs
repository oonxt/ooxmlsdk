/// Defines the SVGBlip Class.
/// When the object is serialized out as xml, it's qualified name is asvg:svgBlip.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "asvg:svgBlip")]
pub struct SvgBlip {
    /// Embedded Picture Reference
    /// Represents the following attribute in the schema: r:embed
    #[xml(attr = "r:embed")]
    pub embed: Option<String>,
    /// Linked Picture Reference
    /// Represents the following attribute in the schema: r:link
    #[xml(attr = "r:link")]
    pub link: Option<String>,
}
