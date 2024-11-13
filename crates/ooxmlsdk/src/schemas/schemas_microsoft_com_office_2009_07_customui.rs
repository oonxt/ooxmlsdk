#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GalleryShowInRibbonValues {
    #[default]
    False,
    Zero,
}
crate::__string_enum! {
    GalleryShowInRibbonValues { False = "false", Zero = "0", }
}
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
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TaskSizesValues {
    #[default]
    LargeMediumSmall,
    LargeMedium,
    Large,
    MediumSmall,
    Medium,
    Small,
}
crate::__string_enum! {
    TaskSizesValues { LargeMediumSmall = "largeMediumSmall", LargeMedium = "largeMedium",
    Large = "large", MediumSmall = "mediumSmall", Medium = "medium", Small = "small", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ExpandValues {
    #[default]
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}
crate::__string_enum! {
    ExpandValues { TopLeft = "topLeft", Top = "top", TopRight = "topRight", Left =
    "left", Center = "center", Right = "right", BottomLeft = "bottomLeft", Bottom =
    "bottom", BottomRight = "bottomRight", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StyleValues {
    #[default]
    Normal,
    Warning,
    Error,
}
crate::__string_enum! {
    StyleValues { Normal = "normal", Warning = "warning", Error = "error", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Style2Values {
    #[default]
    Normal,
    Borderless,
    Large,
}
crate::__string_enum! {
    Style2Values { Normal = "normal", Borderless = "borderless", Large = "large", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LayoutChildrenValues {
    #[default]
    Horizontal,
    Vertical,
}
crate::__string_enum! {
    LayoutChildrenValues { Horizontal = "horizontal", Vertical = "vertical", }
}
/// Defines the ControlCloneRegular Class.
/// When the object is serialized out as xml, it's qualified name is mso14:control.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:control")]
pub struct ControlCloneRegular {
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
/// Defines the ButtonRegular Class.
/// When the object is serialized out as xml, it's qualified name is mso14:button.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:button")]
pub struct ButtonRegular {
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
/// When the object is serialized out as xml, it's qualified name is mso14:checkBox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:checkBox")]
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
/// Defines the GalleryRegular Class.
/// When the object is serialized out as xml, it's qualified name is mso14:gallery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:gallery")]
pub struct GalleryRegular {
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
    /// showInRibbon
    /// Represents the following attribute in the schema: :showInRibbon
    #[xml(attr = "showInRibbon")]
    pub show_in_ribbon: Option<GalleryShowInRibbonValues>,
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
    #[xml(child = "mso14:item", child = "mso14:button")]
    pub children: Vec<GalleryRegularChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GalleryRegularChildChoice {
    #[xml(tag = "mso14:item")]
    Mso14Item(Item),
    #[xml(tag = "mso14:button")]
    Mso14Button(ButtonRegular),
}
/// Defines the ToggleButtonRegular Class.
/// When the object is serialized out as xml, it's qualified name is mso14:toggleButton.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:toggleButton")]
pub struct ToggleButtonRegular {
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
/// When the object is serialized out as xml, it's qualified name is mso14:menuSeparator.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:menuSeparator")]
pub struct MenuSeparator {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// getTitle
    /// Represents the following attribute in the schema: :getTitle
    #[xml(attr = "getTitle")]
    pub get_title: Option<String>,
}
/// Defines the SplitButtonRegular Class.
/// When the object is serialized out as xml, it's qualified name is mso14:splitButton.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:splitButton")]
pub struct SplitButtonRegular {
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
    #[xml(child = "mso14:button", child = "mso14:toggleButton", child = "mso14:menu")]
    pub children: Vec<SplitButtonRegularChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SplitButtonRegularChildChoice {
    #[xml(tag = "mso14:button")]
    Mso14Button(VisibleButton),
    #[xml(tag = "mso14:toggleButton")]
    Mso14ToggleButton(VisibleToggleButton),
    #[xml(tag = "mso14:menu")]
    Mso14Menu(MenuRegular),
}
/// Defines the MenuRegular Class.
/// When the object is serialized out as xml, it's qualified name is mso14:menu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:menu")]
pub struct MenuRegular {
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
        child = "mso14:control",
        child = "mso14:button",
        child = "mso14:checkBox",
        child = "mso14:gallery",
        child = "mso14:toggleButton",
        child = "mso14:menuSeparator",
        child = "mso14:splitButton",
        child = "mso14:menu",
        child = "mso14:dynamicMenu",
    )]
    pub children: Vec<MenuRegularChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MenuRegularChildChoice {
    #[xml(tag = "mso14:control")]
    Mso14Control(ControlCloneRegular),
    #[xml(tag = "mso14:button")]
    Mso14Button(ButtonRegular),
    #[xml(tag = "mso14:checkBox")]
    Mso14CheckBox(CheckBox),
    #[xml(tag = "mso14:gallery")]
    Mso14Gallery(GalleryRegular),
    #[xml(tag = "mso14:toggleButton")]
    Mso14ToggleButton(ToggleButtonRegular),
    #[xml(tag = "mso14:menuSeparator")]
    Mso14MenuSeparator(MenuSeparator),
    #[xml(tag = "mso14:splitButton")]
    Mso14SplitButton(SplitButtonRegular),
    #[xml(tag = "mso14:menu")]
    Mso14Menu(MenuRegular),
    #[xml(tag = "mso14:dynamicMenu")]
    Mso14DynamicMenu(DynamicMenuRegular),
}
/// Defines the DynamicMenuRegular Class.
/// When the object is serialized out as xml, it's qualified name is mso14:dynamicMenu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:dynamicMenu")]
pub struct DynamicMenuRegular {
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
/// When the object is serialized out as xml, it's qualified name is mso14:splitButton.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:splitButton")]
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
    #[xml(child = "mso14:button", child = "mso14:toggleButton", child = "mso14:menu")]
    pub children: Vec<SplitButtonWithTitleChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SplitButtonWithTitleChildChoice {
    #[xml(tag = "mso14:button")]
    Mso14Button(VisibleButton),
    #[xml(tag = "mso14:toggleButton")]
    Mso14ToggleButton(VisibleToggleButton),
    #[xml(tag = "mso14:menu")]
    Mso14Menu(MenuWithTitle),
}
/// Defines the MenuWithTitle Class.
/// When the object is serialized out as xml, it's qualified name is mso14:menu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:menu")]
pub struct MenuWithTitle {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
        child = "mso14:control",
        child = "mso14:button",
        child = "mso14:checkBox",
        child = "mso14:gallery",
        child = "mso14:toggleButton",
        child = "mso14:menuSeparator",
        child = "mso14:splitButton",
        child = "mso14:menu",
        child = "mso14:dynamicMenu",
    )]
    pub children: Vec<MenuWithTitleChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MenuWithTitleChildChoice {
    #[xml(tag = "mso14:control")]
    Mso14Control(ControlCloneRegular),
    #[xml(tag = "mso14:button")]
    Mso14Button(ButtonRegular),
    #[xml(tag = "mso14:checkBox")]
    Mso14CheckBox(CheckBox),
    #[xml(tag = "mso14:gallery")]
    Mso14Gallery(GalleryRegular),
    #[xml(tag = "mso14:toggleButton")]
    Mso14ToggleButton(ToggleButtonRegular),
    #[xml(tag = "mso14:menuSeparator")]
    Mso14MenuSeparator(MenuSeparator),
    #[xml(tag = "mso14:splitButton")]
    Mso14SplitButton(SplitButtonWithTitle),
    #[xml(tag = "mso14:menu")]
    Mso14Menu(MenuWithTitle),
    #[xml(tag = "mso14:dynamicMenu")]
    Mso14DynamicMenu(DynamicMenuRegular),
}
/// Defines the MenuSeparatorNoTitle Class.
/// When the object is serialized out as xml, it's qualified name is mso14:menuSeparator.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:menuSeparator")]
pub struct MenuSeparatorNoTitle {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
}
/// Defines the ControlClone Class.
/// When the object is serialized out as xml, it's qualified name is mso14:control.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:control")]
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
/// Defines the LabelControl Class.
/// When the object is serialized out as xml, it's qualified name is mso14:labelControl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:labelControl")]
pub struct LabelControl {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
/// When the object is serialized out as xml, it's qualified name is mso14:button.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:button")]
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
/// When the object is serialized out as xml, it's qualified name is mso14:toggleButton.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:toggleButton")]
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
/// When the object is serialized out as xml, it's qualified name is mso14:editBox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:editBox")]
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
/// When the object is serialized out as xml, it's qualified name is mso14:comboBox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:comboBox")]
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
    #[xml(child = "mso14:item")]
    pub children: Vec<ComboBoxChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ComboBoxChildChoice {
    #[xml(tag = "mso14:item")]
    Mso14Item(Item),
}
/// Defines the DropDownRegular Class.
/// When the object is serialized out as xml, it's qualified name is mso14:dropDown.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:dropDown")]
pub struct DropDownRegular {
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
    #[xml(child = "mso14:item", child = "mso14:button")]
    pub children: Vec<DropDownRegularChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DropDownRegularChildChoice {
    #[xml(tag = "mso14:item")]
    Mso14Item(Item),
    #[xml(tag = "mso14:button")]
    Mso14Button(ButtonRegular),
}
/// Defines the Gallery Class.
/// When the object is serialized out as xml, it's qualified name is mso14:gallery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:gallery")]
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
    /// showInRibbon
    /// Represents the following attribute in the schema: :showInRibbon
    #[xml(attr = "showInRibbon")]
    pub show_in_ribbon: Option<GalleryShowInRibbonValues>,
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
    #[xml(child = "mso14:item", child = "mso14:button")]
    pub children: Vec<GalleryChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GalleryChildChoice {
    #[xml(tag = "mso14:item")]
    Mso14Item(Item),
    #[xml(tag = "mso14:button")]
    Mso14Button(ButtonRegular),
}
/// Defines the Menu Class.
/// When the object is serialized out as xml, it's qualified name is mso14:menu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:menu")]
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
        child = "mso14:control",
        child = "mso14:button",
        child = "mso14:checkBox",
        child = "mso14:gallery",
        child = "mso14:toggleButton",
        child = "mso14:menuSeparator",
        child = "mso14:splitButton",
        child = "mso14:menu",
        child = "mso14:dynamicMenu",
    )]
    pub children: Vec<MenuChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MenuChildChoice {
    #[xml(tag = "mso14:control")]
    Mso14Control(ControlCloneRegular),
    #[xml(tag = "mso14:button")]
    Mso14Button(ButtonRegular),
    #[xml(tag = "mso14:checkBox")]
    Mso14CheckBox(CheckBox),
    #[xml(tag = "mso14:gallery")]
    Mso14Gallery(GalleryRegular),
    #[xml(tag = "mso14:toggleButton")]
    Mso14ToggleButton(ToggleButtonRegular),
    #[xml(tag = "mso14:menuSeparator")]
    Mso14MenuSeparator(MenuSeparator),
    #[xml(tag = "mso14:splitButton")]
    Mso14SplitButton(SplitButtonRegular),
    #[xml(tag = "mso14:menu")]
    Mso14Menu(MenuRegular),
    #[xml(tag = "mso14:dynamicMenu")]
    Mso14DynamicMenu(DynamicMenuRegular),
}
/// Defines the DynamicMenu Class.
/// When the object is serialized out as xml, it's qualified name is mso14:dynamicMenu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:dynamicMenu")]
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
/// When the object is serialized out as xml, it's qualified name is mso14:splitButton.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:splitButton")]
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
    #[xml(child = "mso14:button", child = "mso14:toggleButton", child = "mso14:menu")]
    pub children: Vec<SplitButtonChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SplitButtonChildChoice {
    #[xml(tag = "mso14:button")]
    Mso14Button(VisibleButton),
    #[xml(tag = "mso14:toggleButton")]
    Mso14ToggleButton(VisibleToggleButton),
    #[xml(tag = "mso14:menu")]
    Mso14Menu(MenuRegular),
}
/// Defines the Box Class.
/// When the object is serialized out as xml, it's qualified name is mso14:box.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:box")]
pub struct Box {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
    /// boxStyle
    /// Represents the following attribute in the schema: :boxStyle
    #[xml(attr = "boxStyle")]
    pub box_style: Option<BoxStyleValues>,
    #[xml(
        child = "mso14:control",
        child = "mso14:labelControl",
        child = "mso14:button",
        child = "mso14:toggleButton",
        child = "mso14:checkBox",
        child = "mso14:editBox",
        child = "mso14:comboBox",
        child = "mso14:dropDown",
        child = "mso14:gallery",
        child = "mso14:menu",
        child = "mso14:dynamicMenu",
        child = "mso14:splitButton",
        child = "mso14:box",
        child = "mso14:buttonGroup",
    )]
    pub children: Vec<BoxChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BoxChildChoice {
    #[xml(tag = "mso14:control")]
    Mso14Control(ControlClone),
    #[xml(tag = "mso14:labelControl")]
    Mso14LabelControl(LabelControl),
    #[xml(tag = "mso14:button")]
    Mso14Button(Button),
    #[xml(tag = "mso14:toggleButton")]
    Mso14ToggleButton(ToggleButton),
    #[xml(tag = "mso14:checkBox")]
    Mso14CheckBox(CheckBox),
    #[xml(tag = "mso14:editBox")]
    Mso14EditBox(EditBox),
    #[xml(tag = "mso14:comboBox")]
    Mso14ComboBox(ComboBox),
    #[xml(tag = "mso14:dropDown")]
    Mso14DropDown(DropDownRegular),
    #[xml(tag = "mso14:gallery")]
    Mso14Gallery(Gallery),
    #[xml(tag = "mso14:menu")]
    Mso14Menu(Menu),
    #[xml(tag = "mso14:dynamicMenu")]
    Mso14DynamicMenu(DynamicMenu),
    #[xml(tag = "mso14:splitButton")]
    Mso14SplitButton(SplitButton),
    #[xml(tag = "mso14:box")]
    Mso14Box(Box),
    #[xml(tag = "mso14:buttonGroup")]
    Mso14ButtonGroup(ButtonGroup),
}
/// Defines the ButtonGroup Class.
/// When the object is serialized out as xml, it's qualified name is mso14:buttonGroup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:buttonGroup")]
pub struct ButtonGroup {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
    #[xml(
        child = "mso14:control",
        child = "mso14:button",
        child = "mso14:toggleButton",
        child = "mso14:gallery",
        child = "mso14:menu",
        child = "mso14:dynamicMenu",
        child = "mso14:splitButton",
        child = "mso14:separator",
    )]
    pub children: Vec<ButtonGroupChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ButtonGroupChildChoice {
    #[xml(tag = "mso14:control")]
    Mso14Control(ControlCloneRegular),
    #[xml(tag = "mso14:button")]
    Mso14Button(ButtonRegular),
    #[xml(tag = "mso14:toggleButton")]
    Mso14ToggleButton(ToggleButtonRegular),
    #[xml(tag = "mso14:gallery")]
    Mso14Gallery(GalleryRegular),
    #[xml(tag = "mso14:menu")]
    Mso14Menu(MenuRegular),
    #[xml(tag = "mso14:dynamicMenu")]
    Mso14DynamicMenu(DynamicMenuRegular),
    #[xml(tag = "mso14:splitButton")]
    Mso14SplitButton(SplitButtonRegular),
    #[xml(tag = "mso14:separator")]
    Mso14Separator(Separator),
}
/// Defines the BackstageMenuButton Class.
/// When the object is serialized out as xml, it's qualified name is mso14:button.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:button")]
pub struct BackstageMenuButton {
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// isDefinitive
    /// Represents the following attribute in the schema: :isDefinitive
    #[xml(attr = "isDefinitive")]
    pub is_definitive: Option<bool>,
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
}
/// Defines the BackstageMenuCheckBox Class.
/// When the object is serialized out as xml, it's qualified name is mso14:checkBox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:checkBox")]
pub struct BackstageMenuCheckBox {
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// getPressed
    /// Represents the following attribute in the schema: :getPressed
    #[xml(attr = "getPressed")]
    pub get_pressed: Option<String>,
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
/// Defines the BackstageSubMenu Class.
/// When the object is serialized out as xml, it's qualified name is mso14:menu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:menu")]
pub struct BackstageSubMenu {
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
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
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
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
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    #[xml(child = "mso14:menuGroup")]
    pub children: Vec<BackstageSubMenuChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BackstageSubMenuChildChoice {
    #[xml(tag = "mso14:menuGroup")]
    Mso14MenuGroup(BackstageMenuGroup),
}
/// Defines the BackstageMenuToggleButton Class.
/// When the object is serialized out as xml, it's qualified name is mso14:toggleButton.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:toggleButton")]
pub struct BackstageMenuToggleButton {
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// getPressed
    /// Represents the following attribute in the schema: :getPressed
    #[xml(attr = "getPressed")]
    pub get_pressed: Option<String>,
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
/// Defines the BackstageGroupButton Class.
/// When the object is serialized out as xml, it's qualified name is mso14:button.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:button")]
pub struct BackstageGroupButton {
    /// expand
    /// Represents the following attribute in the schema: :expand
    #[xml(attr = "expand")]
    pub expand: Option<ExpandValues>,
    /// style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<Style2Values>,
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
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// isDefinitive
    /// Represents the following attribute in the schema: :isDefinitive
    #[xml(attr = "isDefinitive")]
    pub is_definitive: Option<bool>,
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
}
/// Defines the BackstageCheckBox Class.
/// When the object is serialized out as xml, it's qualified name is mso14:checkBox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:checkBox")]
pub struct BackstageCheckBox {
    /// expand
    /// Represents the following attribute in the schema: :expand
    #[xml(attr = "expand")]
    pub expand: Option<ExpandValues>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
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
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// getPressed
    /// Represents the following attribute in the schema: :getPressed
    #[xml(attr = "getPressed")]
    pub get_pressed: Option<String>,
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
/// Defines the BackstageEditBox Class.
/// When the object is serialized out as xml, it's qualified name is mso14:editBox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:editBox")]
pub struct BackstageEditBox {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// alignLabel
    /// Represents the following attribute in the schema: :alignLabel
    #[xml(attr = "alignLabel")]
    pub align_label: Option<ExpandValues>,
    /// expand
    /// Represents the following attribute in the schema: :expand
    #[xml(attr = "expand")]
    pub expand: Option<ExpandValues>,
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
    /// getText
    /// Represents the following attribute in the schema: :getText
    #[xml(attr = "getText")]
    pub get_text: Option<String>,
    /// onChange
    /// Represents the following attribute in the schema: :onChange
    #[xml(attr = "onChange")]
    pub on_change: Option<String>,
    /// maxLength
    /// Represents the following attribute in the schema: :maxLength
    #[xml(attr = "maxLength")]
    pub max_length: Option<i32>,
    /// sizeString
    /// Represents the following attribute in the schema: :sizeString
    #[xml(attr = "sizeString")]
    pub size_string: Option<String>,
}
/// Defines the BackstageDropDown Class.
/// When the object is serialized out as xml, it's qualified name is mso14:dropDown.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:dropDown")]
pub struct BackstageDropDown {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// alignLabel
    /// Represents the following attribute in the schema: :alignLabel
    #[xml(attr = "alignLabel")]
    pub align_label: Option<ExpandValues>,
    /// expand
    /// Represents the following attribute in the schema: :expand
    #[xml(attr = "expand")]
    pub expand: Option<ExpandValues>,
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
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
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
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// getSelectedItemIndex
    /// Represents the following attribute in the schema: :getSelectedItemIndex
    #[xml(attr = "getSelectedItemIndex")]
    pub get_selected_item_index: Option<String>,
    /// sizeString
    /// Represents the following attribute in the schema: :sizeString
    #[xml(attr = "sizeString")]
    pub size_string: Option<String>,
    /// getItemCount
    /// Represents the following attribute in the schema: :getItemCount
    #[xml(attr = "getItemCount")]
    pub get_item_count: Option<String>,
    /// getItemLabel
    /// Represents the following attribute in the schema: :getItemLabel
    #[xml(attr = "getItemLabel")]
    pub get_item_label: Option<String>,
    /// getItemID
    /// Represents the following attribute in the schema: :getItemID
    #[xml(attr = "getItemID")]
    pub get_item_id: Option<String>,
    /// _
    #[xml(child = "mso14:item")]
    pub mso14_item: Vec<ItemBackstageItem>,
}
/// Defines the RadioGroup Class.
/// When the object is serialized out as xml, it's qualified name is mso14:radioGroup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:radioGroup")]
pub struct RadioGroup {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// alignLabel
    /// Represents the following attribute in the schema: :alignLabel
    #[xml(attr = "alignLabel")]
    pub align_label: Option<ExpandValues>,
    /// expand
    /// Represents the following attribute in the schema: :expand
    #[xml(attr = "expand")]
    pub expand: Option<ExpandValues>,
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
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// getSelectedItemIndex
    /// Represents the following attribute in the schema: :getSelectedItemIndex
    #[xml(attr = "getSelectedItemIndex")]
    pub get_selected_item_index: Option<String>,
    /// getItemCount
    /// Represents the following attribute in the schema: :getItemCount
    #[xml(attr = "getItemCount")]
    pub get_item_count: Option<String>,
    /// getItemLabel
    /// Represents the following attribute in the schema: :getItemLabel
    #[xml(attr = "getItemLabel")]
    pub get_item_label: Option<String>,
    /// getItemID
    /// Represents the following attribute in the schema: :getItemID
    #[xml(attr = "getItemID")]
    pub get_item_id: Option<String>,
    /// _
    #[xml(child = "mso14:radioButton")]
    pub mso14_radio_button: Vec<RadioButtonBackstageItem>,
}
/// Defines the BackstageComboBox Class.
/// When the object is serialized out as xml, it's qualified name is mso14:comboBox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:comboBox")]
pub struct BackstageComboBox {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// alignLabel
    /// Represents the following attribute in the schema: :alignLabel
    #[xml(attr = "alignLabel")]
    pub align_label: Option<ExpandValues>,
    /// expand
    /// Represents the following attribute in the schema: :expand
    #[xml(attr = "expand")]
    pub expand: Option<ExpandValues>,
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
    /// getItemCount
    /// Represents the following attribute in the schema: :getItemCount
    #[xml(attr = "getItemCount")]
    pub get_item_count: Option<String>,
    /// getItemLabel
    /// Represents the following attribute in the schema: :getItemLabel
    #[xml(attr = "getItemLabel")]
    pub get_item_label: Option<String>,
    /// getItemID
    /// Represents the following attribute in the schema: :getItemID
    #[xml(attr = "getItemID")]
    pub get_item_id: Option<String>,
    /// _
    #[xml(child = "mso14:item")]
    pub mso14_item: Vec<ItemBackstageItem>,
}
/// Defines the Hyperlink Class.
/// When the object is serialized out as xml, it's qualified name is mso14:hyperlink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:hyperlink")]
pub struct Hyperlink {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// alignLabel
    /// Represents the following attribute in the schema: :alignLabel
    #[xml(attr = "alignLabel")]
    pub align_label: Option<ExpandValues>,
    /// expand
    /// Represents the following attribute in the schema: :expand
    #[xml(attr = "expand")]
    pub expand: Option<ExpandValues>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
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
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
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
    /// target
    /// Represents the following attribute in the schema: :target
    #[xml(attr = "target")]
    pub target: Option<String>,
    /// getTarget
    /// Represents the following attribute in the schema: :getTarget
    #[xml(attr = "getTarget")]
    pub get_target: Option<String>,
}
/// Defines the BackstageLabelControl Class.
/// When the object is serialized out as xml, it's qualified name is mso14:labelControl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:labelControl")]
pub struct BackstageLabelControl {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// alignLabel
    /// Represents the following attribute in the schema: :alignLabel
    #[xml(attr = "alignLabel")]
    pub align_label: Option<ExpandValues>,
    /// expand
    /// Represents the following attribute in the schema: :expand
    #[xml(attr = "expand")]
    pub expand: Option<ExpandValues>,
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
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// noWrap
    /// Represents the following attribute in the schema: :noWrap
    #[xml(attr = "noWrap")]
    pub no_wrap: Option<bool>,
}
/// Defines the GroupBox Class.
/// When the object is serialized out as xml, it's qualified name is mso14:groupBox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:groupBox")]
pub struct GroupBox {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// expand
    /// Represents the following attribute in the schema: :expand
    #[xml(attr = "expand")]
    pub expand: Option<ExpandValues>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    #[xml(
        child = "mso14:button",
        child = "mso14:checkBox",
        child = "mso14:editBox",
        child = "mso14:dropDown",
        child = "mso14:radioGroup",
        child = "mso14:comboBox",
        child = "mso14:hyperlink",
        child = "mso14:labelControl",
        child = "mso14:groupBox",
        child = "mso14:layoutContainer",
        child = "mso14:imageControl",
    )]
    pub children: Vec<GroupBoxChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GroupBoxChildChoice {
    #[xml(tag = "mso14:button")]
    Mso14Button(BackstageGroupButton),
    #[xml(tag = "mso14:checkBox")]
    Mso14CheckBox(BackstageCheckBox),
    #[xml(tag = "mso14:editBox")]
    Mso14EditBox(BackstageEditBox),
    #[xml(tag = "mso14:dropDown")]
    Mso14DropDown(BackstageDropDown),
    #[xml(tag = "mso14:radioGroup")]
    Mso14RadioGroup(RadioGroup),
    #[xml(tag = "mso14:comboBox")]
    Mso14ComboBox(BackstageComboBox),
    #[xml(tag = "mso14:hyperlink")]
    Mso14Hyperlink(Hyperlink),
    #[xml(tag = "mso14:labelControl")]
    Mso14LabelControl(BackstageLabelControl),
    #[xml(tag = "mso14:groupBox")]
    Mso14GroupBox(GroupBox),
    #[xml(tag = "mso14:layoutContainer")]
    Mso14LayoutContainer(LayoutContainer),
    #[xml(tag = "mso14:imageControl")]
    Mso14ImageControl(ImageControl),
}
/// Defines the LayoutContainer Class.
/// When the object is serialized out as xml, it's qualified name is mso14:layoutContainer.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:layoutContainer")]
pub struct LayoutContainer {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// align
    /// Represents the following attribute in the schema: :align
    #[xml(attr = "align")]
    pub align: Option<ExpandValues>,
    /// expand
    /// Represents the following attribute in the schema: :expand
    #[xml(attr = "expand")]
    pub expand: Option<ExpandValues>,
    /// layoutChildren
    /// Represents the following attribute in the schema: :layoutChildren
    #[xml(attr = "layoutChildren")]
    pub layout_children: Option<LayoutChildrenValues>,
    #[xml(
        child = "mso14:button",
        child = "mso14:checkBox",
        child = "mso14:editBox",
        child = "mso14:dropDown",
        child = "mso14:radioGroup",
        child = "mso14:comboBox",
        child = "mso14:hyperlink",
        child = "mso14:labelControl",
        child = "mso14:groupBox",
        child = "mso14:layoutContainer",
        child = "mso14:imageControl",
    )]
    pub children: Vec<LayoutContainerChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LayoutContainerChildChoice {
    #[xml(tag = "mso14:button")]
    Mso14Button(BackstageGroupButton),
    #[xml(tag = "mso14:checkBox")]
    Mso14CheckBox(BackstageCheckBox),
    #[xml(tag = "mso14:editBox")]
    Mso14EditBox(BackstageEditBox),
    #[xml(tag = "mso14:dropDown")]
    Mso14DropDown(BackstageDropDown),
    #[xml(tag = "mso14:radioGroup")]
    Mso14RadioGroup(RadioGroup),
    #[xml(tag = "mso14:comboBox")]
    Mso14ComboBox(BackstageComboBox),
    #[xml(tag = "mso14:hyperlink")]
    Mso14Hyperlink(Hyperlink),
    #[xml(tag = "mso14:labelControl")]
    Mso14LabelControl(BackstageLabelControl),
    #[xml(tag = "mso14:groupBox")]
    Mso14GroupBox(GroupBox),
    #[xml(tag = "mso14:layoutContainer")]
    Mso14LayoutContainer(LayoutContainer),
    #[xml(tag = "mso14:imageControl")]
    Mso14ImageControl(ImageControl),
}
/// Defines the ImageControl Class.
/// When the object is serialized out as xml, it's qualified name is mso14:imageControl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:imageControl")]
pub struct ImageControl {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// getEnabled
    /// Represents the following attribute in the schema: :getEnabled
    #[xml(attr = "getEnabled")]
    pub get_enabled: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
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
    /// altText
    /// Represents the following attribute in the schema: :altText
    #[xml(attr = "altText")]
    pub alt_text: Option<String>,
    /// getAltText
    /// Represents the following attribute in the schema: :getAltText
    #[xml(attr = "getAltText")]
    pub get_alt_text: Option<String>,
}
/// Defines the BackstageGroup Class.
/// When the object is serialized out as xml, it's qualified name is mso14:group.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:group")]
pub struct BackstageGroup {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<StyleValues>,
    /// getStyle
    /// Represents the following attribute in the schema: :getStyle
    #[xml(attr = "getStyle")]
    pub get_style: Option<String>,
    /// helperText
    /// Represents the following attribute in the schema: :helperText
    #[xml(attr = "helperText")]
    pub helper_text: Option<String>,
    /// getHelperText
    /// Represents the following attribute in the schema: :getHelperText
    #[xml(attr = "getHelperText")]
    pub get_helper_text: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    #[xml(
        child = "mso14:primaryItem",
        child = "mso14:topItems",
        child = "mso14:bottomItems",
    )]
    pub children: Vec<BackstageGroupChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BackstageGroupChildChoice {
    #[xml(tag = "mso14:primaryItem")]
    Mso14PrimaryItem(PrimaryItem),
    #[xml(tag = "mso14:topItems")]
    Mso14TopItems(TopItemsGroupControls),
    #[xml(tag = "mso14:bottomItems")]
    Mso14BottomItems(BottomItemsGroupControls),
}
/// Defines the TaskGroup Class.
/// When the object is serialized out as xml, it's qualified name is mso14:taskGroup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:taskGroup")]
pub struct TaskGroup {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// helperText
    /// Represents the following attribute in the schema: :helperText
    #[xml(attr = "helperText")]
    pub helper_text: Option<String>,
    /// getHelperText
    /// Represents the following attribute in the schema: :getHelperText
    #[xml(attr = "getHelperText")]
    pub get_helper_text: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// allowedTaskSizes
    /// Represents the following attribute in the schema: :allowedTaskSizes
    #[xml(attr = "allowedTaskSizes")]
    pub allowed_task_sizes: Option<TaskSizesValues>,
    /// _
    #[xml(child = "mso14:category")]
    pub mso14_category: Vec<TaskGroupCategory>,
}
/// Defines the MenuRoot Class.
/// When the object is serialized out as xml, it's qualified name is mso14:menu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:menu")]
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
        child = "mso14:control",
        child = "mso14:button",
        child = "mso14:checkBox",
        child = "mso14:gallery",
        child = "mso14:toggleButton",
        child = "mso14:menuSeparator",
        child = "mso14:splitButton",
        child = "mso14:menu",
        child = "mso14:dynamicMenu",
    )]
    pub children: Vec<MenuRootChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MenuRootChildChoice {
    #[xml(tag = "mso14:control")]
    Mso14Control(ControlCloneRegular),
    #[xml(tag = "mso14:button")]
    Mso14Button(ButtonRegular),
    #[xml(tag = "mso14:checkBox")]
    Mso14CheckBox(CheckBox),
    #[xml(tag = "mso14:gallery")]
    Mso14Gallery(GalleryRegular),
    #[xml(tag = "mso14:toggleButton")]
    Mso14ToggleButton(ToggleButtonRegular),
    #[xml(tag = "mso14:menuSeparator")]
    Mso14MenuSeparator(MenuSeparator),
    #[xml(tag = "mso14:splitButton")]
    Mso14SplitButton(SplitButtonRegular),
    #[xml(tag = "mso14:menu")]
    Mso14Menu(MenuRegular),
    #[xml(tag = "mso14:dynamicMenu")]
    Mso14DynamicMenu(DynamicMenuRegular),
}
/// Defines the CustomUI Class.
/// When the object is serialized out as xml, it's qualified name is mso14:customUI.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:customUI")]
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
    #[xml(child = "mso14:commands")]
    pub commands: Option<Commands>,
    /// _
    #[xml(child = "mso14:ribbon")]
    pub ribbon: Option<Ribbon>,
    /// _
    #[xml(child = "mso14:backstage")]
    pub backstage: Option<Backstage>,
    /// _
    #[xml(child = "mso14:contextMenus")]
    pub context_menus: Option<ContextMenus>,
}
mod custom_ui_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/2009/07/customui")
    }
}
/// Defines the Item Class.
/// When the object is serialized out as xml, it's qualified name is mso14:item.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:item")]
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
/// When the object is serialized out as xml, it's qualified name is mso14:button.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:button")]
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
/// When the object is serialized out as xml, it's qualified name is mso14:toggleButton.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:toggleButton")]
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
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
/// Defines the Separator Class.
/// When the object is serialized out as xml, it's qualified name is mso14:separator.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:separator")]
pub struct Separator {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
}
/// Defines the DialogBoxLauncher Class.
/// When the object is serialized out as xml, it's qualified name is mso14:dialogBoxLauncher.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:dialogBoxLauncher")]
pub struct DialogBoxLauncher {
    /// _
    #[xml(child = "mso14:button")]
    pub button_regular: ButtonRegular,
}
/// Defines the Group Class.
/// When the object is serialized out as xml, it's qualified name is mso14:group.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:group")]
pub struct Group {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
    /// autoScale
    /// Represents the following attribute in the schema: :autoScale
    #[xml(attr = "autoScale")]
    pub auto_scale: Option<bool>,
    /// centerVertically
    /// Represents the following attribute in the schema: :centerVertically
    #[xml(attr = "centerVertically")]
    pub center_vertically: Option<bool>,
    #[xml(
        child = "mso14:control",
        child = "mso14:labelControl",
        child = "mso14:button",
        child = "mso14:toggleButton",
        child = "mso14:checkBox",
        child = "mso14:editBox",
        child = "mso14:comboBox",
        child = "mso14:dropDown",
        child = "mso14:gallery",
        child = "mso14:menu",
        child = "mso14:dynamicMenu",
        child = "mso14:splitButton",
        child = "mso14:box",
        child = "mso14:buttonGroup",
        child = "mso14:separator",
        child = "mso14:dialogBoxLauncher",
    )]
    pub children: Vec<GroupChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GroupChildChoice {
    #[xml(tag = "mso14:control")]
    Mso14Control(ControlClone),
    #[xml(tag = "mso14:labelControl")]
    Mso14LabelControl(LabelControl),
    #[xml(tag = "mso14:button")]
    Mso14Button(Button),
    #[xml(tag = "mso14:toggleButton")]
    Mso14ToggleButton(ToggleButton),
    #[xml(tag = "mso14:checkBox")]
    Mso14CheckBox(CheckBox),
    #[xml(tag = "mso14:editBox")]
    Mso14EditBox(EditBox),
    #[xml(tag = "mso14:comboBox")]
    Mso14ComboBox(ComboBox),
    #[xml(tag = "mso14:dropDown")]
    Mso14DropDown(DropDownRegular),
    #[xml(tag = "mso14:gallery")]
    Mso14Gallery(Gallery),
    #[xml(tag = "mso14:menu")]
    Mso14Menu(Menu),
    #[xml(tag = "mso14:dynamicMenu")]
    Mso14DynamicMenu(DynamicMenu),
    #[xml(tag = "mso14:splitButton")]
    Mso14SplitButton(SplitButton),
    #[xml(tag = "mso14:box")]
    Mso14Box(Box),
    #[xml(tag = "mso14:buttonGroup")]
    Mso14ButtonGroup(ButtonGroup),
    #[xml(tag = "mso14:separator")]
    Mso14Separator(Separator),
    #[xml(tag = "mso14:dialogBoxLauncher")]
    Mso14DialogBoxLauncher(DialogBoxLauncher),
}
/// Defines the ControlCloneQat Class.
/// When the object is serialized out as xml, it's qualified name is mso14:control.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:control")]
pub struct ControlCloneQat {
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
/// Defines the SharedControlsQatItems Class.
/// When the object is serialized out as xml, it's qualified name is mso14:sharedControls.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:sharedControls")]
pub struct SharedControlsQatItems {
    #[xml(child = "mso14:control", child = "mso14:button", child = "mso14:separator")]
    pub children: Vec<SharedControlsQatItemsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SharedControlsQatItemsChildChoice {
    #[xml(tag = "mso14:control")]
    Mso14Control(ControlCloneQat),
    #[xml(tag = "mso14:button")]
    Mso14Button(ButtonRegular),
    #[xml(tag = "mso14:separator")]
    Mso14Separator(Separator),
}
/// Defines the DocumentControlsQatItems Class.
/// When the object is serialized out as xml, it's qualified name is mso14:documentControls.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:documentControls")]
pub struct DocumentControlsQatItems {
    #[xml(child = "mso14:control", child = "mso14:button", child = "mso14:separator")]
    pub children: Vec<DocumentControlsQatItemsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DocumentControlsQatItemsChildChoice {
    #[xml(tag = "mso14:control")]
    Mso14Control(ControlCloneQat),
    #[xml(tag = "mso14:button")]
    Mso14Button(ButtonRegular),
    #[xml(tag = "mso14:separator")]
    Mso14Separator(Separator),
}
/// Defines the QatItemsType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct QatItemsType {
    #[xml(child = "mso14:control", child = "mso14:button", child = "mso14:separator")]
    pub children: Vec<QatItemsTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum QatItemsTypeChildChoice {
    #[xml(tag = "mso14:control")]
    Mso14Control(ControlCloneQat),
    #[xml(tag = "mso14:button")]
    Mso14Button(ButtonRegular),
    #[xml(tag = "mso14:separator")]
    Mso14Separator(Separator),
}
/// Defines the Tab Class.
/// When the object is serialized out as xml, it's qualified name is mso14:tab.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:tab")]
pub struct Tab {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
    #[xml(child = "mso14:group")]
    pub children: Vec<TabChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TabChildChoice {
    #[xml(tag = "mso14:group")]
    Mso14Group(Group),
}
/// Defines the TabSet Class.
/// When the object is serialized out as xml, it's qualified name is mso14:tabSet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:tabSet")]
pub struct TabSet {
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
    #[xml(child = "mso14:tab")]
    pub mso14_tab: Vec<Tab>,
}
/// Defines the Command Class.
/// When the object is serialized out as xml, it's qualified name is mso14:command.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:command")]
pub struct Command {
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
/// Defines the QuickAccessToolbar Class.
/// When the object is serialized out as xml, it's qualified name is mso14:qat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:qat")]
pub struct QuickAccessToolbar {
    /// _
    #[xml(child = "mso14:sharedControls")]
    pub shared_controls_qat_items: Option<SharedControlsQatItems>,
    /// _
    #[xml(child = "mso14:documentControls")]
    pub document_controls_qat_items: Option<DocumentControlsQatItems>,
}
/// Defines the Tabs Class.
/// When the object is serialized out as xml, it's qualified name is mso14:tabs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:tabs")]
pub struct Tabs {
    /// _
    #[xml(child = "mso14:tab")]
    pub mso14_tab: Vec<Tab>,
}
/// Defines the ContextualTabs Class.
/// When the object is serialized out as xml, it's qualified name is mso14:contextualTabs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:contextualTabs")]
pub struct ContextualTabs {
    /// _
    #[xml(child = "mso14:tabSet")]
    pub mso14_tab_set: Vec<TabSet>,
}
/// Defines the ContextMenu Class.
/// When the object is serialized out as xml, it's qualified name is mso14:contextMenu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:contextMenu")]
pub struct ContextMenu {
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    #[xml(
        child = "mso14:control",
        child = "mso14:button",
        child = "mso14:checkBox",
        child = "mso14:gallery",
        child = "mso14:toggleButton",
        child = "mso14:splitButton",
        child = "mso14:menu",
        child = "mso14:dynamicMenu",
        child = "mso14:menuSeparator",
    )]
    pub children: Vec<ContextMenuChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ContextMenuChildChoice {
    #[xml(tag = "mso14:control")]
    Mso14Control(ControlCloneRegular),
    #[xml(tag = "mso14:button")]
    Mso14Button(ButtonRegular),
    #[xml(tag = "mso14:checkBox")]
    Mso14CheckBox(CheckBox),
    #[xml(tag = "mso14:gallery")]
    Mso14Gallery(GalleryRegular),
    #[xml(tag = "mso14:toggleButton")]
    Mso14ToggleButton(ToggleButtonRegular),
    #[xml(tag = "mso14:splitButton")]
    Mso14SplitButton(SplitButtonRegular),
    #[xml(tag = "mso14:menu")]
    Mso14Menu(MenuRegular),
    #[xml(tag = "mso14:dynamicMenu")]
    Mso14DynamicMenu(DynamicMenuRegular),
    #[xml(tag = "mso14:menuSeparator")]
    Mso14MenuSeparator(MenuSeparatorNoTitle),
}
/// Defines the ItemBackstageItem Class.
/// When the object is serialized out as xml, it's qualified name is mso14:item.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:item")]
pub struct ItemBackstageItem {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
}
/// Defines the RadioButtonBackstageItem Class.
/// When the object is serialized out as xml, it's qualified name is mso14:radioButton.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:radioButton")]
pub struct RadioButtonBackstageItem {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
}
/// Defines the BackstageItemType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct BackstageItemType {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
}
/// Defines the BackstageRegularButton Class.
/// When the object is serialized out as xml, it's qualified name is mso14:button.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:button")]
pub struct BackstageRegularButton {
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
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// isDefinitive
    /// Represents the following attribute in the schema: :isDefinitive
    #[xml(attr = "isDefinitive")]
    pub is_definitive: Option<bool>,
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
}
/// Defines the BackstagePrimaryMenu Class.
/// When the object is serialized out as xml, it's qualified name is mso14:menu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:menu")]
pub struct BackstagePrimaryMenu {
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
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
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
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
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
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    #[xml(child = "mso14:menuGroup")]
    pub children: Vec<BackstagePrimaryMenuChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BackstagePrimaryMenuChildChoice {
    #[xml(tag = "mso14:menuGroup")]
    Mso14MenuGroup(BackstageMenuGroup),
}
/// Defines the BackstageMenuGroup Class.
/// When the object is serialized out as xml, it's qualified name is mso14:menuGroup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:menuGroup")]
pub struct BackstageMenuGroup {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
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
    /// itemSize
    /// Represents the following attribute in the schema: :itemSize
    #[xml(attr = "itemSize")]
    pub item_size: Option<ItemSizeValues>,
    #[xml(
        child = "mso14:button",
        child = "mso14:checkBox",
        child = "mso14:menu",
        child = "mso14:toggleButton",
    )]
    pub children: Vec<BackstageMenuGroupChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BackstageMenuGroupChildChoice {
    #[xml(tag = "mso14:button")]
    Mso14Button(BackstageMenuButton),
    #[xml(tag = "mso14:checkBox")]
    Mso14CheckBox(BackstageMenuCheckBox),
    #[xml(tag = "mso14:menu")]
    Mso14Menu(BackstageSubMenu),
    #[xml(tag = "mso14:toggleButton")]
    Mso14ToggleButton(BackstageMenuToggleButton),
}
/// Defines the PrimaryItem Class.
/// When the object is serialized out as xml, it's qualified name is mso14:primaryItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:primaryItem")]
pub struct PrimaryItem {
    #[xml(child = "mso14:button", child = "mso14:menu")]
    pub children: Vec<PrimaryItemChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PrimaryItemChildChoice {
    #[xml(tag = "mso14:button")]
    Mso14Button(BackstageRegularButton),
    #[xml(tag = "mso14:menu")]
    Mso14Menu(BackstagePrimaryMenu),
}
/// Defines the TopItemsGroupControls Class.
/// When the object is serialized out as xml, it's qualified name is mso14:topItems.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:topItems")]
pub struct TopItemsGroupControls {
    #[xml(
        child = "mso14:button",
        child = "mso14:checkBox",
        child = "mso14:editBox",
        child = "mso14:dropDown",
        child = "mso14:radioGroup",
        child = "mso14:comboBox",
        child = "mso14:hyperlink",
        child = "mso14:labelControl",
        child = "mso14:groupBox",
        child = "mso14:layoutContainer",
        child = "mso14:imageControl",
    )]
    pub children: Vec<TopItemsGroupControlsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TopItemsGroupControlsChildChoice {
    #[xml(tag = "mso14:button")]
    Mso14Button(BackstageGroupButton),
    #[xml(tag = "mso14:checkBox")]
    Mso14CheckBox(BackstageCheckBox),
    #[xml(tag = "mso14:editBox")]
    Mso14EditBox(BackstageEditBox),
    #[xml(tag = "mso14:dropDown")]
    Mso14DropDown(BackstageDropDown),
    #[xml(tag = "mso14:radioGroup")]
    Mso14RadioGroup(RadioGroup),
    #[xml(tag = "mso14:comboBox")]
    Mso14ComboBox(BackstageComboBox),
    #[xml(tag = "mso14:hyperlink")]
    Mso14Hyperlink(Hyperlink),
    #[xml(tag = "mso14:labelControl")]
    Mso14LabelControl(BackstageLabelControl),
    #[xml(tag = "mso14:groupBox")]
    Mso14GroupBox(GroupBox),
    #[xml(tag = "mso14:layoutContainer")]
    Mso14LayoutContainer(LayoutContainer),
    #[xml(tag = "mso14:imageControl")]
    Mso14ImageControl(ImageControl),
}
/// Defines the BottomItemsGroupControls Class.
/// When the object is serialized out as xml, it's qualified name is mso14:bottomItems.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:bottomItems")]
pub struct BottomItemsGroupControls {
    #[xml(
        child = "mso14:button",
        child = "mso14:checkBox",
        child = "mso14:editBox",
        child = "mso14:dropDown",
        child = "mso14:radioGroup",
        child = "mso14:comboBox",
        child = "mso14:hyperlink",
        child = "mso14:labelControl",
        child = "mso14:groupBox",
        child = "mso14:layoutContainer",
        child = "mso14:imageControl",
    )]
    pub children: Vec<BottomItemsGroupControlsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BottomItemsGroupControlsChildChoice {
    #[xml(tag = "mso14:button")]
    Mso14Button(BackstageGroupButton),
    #[xml(tag = "mso14:checkBox")]
    Mso14CheckBox(BackstageCheckBox),
    #[xml(tag = "mso14:editBox")]
    Mso14EditBox(BackstageEditBox),
    #[xml(tag = "mso14:dropDown")]
    Mso14DropDown(BackstageDropDown),
    #[xml(tag = "mso14:radioGroup")]
    Mso14RadioGroup(RadioGroup),
    #[xml(tag = "mso14:comboBox")]
    Mso14ComboBox(BackstageComboBox),
    #[xml(tag = "mso14:hyperlink")]
    Mso14Hyperlink(Hyperlink),
    #[xml(tag = "mso14:labelControl")]
    Mso14LabelControl(BackstageLabelControl),
    #[xml(tag = "mso14:groupBox")]
    Mso14GroupBox(GroupBox),
    #[xml(tag = "mso14:layoutContainer")]
    Mso14LayoutContainer(LayoutContainer),
    #[xml(tag = "mso14:imageControl")]
    Mso14ImageControl(ImageControl),
}
/// Defines the GroupControlsType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct GroupControlsType {
    #[xml(
        child = "mso14:button",
        child = "mso14:checkBox",
        child = "mso14:editBox",
        child = "mso14:dropDown",
        child = "mso14:radioGroup",
        child = "mso14:comboBox",
        child = "mso14:hyperlink",
        child = "mso14:labelControl",
        child = "mso14:groupBox",
        child = "mso14:layoutContainer",
        child = "mso14:imageControl",
    )]
    pub children: Vec<GroupControlsTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GroupControlsTypeChildChoice {
    #[xml(tag = "mso14:button")]
    Mso14Button(BackstageGroupButton),
    #[xml(tag = "mso14:checkBox")]
    Mso14CheckBox(BackstageCheckBox),
    #[xml(tag = "mso14:editBox")]
    Mso14EditBox(BackstageEditBox),
    #[xml(tag = "mso14:dropDown")]
    Mso14DropDown(BackstageDropDown),
    #[xml(tag = "mso14:radioGroup")]
    Mso14RadioGroup(RadioGroup),
    #[xml(tag = "mso14:comboBox")]
    Mso14ComboBox(BackstageComboBox),
    #[xml(tag = "mso14:hyperlink")]
    Mso14Hyperlink(Hyperlink),
    #[xml(tag = "mso14:labelControl")]
    Mso14LabelControl(BackstageLabelControl),
    #[xml(tag = "mso14:groupBox")]
    Mso14GroupBox(GroupBox),
    #[xml(tag = "mso14:layoutContainer")]
    Mso14LayoutContainer(LayoutContainer),
    #[xml(tag = "mso14:imageControl")]
    Mso14ImageControl(ImageControl),
}
/// Defines the TaskGroupCategory Class.
/// When the object is serialized out as xml, it's qualified name is mso14:category.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:category")]
pub struct TaskGroupCategory {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// _
    #[xml(child = "mso14:task")]
    pub mso14_task: Vec<TaskGroupTask>,
}
/// Defines the TaskGroupTask Class.
/// When the object is serialized out as xml, it's qualified name is mso14:task.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:task")]
pub struct TaskGroupTask {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// isDefinitive
    /// Represents the following attribute in the schema: :isDefinitive
    #[xml(attr = "isDefinitive")]
    pub is_definitive: Option<bool>,
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
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
}
/// Defines the TaskFormGroupCategory Class.
/// When the object is serialized out as xml, it's qualified name is mso14:category.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:category")]
pub struct TaskFormGroupCategory {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// _
    #[xml(child = "mso14:task")]
    pub mso14_task: Vec<TaskFormGroupTask>,
}
/// Defines the TaskFormGroupTask Class.
/// When the object is serialized out as xml, it's qualified name is mso14:task.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:task")]
pub struct TaskFormGroupTask {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// getDescription
    /// Represents the following attribute in the schema: :getDescription
    #[xml(attr = "getDescription")]
    pub get_description: Option<String>,
    /// keytip
    /// Represents the following attribute in the schema: :keytip
    #[xml(attr = "keytip")]
    pub keytip: Option<String>,
    /// getKeytip
    /// Represents the following attribute in the schema: :getKeytip
    #[xml(attr = "getKeytip")]
    pub get_keytip: Option<String>,
    /// _
    #[xml(child = "mso14:group")]
    pub mso14_group: Vec<BackstageGroup>,
}
/// Defines the TaskFormGroup Class.
/// When the object is serialized out as xml, it's qualified name is mso14:taskFormGroup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:taskFormGroup")]
pub struct TaskFormGroup {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<String>,
    /// getLabel
    /// Represents the following attribute in the schema: :getLabel
    #[xml(attr = "getLabel")]
    pub get_label: Option<String>,
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    /// getVisible
    /// Represents the following attribute in the schema: :getVisible
    #[xml(attr = "getVisible")]
    pub get_visible: Option<String>,
    /// helperText
    /// Represents the following attribute in the schema: :helperText
    #[xml(attr = "helperText")]
    pub helper_text: Option<String>,
    /// getHelperText
    /// Represents the following attribute in the schema: :getHelperText
    #[xml(attr = "getHelperText")]
    pub get_helper_text: Option<String>,
    /// showLabel
    /// Represents the following attribute in the schema: :showLabel
    #[xml(attr = "showLabel")]
    pub show_label: Option<bool>,
    /// getShowLabel
    /// Represents the following attribute in the schema: :getShowLabel
    #[xml(attr = "getShowLabel")]
    pub get_show_label: Option<String>,
    /// allowedTaskSizes
    /// Represents the following attribute in the schema: :allowedTaskSizes
    #[xml(attr = "allowedTaskSizes")]
    pub allowed_task_sizes: Option<TaskSizesValues>,
    /// _
    #[xml(child = "mso14:category")]
    pub mso14_category: Vec<TaskFormGroupCategory>,
}
/// Defines the BackstageGroups Class.
/// When the object is serialized out as xml, it's qualified name is mso14:firstColumn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:firstColumn")]
pub struct BackstageGroups {
    #[xml(
        child = "mso14:taskFormGroup",
        child = "mso14:group",
        child = "mso14:taskGroup",
    )]
    pub children: Vec<BackstageGroupsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BackstageGroupsChildChoice {
    #[xml(tag = "mso14:taskFormGroup")]
    Mso14TaskFormGroup(TaskFormGroup),
    #[xml(tag = "mso14:group")]
    Mso14Group(BackstageGroup),
    #[xml(tag = "mso14:taskGroup")]
    Mso14TaskGroup(TaskGroup),
}
/// Defines the SimpleGroups Class.
/// When the object is serialized out as xml, it's qualified name is mso14:secondColumn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:secondColumn")]
pub struct SimpleGroups {
    #[xml(child = "mso14:group", child = "mso14:taskGroup")]
    pub children: Vec<SimpleGroupsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SimpleGroupsChildChoice {
    #[xml(tag = "mso14:group")]
    Mso14Group(BackstageGroup),
    #[xml(tag = "mso14:taskGroup")]
    Mso14TaskGroup(TaskGroup),
}
/// Defines the BackstageTab Class.
/// When the object is serialized out as xml, it's qualified name is mso14:tab.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:tab")]
pub struct BackstageTab {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
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
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// getTitle
    /// Represents the following attribute in the schema: :getTitle
    #[xml(attr = "getTitle")]
    pub get_title: Option<String>,
    /// columnWidthPercent
    /// Represents the following attribute in the schema: :columnWidthPercent
    #[xml(attr = "columnWidthPercent")]
    pub column_width_percent: Option<i32>,
    /// firstColumnMinWidth
    /// Represents the following attribute in the schema: :firstColumnMinWidth
    #[xml(attr = "firstColumnMinWidth")]
    pub first_column_min_width: Option<i32>,
    /// firstColumnMaxWidth
    /// Represents the following attribute in the schema: :firstColumnMaxWidth
    #[xml(attr = "firstColumnMaxWidth")]
    pub first_column_max_width: Option<i32>,
    /// secondColumnMinWidth
    /// Represents the following attribute in the schema: :secondColumnMinWidth
    #[xml(attr = "secondColumnMinWidth")]
    pub second_column_min_width: Option<i32>,
    /// secondColumnMaxWidth
    /// Represents the following attribute in the schema: :secondColumnMaxWidth
    #[xml(attr = "secondColumnMaxWidth")]
    pub second_column_max_width: Option<i32>,
    /// _
    #[xml(child = "mso14:firstColumn")]
    pub backstage_groups: Option<BackstageGroups>,
    /// _
    #[xml(child = "mso14:secondColumn")]
    pub simple_groups: Option<SimpleGroups>,
}
/// Defines the BackstageFastCommandButton Class.
/// When the object is serialized out as xml, it's qualified name is mso14:button.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:button")]
pub struct BackstageFastCommandButton {
    /// idMso
    /// Represents the following attribute in the schema: :idMso
    #[xml(attr = "idMso")]
    pub id_mso: Option<String>,
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
    pub insert_after_qulified_id: Option<String>,
    /// insertBeforeQ
    /// Represents the following attribute in the schema: :insertBeforeQ
    #[xml(attr = "insertBeforeQ")]
    pub insert_before_qulified_id: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// idQ
    /// Represents the following attribute in the schema: :idQ
    #[xml(attr = "idQ")]
    pub qualified_id: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// onAction
    /// Represents the following attribute in the schema: :onAction
    #[xml(attr = "onAction")]
    pub on_action: Option<String>,
    /// isDefinitive
    /// Represents the following attribute in the schema: :isDefinitive
    #[xml(attr = "isDefinitive")]
    pub is_definitive: Option<bool>,
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
}
/// Defines the Commands Class.
/// When the object is serialized out as xml, it's qualified name is mso14:commands.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:commands")]
pub struct Commands {
    /// _
    #[xml(child = "mso14:command")]
    pub mso14_command: Vec<Command>,
}
/// Defines the Ribbon Class.
/// When the object is serialized out as xml, it's qualified name is mso14:ribbon.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:ribbon")]
pub struct Ribbon {
    /// startFromScratch
    /// Represents the following attribute in the schema: :startFromScratch
    #[xml(attr = "startFromScratch")]
    pub start_from_scratch: Option<bool>,
    #[xml(child = "mso14:qat", child = "mso14:tabs", child = "mso14:contextualTabs")]
    pub children: Vec<RibbonChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RibbonChildChoice {
    #[xml(tag = "mso14:qat")]
    Mso14Qat(QuickAccessToolbar),
    #[xml(tag = "mso14:tabs")]
    Mso14Tabs(Tabs),
    #[xml(tag = "mso14:contextualTabs")]
    Mso14ContextualTabs(ContextualTabs),
}
/// Defines the Backstage Class.
/// When the object is serialized out as xml, it's qualified name is mso14:backstage.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:backstage")]
pub struct Backstage {
    /// onShow
    /// Represents the following attribute in the schema: :onShow
    #[xml(attr = "onShow")]
    pub on_show: Option<String>,
    /// onHide
    /// Represents the following attribute in the schema: :onHide
    #[xml(attr = "onHide")]
    pub on_hide: Option<String>,
    #[xml(child = "mso14:tab", child = "mso14:button")]
    pub children: Vec<BackstageChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BackstageChildChoice {
    #[xml(tag = "mso14:tab")]
    Mso14Tab(BackstageTab),
    #[xml(tag = "mso14:button")]
    Mso14Button(BackstageFastCommandButton),
}
/// Defines the ContextMenus Class.
/// When the object is serialized out as xml, it's qualified name is mso14:contextMenus.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "mso14:contextMenus")]
pub struct ContextMenus {
    /// _
    #[xml(child = "mso14:contextMenu")]
    pub mso14_context_menu: Vec<ContextMenu>,
}
