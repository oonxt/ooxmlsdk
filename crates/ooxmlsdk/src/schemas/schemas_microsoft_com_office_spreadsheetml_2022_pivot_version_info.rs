/// Defines the CacheVersionInfo Class.
/// When the object is serialized out as xml, it's qualified name is xxpvi:cacheVersionInfo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxpvi:cacheVersionInfo")]
pub struct CacheVersionInfo {
    /// _
    #[xml(child = "xxpvi:requiredFeature")]
    pub xxpvi_required_feature: Vec<RequiredFeatureXsdstring>,
    /// _
    #[xml(child = "xxpvi:lastRefreshFeature")]
    pub xxpvi_last_refresh_feature: Vec<LastRefreshFeatureXsdstring>,
}
/// Defines the PivotVersionInfo Class.
/// When the object is serialized out as xml, it's qualified name is xxpvi:pivotVersionInfo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxpvi:pivotVersionInfo")]
pub struct PivotVersionInfo {
    /// _
    #[xml(child = "xxpvi:requiredFeature")]
    pub xxpvi_required_feature: Vec<RequiredFeatureXsdstring>,
    /// _
    #[xml(child = "xxpvi:lastUpdateFeature")]
    pub xxpvi_last_update_feature: Vec<LastUpdateFeatureXsdstring>,
}
/// Defines the RequiredFeatureXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is xxpvi:requiredFeature.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxpvi:requiredFeature")]
pub struct RequiredFeatureXsdstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the LastRefreshFeatureXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is xxpvi:lastRefreshFeature.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxpvi:lastRefreshFeature")]
pub struct LastRefreshFeatureXsdstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the LastUpdateFeatureXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is xxpvi:lastUpdateFeature.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxpvi:lastUpdateFeature")]
pub struct LastUpdateFeatureXsdstring {
    #[xml(text)]
    pub child: String,
}
