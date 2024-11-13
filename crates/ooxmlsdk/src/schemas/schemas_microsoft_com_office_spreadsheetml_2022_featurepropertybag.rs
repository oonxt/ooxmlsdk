/// Defines the FeaturePropertyBags Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:FeaturePropertyBags.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:FeaturePropertyBags")]
pub struct FeaturePropertyBags {
    #[xml(attr = "xmlns", with = "feature_property_bags_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "xfpb:bagExt")]
    pub xfpb_bag_ext: Vec<BagExtensions>,
    /// _
    #[xml(child = "xfpb:bag")]
    pub xfpb_bag: Vec<FeaturePropertyBag>,
    /// _
    #[xml(child = "xfpb:extLst")]
    pub xfpb_ext_lst: Option<ExtensionList>,
}
mod feature_property_bags_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/spreadsheetml/2022/featurepropertybag")
    }
}
/// Defines the FpbsFeaturePropertyBags Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:fpbs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:fpbs")]
pub struct FpbsFeaturePropertyBags {
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    #[xml(child = "xfpb:bagExt", child = "xfpb:bag", child = "xfpb:extLst")]
    pub children: Vec<FpbsFeaturePropertyBagsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FpbsFeaturePropertyBagsChildChoice {
    #[xml(tag = "xfpb:bagExt")]
    XfpbBagExt(BagExtensions),
    #[xml(tag = "xfpb:bag")]
    XfpbBag(FeaturePropertyBag),
    #[xml(tag = "xfpb:extLst")]
    XfpbExtLst(ExtensionList),
}
/// Defines the OpenXmlFeaturePropertyBagsElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlFeaturePropertyBagsElement {
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    #[xml(child = "xfpb:bagExt", child = "xfpb:bag", child = "xfpb:extLst")]
    pub children: Vec<OpenXmlFeaturePropertyBagsElementChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OpenXmlFeaturePropertyBagsElementChildChoice {
    #[xml(tag = "xfpb:bagExt")]
    XfpbBagExt(BagExtensions),
    #[xml(tag = "xfpb:bag")]
    XfpbBag(FeaturePropertyBag),
    #[xml(tag = "xfpb:extLst")]
    XfpbExtLst(ExtensionList),
}
/// Defines the XfComplement Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:xfComplement.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:xfComplement")]
pub struct XfComplement {
    /// i
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub i: i32,
}
/// Defines the DXFComplement Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:DXFComplement.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:DXFComplement")]
pub struct DxfComplement {
    /// i
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub i: i32,
}
/// Defines the RevDxf Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:revdxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:revdxf")]
pub struct RevDxf {
    /// _
    #[xml(child = "xfpb:fpbs")]
    pub fpbs_feature_property_bags: FpbsFeaturePropertyBags,
    /// _
    #[xml(child = "xfpb:dxf")]
    pub differential_format_type: Option<DifferentialFormatType>,
}
/// Defines the HeaderRowRevDxfTableRevDxf Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:headerRowRevDxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:headerRowRevDxf")]
pub struct HeaderRowRevDxfTableRevDxf {
    /// _
    #[xml(child = "xfpb:fpbs")]
    pub fpbs_feature_property_bags: FpbsFeaturePropertyBags,
    /// _
    #[xml(child = "xfpb:dxf")]
    pub differential_format_type: Option<DifferentialFormatType>,
}
/// Defines the DataRevDxfTableRevDxf Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:dataRevDxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:dataRevDxf")]
pub struct DataRevDxfTableRevDxf {
    /// _
    #[xml(child = "xfpb:fpbs")]
    pub fpbs_feature_property_bags: FpbsFeaturePropertyBags,
    /// _
    #[xml(child = "xfpb:dxf")]
    pub differential_format_type: Option<DifferentialFormatType>,
}
/// Defines the TotalsRowRevDxfTableRevDxf Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:totalsRowRevDxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:totalsRowRevDxf")]
pub struct TotalsRowRevDxfTableRevDxf {
    /// _
    #[xml(child = "xfpb:fpbs")]
    pub fpbs_feature_property_bags: FpbsFeaturePropertyBags,
    /// _
    #[xml(child = "xfpb:dxf")]
    pub differential_format_type: Option<DifferentialFormatType>,
}
/// Defines the HeaderRowBorderRevDxfTableRevDxf Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:headerRowBorderRevDxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:headerRowBorderRevDxf")]
pub struct HeaderRowBorderRevDxfTableRevDxf {
    /// _
    #[xml(child = "xfpb:fpbs")]
    pub fpbs_feature_property_bags: FpbsFeaturePropertyBags,
    /// _
    #[xml(child = "xfpb:dxf")]
    pub differential_format_type: Option<DifferentialFormatType>,
}
/// Defines the TableBorderRevDxfTableRevDxf Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:tableBorderRevDxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:tableBorderRevDxf")]
pub struct TableBorderRevDxfTableRevDxf {
    /// _
    #[xml(child = "xfpb:fpbs")]
    pub fpbs_feature_property_bags: FpbsFeaturePropertyBags,
    /// _
    #[xml(child = "xfpb:dxf")]
    pub differential_format_type: Option<DifferentialFormatType>,
}
/// Defines the TotalsRowBorderRevDxfTableRevDxf Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:totalsRowBorderRevDxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:totalsRowBorderRevDxf")]
pub struct TotalsRowBorderRevDxfTableRevDxf {
    /// _
    #[xml(child = "xfpb:fpbs")]
    pub fpbs_feature_property_bags: FpbsFeaturePropertyBags,
    /// _
    #[xml(child = "xfpb:dxf")]
    pub differential_format_type: Option<DifferentialFormatType>,
}
/// Defines the ColumnHeaderRevDxfTableRevDxf Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:columnHeaderRevDxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:columnHeaderRevDxf")]
pub struct ColumnHeaderRevDxfTableRevDxf {
    /// _
    #[xml(child = "xfpb:fpbs")]
    pub fpbs_feature_property_bags: FpbsFeaturePropertyBags,
    /// _
    #[xml(child = "xfpb:dxf")]
    pub differential_format_type: Option<DifferentialFormatType>,
}
/// Defines the ColumnBodyRevDxfTableRevDxf Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:columnBodyRevDxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:columnBodyRevDxf")]
pub struct ColumnBodyRevDxfTableRevDxf {
    /// _
    #[xml(child = "xfpb:fpbs")]
    pub fpbs_feature_property_bags: FpbsFeaturePropertyBags,
    /// _
    #[xml(child = "xfpb:dxf")]
    pub differential_format_type: Option<DifferentialFormatType>,
}
/// Defines the ColumnTotalsRevDxfTableRevDxf Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:columnTotalsRevDxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:columnTotalsRevDxf")]
pub struct ColumnTotalsRevDxfTableRevDxf {
    /// _
    #[xml(child = "xfpb:fpbs")]
    pub fpbs_feature_property_bags: FpbsFeaturePropertyBags,
    /// _
    #[xml(child = "xfpb:dxf")]
    pub differential_format_type: Option<DifferentialFormatType>,
}
/// Defines the OpenXmlTableRevDxfElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlTableRevDxfElement {}
/// Defines the BagExtensions Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:bagExt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:bagExt")]
pub struct BagExtensions {
    /// _
    #[xml(child = "xfpb:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the FeaturePropertyBag Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:bag.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:bag")]
pub struct FeaturePropertyBag {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: String,
    /// extRef
    /// Represents the following attribute in the schema: :extRef
    #[xml(attr = "extRef")]
    pub ext_ref: Option<String>,
    /// bagExtId
    /// Represents the following attribute in the schema: :bagExtId
    #[xml(attr = "bagExtId")]
    pub bag_ext_id: Option<i32>,
    /// att
    /// Represents the following attribute in the schema: :att
    #[xml(attr = "att")]
    pub att: Option<String>,
    #[xml(
        child = "xfpb:a",
        child = "xfpb:bagId",
        child = "xfpb:i",
        child = "xfpb:s",
        child = "xfpb:b",
        child = "xfpb:d",
        child = "xfpb:rel",
    )]
    pub children: Vec<FeaturePropertyBagChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FeaturePropertyBagChildChoice {
    #[xml(tag = "xfpb:a")]
    XfpbA(ArrayFeatureProperty),
    #[xml(tag = "xfpb:bagId")]
    XfpbBagId(BagFeatureProperty),
    #[xml(tag = "xfpb:i")]
    XfpbI(IntFeatureProperty),
    #[xml(tag = "xfpb:s")]
    XfpbS(StringFeatureProperty),
    #[xml(tag = "xfpb:b")]
    XfpbB(BoolFeatureProperty),
    #[xml(tag = "xfpb:d")]
    XfpbD(DecimalFeatureProperty),
    #[xml(tag = "xfpb:rel")]
    XfpbRel(RelFeatureProperty),
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:extLst")]
pub struct ExtensionList {
    #[xml(child = "x:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "x:ext")]
    XExt(crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension),
}
/// Defines the ArrayFeatureProperty Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:a.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:a")]
pub struct ArrayFeatureProperty {
    /// Name of the key for the key value pair.
    /// Represents the following attribute in the schema: :k
    #[xml(attr = "k")]
    pub k: String,
    #[xml(
        child = "xfpb:bagId",
        child = "xfpb:i",
        child = "xfpb:s",
        child = "xfpb:b",
        child = "xfpb:d",
        child = "xfpb:rel",
    )]
    pub children: Vec<ArrayFeaturePropertyChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ArrayFeaturePropertyChildChoice {
    #[xml(tag = "xfpb:bagId")]
    XfpbBagId(XsdunsignedInt),
    #[xml(tag = "xfpb:i")]
    XfpbI(Xsdinteger),
    #[xml(tag = "xfpb:s")]
    XfpbS(SXsdstring),
    #[xml(tag = "xfpb:b")]
    XfpbB(Xsdboolean),
    #[xml(tag = "xfpb:d")]
    XfpbD(Xsddouble),
    #[xml(tag = "xfpb:rel")]
    XfpbRel(RelXsdstring),
}
/// Defines the BagFeatureProperty Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:bagId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:bagId")]
pub struct BagFeatureProperty {
    /// k
    /// Represents the following attribute in the schema: :k
    #[xml(attr = "k")]
    pub k: String,
    #[xml(text)]
    pub child: i32,
}
/// Defines the IntFeatureProperty Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:i.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:i")]
pub struct IntFeatureProperty {
    /// k
    /// Represents the following attribute in the schema: :k
    #[xml(attr = "k")]
    pub k: String,
    #[xml(text)]
    pub child: i32,
}
/// Defines the StringFeatureProperty Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:s.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:s")]
pub struct StringFeatureProperty {
    /// k
    /// Represents the following attribute in the schema: :k
    #[xml(attr = "k")]
    pub k: String,
    #[xml(text)]
    pub child: String,
}
/// Defines the BoolFeatureProperty Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:b.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:b")]
pub struct BoolFeatureProperty {
    /// k
    /// Represents the following attribute in the schema: :k
    #[xml(attr = "k")]
    pub k: String,
    #[xml(text)]
    pub child: bool,
}
/// Defines the DecimalFeatureProperty Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:d.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:d")]
pub struct DecimalFeatureProperty {
    /// k
    /// Represents the following attribute in the schema: :k
    #[xml(attr = "k")]
    pub k: String,
    #[xml(text)]
    pub child: f64,
}
/// Defines the RelFeatureProperty Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:rel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:rel")]
pub struct RelFeatureProperty {
    /// Name of the key for the key value pair.
    /// Represents the following attribute in the schema: :k
    #[xml(attr = "k")]
    pub k: String,
    #[xml(text)]
    pub child: String,
}
/// Defines the DifferentialFormatType Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:dxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:dxf")]
pub struct DifferentialFormatType {
    ///Font Properties
    #[xml(child = "x:font")]
    pub font: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Font,
    >,
    ///Number Format
    #[xml(child = "x:numFmt")]
    pub numbering_format: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::NumberingFormat,
    >,
    ///Fill
    #[xml(child = "x:fill")]
    pub fill: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Fill,
    >,
    ///Alignment
    #[xml(child = "x:alignment")]
    pub alignment: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Alignment,
    >,
    ///Border Properties
    #[xml(child = "x:border")]
    pub border: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Border,
    >,
    ///Protection Properties
    #[xml(child = "x:protection")]
    pub protection: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Protection,
    >,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList,
    >,
}
/// Defines the XsdunsignedInt Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:bagId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:bagId")]
pub struct XsdunsignedInt {
    #[xml(text)]
    pub child: i32,
}
/// Defines the Xsdinteger Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:i.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:i")]
pub struct Xsdinteger {
    #[xml(text)]
    pub child: i32,
}
/// Defines the SXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:s.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:s")]
pub struct SXsdstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the RelXsdstring Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:rel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:rel")]
pub struct RelXsdstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the Xsdboolean Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:b.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:b")]
pub struct Xsdboolean {
    #[xml(text)]
    pub child: bool,
}
/// Defines the Xsddouble Class.
/// When the object is serialized out as xml, it's qualified name is xfpb:d.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xfpb:d")]
pub struct Xsddouble {
    #[xml(text)]
    pub child: f64,
}
