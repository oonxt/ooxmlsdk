/// Custom File Properties.
/// When the object is serialized out as xml, it's qualified name is Properties.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "Properties")]
pub struct Properties {
    #[xml(attr = "xmlns", with = "properties_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "property")]
    pub op_property: Vec<CustomDocumentProperty>,
}
mod properties_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/officeDocument/2006/custom-properties")
    }
}
/// Custom File Property.
/// When the object is serialized out as xml, it's qualified name is property.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "property")]
pub struct CustomDocumentProperty {
    /// Format ID
    /// Represents the following attribute in the schema: :fmtid
    #[xml(attr = "fmtid")]
    pub format_id: String,
    /// Property ID
    /// Represents the following attribute in the schema: :pid
    #[xml(attr = "pid")]
    pub property_id: i32,
    /// Custom File Property Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Bookmark Link Target
    /// Represents the following attribute in the schema: :linkTarget
    #[xml(attr = "linkTarget")]
    pub link_target: Option<String>,
    #[xml(
        child = "vt:vector",
        child = "vt:array",
        child = "vt:blob",
        child = "vt:oblob",
        child = "vt:empty",
        child = "vt:null",
        child = "vt:i1",
        child = "vt:i2",
        child = "vt:i4",
        child = "vt:i8",
        child = "vt:int",
        child = "vt:ui1",
        child = "vt:ui2",
        child = "vt:ui4",
        child = "vt:ui8",
        child = "vt:uint",
        child = "vt:r4",
        child = "vt:r8",
        child = "vt:decimal",
        child = "vt:lpstr",
        child = "vt:lpwstr",
        child = "vt:bstr",
        child = "vt:date",
        child = "vt:filetime",
        child = "vt:bool",
        child = "vt:cy",
        child = "vt:error",
        child = "vt:stream",
        child = "vt:ostream",
        child = "vt:storage",
        child = "vt:ostorage",
        child = "vt:vstream",
        child = "vt:clsid",
        child = "vt:cf",
    )]
    pub children: Vec<CustomDocumentPropertyChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CustomDocumentPropertyChildChoice {
    #[xml(tag = "vt:vector")]
    VtVector(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtVector,
    ),
    #[xml(tag = "vt:array")]
    VtArray(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtArray,
    ),
    #[xml(tag = "vt:blob")]
    VtBlob(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtBlob,
    ),
    #[xml(tag = "vt:oblob")]
    VtOblob(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtoBlob,
    ),
    #[xml(tag = "vt:empty")]
    VtEmpty(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtEmpty,
    ),
    #[xml(tag = "vt:null")]
    VtNull(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtNull,
    ),
    #[xml(tag = "vt:i1")]
    VtI1(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtByte,
    ),
    #[xml(tag = "vt:i2")]
    VtI2(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtShort,
    ),
    #[xml(tag = "vt:i4")]
    VtI4(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtInt32,
    ),
    #[xml(tag = "vt:i8")]
    VtI8(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtInt64,
    ),
    #[xml(tag = "vt:int")]
    VtInt(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtInteger,
    ),
    #[xml(tag = "vt:ui1")]
    VtUi1(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtUnsignedByte,
    ),
    #[xml(tag = "vt:ui2")]
    VtUi2(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtUnsignedShort,
    ),
    #[xml(tag = "vt:ui4")]
    VtUi4(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtUnsignedInt32,
    ),
    #[xml(tag = "vt:ui8")]
    VtUi8(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtUnsignedInt64,
    ),
    #[xml(tag = "vt:uint")]
    VtUint(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtUnsignedInteger,
    ),
    #[xml(tag = "vt:r4")]
    VtR4(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtFloat,
    ),
    #[xml(tag = "vt:r8")]
    VtR8(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtDouble,
    ),
    #[xml(tag = "vt:decimal")]
    VtDecimal(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtDecimal,
    ),
    #[xml(tag = "vt:lpstr")]
    VtLpstr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::Vtlpstr,
    ),
    #[xml(tag = "vt:lpwstr")]
    VtLpwstr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::Vtlpwstr,
    ),
    #[xml(tag = "vt:bstr")]
    VtBstr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtbString,
    ),
    #[xml(tag = "vt:date")]
    VtDate(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtDate,
    ),
    #[xml(tag = "vt:filetime")]
    VtFiletime(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtFileTime,
    ),
    #[xml(tag = "vt:bool")]
    VtBool(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtBool,
    ),
    #[xml(tag = "vt:cy")]
    VtCy(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtCurrency,
    ),
    #[xml(tag = "vt:error")]
    VtError(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtError,
    ),
    #[xml(tag = "vt:stream")]
    VtStream(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtStreamData,
    ),
    #[xml(tag = "vt:ostream")]
    VtOstream(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtoStreamData,
    ),
    #[xml(tag = "vt:storage")]
    VtStorage(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtStorage,
    ),
    #[xml(tag = "vt:ostorage")]
    VtOstorage(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtoStorage,
    ),
    #[xml(tag = "vt:vstream")]
    VtVstream(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtvStreamData,
    ),
    #[xml(tag = "vt:clsid")]
    VtClsid(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtClassId,
    ),
    #[xml(tag = "vt:cf")]
    VtCf(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtClipboardData,
    ),
}
