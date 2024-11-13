#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SizeValues {
    #[default]
    Normal,
    Large,
}
crate::__string_enum! {
    SizeValues { Normal = "normal", Large = "large", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ItemSizeValues {
    #[default]
    Normal,
    Large,
}
crate::__string_enum! {
    ItemSizeValues { Normal = "normal", Large = "large", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BoxStyleValues {
    #[default]
    Horizontal,
    Vertical,
}
crate::__string_enum! {
    BoxStyleValues { Horizontal = "horizontal", Vertical = "vertical", }
}
/// Defines the UnsizedControlClone Class.
/// When the object is serialized out as xml, it's qualified name is mso:control.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:control")]
pub struct UnsizedControlClone {
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
}
/// Defines the UnsizedButton Class.
/// When the object is serialized out as xml, it's qualified name is mso:button.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:button")]
pub struct UnsizedButton {
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
}
/// Defines the CheckBox Class.
/// When the object is serialized out as xml, it's qualified name is mso:checkBox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:checkBox")]
pub struct CheckBox {
    /// getPressed
    /// Represents the following attribute in the schema: :getPressed
    #[xml(attr = "getPressed")]
    pub get_pressed: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
}
/// Defines the UnsizedGallery Class.
/// When the object is serialized out as xml, it's qualified name is mso:gallery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:gallery")]
pub struct UnsizedGallery {
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// invalidateContentOnDrop
    /// Represents the following attribute in the schema: :invalidateContentOnDrop
    #[xml(attr = "invalidateContentOnDrop")]
    pub invalidate_content_on_drop: Option<bool>,
    /// columns
    /// Represents the following attribute in the schema: :columns
    #[xml(attr = "columns")]
    pub columns: Option<i32>,
    /// rows
    /// Represents the following attribute in the schema: :rows
    #[xml(attr = "rows")]
    pub rows: Option<i32>,
    /// itemWidth
    /// Represents the following attribute in the schema: :itemWidth
    #[xml(attr = "itemWidth")]
    pub item_width: Option<i32>,
    /// itemHeight
    /// Represents the following attribute in the schema: :itemHeight
    #[xml(attr = "itemHeight")]
    pub item_height: Option<i32>,
    /// getItemWidth
    /// Represents the following attribute in the schema: :getItemWidth
    #[xml(attr = "getItemWidth")]
    pub get_item_width: Option<String>,
    /// getItemHeight
    /// Represents the following attribute in the schema: :getItemHeight
    #[xml(attr = "getItemHeight")]
    pub get_item_height: Option<String>,
    /// showItemLabel
    /// Represents the following attribute in the schema: :showItemLabel
    #[xml(attr = "showItemLabel")]
    pub show_item_label: Option<bool>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// showItemImage
    /// Represents the following attribute in the schema: :showItemImage
    #[xml(attr = "showItemImage")]
    pub show_item_image: Option<bool>,
    /// getItemCount
    /// Represents the following attribute in the schema: :getItemCount
    #[xml(attr = "getItemCount")]
    pub get_item_count: Option<String>,
    /// getItemLabel
    /// Represents the following attribute in the schema: :getItemLabel
    #[xml(attr = "getItemLabel")]
    pub get_item_label: Option<String>,
    /// getItemScreentip
    /// Represents the following attribute in the schema: :getItemScreentip
    #[xml(attr = "getItemScreentip")]
    pub get_item_screentip: Option<String>,
    /// getItemSupertip
    /// Represents the following attribute in the schema: :getItemSupertip
    #[xml(attr = "getItemSupertip")]
    pub get_item_supertip: Option<String>,
    /// getItemImage
    /// Represents the following attribute in the schema: :getItemImage
    #[xml(attr = "getItemImage")]
    pub get_item_image: Option<String>,
    /// getItemID
    /// Represents the following attribute in the schema: :getItemID
    #[xml(attr = "getItemID")]
    pub get_item_id: Option<String>,
    /// sizeString
    /// Represents the following attribute in the schema: :sizeString
    #[xml(attr = "sizeString")]
    pub size_string: Option<String>,
    /// getSelectedItemID
    /// Represents the following attribute in the schema: :getSelectedItemID
    #[xml(attr = "getSelectedItemID")]
    pub get_selected_item_id: Option<String>,
    /// getSelectedItemIndex
    /// Represents the following attribute in the schema: :getSelectedItemIndex
    #[xml(attr = "getSelectedItemIndex")]
    pub get_selected_item_index: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
    #[xml(child = "mso:item", child = "mso:button")]
    pub children: Vec<UnsizedGalleryChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum UnsizedGalleryChildChoice {
    #[xml(tag = "mso:item")]
    MsoItem(Item),
    #[xml(tag = "mso:button")]
    MsoButton(UnsizedButton),
}
/// Defines the UnsizedToggleButton Class.
/// When the object is serialized out as xml, it's qualified name is mso:toggleButton.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:toggleButton")]
pub struct UnsizedToggleButton {
    /// getPressed
    /// Represents the following attribute in the schema: :getPressed
    #[xml(attr = "getPressed")]
    pub get_pressed: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
}
/// Defines the MenuSeparator Class.
/// When the object is serialized out as xml, it's qualified name is mso:menuSeparator.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:menuSeparator")]
pub struct MenuSeparator {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// getTitle
    /// Represents the following attribute in the schema: :getTitle
    #[xml(attr = "getTitle")]
    pub get_title: Option<String>,
}
/// Defines the UnsizedSplitButton Class.
/// When the object is serialized out as xml, it's qualified name is mso:splitButton.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:splitButton")]
pub struct UnsizedSplitButton {
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    #[xml(child = "mso:button", child = "mso:toggleButton", child = "mso:menu")]
    pub children: Vec<UnsizedSplitButtonChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum UnsizedSplitButtonChildChoice {
    #[xml(tag = "mso:button")]
    MsoButton(VisibleButton),
    #[xml(tag = "mso:toggleButton")]
    MsoToggleButton(VisibleToggleButton),
    #[xml(tag = "mso:menu")]
    MsoMenu(UnsizedMenu),
}
/// Defines the UnsizedMenu Class.
/// When the object is serialized out as xml, it's qualified name is mso:menu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:menu")]
pub struct UnsizedMenu {
    /// itemSize
    /// Represents the following attribute in the schema: :itemSize
    #[xml(attr = "itemSize")]
    pub item_size: Option<ItemSizeValues>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
    #[xml(
        child = "mso:control",
        child = "mso:button",
        child = "mso:checkBox",
        child = "mso:gallery",
        child = "mso:toggleButton",
        child = "mso:menuSeparator",
        child = "mso:splitButton",
        child = "mso:menu",
        child = "mso:dynamicMenu",
    )]
    pub children: Vec<UnsizedMenuChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum UnsizedMenuChildChoice {
    #[xml(tag = "mso:control")]
    MsoControl(UnsizedControlClone),
    #[xml(tag = "mso:button")]
    MsoButton(UnsizedButton),
    #[xml(tag = "mso:checkBox")]
    MsoCheckBox(CheckBox),
    #[xml(tag = "mso:gallery")]
    MsoGallery(UnsizedGallery),
    #[xml(tag = "mso:toggleButton")]
    MsoToggleButton(UnsizedToggleButton),
    #[xml(tag = "mso:menuSeparator")]
    MsoMenuSeparator(MenuSeparator),
    #[xml(tag = "mso:splitButton")]
    MsoSplitButton(UnsizedSplitButton),
    #[xml(tag = "mso:menu")]
    MsoMenu(UnsizedMenu),
    #[xml(tag = "mso:dynamicMenu")]
    MsoDynamicMenu(UnsizedDynamicMenu),
}
/// Defines the UnsizedDynamicMenu Class.
/// When the object is serialized out as xml, it's qualified name is mso:dynamicMenu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:dynamicMenu")]
pub struct UnsizedDynamicMenu {
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// getContent
    /// Represents the following attribute in the schema: :getContent
    #[xml(attr = "getContent")]
    pub get_content: String,
    /// invalidateContentOnDrop
    /// Represents the following attribute in the schema: :invalidateContentOnDrop
    #[xml(attr = "invalidateContentOnDrop")]
    pub invalidate_content_on_drop: Option<bool>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
}
/// Defines the SplitButtonWithTitle Class.
/// When the object is serialized out as xml, it's qualified name is mso:splitButton.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:splitButton")]
pub struct SplitButtonWithTitle {
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    #[xml(child = "mso:button", child = "mso:toggleButton", child = "mso:menu")]
    pub children: Vec<SplitButtonWithTitleChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SplitButtonWithTitleChildChoice {
    #[xml(tag = "mso:button")]
    MsoButton(VisibleButton),
    #[xml(tag = "mso:toggleButton")]
    MsoToggleButton(VisibleToggleButton),
    #[xml(tag = "mso:menu")]
    MsoMenu(MenuWithTitle),
}
/// Defines the MenuWithTitle Class.
/// When the object is serialized out as xml, it's qualified name is mso:menu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:menu")]
pub struct MenuWithTitle {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// itemSize
    /// Represents the following attribute in the schema: :itemSize
    #[xml(attr = "itemSize")]
    pub item_size: Option<ItemSizeValues>,
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// getTitle
    /// Represents the following attribute in the schema: :getTitle
    #[xml(attr = "getTitle")]
    pub get_title: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
    #[xml(
        child = "mso:control",
        child = "mso:button",
        child = "mso:checkBox",
        child = "mso:gallery",
        child = "mso:toggleButton",
        child = "mso:menuSeparator",
        child = "mso:splitButton",
        child = "mso:menu",
        child = "mso:dynamicMenu",
    )]
    pub children: Vec<MenuWithTitleChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MenuWithTitleChildChoice {
    #[xml(tag = "mso:control")]
    MsoControl(UnsizedControlClone),
    #[xml(tag = "mso:button")]
    MsoButton(UnsizedButton),
    #[xml(tag = "mso:checkBox")]
    MsoCheckBox(CheckBox),
    #[xml(tag = "mso:gallery")]
    MsoGallery(UnsizedGallery),
    #[xml(tag = "mso:toggleButton")]
    MsoToggleButton(UnsizedToggleButton),
    #[xml(tag = "mso:menuSeparator")]
    MsoMenuSeparator(MenuSeparator),
    #[xml(tag = "mso:splitButton")]
    MsoSplitButton(SplitButtonWithTitle),
    #[xml(tag = "mso:menu")]
    MsoMenu(MenuWithTitle),
    #[xml(tag = "mso:dynamicMenu")]
    MsoDynamicMenu(UnsizedDynamicMenu),
}
/// Defines the ControlClone Class.
/// When the object is serialized out as xml, it's qualified name is mso:control.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:control")]
pub struct ControlClone {
    /// size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: Option<SizeValues>,
    /// getSize
    /// Represents the following attribute in the schema: :getSize
    #[xml(attr = "getSize")]
    pub get_size: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
}
/// Defines the TextLabel Class.
/// When the object is serialized out as xml, it's qualified name is mso:labelControl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:labelControl")]
pub struct TextLabel {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
}
/// Defines the Button Class.
/// When the object is serialized out as xml, it's qualified name is mso:button.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:button")]
pub struct Button {
    /// size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: Option<SizeValues>,
    /// getSize
    /// Represents the following attribute in the schema: :getSize
    #[xml(attr = "getSize")]
    pub get_size: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
}
/// Defines the ToggleButton Class.
/// When the object is serialized out as xml, it's qualified name is mso:toggleButton.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:toggleButton")]
pub struct ToggleButton {
    /// size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: Option<SizeValues>,
    /// getSize
    /// Represents the following attribute in the schema: :getSize
    #[xml(attr = "getSize")]
    pub get_size: Option<String>,
    /// getPressed
    /// Represents the following attribute in the schema: :getPressed
    #[xml(attr = "getPressed")]
    pub get_pressed: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
}
/// Defines the EditBox Class.
/// When the object is serialized out as xml, it's qualified name is mso:editBox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:editBox")]
pub struct EditBox {
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// maxLength
    /// Represents the following attribute in the schema: :maxLength
    #[xml(attr = "maxLength")]
    pub max_length: Option<i32>,
    /// getText
    /// Represents the following attribute in the schema: :getText
    #[xml(attr = "getText")]
    pub get_text: Option<String>,
    /// onChange
    /// Represents the following attribute in the schema: :onChange
    #[xml(attr = "onChange")]
    pub on_change: Option<String>,
    /// sizeString
    /// Represents the following attribute in the schema: :sizeString
    #[xml(attr = "sizeString")]
    pub size_string: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
}
/// Defines the ComboBox Class.
/// When the object is serialized out as xml, it's qualified name is mso:comboBox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:comboBox")]
pub struct ComboBox {
    /// showItemImage
    /// Represents the following attribute in the schema: :showItemImage
    #[xml(attr = "showItemImage")]
    pub show_item_image: Option<bool>,
    /// getItemCount
    /// Represents the following attribute in the schema: :getItemCount
    #[xml(attr = "getItemCount")]
    pub get_item_count: Option<String>,
    /// getItemLabel
    /// Represents the following attribute in the schema: :getItemLabel
    #[xml(attr = "getItemLabel")]
    pub get_item_label: Option<String>,
    /// getItemScreentip
    /// Represents the following attribute in the schema: :getItemScreentip
    #[xml(attr = "getItemScreentip")]
    pub get_item_screentip: Option<String>,
    /// getItemSupertip
    /// Represents the following attribute in the schema: :getItemSupertip
    #[xml(attr = "getItemSupertip")]
    pub get_item_supertip: Option<String>,
    /// getItemImage
    /// Represents the following attribute in the schema: :getItemImage
    #[xml(attr = "getItemImage")]
    pub get_item_image: Option<String>,
    /// getItemID
    /// Represents the following attribute in the schema: :getItemID
    #[xml(attr = "getItemID")]
    pub get_item_id: Option<String>,
    /// sizeString
    /// Represents the following attribute in the schema: :sizeString
    #[xml(attr = "sizeString")]
    pub size_string: Option<String>,
    /// invalidateContentOnDrop
    /// Represents the following attribute in the schema: :invalidateContentOnDrop
    #[xml(attr = "invalidateContentOnDrop")]
    pub invalidate_content_on_drop: Option<bool>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// maxLength
    /// Represents the following attribute in the schema: :maxLength
    #[xml(attr = "maxLength")]
    pub max_length: Option<i32>,
    /// getText
    /// Represents the following attribute in the schema: :getText
    #[xml(attr = "getText")]
    pub get_text: Option<String>,
    /// onChange
    /// Represents the following attribute in the schema: :onChange
    #[xml(attr = "onChange")]
    pub on_change: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
    #[xml(child = "mso:item")]
    pub children: Vec<ComboBoxChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ComboBoxChildChoice {
    #[xml(tag = "mso:item")]
    MsoItem(Item),
}
/// Defines the DropDown Class.
/// When the object is serialized out as xml, it's qualified name is mso:dropDown.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:dropDown")]
pub struct DropDown {
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// showItemImage
    /// Represents the following attribute in the schema: :showItemImage
    #[xml(attr = "showItemImage")]
    pub show_item_image: Option<bool>,
    /// getItemCount
    /// Represents the following attribute in the schema: :getItemCount
    #[xml(attr = "getItemCount")]
    pub get_item_count: Option<String>,
    /// getItemLabel
    /// Represents the following attribute in the schema: :getItemLabel
    #[xml(attr = "getItemLabel")]
    pub get_item_label: Option<String>,
    /// getItemScreentip
    /// Represents the following attribute in the schema: :getItemScreentip
    #[xml(attr = "getItemScreentip")]
    pub get_item_screentip: Option<String>,
    /// getItemSupertip
    /// Represents the following attribute in the schema: :getItemSupertip
    #[xml(attr = "getItemSupertip")]
    pub get_item_supertip: Option<String>,
    /// getItemImage
    /// Represents the following attribute in the schema: :getItemImage
    #[xml(attr = "getItemImage")]
    pub get_item_image: Option<String>,
    /// getItemID
    /// Represents the following attribute in the schema: :getItemID
    #[xml(attr = "getItemID")]
    pub get_item_id: Option<String>,
    /// sizeString
    /// Represents the following attribute in the schema: :sizeString
    #[xml(attr = "sizeString")]
    pub size_string: Option<String>,
    /// getSelectedItemID
    /// Represents the following attribute in the schema: :getSelectedItemID
    #[xml(attr = "getSelectedItemID")]
    pub get_selected_item_id: Option<String>,
    /// getSelectedItemIndex
    /// Represents the following attribute in the schema: :getSelectedItemIndex
    #[xml(attr = "getSelectedItemIndex")]
    pub get_selected_item_index: Option<String>,
    /// showItemLabel
    /// Represents the following attribute in the schema: :showItemLabel
    #[xml(attr = "showItemLabel")]
    pub show_item_label: Option<bool>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
    #[xml(child = "mso:item", child = "mso:button")]
    pub children: Vec<DropDownChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DropDownChildChoice {
    #[xml(tag = "mso:item")]
    MsoItem(Item),
    #[xml(tag = "mso:button")]
    MsoButton(UnsizedButton),
}
/// Defines the Gallery Class.
/// When the object is serialized out as xml, it's qualified name is mso:gallery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:gallery")]
pub struct Gallery {
    /// size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: Option<SizeValues>,
    /// getSize
    /// Represents the following attribute in the schema: :getSize
    #[xml(attr = "getSize")]
    pub get_size: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// invalidateContentOnDrop
    /// Represents the following attribute in the schema: :invalidateContentOnDrop
    #[xml(attr = "invalidateContentOnDrop")]
    pub invalidate_content_on_drop: Option<bool>,
    /// columns
    /// Represents the following attribute in the schema: :columns
    #[xml(attr = "columns")]
    pub columns: Option<i32>,
    /// rows
    /// Represents the following attribute in the schema: :rows
    #[xml(attr = "rows")]
    pub rows: Option<i32>,
    /// itemWidth
    /// Represents the following attribute in the schema: :itemWidth
    #[xml(attr = "itemWidth")]
    pub item_width: Option<i32>,
    /// itemHeight
    /// Represents the following attribute in the schema: :itemHeight
    #[xml(attr = "itemHeight")]
    pub item_height: Option<i32>,
    /// getItemWidth
    /// Represents the following attribute in the schema: :getItemWidth
    #[xml(attr = "getItemWidth")]
    pub get_item_width: Option<String>,
    /// getItemHeight
    /// Represents the following attribute in the schema: :getItemHeight
    #[xml(attr = "getItemHeight")]
    pub get_item_height: Option<String>,
    /// showItemLabel
    /// Represents the following attribute in the schema: :showItemLabel
    #[xml(attr = "showItemLabel")]
    pub show_item_label: Option<bool>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// showItemImage
    /// Represents the following attribute in the schema: :showItemImage
    #[xml(attr = "showItemImage")]
    pub show_item_image: Option<bool>,
    /// getItemCount
    /// Represents the following attribute in the schema: :getItemCount
    #[xml(attr = "getItemCount")]
    pub get_item_count: Option<String>,
    /// getItemLabel
    /// Represents the following attribute in the schema: :getItemLabel
    #[xml(attr = "getItemLabel")]
    pub get_item_label: Option<String>,
    /// getItemScreentip
    /// Represents the following attribute in the schema: :getItemScreentip
    #[xml(attr = "getItemScreentip")]
    pub get_item_screentip: Option<String>,
    /// getItemSupertip
    /// Represents the following attribute in the schema: :getItemSupertip
    #[xml(attr = "getItemSupertip")]
    pub get_item_supertip: Option<String>,
    /// getItemImage
    /// Represents the following attribute in the schema: :getItemImage
    #[xml(attr = "getItemImage")]
    pub get_item_image: Option<String>,
    /// getItemID
    /// Represents the following attribute in the schema: :getItemID
    #[xml(attr = "getItemID")]
    pub get_item_id: Option<String>,
    /// sizeString
    /// Represents the following attribute in the schema: :sizeString
    #[xml(attr = "sizeString")]
    pub size_string: Option<String>,
    /// getSelectedItemID
    /// Represents the following attribute in the schema: :getSelectedItemID
    #[xml(attr = "getSelectedItemID")]
    pub get_selected_item_id: Option<String>,
    /// getSelectedItemIndex
    /// Represents the following attribute in the schema: :getSelectedItemIndex
    #[xml(attr = "getSelectedItemIndex")]
    pub get_selected_item_index: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
    #[xml(child = "mso:item", child = "mso:button")]
    pub children: Vec<GalleryChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GalleryChildChoice {
    #[xml(tag = "mso:item")]
    MsoItem(Item),
    #[xml(tag = "mso:button")]
    MsoButton(UnsizedButton),
}
/// Defines the Menu Class.
/// When the object is serialized out as xml, it's qualified name is mso:menu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:menu")]
pub struct Menu {
    /// size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: Option<SizeValues>,
    /// getSize
    /// Represents the following attribute in the schema: :getSize
    #[xml(attr = "getSize")]
    pub get_size: Option<String>,
    /// itemSize
    /// Represents the following attribute in the schema: :itemSize
    #[xml(attr = "itemSize")]
    pub item_size: Option<ItemSizeValues>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
    #[xml(
        child = "mso:control",
        child = "mso:button",
        child = "mso:checkBox",
        child = "mso:gallery",
        child = "mso:toggleButton",
        child = "mso:menuSeparator",
        child = "mso:splitButton",
        child = "mso:menu",
        child = "mso:dynamicMenu",
    )]
    pub children: Vec<MenuChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MenuChildChoice {
    #[xml(tag = "mso:control")]
    MsoControl(UnsizedControlClone),
    #[xml(tag = "mso:button")]
    MsoButton(UnsizedButton),
    #[xml(tag = "mso:checkBox")]
    MsoCheckBox(CheckBox),
    #[xml(tag = "mso:gallery")]
    MsoGallery(UnsizedGallery),
    #[xml(tag = "mso:toggleButton")]
    MsoToggleButton(UnsizedToggleButton),
    #[xml(tag = "mso:menuSeparator")]
    MsoMenuSeparator(MenuSeparator),
    #[xml(tag = "mso:splitButton")]
    MsoSplitButton(UnsizedSplitButton),
    #[xml(tag = "mso:menu")]
    MsoMenu(UnsizedMenu),
    #[xml(tag = "mso:dynamicMenu")]
    MsoDynamicMenu(UnsizedDynamicMenu),
}
/// Defines the DynamicMenu Class.
/// When the object is serialized out as xml, it's qualified name is mso:dynamicMenu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:dynamicMenu")]
pub struct DynamicMenu {
    /// size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: Option<SizeValues>,
    /// getSize
    /// Represents the following attribute in the schema: :getSize
    #[xml(attr = "getSize")]
    pub get_size: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// getContent
    /// Represents the following attribute in the schema: :getContent
    #[xml(attr = "getContent")]
    pub get_content: String,
    /// invalidateContentOnDrop
    /// Represents the following attribute in the schema: :invalidateContentOnDrop
    #[xml(attr = "invalidateContentOnDrop")]
    pub invalidate_content_on_drop: Option<bool>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
}
/// Defines the SplitButton Class.
/// When the object is serialized out as xml, it's qualified name is mso:splitButton.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:splitButton")]
pub struct SplitButton {
    /// size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: Option<SizeValues>,
    /// getSize
    /// Represents the following attribute in the schema: :getSize
    #[xml(attr = "getSize")]
    pub get_size: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    #[xml(child = "mso:button", child = "mso:toggleButton", child = "mso:menu")]
    pub children: Vec<SplitButtonChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SplitButtonChildChoice {
    #[xml(tag = "mso:button")]
    MsoButton(VisibleButton),
    #[xml(tag = "mso:toggleButton")]
    MsoToggleButton(VisibleToggleButton),
    #[xml(tag = "mso:menu")]
    MsoMenu(UnsizedMenu),
}
/// Defines the Box Class.
/// When the object is serialized out as xml, it's qualified name is mso:box.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:box")]
pub struct Box {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// boxStyle
    /// Represents the following attribute in the schema: :boxStyle
    #[xml(attr = "boxStyle")]
    pub box_style: Option<BoxStyleValues>,
    #[xml(
        child = "mso:control",
        child = "mso:labelControl",
        child = "mso:button",
        child = "mso:toggleButton",
        child = "mso:checkBox",
        child = "mso:editBox",
        child = "mso:comboBox",
        child = "mso:dropDown",
        child = "mso:gallery",
        child = "mso:menu",
        child = "mso:dynamicMenu",
        child = "mso:splitButton",
        child = "mso:box",
        child = "mso:buttonGroup",
    )]
    pub children: Vec<BoxChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BoxChildChoice {
    #[xml(tag = "mso:control")]
    MsoControl(ControlClone),
    #[xml(tag = "mso:labelControl")]
    MsoLabelControl(TextLabel),
    #[xml(tag = "mso:button")]
    MsoButton(Button),
    #[xml(tag = "mso:toggleButton")]
    MsoToggleButton(ToggleButton),
    #[xml(tag = "mso:checkBox")]
    MsoCheckBox(CheckBox),
    #[xml(tag = "mso:editBox")]
    MsoEditBox(EditBox),
    #[xml(tag = "mso:comboBox")]
    MsoComboBox(ComboBox),
    #[xml(tag = "mso:dropDown")]
    MsoDropDown(DropDown),
    #[xml(tag = "mso:gallery")]
    MsoGallery(Gallery),
    #[xml(tag = "mso:menu")]
    MsoMenu(Menu),
    #[xml(tag = "mso:dynamicMenu")]
    MsoDynamicMenu(DynamicMenu),
    #[xml(tag = "mso:splitButton")]
    MsoSplitButton(SplitButton),
    #[xml(tag = "mso:box")]
    MsoBox(Box),
    #[xml(tag = "mso:buttonGroup")]
    MsoButtonGroup(ButtonGroup),
}
/// Defines the ButtonGroup Class.
/// When the object is serialized out as xml, it's qualified name is mso:buttonGroup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:buttonGroup")]
pub struct ButtonGroup {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    #[xml(
        child = "mso:control",
        child = "mso:button",
        child = "mso:toggleButton",
        child = "mso:gallery",
        child = "mso:menu",
        child = "mso:dynamicMenu",
        child = "mso:splitButton",
    )]
    pub children: Vec<ButtonGroupChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ButtonGroupChildChoice {
    #[xml(tag = "mso:control")]
    MsoControl(UnsizedControlClone),
    #[xml(tag = "mso:button")]
    MsoButton(UnsizedButton),
    #[xml(tag = "mso:toggleButton")]
    MsoToggleButton(UnsizedToggleButton),
    #[xml(tag = "mso:gallery")]
    MsoGallery(UnsizedGallery),
    #[xml(tag = "mso:menu")]
    MsoMenu(UnsizedMenu),
    #[xml(tag = "mso:dynamicMenu")]
    MsoDynamicMenu(UnsizedDynamicMenu),
    #[xml(tag = "mso:splitButton")]
    MsoSplitButton(UnsizedSplitButton),
}
/// Defines the MenuRoot Class.
/// When the object is serialized out as xml, it's qualified name is mso:menu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:menu")]
pub struct MenuRoot {
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// getTitle
    /// Represents the following attribute in the schema: :getTitle
    #[xml(attr = "getTitle")]
    pub get_title: Option<String>,
    /// itemSize
    /// Represents the following attribute in the schema: :itemSize
    #[xml(attr = "itemSize")]
    pub item_size: Option<ItemSizeValues>,
    #[xml(
        child = "mso:control",
        child = "mso:button",
        child = "mso:checkBox",
        child = "mso:gallery",
        child = "mso:toggleButton",
        child = "mso:menuSeparator",
        child = "mso:splitButton",
        child = "mso:menu",
        child = "mso:dynamicMenu",
    )]
    pub children: Vec<MenuRootChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MenuRootChildChoice {
    #[xml(tag = "mso:control")]
    MsoControl(UnsizedControlClone),
    #[xml(tag = "mso:button")]
    MsoButton(UnsizedButton),
    #[xml(tag = "mso:checkBox")]
    MsoCheckBox(CheckBox),
    #[xml(tag = "mso:gallery")]
    MsoGallery(UnsizedGallery),
    #[xml(tag = "mso:toggleButton")]
    MsoToggleButton(UnsizedToggleButton),
    #[xml(tag = "mso:menuSeparator")]
    MsoMenuSeparator(MenuSeparator),
    #[xml(tag = "mso:splitButton")]
    MsoSplitButton(UnsizedSplitButton),
    #[xml(tag = "mso:menu")]
    MsoMenu(UnsizedMenu),
    #[xml(tag = "mso:dynamicMenu")]
    MsoDynamicMenu(UnsizedDynamicMenu),
}
/// Defines the CustomUI Class.
/// When the object is serialized out as xml, it's qualified name is mso:customUI.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:customUI")]
pub struct CustomUi {
    #[xml(attr = "xmlns", with = "custom_ui_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// onLoad
    /// Represents the following attribute in the schema: :onLoad
    #[xml(attr = "onLoad")]
    pub on_load: Option<String>,
    /// loadImage
    /// Represents the following attribute in the schema: :loadImage
    #[xml(attr = "loadImage")]
    pub load_image: Option<String>,
    /// _
    #[xml(child = "mso:commands")]
    pub repurposed_commands: Option<RepurposedCommands>,
    /// _
    #[xml(child = "mso:ribbon")]
    pub ribbon: Option<Ribbon>,
}
mod custom_ui_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/2006/01/customui")
    }
}
/// Defines the Item Class.
/// When the object is serialized out as xml, it's qualified name is mso:item.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:item")]
pub struct Item {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
}
/// Defines the VisibleButton Class.
/// When the object is serialized out as xml, it's qualified name is mso:button.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:button")]
pub struct VisibleButton {
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
}
/// Defines the VisibleToggleButton Class.
/// When the object is serialized out as xml, it's qualified name is mso:toggleButton.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:toggleButton")]
pub struct VisibleToggleButton {
    /// getPressed
    /// Represents the following attribute in the schema: :getPressed
    #[xml(attr = "getPressed")]
    pub get_pressed: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
}
/// Defines the VerticalSeparator Class.
/// When the object is serialized out as xml, it's qualified name is mso:separator.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:separator")]
pub struct VerticalSeparator {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
}
/// Defines the DialogBoxLauncher Class.
/// When the object is serialized out as xml, it's qualified name is mso:dialogBoxLauncher.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:dialogBoxLauncher")]
pub struct DialogBoxLauncher {
    /// _
    #[xml(child = "mso:button")]
    pub unsized_button: UnsizedButton,
}
/// Defines the Group Class.
/// When the object is serialized out as xml, it's qualified name is mso:group.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:group")]
pub struct Group {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    #[xml(
        child = "mso:control",
        child = "mso:labelControl",
        child = "mso:button",
        child = "mso:toggleButton",
        child = "mso:checkBox",
        child = "mso:editBox",
        child = "mso:comboBox",
        child = "mso:dropDown",
        child = "mso:gallery",
        child = "mso:menu",
        child = "mso:dynamicMenu",
        child = "mso:splitButton",
        child = "mso:box",
        child = "mso:buttonGroup",
        child = "mso:separator",
        child = "mso:dialogBoxLauncher",
    )]
    pub children: Vec<GroupChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GroupChildChoice {
    #[xml(tag = "mso:control")]
    MsoControl(ControlClone),
    #[xml(tag = "mso:labelControl")]
    MsoLabelControl(TextLabel),
    #[xml(tag = "mso:button")]
    MsoButton(Button),
    #[xml(tag = "mso:toggleButton")]
    MsoToggleButton(ToggleButton),
    #[xml(tag = "mso:checkBox")]
    MsoCheckBox(CheckBox),
    #[xml(tag = "mso:editBox")]
    MsoEditBox(EditBox),
    #[xml(tag = "mso:comboBox")]
    MsoComboBox(ComboBox),
    #[xml(tag = "mso:dropDown")]
    MsoDropDown(DropDown),
    #[xml(tag = "mso:gallery")]
    MsoGallery(Gallery),
    #[xml(tag = "mso:menu")]
    MsoMenu(Menu),
    #[xml(tag = "mso:dynamicMenu")]
    MsoDynamicMenu(DynamicMenu),
    #[xml(tag = "mso:splitButton")]
    MsoSplitButton(SplitButton),
    #[xml(tag = "mso:box")]
    MsoBox(Box),
    #[xml(tag = "mso:buttonGroup")]
    MsoButtonGroup(ButtonGroup),
    #[xml(tag = "mso:separator")]
    MsoSeparator(VerticalSeparator),
    #[xml(tag = "mso:dialogBoxLauncher")]
    MsoDialogBoxLauncher(DialogBoxLauncher),
}
/// Defines the QuickAccessToolbarControlClone Class.
/// When the object is serialized out as xml, it's qualified name is mso:control.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:control")]
pub struct QuickAccessToolbarControlClone {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: Option<SizeValues>,
    /// getSize
    /// Represents the following attribute in the schema: :getSize
    #[xml(attr = "getSize")]
    pub get_size: Option<String>,
    /// image
    /// Represents the following attribute in the schema: :image
    #[xml(attr = "image")]
    pub image: Option<String>,
    /// imageMso
    /// Represents the following attribute in the schema: :imageMso
    #[xml(attr = "imageMso")]
    pub image_mso: Option<String>,
    /// getImage
    /// Represents the following attribute in the schema: :getImage
    #[xml(attr = "getImage")]
    pub get_image: Option<String>,
    /// screentip
    /// Represents the following attribute in the schema: :screentip
    #[xml(attr = "screentip")]
    pub screentip: Option<String>,
    /// getScreentip
    /// Represents the following attribute in the schema: :getScreentip
    #[xml(attr = "getScreentip")]
    pub get_screentip: Option<String>,
    /// supertip
    /// Represents the following attribute in the schema: :supertip
    #[xml(attr = "supertip")]
    pub supertip: Option<String>,
    /// getSupertip
    /// Represents the following attribute in the schema: :getSupertip
    #[xml(attr = "getSupertip")]
    pub get_supertip: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// showImage
    /// Represents the following attribute in the schema: :showImage
    #[xml(attr = "showImage")]
    pub show_image: Option<bool>,
    /// getShowImage
    /// Represents the following attribute in the schema: :getShowImage
    #[xml(attr = "getShowImage")]
    pub get_show_image: Option<String>,
}
/// Defines the SharedQatControls Class.
/// When the object is serialized out as xml, it's qualified name is mso:sharedControls.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:sharedControls")]
pub struct SharedQatControls {
    #[xml(child = "mso:control", child = "mso:button", child = "mso:separator")]
    pub children: Vec<SharedQatControlsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SharedQatControlsChildChoice {
    #[xml(tag = "mso:control")]
    MsoControl(QuickAccessToolbarControlClone),
    #[xml(tag = "mso:button")]
    MsoButton(UnsizedButton),
    #[xml(tag = "mso:separator")]
    MsoSeparator(VerticalSeparator),
}
/// Defines the DocumentSpecificQuickAccessToolbarControls Class.
/// When the object is serialized out as xml, it's qualified name is mso:documentControls.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:documentControls")]
pub struct DocumentSpecificQuickAccessToolbarControls {
    #[xml(child = "mso:control", child = "mso:button", child = "mso:separator")]
    pub children: Vec<DocumentSpecificQuickAccessToolbarControlsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DocumentSpecificQuickAccessToolbarControlsChildChoice {
    #[xml(tag = "mso:control")]
    MsoControl(QuickAccessToolbarControlClone),
    #[xml(tag = "mso:button")]
    MsoButton(UnsizedButton),
    #[xml(tag = "mso:separator")]
    MsoSeparator(VerticalSeparator),
}
/// Defines the QatItemsType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct QatItemsType {
    #[xml(child = "mso:control", child = "mso:button", child = "mso:separator")]
    pub children: Vec<QatItemsTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum QatItemsTypeChildChoice {
    #[xml(tag = "mso:control")]
    MsoControl(QuickAccessToolbarControlClone),
    #[xml(tag = "mso:button")]
    MsoButton(UnsizedButton),
    #[xml(tag = "mso:separator")]
    MsoSeparator(VerticalSeparator),
}
/// Defines the Tab Class.
/// When the object is serialized out as xml, it's qualified name is mso:tab.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:tab")]
pub struct Tab {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub id_q: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// insertAfterMso
    /// Represents the following attribute in the schema: :insertAfterMso
    #[xml(attr = "insertAfterMso")]
    pub insert_after_mso: Option<String>,
    /// insertBeforeMso
    /// Represents the following attribute in the schema: :insertBeforeMso
    #[xml(attr = "insertBeforeMso")]
    pub insert_before_mso: Option<String>,
    /// insertAfterQ
    /// Represents the following attribute in the schema: :insertAfterQ
    #[xml(attr = "insertAfterQ")]
    pub insert_after_q: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_q: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    #[xml(child = "mso:group")]
    pub children: Vec<TabChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TabChildChoice {
    #[xml(tag = "mso:group")]
    MsoGroup(Group),
}
/// Defines the ContextualTabSet Class.
/// When the object is serialized out as xml, it's qualified name is mso:tabSet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:tabSet")]
pub struct ContextualTabSet {
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: String,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// _
    #[xml(child = "mso:tab")]
    pub mso_tab: Vec<Tab>,
}
/// Defines the RepurposedCommand Class.
/// When the object is serialized out as xml, it's qualified name is mso:command.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:command")]
pub struct RepurposedCommand {
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
}
/// Defines the OfficeMenu Class.
/// When the object is serialized out as xml, it's qualified name is mso:officeMenu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:officeMenu")]
pub struct OfficeMenu {
    #[xml(
        child = "mso:control",
        child = "mso:button",
        child = "mso:checkBox",
        child = "mso:gallery",
        child = "mso:toggleButton",
        child = "mso:menuSeparator",
        child = "mso:splitButton",
        child = "mso:menu",
        child = "mso:dynamicMenu",
    )]
    pub children: Vec<OfficeMenuChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeMenuChildChoice {
    #[xml(tag = "mso:control")]
    MsoControl(UnsizedControlClone),
    #[xml(tag = "mso:button")]
    MsoButton(UnsizedButton),
    #[xml(tag = "mso:checkBox")]
    MsoCheckBox(CheckBox),
    #[xml(tag = "mso:gallery")]
    MsoGallery(UnsizedGallery),
    #[xml(tag = "mso:toggleButton")]
    MsoToggleButton(UnsizedToggleButton),
    #[xml(tag = "mso:menuSeparator")]
    MsoMenuSeparator(MenuSeparator),
    #[xml(tag = "mso:splitButton")]
    MsoSplitButton(SplitButtonWithTitle),
    #[xml(tag = "mso:menu")]
    MsoMenu(MenuWithTitle),
    #[xml(tag = "mso:dynamicMenu")]
    MsoDynamicMenu(UnsizedDynamicMenu),
}
/// Defines the QuickAccessToolbar Class.
/// When the object is serialized out as xml, it's qualified name is mso:qat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:qat")]
pub struct QuickAccessToolbar {
    /// _
    #[xml(child = "mso:sharedControls")]
    pub shared_qat_controls: Option<SharedQatControls>,
    /// _
    #[xml(child = "mso:documentControls")]
    pub document_specific_quick_access_toolbar_controls: Option<
        DocumentSpecificQuickAccessToolbarControls,
    >,
}
/// Defines the Tabs Class.
/// When the object is serialized out as xml, it's qualified name is mso:tabs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:tabs")]
pub struct Tabs {
    /// _
    #[xml(child = "mso:tab")]
    pub mso_tab: Vec<Tab>,
}
/// Defines the ContextualTabSets Class.
/// When the object is serialized out as xml, it's qualified name is mso:contextualTabs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:contextualTabs")]
pub struct ContextualTabSets {
    /// _
    #[xml(child = "mso:tabSet")]
    pub mso_tab_set: Vec<ContextualTabSet>,
}
/// Defines the RepurposedCommands Class.
/// When the object is serialized out as xml, it's qualified name is mso:commands.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:commands")]
pub struct RepurposedCommands {
    /// _
    #[xml(child = "mso:command")]
    pub mso_command: Vec<RepurposedCommand>,
}
/// Defines the Ribbon Class.
/// When the object is serialized out as xml, it's qualified name is mso:ribbon.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso:ribbon")]
pub struct Ribbon {
    /// startFromScratch
    /// Represents the following attribute in the schema: :startFromScratch
    #[xml(attr = "startFromScratch")]
    pub start_from_scratch: Option<bool>,
    #[xml(
        child = "mso:officeMenu",
        child = "mso:qat",
        child = "mso:tabs",
        child = "mso:contextualTabs",
    )]
    pub children: Vec<RibbonChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RibbonChildChoice {
    #[xml(tag = "mso:officeMenu")]
    MsoOfficeMenu(OfficeMenu),
    #[xml(tag = "mso:qat")]
    MsoQat(QuickAccessToolbar),
    #[xml(tag = "mso:tabs")]
    MsoTabs(Tabs),
    #[xml(tag = "mso:contextualTabs")]
    MsoContextualTabs(ContextualTabSets),
}
