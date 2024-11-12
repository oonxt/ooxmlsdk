/// Defines the ExternalLinksPr Class.
/// When the object is serialized out as xml, it's qualified name is xxlnp:externalLinksPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxlnp:externalLinksPr")]
pub struct ExternalLinksPr {
    /// autoRefresh
    /// Represents the following attribute in the schema: :autoRefresh
    #[xml(attr = "autoRefresh")]
    pub auto_refresh: Option<bool>,
}
