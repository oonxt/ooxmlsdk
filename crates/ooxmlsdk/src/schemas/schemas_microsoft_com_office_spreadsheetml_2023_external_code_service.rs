/// Defines the ExternalCodeService Class.
/// When the object is serialized out as xml, it's qualified name is xlecs:externalCodeService.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlecs:externalCodeService")]
pub struct ExternalCodeService {
    /// autoShow
    /// Represents the following attribute in the schema: :autoShow
    #[xml(attr = "autoShow")]
    pub auto_show: Option<i32>,
    /// timeout
    /// Represents the following attribute in the schema: :timeout
    #[xml(attr = "timeout")]
    pub timeout: Option<i32>,
}
