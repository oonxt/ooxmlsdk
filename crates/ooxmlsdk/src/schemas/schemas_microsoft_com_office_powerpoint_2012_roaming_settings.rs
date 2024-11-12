/// Defines the Key Class.
/// When the object is serialized out as xml, it's qualified name is pRoam:key.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pRoam:key")]
pub struct Key {
    #[xml(text)]
    pub child: String,
}
/// Defines the Value Class.
/// When the object is serialized out as xml, it's qualified name is pRoam:value.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pRoam:value")]
pub struct Value {
    #[xml(text)]
    pub child: String,
}
/// Defines the RoamingProperty Class.
/// When the object is serialized out as xml, it's qualified name is pRoam:props.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pRoam:props")]
pub struct RoamingProperty {
    /// _
    #[xml(child = "pRoam:key")]
    pub key: Key,
    /// _
    #[xml(child = "pRoam:value")]
    pub value: Value,
}
