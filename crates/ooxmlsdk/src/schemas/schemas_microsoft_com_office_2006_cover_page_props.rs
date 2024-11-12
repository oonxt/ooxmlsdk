/// Defines the CoverPageProperties Class.
/// When the object is serialized out as xml, it's qualified name is cppr:CoverPageProperties.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cppr:CoverPageProperties")]
pub struct CoverPageProperties {
    /// _
    #[xml(child = "cppr:PublishDate")]
    pub publish_date: PublishDate,
    /// _
    #[xml(child = "cppr:Abstract")]
    pub document_abstract: DocumentAbstract,
    /// _
    #[xml(child = "cppr:CompanyAddress")]
    pub company_address: CompanyAddress,
    /// _
    #[xml(child = "cppr:CompanyPhone")]
    pub company_phone_number: CompanyPhoneNumber,
    /// _
    #[xml(child = "cppr:CompanyFax")]
    pub company_fax_number: CompanyFaxNumber,
    /// _
    #[xml(child = "cppr:CompanyEmail")]
    pub company_email_address: CompanyEmailAddress,
}
/// Defines the PublishDate Class.
/// When the object is serialized out as xml, it's qualified name is cppr:PublishDate.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cppr:PublishDate")]
pub struct PublishDate {
    #[xml(text)]
    pub child: String,
}
/// Defines the DocumentAbstract Class.
/// When the object is serialized out as xml, it's qualified name is cppr:Abstract.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cppr:Abstract")]
pub struct DocumentAbstract {
    #[xml(text)]
    pub child: String,
}
/// Defines the CompanyAddress Class.
/// When the object is serialized out as xml, it's qualified name is cppr:CompanyAddress.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cppr:CompanyAddress")]
pub struct CompanyAddress {
    #[xml(text)]
    pub child: String,
}
/// Defines the CompanyPhoneNumber Class.
/// When the object is serialized out as xml, it's qualified name is cppr:CompanyPhone.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cppr:CompanyPhone")]
pub struct CompanyPhoneNumber {
    #[xml(text)]
    pub child: String,
}
/// Defines the CompanyFaxNumber Class.
/// When the object is serialized out as xml, it's qualified name is cppr:CompanyFax.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cppr:CompanyFax")]
pub struct CompanyFaxNumber {
    #[xml(text)]
    pub child: String,
}
/// Defines the CompanyEmailAddress Class.
/// When the object is serialized out as xml, it's qualified name is cppr:CompanyEmail.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cppr:CompanyEmail")]
pub struct CompanyEmailAddress {
    #[xml(text)]
    pub child: String,
}
