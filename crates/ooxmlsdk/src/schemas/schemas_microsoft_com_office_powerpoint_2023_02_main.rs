/// Defines the PlaceholderTypeExtension Class.
/// When the object is serialized out as xml, it's qualified name is p232:phTypeExt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p232:phTypeExt")]
pub struct PlaceholderTypeExtension {
    /// _
    #[xml(child = "p232:type")]
    pub placeholder_type_acb: PlaceholderTypeAcb,
}
/// Defines the CameoEmpty Class.
/// When the object is serialized out as xml, it's qualified name is p232:cameo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p232:cameo")]
pub struct CameoEmpty {}
/// Defines the UnknownEmpty Class.
/// When the object is serialized out as xml, it's qualified name is p232:unknown.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p232:unknown")]
pub struct UnknownEmpty {}
/// Defines the EmptyType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct EmptyType {}
/// Defines the PlaceholderTypeACB Class.
/// When the object is serialized out as xml, it's qualified name is p232:type.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p232:type")]
pub struct PlaceholderTypeAcb {
    #[xml(child = "p232:cameo", child = "p232:unknown")]
    pub children: Vec<PlaceholderTypeAcbChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PlaceholderTypeAcbChildChoice {
    #[xml(tag = "p232:cameo")]
    P232Cameo(CameoEmpty),
    #[xml(tag = "p232:unknown")]
    P232Unknown(UnknownEmpty),
}
