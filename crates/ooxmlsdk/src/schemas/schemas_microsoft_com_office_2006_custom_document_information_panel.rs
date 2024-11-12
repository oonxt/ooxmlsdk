/// Defines the CustomPropertyEditors Class.
/// When the object is serialized out as xml, it's qualified name is cdip:customPropertyEditors.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdip:customPropertyEditors")]
pub struct CustomPropertyEditors {
    /// _
    #[xml(child = "cdip:showOnOpen")]
    pub show_on_open: ShowOnOpen,
    /// _
    #[xml(child = "cdip:defaultPropertyEditorNamespace")]
    pub default_property_editor_namespace: DefaultPropertyEditorNamespace,
    /// _
    #[xml(child = "cdip:customPropertyEditor")]
    pub cdip_custom_property_editor: Vec<CustomPropertyEditor>,
}
/// Defines the PropertyEditorNamespace Class.
/// When the object is serialized out as xml, it's qualified name is cdip:XMLNamespace.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdip:XMLNamespace")]
pub struct PropertyEditorNamespace {
    #[xml(text)]
    pub child: String,
}
/// Defines the DefaultPropertyEditorNamespace Class.
/// When the object is serialized out as xml, it's qualified name is cdip:defaultPropertyEditorNamespace.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdip:defaultPropertyEditorNamespace")]
pub struct DefaultPropertyEditorNamespace {
    #[xml(text)]
    pub child: String,
}
/// Defines the XsnFileLocation Class.
/// When the object is serialized out as xml, it's qualified name is cdip:XSNLocation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdip:XSNLocation")]
pub struct XsnFileLocation {
    #[xml(text)]
    pub child: String,
}
/// Defines the ShowOnOpen Class.
/// When the object is serialized out as xml, it's qualified name is cdip:showOnOpen.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdip:showOnOpen")]
pub struct ShowOnOpen {
    #[xml(text)]
    pub child: bool,
}
/// Defines the CustomPropertyEditor Class.
/// When the object is serialized out as xml, it's qualified name is cdip:customPropertyEditor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cdip:customPropertyEditor")]
pub struct CustomPropertyEditor {
    /// _
    #[xml(child = "cdip:XMLNamespace")]
    pub property_editor_namespace: PropertyEditorNamespace,
    /// _
    #[xml(child = "cdip:XSNLocation")]
    pub xsn_file_location: XsnFileLocation,
}
