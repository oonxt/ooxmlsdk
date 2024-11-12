#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VectorBaseValues {
    #[default]
    Variant,
    OneByteSignedInteger,
    TwoBytesSignedInteger,
    FourBytesSignedInteger,
    EightBytesSignedInteger,
    OneByteUnsignedInteger,
    TwoBytesUnsignedInteger,
    FourBytesUnsignedInteger,
    EightBytesUnsignedInteger,
    FourBytesReal,
    EightBytesReal,
    Lpstr,
    Lpwstr,
    Bstr,
    Date,
    Filetime,
    Bool,
    Currency,
    Error,
    ClassId,
    ClipboardData,
}
crate::__string_enum! {
    VectorBaseValues { Variant = "variant", OneByteSignedInteger = "i1",
    TwoBytesSignedInteger = "i2", FourBytesSignedInteger = "i4", EightBytesSignedInteger
    = "i8", OneByteUnsignedInteger = "ui1", TwoBytesUnsignedInteger = "ui2",
    FourBytesUnsignedInteger = "ui4", EightBytesUnsignedInteger = "ui8", FourBytesReal =
    "r4", EightBytesReal = "r8", Lpstr = "lpstr", Lpwstr = "lpwstr", Bstr = "bstr", Date
    = "date", Filetime = "filetime", Bool = "bool", Currency = "cy", Error = "error",
    ClassId = "clsid", ClipboardData = "cf", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ArrayBaseValues {
    #[default]
    Variant,
    OneByteSignedInteger,
    TwoBytesSignedInteger,
    FourBytesSignedInteger,
    Integer,
    OneByteUnsignedInteger,
    TwoBytesUnsignedInteger,
    FourBytesUnsignedInteger,
    UnsignedInteger,
    FourBytesReal,
    EightBytesReal,
    Decimal,
    Bstr,
    Date,
    Bool,
    Currency,
    Error,
}
crate::__string_enum! {
    ArrayBaseValues { Variant = "variant", OneByteSignedInteger = "i1",
    TwoBytesSignedInteger = "i2", FourBytesSignedInteger = "i4", Integer = "int",
    OneByteUnsignedInteger = "ui1", TwoBytesUnsignedInteger = "ui2",
    FourBytesUnsignedInteger = "ui4", UnsignedInteger = "uint", FourBytesReal = "r4",
    EightBytesReal = "r8", Decimal = "decimal", Bstr = "bstr", Date = "date", Bool =
    "bool", Currency = "cy", Error = "error", }
}
/// Variant.
/// When the object is serialized out as xml, it's qualified name is vt:variant.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:variant")]
pub struct Variant {
    #[xml(
        child = "vt:variant",
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
    pub children: Vec<VariantChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum VariantChildChoice {
    #[xml(tag = "vt:variant")]
    VtVariant(Variant),
    #[xml(tag = "vt:vector")]
    VtVector(VtVector),
    #[xml(tag = "vt:array")]
    VtArray(VtArray),
    #[xml(tag = "vt:blob")]
    VtBlob(VtBlob),
    #[xml(tag = "vt:oblob")]
    VtOblob(VtoBlob),
    #[xml(tag = "vt:empty")]
    VtEmpty(VtEmpty),
    #[xml(tag = "vt:null")]
    VtNull(VtNull),
    #[xml(tag = "vt:i1")]
    VtI1(VtByte),
    #[xml(tag = "vt:i2")]
    VtI2(VtShort),
    #[xml(tag = "vt:i4")]
    VtI4(VtInt32),
    #[xml(tag = "vt:i8")]
    VtI8(VtInt64),
    #[xml(tag = "vt:int")]
    VtInt(VtInteger),
    #[xml(tag = "vt:ui1")]
    VtUi1(VtUnsignedByte),
    #[xml(tag = "vt:ui2")]
    VtUi2(VtUnsignedShort),
    #[xml(tag = "vt:ui4")]
    VtUi4(VtUnsignedInt32),
    #[xml(tag = "vt:ui8")]
    VtUi8(VtUnsignedInt64),
    #[xml(tag = "vt:uint")]
    VtUint(VtUnsignedInteger),
    #[xml(tag = "vt:r4")]
    VtR4(VtFloat),
    #[xml(tag = "vt:r8")]
    VtR8(VtDouble),
    #[xml(tag = "vt:decimal")]
    VtDecimal(VtDecimal),
    #[xml(tag = "vt:lpstr")]
    VtLpstr(Vtlpstr),
    #[xml(tag = "vt:lpwstr")]
    VtLpwstr(Vtlpwstr),
    #[xml(tag = "vt:bstr")]
    VtBstr(VtbString),
    #[xml(tag = "vt:date")]
    VtDate(VtDate),
    #[xml(tag = "vt:filetime")]
    VtFiletime(VtFileTime),
    #[xml(tag = "vt:bool")]
    VtBool(VtBool),
    #[xml(tag = "vt:cy")]
    VtCy(VtCurrency),
    #[xml(tag = "vt:error")]
    VtError(VtError),
    #[xml(tag = "vt:stream")]
    VtStream(VtStreamData),
    #[xml(tag = "vt:ostream")]
    VtOstream(VtoStreamData),
    #[xml(tag = "vt:storage")]
    VtStorage(VtStorage),
    #[xml(tag = "vt:ostorage")]
    VtOstorage(VtoStorage),
    #[xml(tag = "vt:vstream")]
    VtVstream(VtvStreamData),
    #[xml(tag = "vt:clsid")]
    VtClsid(VtClassId),
    #[xml(tag = "vt:cf")]
    VtCf(VtClipboardData),
}
/// Vector.
/// When the object is serialized out as xml, it's qualified name is vt:vector.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:vector")]
pub struct VtVector {
    /// Vector Base Type
    /// Represents the following attribute in the schema: :baseType
    #[xml(attr = "baseType")]
    pub base_type: VectorBaseValues,
    /// Vector Size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: i32,
    #[xml(
        child = "vt:variant",
        child = "vt:i1",
        child = "vt:i2",
        child = "vt:i4",
        child = "vt:i8",
        child = "vt:ui1",
        child = "vt:ui2",
        child = "vt:ui4",
        child = "vt:ui8",
        child = "vt:r4",
        child = "vt:r8",
        child = "vt:lpstr",
        child = "vt:lpwstr",
        child = "vt:bstr",
        child = "vt:date",
        child = "vt:filetime",
        child = "vt:bool",
        child = "vt:cy",
        child = "vt:error",
        child = "vt:clsid",
        child = "vt:cf",
    )]
    pub children: Vec<VtVectorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum VtVectorChildChoice {
    #[xml(tag = "vt:variant")]
    VtVariant(Variant),
    #[xml(tag = "vt:i1")]
    VtI1(VtByte),
    #[xml(tag = "vt:i2")]
    VtI2(VtShort),
    #[xml(tag = "vt:i4")]
    VtI4(VtInt32),
    #[xml(tag = "vt:i8")]
    VtI8(VtInt64),
    #[xml(tag = "vt:ui1")]
    VtUi1(VtUnsignedByte),
    #[xml(tag = "vt:ui2")]
    VtUi2(VtUnsignedShort),
    #[xml(tag = "vt:ui4")]
    VtUi4(VtUnsignedInt32),
    #[xml(tag = "vt:ui8")]
    VtUi8(VtUnsignedInt64),
    #[xml(tag = "vt:r4")]
    VtR4(VtFloat),
    #[xml(tag = "vt:r8")]
    VtR8(VtDouble),
    #[xml(tag = "vt:lpstr")]
    VtLpstr(Vtlpstr),
    #[xml(tag = "vt:lpwstr")]
    VtLpwstr(Vtlpwstr),
    #[xml(tag = "vt:bstr")]
    VtBstr(VtbString),
    #[xml(tag = "vt:date")]
    VtDate(VtDate),
    #[xml(tag = "vt:filetime")]
    VtFiletime(VtFileTime),
    #[xml(tag = "vt:bool")]
    VtBool(VtBool),
    #[xml(tag = "vt:cy")]
    VtCy(VtCurrency),
    #[xml(tag = "vt:error")]
    VtError(VtError),
    #[xml(tag = "vt:clsid")]
    VtClsid(VtClassId),
    #[xml(tag = "vt:cf")]
    VtCf(VtClipboardData),
}
/// Array.
/// When the object is serialized out as xml, it's qualified name is vt:array.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:array")]
pub struct VtArray {
    /// Array Lower Bounds Attribute
    /// Represents the following attribute in the schema: :lBound
    #[xml(attr = "lBound")]
    pub lower_bounds: i32,
    /// Array Upper Bounds Attribute
    /// Represents the following attribute in the schema: :uBound
    #[xml(attr = "uBound")]
    pub upper_bounds: i32,
    /// Array Base Type
    /// Represents the following attribute in the schema: :baseType
    #[xml(attr = "baseType")]
    pub base_type: ArrayBaseValues,
    #[xml(
        child = "vt:variant",
        child = "vt:i1",
        child = "vt:i2",
        child = "vt:i4",
        child = "vt:int",
        child = "vt:ui1",
        child = "vt:ui2",
        child = "vt:ui4",
        child = "vt:uint",
        child = "vt:r4",
        child = "vt:r8",
        child = "vt:decimal",
        child = "vt:bstr",
        child = "vt:date",
        child = "vt:bool",
        child = "vt:error",
        child = "vt:cy",
    )]
    pub children: Vec<VtArrayChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum VtArrayChildChoice {
    #[xml(tag = "vt:variant")]
    VtVariant(Variant),
    #[xml(tag = "vt:i1")]
    VtI1(VtByte),
    #[xml(tag = "vt:i2")]
    VtI2(VtShort),
    #[xml(tag = "vt:i4")]
    VtI4(VtInt32),
    #[xml(tag = "vt:int")]
    VtInt(VtInteger),
    #[xml(tag = "vt:ui1")]
    VtUi1(VtUnsignedByte),
    #[xml(tag = "vt:ui2")]
    VtUi2(VtUnsignedShort),
    #[xml(tag = "vt:ui4")]
    VtUi4(VtUnsignedInt32),
    #[xml(tag = "vt:uint")]
    VtUint(VtUnsignedInteger),
    #[xml(tag = "vt:r4")]
    VtR4(VtFloat),
    #[xml(tag = "vt:r8")]
    VtR8(VtDouble),
    #[xml(tag = "vt:decimal")]
    VtDecimal(VtDecimal),
    #[xml(tag = "vt:bstr")]
    VtBstr(VtbString),
    #[xml(tag = "vt:date")]
    VtDate(VtDate),
    #[xml(tag = "vt:bool")]
    VtBool(VtBool),
    #[xml(tag = "vt:error")]
    VtError(VtError),
    #[xml(tag = "vt:cy")]
    VtCy(VtCurrency),
}
/// Binary Blob.
/// When the object is serialized out as xml, it's qualified name is vt:blob.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:blob")]
pub struct VtBlob {
    #[xml(text)]
    pub child: String,
}
/// Binary Blob Object.
/// When the object is serialized out as xml, it's qualified name is vt:oblob.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:oblob")]
pub struct VtoBlob {
    #[xml(text)]
    pub child: String,
}
/// Binary Stream.
/// When the object is serialized out as xml, it's qualified name is vt:stream.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:stream")]
pub struct VtStreamData {
    #[xml(text)]
    pub child: String,
}
/// Binary Stream Object.
/// When the object is serialized out as xml, it's qualified name is vt:ostream.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:ostream")]
pub struct VtoStreamData {
    #[xml(text)]
    pub child: String,
}
/// Binary Storage.
/// When the object is serialized out as xml, it's qualified name is vt:storage.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:storage")]
pub struct VtStorage {
    #[xml(text)]
    pub child: String,
}
/// Binary Storage Object.
/// When the object is serialized out as xml, it's qualified name is vt:ostorage.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:ostorage")]
pub struct VtoStorage {
    #[xml(text)]
    pub child: String,
}
/// Empty.
/// When the object is serialized out as xml, it's qualified name is vt:empty.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:empty")]
pub struct VtEmpty {}
/// Null.
/// When the object is serialized out as xml, it's qualified name is vt:null.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:null")]
pub struct VtNull {}
/// 1-Byte Signed Integer.
/// When the object is serialized out as xml, it's qualified name is vt:i1.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:i1")]
pub struct VtByte {
    #[xml(text)]
    pub child: u8,
}
/// 2-Byte Signed Integer.
/// When the object is serialized out as xml, it's qualified name is vt:i2.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:i2")]
pub struct VtShort {
    #[xml(text)]
    pub child: i16,
}
/// 4-Byte Signed Integer.
/// When the object is serialized out as xml, it's qualified name is vt:i4.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:i4")]
pub struct VtInt32 {
    #[xml(text)]
    pub child: i32,
}
/// Integer.
/// When the object is serialized out as xml, it's qualified name is vt:int.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:int")]
pub struct VtInteger {
    #[xml(text)]
    pub child: i32,
}
/// 8-Byte Signed Integer.
/// When the object is serialized out as xml, it's qualified name is vt:i8.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:i8")]
pub struct VtInt64 {
    #[xml(text)]
    pub child: i64,
}
/// 1-Byte Unsigned Integer.
/// When the object is serialized out as xml, it's qualified name is vt:ui1.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:ui1")]
pub struct VtUnsignedByte {
    #[xml(text)]
    pub child: u8,
}
/// 2-Byte Unsigned Integer.
/// When the object is serialized out as xml, it's qualified name is vt:ui2.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:ui2")]
pub struct VtUnsignedShort {
    #[xml(text)]
    pub child: i16,
}
/// 4-Byte Unsigned Integer.
/// When the object is serialized out as xml, it's qualified name is vt:ui4.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:ui4")]
pub struct VtUnsignedInt32 {
    #[xml(text)]
    pub child: i32,
}
/// Unsigned Integer.
/// When the object is serialized out as xml, it's qualified name is vt:uint.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:uint")]
pub struct VtUnsignedInteger {
    #[xml(text)]
    pub child: i32,
}
/// 8-Byte Unsigned Integer.
/// When the object is serialized out as xml, it's qualified name is vt:ui8.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:ui8")]
pub struct VtUnsignedInt64 {
    #[xml(text)]
    pub child: i64,
}
/// 4-Byte Real Number.
/// When the object is serialized out as xml, it's qualified name is vt:r4.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:r4")]
pub struct VtFloat {
    #[xml(text)]
    pub child: f32,
}
/// 8-Byte Real Number.
/// When the object is serialized out as xml, it's qualified name is vt:r8.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:r8")]
pub struct VtDouble {
    #[xml(text)]
    pub child: f64,
}
/// Decimal.
/// When the object is serialized out as xml, it's qualified name is vt:decimal.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:decimal")]
pub struct VtDecimal {
    #[xml(text)]
    pub child: String,
}
/// LPSTR.
/// When the object is serialized out as xml, it's qualified name is vt:lpstr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:lpstr")]
pub struct Vtlpstr {
    #[xml(text)]
    pub child: String,
}
/// LPWSTR.
/// When the object is serialized out as xml, it's qualified name is vt:lpwstr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:lpwstr")]
pub struct Vtlpwstr {
    #[xml(text)]
    pub child: String,
}
/// Basic String.
/// When the object is serialized out as xml, it's qualified name is vt:bstr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:bstr")]
pub struct VtbString {
    #[xml(text)]
    pub child: String,
}
/// Date and Time.
/// When the object is serialized out as xml, it's qualified name is vt:date.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:date")]
pub struct VtDate {
    #[xml(text)]
    pub child: String,
}
/// File Time.
/// When the object is serialized out as xml, it's qualified name is vt:filetime.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:filetime")]
pub struct VtFileTime {
    #[xml(text)]
    pub child: String,
}
/// Boolean.
/// When the object is serialized out as xml, it's qualified name is vt:bool.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:bool")]
pub struct VtBool {
    #[xml(text)]
    pub child: bool,
}
/// Currency.
/// When the object is serialized out as xml, it's qualified name is vt:cy.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:cy")]
pub struct VtCurrency {
    #[xml(text)]
    pub child: String,
}
/// Error Status Code.
/// When the object is serialized out as xml, it's qualified name is vt:error.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:error")]
pub struct VtError {
    #[xml(text)]
    pub child: String,
}
/// Binary Versioned Stream.
/// When the object is serialized out as xml, it's qualified name is vt:vstream.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:vstream")]
pub struct VtvStreamData {
    /// VSTREAM Version Attribute
    /// Represents the following attribute in the schema: :version
    #[xml(attr = "version")]
    pub version: String,
    #[xml(text)]
    pub child: String,
}
/// Class ID.
/// When the object is serialized out as xml, it's qualified name is vt:clsid.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:clsid")]
pub struct VtClassId {
    #[xml(text)]
    pub child: String,
}
/// Clipboard Data.
/// When the object is serialized out as xml, it's qualified name is vt:cf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "vt:cf")]
pub struct VtClipboardData {
    /// Format Attribute
    /// Represents the following attribute in the schema: :format
    #[xml(attr = "format")]
    pub format: Option<i32>,
    /// size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: i32,
    #[xml(text)]
    pub child: String,
}
