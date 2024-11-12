/// Defines the Xsdboolean Class.
/// When the object is serialized out as xml, it's qualified name is xlpar:autoRefresh.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlpar:autoRefresh")]
pub struct Xsdboolean {
    #[xml(text)]
    pub child: bool,
}
