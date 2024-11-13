/// Defines the CreationId Class.
/// When the object is serialized out as xml, it's qualified name is a16:creationId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a16:creationId")]
pub struct CreationId {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
}
/// Defines the PredecessorDrawingElementReference Class.
/// When the object is serialized out as xml, it's qualified name is a16:predDERef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a16:predDERef")]
pub struct PredecessorDrawingElementReference {
    /// pred
    /// Represents the following attribute in the schema: :pred
    #[xml(attr = "pred")]
    pub pred: Option<String>,
}
/// Defines the ConnectableReferences Class.
/// When the object is serialized out as xml, it's qualified name is a16:cxnDERefs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a16:cxnDERefs")]
pub struct ConnectableReferences {
    /// st
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub st: Option<String>,
    /// end
    /// Represents the following attribute in the schema: :end
    #[xml(attr = "end")]
    pub end: Option<String>,
}
/// Defines the RowIdIdentifier Class.
/// When the object is serialized out as xml, it's qualified name is a16:rowId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a16:rowId")]
pub struct RowIdIdentifier {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: u32,
}
/// Defines the ColIdIdentifier Class.
/// When the object is serialized out as xml, it's qualified name is a16:colId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a16:colId")]
pub struct ColIdIdentifier {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: u32,
}
/// Defines the OpenXmlIdentifierElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlIdentifierElement {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: u32,
}
