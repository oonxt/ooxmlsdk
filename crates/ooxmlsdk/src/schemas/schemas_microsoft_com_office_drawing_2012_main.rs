#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TargetScreenSize {
    #[default]
    Sz544x376,
    Sz640x480,
    Sz720x512,
    Sz800x600,
    Sz1024x768,
    Sz1152x882,
    Sz1152x900,
    Sz1280x1024,
    Sz1600x1200,
    Sz1800x1440,
    Sz1920x1200,
}
crate::__string_enum! {
    TargetScreenSize { Sz544x376 = "544x376", Sz640x480 = "640x480", Sz720x512 =
    "720x512", Sz800x600 = "800x600", Sz1024x768 = "1024x768", Sz1152x882 = "1152x882",
    Sz1152x900 = "1152x900", Sz1280x1024 = "1280x1024", Sz1600x1200 = "1600x1200",
    Sz1800x1440 = "1800x1440", Sz1920x1200 = "1920x1200", }
}
/// Defines the BackgroundProperties Class.
/// When the object is serialized out as xml, it's qualified name is a15:backgroundPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a15:backgroundPr")]
pub struct BackgroundProperties {
    /// bwMode
    /// Represents the following attribute in the schema: :bwMode
    #[xml(attr = "bwMode")]
    pub mode: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    /// bwPure
    /// Represents the following attribute in the schema: :bwPure
    #[xml(attr = "bwPure")]
    pub pure: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    /// bwNormal
    /// Represents the following attribute in the schema: :bwNormal
    #[xml(attr = "bwNormal")]
    pub normal: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    /// targetScreenSize
    /// Represents the following attribute in the schema: :targetScreenSize
    #[xml(attr = "targetScreenSize")]
    pub target_screen_size: Option<TargetScreenSize>,
}
/// Defines the NonVisualGroupProperties Class.
/// When the object is serialized out as xml, it's qualified name is a15:nonVisualGroupProps.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a15:nonVisualGroupProps")]
pub struct NonVisualGroupProperties {
    /// isLegacyGroup
    /// Represents the following attribute in the schema: :isLegacyGroup
    #[xml(attr = "isLegacyGroup")]
    pub is_legacy_group: Option<bool>,
}
/// Defines the ObjectProperties Class.
/// When the object is serialized out as xml, it's qualified name is a15:objectPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a15:objectPr")]
pub struct ObjectProperties {
    /// objectId
    /// Represents the following attribute in the schema: :objectId
    #[xml(attr = "objectId")]
    pub id: Option<String>,
    /// isActiveX
    /// Represents the following attribute in the schema: :isActiveX
    #[xml(attr = "isActiveX")]
    pub is_active_x: Option<bool>,
    /// linkType
    /// Represents the following attribute in the schema: :linkType
    #[xml(attr = "linkType")]
    pub link_type: Option<String>,
}
/// Defines the SignatureLine Class.
/// When the object is serialized out as xml, it's qualified name is a15:signatureLine.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a15:signatureLine")]
pub struct SignatureLine {
    /// isSignatureLine
    /// Represents the following attribute in the schema: :isSignatureLine
    #[xml(attr = "isSignatureLine")]
    pub is_signature_line: Option<bool>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// provId
    /// Represents the following attribute in the schema: :provId
    #[xml(attr = "provId")]
    pub provider_id: Option<String>,
    /// signingInstructionsSet
    /// Represents the following attribute in the schema: :signingInstructionsSet
    #[xml(attr = "signingInstructionsSet")]
    pub signing_instructions_set: Option<bool>,
    /// allowComments
    /// Represents the following attribute in the schema: :allowComments
    #[xml(attr = "allowComments")]
    pub allow_comments: Option<bool>,
    /// showSignDate
    /// Represents the following attribute in the schema: :showSignDate
    #[xml(attr = "showSignDate")]
    pub show_sign_date: Option<bool>,
    /// suggestedSigner
    /// Represents the following attribute in the schema: :suggestedSigner
    #[xml(attr = "suggestedSigner")]
    pub suggested_signer: Option<String>,
    /// suggestedSigner2
    /// Represents the following attribute in the schema: :suggestedSigner2
    #[xml(attr = "suggestedSigner2")]
    pub suggested_signer2: Option<String>,
    /// suggestedSignerEmail
    /// Represents the following attribute in the schema: :suggestedSignerEmail
    #[xml(attr = "suggestedSignerEmail")]
    pub suggested_signer_email: Option<String>,
    /// signingInstructions
    /// Represents the following attribute in the schema: :signingInstructions
    #[xml(attr = "signingInstructions")]
    pub signing_instructions: Option<String>,
    /// addlXml
    /// Represents the following attribute in the schema: :addlXml
    #[xml(attr = "addlXml")]
    pub additional_xml: Option<String>,
    /// sigProvUrl
    /// Represents the following attribute in the schema: :sigProvUrl
    #[xml(attr = "sigProvUrl")]
    pub signature_provider_url: Option<String>,
}
