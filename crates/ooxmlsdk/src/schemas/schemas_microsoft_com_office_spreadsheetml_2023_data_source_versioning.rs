/// Defines the VersionInfo Class.
/// When the object is serialized out as xml, it's qualified name is xxdsv:versionInfo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxdsv:versionInfo")]
pub struct VersionInfo {
    /// _
    #[xml(child = "xxdsv:requiredFeature")]
    pub xxdsv_required_feature: Vec<RequiredFeatureXsdstring>,
    /// _
    #[xml(child = "xxdsv:lastRefreshFeature")]
    pub xxdsv_last_refresh_feature: Vec<LastRefreshFeatureXsdstring>,
    /// _
    #[xml(child = "xxdsv:lastEditFeature")]
    pub xxdsv_last_edit_feature: Vec<LastEditFeatureXsdstring>,
}
/// Defines the RequiredFeatureXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is xxdsv:requiredFeature.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxdsv:requiredFeature")]
pub struct RequiredFeatureXsdstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the LastRefreshFeatureXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is xxdsv:lastRefreshFeature.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxdsv:lastRefreshFeature")]
pub struct LastRefreshFeatureXsdstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the LastEditFeatureXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is xxdsv:lastEditFeature.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxdsv:lastEditFeature")]
pub struct LastEditFeatureXsdstring {
    #[xml(text)]
    pub child: String,
}
