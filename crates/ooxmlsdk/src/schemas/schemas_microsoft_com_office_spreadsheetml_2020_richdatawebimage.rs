/// Defines the WebImagesSupportingRichData Class.
/// When the object is serialized out as xml, it's qualified name is xlrdwi:webImagesSrd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrdwi:webImagesSrd")]
pub struct WebImagesSupportingRichData {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "xlrdwi:webImageSrd")]
    pub xlrdwi_web_image_srd: Vec<WebImageSupportingRichData>,
    /// _
    #[xml(child = "xlrdwi:extLst")]
    pub xlrdwi_ext_lst: Option<ExtensionList>,
}
/// Defines the WebImageSupportingRichData Class.
/// When the object is serialized out as xml, it's qualified name is xlrdwi:webImageSrd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrdwi:webImageSrd")]
pub struct WebImageSupportingRichData {
    /// _
    #[xml(child = "xlrdwi:address")]
    pub address_web_image_supporting_rich_data_relationship: AddressWebImageSupportingRichDataRelationship,
    /// _
    #[xml(child = "xlrdwi:moreImagesAddress")]
    pub more_images_address_web_image_supporting_rich_data_relationship: Option<
        MoreImagesAddressWebImageSupportingRichDataRelationship,
    >,
    /// _
    #[xml(child = "xlrdwi:blip")]
    pub blip_web_image_supporting_rich_data_relationship: Option<
        BlipWebImageSupportingRichDataRelationship,
    >,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is xlrdwi:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrdwi:extLst")]
pub struct ExtensionList {
    #[xml(child = "x:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "x:ext")]
    XExt(crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension),
}
/// Defines the AddressWebImageSupportingRichDataRelationship Class.
/// When the object is serialized out as xml, it's qualified name is xlrdwi:address.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrdwi:address")]
pub struct AddressWebImageSupportingRichDataRelationship {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the MoreImagesAddressWebImageSupportingRichDataRelationship Class.
/// When the object is serialized out as xml, it's qualified name is xlrdwi:moreImagesAddress.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrdwi:moreImagesAddress")]
pub struct MoreImagesAddressWebImageSupportingRichDataRelationship {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the BlipWebImageSupportingRichDataRelationship Class.
/// When the object is serialized out as xml, it's qualified name is xlrdwi:blip.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrdwi:blip")]
pub struct BlipWebImageSupportingRichDataRelationship {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the OpenXmlWebImageSupportingRichDataRelationshipElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlWebImageSupportingRichDataRelationshipElement {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
