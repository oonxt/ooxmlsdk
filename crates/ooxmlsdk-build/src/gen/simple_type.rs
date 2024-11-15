use proc_macro2::TokenStream;
use quote::quote;

pub fn simple_type_mapping(name: &str) -> &str {
    match name {
        "a:ST_Guid" => "StringValue",
        "xsd:string" => "StringValue",
        "c:ST_Xstring" => "StringValue",
        "cdr:ST_MarkerCoordinate" => "DoubleValue",
        "wp:ST_PositionOffset" => "Int32Value",
        "xdr:ST_ColID" => "Int32Value",
        "a:ST_Coordinate" => "Int64Value",
        "a:ST_Percentage" => "Int32Value",
        "a:ST_PositivePercentage" => "Int32Value",
        "ask:ST_LineSketchSeed" => "UInt32Value",
        "b:ST_String255" => "StringValue",
        "cppr:ST_PublishDate" => "StringValue",
        "cx:CT_Formula" => "StringValue",
        "cx:CT_NumericValue" => "DoubleValue",
        "cx:CT_StringValue" => "StringValue",
        "cx:ST_AxisId" => "UInt32Value",
        "emma:CT_Literal" => "StringValue",
        "inkml:CT_Annotation" => "StringValue",
        "inkml:CT_Matrix" => "StringValue",
        "inkml:CT_Table" => "StringValue",
        "inkml:CT_Trace" => "StringValue",
        "lp:CT_LongProp" => "StringValue",
        "m:CT_Text" => "StringValue",
        "msink:CT_Property" => "HexBinaryValue",
        "o:ST_TrueFalseBlank" => "TrueFalseBlankValue",
        "oac:CT_ImgData" => "Base64BinaryValue",
        "vt:CT_Cf" => "Base64BinaryValue",
        "vt:CT_Vstream" => "Base64BinaryValue",
        "vt:ST_Clsid" => "StringValue",
        "vt:ST_Cy" => "StringValue",
        "vt:ST_Error" => "StringValue",
        "w:CT_Base64BinaryText" => "Base64BinaryValue",
        "w:CT_Text" => "StringValue",
        "x:CT_CellFormula" => "StringValue",
        "x:CT_DefinedName" => "StringValue",
        "x:CT_TableFormula" => "StringValue",
        "x:CT_Xstring" => "StringValue",
        "x:ST_Formula" => "StringValue",
        "x:ST_Guid" => "StringValue",
        "x:ST_Xstring" => "StringValue",
        "x14:CT_DefinedNameArgumentDescription" => "StringValue",
        "x14:CT_PivotEditValue" => "StringValue",
        "xdr:ST_RowID" => "Int32Value",
        "xlrd:CT_RichValueFallback" => "StringValue",
        "xlrd:CT_Value" => "StringValue",
        "xlrd2:CT_ArrayValue" => "StringValue",
        "xlrd2:CT_RichStylePropertyValue" => "StringValue",
        "xlrd2:CT_SupportingPropertyBagArrayValue" => "StringValue",
        "xlrd2:CT_SupportingPropertyBagValue" => "StringValue",
        "xne:ST_Sqref" => "StringValue",
        "xsd:anyURI" => "StringValue",
        "xsd:base64Binary" => "Base64BinaryValue",
        "xsd:boolean" => "BooleanValue",
        "xsd:byte" => "SByteValue",
        "xsd:dateTime" => "DateTimeValue",
        "xsd:decimal" => "DecimalValue",
        "xsd:double" => "DoubleValue",
        "xsd:float" => "SingleValue",
        "xsd:int" => "Int32Value",
        "xsd:integer" => "IntegerValue",
        "xsd:long" => "Int64Value",
        "xsd:nonNegativeInteger" => "IntegerValue",
        "xsd:short" => "Int16Value",
        "xsd:unsignedByte" => "ByteValue",
        "xsd:unsignedInt" => "UInt32Value",
        "xsd:unsignedLong" => "UInt64Value",
        "xsd:unsignedShort" => "UInt16Value",
        "xvml:ST_Macro" => "StringValue",
        "a:ST_Angle" => "Int32Value",
        "a:ST_DrawingElementId" => "UInt32Value",
        "a:ST_PositiveFixedPercentage" => "Int32Value",
        "msink:ST_Point" => "StringValue",
        "w:ST_DecimalNumber" => "Int32Value",
        "w:ST_HexColorRGB" => "HexBinaryValue",
        "w:ST_HpsMeasure_O12" => "UInt32Value",
        "w:ST_NonNegativeDecimalNumber" => "Int32Value",
        "w:ST_SignedDecimalNumberMax-1" => "Int32Value",
        "w:ST_SignedDecimalNumberMax-2" => "Int32Value",
        "w:ST_SignedHpsMeasure_O12" => "Int32Value",
        "w:ST_SignedTwipsMeasure_O12" => "Int32Value",
        "w:ST_StylePaneSortMethods_O12" => "HexBinaryValue",
        "w:ST_TwipsMeasure_O12" => "UInt32Value",
        "w:ST_UnsignedDecimalNumber" => "UInt32Value",
        "w:ST_UnsignedDecimalNumberMin1" => "Int32Value",
        "xne:ST_Ref" => "StringValue",
        "xfpb:CT_BagFeatureProperty" => "UInt32Value",
        "xfpb:CT_IntFeatureProperty" => "Int32Value",
        "xfpb:CT_StringFeatureProperty" => "StringValue",
        "xfpb:CT_BoolFeatureProperty" => "BooleanValue",
        "xfpb:CT_DecimalFeatureProperty" => "DoubleValue",
        "xfpb:CT_RelFeatureProperty" => "StringValue",
        _ => name,
    }
}

pub fn get_type_name(name: &str) -> String {
    if name.ends_with("Base64BinaryValue") {
        "String".to_string()
    } else if name.ends_with("BooleanValue") {
        "bool".to_string()
    } else if name.ends_with("ByteValue") {
        "u8".to_string()
    } else if name.ends_with("DateTimeValue") {
        "String".to_string()
    } else if name.ends_with("DecimalValue") {
        "String".to_string()
    } else if name.ends_with("DoubleValue") {
        "f64".to_string()
    } else if name.ends_with("HexBinaryValue") {
        "String".to_string()
    } else if name.ends_with("UInt16Value") {
        "u16".to_string()
    } else if name.ends_with("UInt32Value") {
        "u32".to_string()
    } else if name.ends_with("UInt64Value") {
        "u64".to_string()
    } else if name.ends_with("Int16Value") {
        "i16".to_string()
    } else if name.ends_with("Int32Value") {
        "i32".to_string()
    } else if name.ends_with("Int64Value") {
        "i32".to_string()
    } else if name.ends_with("IntegerValue") {
        "i64".to_string()
    } else if name.ends_with("OnOffValue") {
        "bool".to_string()
    } else if name.ends_with("SByteValue") {
        "String".to_string()
    } else if name.ends_with("SingleValue") {
        "f32".to_string()
    } else if name.ends_with("StringValue") {
        "String".to_string()
    } else if name.ends_with("TrueFalseBlankValue") {
        "bool".to_string()
    } else if name.ends_with("TrueFalseValue") {
        "bool".to_string()
    } else {
        "".to_string()
    }
}
