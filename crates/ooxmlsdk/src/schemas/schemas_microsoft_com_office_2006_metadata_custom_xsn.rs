/// Defines the CustomXsn Class.
/// When the object is serialized out as xml, it's qualified name is ntns:customXsn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ntns:customXsn")]
pub struct CustomXsn {
    /// _
    #[xml(child = "ntns:xsnLocation")]
    pub xsn_location: XsnLocation,
    /// _
    #[xml(child = "ntns:cached")]
    pub cached_view: CachedView,
    /// _
    #[xml(child = "ntns:openByDefault")]
    pub open_by_default: OpenByDefault,
    /// _
    #[xml(child = "ntns:xsnScope")]
    pub scope: Scope,
}
/// Defines the XsnLocation Class.
/// When the object is serialized out as xml, it's qualified name is ntns:xsnLocation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ntns:xsnLocation")]
pub struct XsnLocation {
    #[xml(text)]
    pub child: String,
}
/// Defines the CachedView Class.
/// When the object is serialized out as xml, it's qualified name is ntns:cached.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ntns:cached")]
pub struct CachedView {
    #[xml(text)]
    pub child: String,
}
/// Defines the OpenByDefault Class.
/// When the object is serialized out as xml, it's qualified name is ntns:openByDefault.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ntns:openByDefault")]
pub struct OpenByDefault {
    #[xml(text)]
    pub child: String,
}
/// Defines the Scope Class.
/// When the object is serialized out as xml, it's qualified name is ntns:xsnScope.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ntns:xsnScope")]
pub struct Scope {
    #[xml(text)]
    pub child: String,
}
