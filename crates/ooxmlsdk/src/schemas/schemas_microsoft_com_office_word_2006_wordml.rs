/// Defines the TemplateCommandGroup Class.
/// When the object is serialized out as xml, it's qualified name is wne:tcg.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:tcg")]
pub struct TemplateCommandGroup {
    #[xml(attr = "xmlns", with = "template_command_group_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "wne:keymaps")]
    pub wne_keymaps: Vec<KeyMapCustomizations>,
    /// _
    #[xml(child = "wne:keymapsBad")]
    pub wne_keymaps_bad: Vec<MismatchedKeyMapCustomization>,
    /// _
    #[xml(child = "wne:toolbars")]
    pub wne_toolbars: Option<Toolbars>,
    /// _
    #[xml(child = "wne:acds")]
    pub wne_acds: Vec<AllocatedCommands>,
}
mod template_command_group_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/word/2006/wordml")
    }
}
/// Defines the Mcds Class.
/// When the object is serialized out as xml, it's qualified name is wne:mcds.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:mcds")]
pub struct Mcds {
    /// _
    #[xml(child = "wne:mcd")]
    pub wne_mcd: Vec<Mcd>,
}
/// Defines the VbaSuppData Class.
/// When the object is serialized out as xml, it's qualified name is wne:vbaSuppData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:vbaSuppData")]
pub struct VbaSuppData {
    #[xml(attr = "xmlns", with = "vba_supp_data_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "wne:docEvents")]
    pub doc_events: Option<DocEvents>,
    /// _
    #[xml(child = "wne:mcds")]
    pub mcds: Option<Mcds>,
}
mod vba_supp_data_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/word/2006/wordml")
    }
}
/// Defines the MailMergeRecipients Class.
/// When the object is serialized out as xml, it's qualified name is wne:recipients.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:recipients")]
pub struct MailMergeRecipients {
    #[xml(attr = "xmlns", with = "mail_merge_recipients_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "wne:recipientData")]
    pub wne_recipient_data: Vec<SingleDataSourceRecord>,
}
mod mail_merge_recipients_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/word/2006/wordml")
    }
}
/// Defines the FixedCommandKeyboardCustomization Class.
/// When the object is serialized out as xml, it's qualified name is wne:fci.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:fci")]
pub struct FixedCommandKeyboardCustomization {
    /// fciName
    /// Represents the following attribute in the schema: wne:fciName
    #[xml(attr = "wne:fciName")]
    pub command_name: Option<String>,
    /// fciIndex
    /// Represents the following attribute in the schema: wne:fciIndex
    #[xml(attr = "wne:fciIndex")]
    pub command_index: Option<String>,
    /// swArg
    /// Represents the following attribute in the schema: wne:swArg
    #[xml(attr = "wne:swArg")]
    pub argument: Option<String>,
}
/// Defines the MacroKeyboardCustomization Class.
/// When the object is serialized out as xml, it's qualified name is wne:macro.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:macro")]
pub struct MacroKeyboardCustomization {
    /// macroName
    /// Represents the following attribute in the schema: wne:macroName
    #[xml(attr = "wne:macroName")]
    pub macro_name: Option<String>,
}
/// Defines the WllMacroKeyboardCustomization Class.
/// When the object is serialized out as xml, it's qualified name is wne:wll.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:wll")]
pub struct WllMacroKeyboardCustomization {
    /// macroName
    /// Represents the following attribute in the schema: wne:macroName
    #[xml(attr = "wne:macroName")]
    pub macro_name: Option<String>,
}
/// Defines the MacroWllType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct MacroWllType {
    /// macroName
    /// Represents the following attribute in the schema: wne:macroName
    #[xml(attr = "wne:macroName")]
    pub macro_name: Option<String>,
}
/// Defines the AllocatedCommandKeyboardCustomization Class.
/// When the object is serialized out as xml, it's qualified name is wne:acd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:acd")]
pub struct AllocatedCommandKeyboardCustomization {
    /// acdName
    /// Represents the following attribute in the schema: wne:acdName
    #[xml(attr = "wne:acdName")]
    pub accelerator_name: Option<String>,
}
/// Defines the AllocatedCommandManifestEntry Class.
/// When the object is serialized out as xml, it's qualified name is wne:acdEntry.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:acdEntry")]
pub struct AllocatedCommandManifestEntry {
    /// acdName
    /// Represents the following attribute in the schema: wne:acdName
    #[xml(attr = "wne:acdName")]
    pub accelerator_name: Option<String>,
}
/// Defines the AcceleratorKeymapType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct AcceleratorKeymapType {
    /// acdName
    /// Represents the following attribute in the schema: wne:acdName
    #[xml(attr = "wne:acdName")]
    pub accelerator_name: Option<String>,
}
/// Defines the CharacterInsertion Class.
/// When the object is serialized out as xml, it's qualified name is wne:wch.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:wch")]
pub struct CharacterInsertion {
    /// val
    /// Represents the following attribute in the schema: wne:val
    #[xml(attr = "wne:val")]
    pub val: String,
}
/// Defines the KeyMapEntry Class.
/// When the object is serialized out as xml, it's qualified name is wne:keymap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:keymap")]
pub struct KeyMapEntry {
    /// chmPrimary
    /// Represents the following attribute in the schema: wne:chmPrimary
    #[xml(attr = "wne:chmPrimary")]
    pub character_map_primary: Option<String>,
    /// chmSecondary
    /// Represents the following attribute in the schema: wne:chmSecondary
    #[xml(attr = "wne:chmSecondary")]
    pub character_map_secondary: Option<String>,
    /// kcmPrimary
    /// Represents the following attribute in the schema: wne:kcmPrimary
    #[xml(attr = "wne:kcmPrimary")]
    pub key_code_primary: Option<String>,
    /// kcmSecondary
    /// Represents the following attribute in the schema: wne:kcmSecondary
    #[xml(attr = "wne:kcmSecondary")]
    pub key_code_secondary: Option<String>,
    /// mask
    /// Represents the following attribute in the schema: wne:mask
    #[xml(attr = "wne:mask")]
    pub mask: Option<bool>,
    #[xml(
        child = "wne:fci",
        child = "wne:macro",
        child = "wne:acd",
        child = "wne:wll",
        child = "wne:wch",
    )]
    pub children: Vec<KeyMapEntryChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum KeyMapEntryChildChoice {
    #[xml(tag = "wne:fci")]
    WneFci(FixedCommandKeyboardCustomization),
    #[xml(tag = "wne:macro")]
    WneMacro(MacroKeyboardCustomization),
    #[xml(tag = "wne:acd")]
    WneAcd(AllocatedCommandKeyboardCustomization),
    #[xml(tag = "wne:wll")]
    WneWll(WllMacroKeyboardCustomization),
    #[xml(tag = "wne:wch")]
    WneWch(CharacterInsertion),
}
/// Defines the AllocatedCommand Class.
/// When the object is serialized out as xml, it's qualified name is wne:acd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:acd")]
pub struct AllocatedCommand {
    /// argValue
    /// Represents the following attribute in the schema: wne:argValue
    #[xml(attr = "wne:argValue")]
    pub argument_value: Option<String>,
    /// fciBasedOn
    /// Represents the following attribute in the schema: wne:fciBasedOn
    #[xml(attr = "wne:fciBasedOn")]
    pub command_based_on: Option<String>,
    /// fciIndexBasedOn
    /// Represents the following attribute in the schema: wne:fciIndexBasedOn
    #[xml(attr = "wne:fciIndexBasedOn")]
    pub command_index_based_on: Option<String>,
    /// acdName
    /// Represents the following attribute in the schema: wne:acdName
    #[xml(attr = "wne:acdName")]
    pub accelerator_name: Option<String>,
}
/// Defines the Mcd Class.
/// When the object is serialized out as xml, it's qualified name is wne:mcd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:mcd")]
pub struct Mcd {
    /// macroName
    /// Represents the following attribute in the schema: wne:macroName
    #[xml(attr = "wne:macroName")]
    pub wne_macro_name: Option<String>,
    /// name
    /// Represents the following attribute in the schema: wne:name
    #[xml(attr = "wne:name")]
    pub wne_name: Option<String>,
    /// menuHelp
    /// Represents the following attribute in the schema: wne:menuHelp
    #[xml(attr = "wne:menuHelp")]
    pub wne_menu_help: Option<String>,
    /// bEncrypt
    /// Represents the following attribute in the schema: wne:bEncrypt
    #[xml(attr = "wne:bEncrypt")]
    pub wne_b_encrypt: Option<String>,
    /// cmg
    /// Represents the following attribute in the schema: wne:cmg
    #[xml(attr = "wne:cmg")]
    pub wne_cmg: Option<String>,
}
/// Defines the EventDocNewXsdString Class.
/// When the object is serialized out as xml, it's qualified name is wne:eventDocNew.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:eventDocNew")]
pub struct EventDocNewXsdString {
    #[xml(text)]
    pub child: String,
}
/// Defines the EventDocOpenXsdString Class.
/// When the object is serialized out as xml, it's qualified name is wne:eventDocOpen.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:eventDocOpen")]
pub struct EventDocOpenXsdString {
    #[xml(text)]
    pub child: String,
}
/// Defines the EventDocCloseXsdString Class.
/// When the object is serialized out as xml, it's qualified name is wne:eventDocClose.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:eventDocClose")]
pub struct EventDocCloseXsdString {
    #[xml(text)]
    pub child: String,
}
/// Defines the EventDocSyncXsdString Class.
/// When the object is serialized out as xml, it's qualified name is wne:eventDocSync.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:eventDocSync")]
pub struct EventDocSyncXsdString {
    #[xml(text)]
    pub child: String,
}
/// Defines the EventDocXmlAfterInsertXsdString Class.
/// When the object is serialized out as xml, it's qualified name is wne:eventDocXmlAfterInsert.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:eventDocXmlAfterInsert")]
pub struct EventDocXmlAfterInsertXsdString {
    #[xml(text)]
    pub child: String,
}
/// Defines the EventDocXmlBeforeDeleteXsdString Class.
/// When the object is serialized out as xml, it's qualified name is wne:eventDocXmlBeforeDelete.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:eventDocXmlBeforeDelete")]
pub struct EventDocXmlBeforeDeleteXsdString {
    #[xml(text)]
    pub child: String,
}
/// Defines the EventDocContentControlAfterInsertXsdString Class.
/// When the object is serialized out as xml, it's qualified name is wne:eventDocContentControlAfterInsert.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:eventDocContentControlAfterInsert")]
pub struct EventDocContentControlAfterInsertXsdString {
    #[xml(text)]
    pub child: String,
}
/// Defines the EventDocContentControlBeforeDeleteXsdString Class.
/// When the object is serialized out as xml, it's qualified name is wne:eventDocContentControlBeforeDelete.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:eventDocContentControlBeforeDelete")]
pub struct EventDocContentControlBeforeDeleteXsdString {
    #[xml(text)]
    pub child: String,
}
/// Defines the EventDocContentControlOnExistXsdString Class.
/// When the object is serialized out as xml, it's qualified name is wne:eventDocContentControlOnExit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:eventDocContentControlOnExit")]
pub struct EventDocContentControlOnExistXsdString {
    #[xml(text)]
    pub child: String,
}
/// Defines the EventDocContentControlOnEnterXsdString Class.
/// When the object is serialized out as xml, it's qualified name is wne:eventDocContentControlOnEnter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:eventDocContentControlOnEnter")]
pub struct EventDocContentControlOnEnterXsdString {
    #[xml(text)]
    pub child: String,
}
/// Defines the EventDocStoreUpdateXsdString Class.
/// When the object is serialized out as xml, it's qualified name is wne:eventDocStoreUpdate.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:eventDocStoreUpdate")]
pub struct EventDocStoreUpdateXsdString {
    #[xml(text)]
    pub child: String,
}
/// Defines the EventDocContentControlUpdateXsdString Class.
/// When the object is serialized out as xml, it's qualified name is wne:eventDocContentControlContentUpdate.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:eventDocContentControlContentUpdate")]
pub struct EventDocContentControlUpdateXsdString {
    #[xml(text)]
    pub child: String,
}
/// Defines the EventDocBuildingBlockAfterInsertXsdString Class.
/// When the object is serialized out as xml, it's qualified name is wne:eventDocBuildingBlockAfterInsert.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:eventDocBuildingBlockAfterInsert")]
pub struct EventDocBuildingBlockAfterInsertXsdString {
    #[xml(text)]
    pub child: String,
}
/// Defines the DocEvents Class.
/// When the object is serialized out as xml, it's qualified name is wne:docEvents.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:docEvents")]
pub struct DocEvents {
    /// _
    #[xml(child = "wne:eventDocNew")]
    pub event_doc_new_xsd_string: Option<EventDocNewXsdString>,
    /// _
    #[xml(child = "wne:eventDocOpen")]
    pub event_doc_open_xsd_string: Option<EventDocOpenXsdString>,
    /// _
    #[xml(child = "wne:eventDocClose")]
    pub event_doc_close_xsd_string: Option<EventDocCloseXsdString>,
    /// _
    #[xml(child = "wne:eventDocSync")]
    pub event_doc_sync_xsd_string: Option<EventDocSyncXsdString>,
    /// _
    #[xml(child = "wne:eventDocXmlAfterInsert")]
    pub event_doc_xml_after_insert_xsd_string: Option<EventDocXmlAfterInsertXsdString>,
    /// _
    #[xml(child = "wne:eventDocXmlBeforeDelete")]
    pub event_doc_xml_before_delete_xsd_string: Option<EventDocXmlBeforeDeleteXsdString>,
    /// _
    #[xml(child = "wne:eventDocContentControlAfterInsert")]
    pub event_doc_content_control_after_insert_xsd_string: Option<
        EventDocContentControlAfterInsertXsdString,
    >,
    /// _
    #[xml(child = "wne:eventDocContentControlBeforeDelete")]
    pub event_doc_content_control_before_delete_xsd_string: Option<
        EventDocContentControlBeforeDeleteXsdString,
    >,
    /// _
    #[xml(child = "wne:eventDocContentControlOnExit")]
    pub event_doc_content_control_on_exist_xsd_string: Option<
        EventDocContentControlOnExistXsdString,
    >,
    /// _
    #[xml(child = "wne:eventDocContentControlOnEnter")]
    pub event_doc_content_control_on_enter_xsd_string: Option<
        EventDocContentControlOnEnterXsdString,
    >,
    /// _
    #[xml(child = "wne:eventDocStoreUpdate")]
    pub event_doc_store_update_xsd_string: Option<EventDocStoreUpdateXsdString>,
    /// _
    #[xml(child = "wne:eventDocContentControlContentUpdate")]
    pub event_doc_content_control_update_xsd_string: Option<
        EventDocContentControlUpdateXsdString,
    >,
    /// _
    #[xml(child = "wne:eventDocBuildingBlockAfterInsert")]
    pub event_doc_building_block_after_insert_xsd_string: Option<
        EventDocBuildingBlockAfterInsertXsdString,
    >,
}
/// Defines the AllocatedCommandManifest Class.
/// When the object is serialized out as xml, it's qualified name is wne:acdManifest.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:acdManifest")]
pub struct AllocatedCommandManifest {
    /// _
    #[xml(child = "wne:acdEntry")]
    pub wne_acd_entry: Vec<AllocatedCommandManifestEntry>,
}
/// Defines the ToolbarData Class.
/// When the object is serialized out as xml, it's qualified name is wne:toolbarData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:toolbarData")]
pub struct ToolbarData {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the KeyMapCustomizations Class.
/// When the object is serialized out as xml, it's qualified name is wne:keymaps.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:keymaps")]
pub struct KeyMapCustomizations {
    #[xml(child = "wne:keymap")]
    pub children: Vec<KeyMapCustomizationsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum KeyMapCustomizationsChildChoice {
    #[xml(tag = "wne:keymap")]
    WneKeymap(KeyMapEntry),
}
/// Defines the MismatchedKeyMapCustomization Class.
/// When the object is serialized out as xml, it's qualified name is wne:keymapsBad.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:keymapsBad")]
pub struct MismatchedKeyMapCustomization {
    #[xml(child = "wne:keymap")]
    pub children: Vec<MismatchedKeyMapCustomizationChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MismatchedKeyMapCustomizationChildChoice {
    #[xml(tag = "wne:keymap")]
    WneKeymap(KeyMapEntry),
}
/// Defines the KeymapsType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct KeymapsType {
    #[xml(child = "wne:keymap")]
    pub children: Vec<KeymapsTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum KeymapsTypeChildChoice {
    #[xml(tag = "wne:keymap")]
    WneKeymap(KeyMapEntry),
}
/// Defines the Toolbars Class.
/// When the object is serialized out as xml, it's qualified name is wne:toolbars.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:toolbars")]
pub struct Toolbars {
    /// _
    #[xml(child = "wne:acdManifest")]
    pub wne_acd_manifest: Vec<AllocatedCommandManifest>,
    /// _
    #[xml(child = "wne:toolbarData")]
    pub wne_toolbar_data: Vec<ToolbarData>,
}
/// Defines the AllocatedCommands Class.
/// When the object is serialized out as xml, it's qualified name is wne:acds.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:acds")]
pub struct AllocatedCommands {
    /// _
    #[xml(child = "wne:acd")]
    pub wne_acd: Vec<AllocatedCommand>,
}
/// Defines the RecordIncluded Class.
/// When the object is serialized out as xml, it's qualified name is wne:active.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:active")]
pub struct RecordIncluded {
    /// val
    /// Represents the following attribute in the schema: wne:val
    #[xml(attr = "wne:val")]
    pub val: Option<bool>,
}
/// Defines the RecordHashCode Class.
/// When the object is serialized out as xml, it's qualified name is wne:hash.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:hash")]
pub struct RecordHashCode {
    /// val
    /// Represents the following attribute in the schema: wne:val
    #[xml(attr = "wne:val")]
    pub val: i64,
}
/// Defines the SingleDataSourceRecord Class.
/// When the object is serialized out as xml, it's qualified name is wne:recipientData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wne:recipientData")]
pub struct SingleDataSourceRecord {
    /// _
    #[xml(child = "wne:active")]
    pub record_included: Option<RecordIncluded>,
    /// _
    #[xml(child = "wne:hash")]
    pub record_hash_code: RecordHashCode,
}
