#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ClipboardFormatValues {
    #[default]
    PictureOld,
    Picture,
    Bitmap,
    PicturePrint,
    PictureScreen,
}
crate::__string_enum! {
    ClipboardFormatValues { PictureOld = "pictOld", Picture = "pict", Bitmap = "bitmap",
    PicturePrint = "pictPrint", PictureScreen = "pictScreen", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ObjectValues {
    #[default]
    Button,
    Checkbox,
    Dialog,
    Drop,
    Edit,
    GroupBox,
    Label,
    AuditingLine,
    List,
    Movie,
    Note,
    Picture,
    Radio,
    AuditingRectangle,
    Scroll,
    Spin,
    Shape,
    Group,
    Rectangle,
}
crate::__string_enum! {
    ObjectValues { Button = "button", Checkbox = "checkbox", Dialog = "dialog", Drop =
    "drop", Edit = "edit", GroupBox = "gBox", Label = "label", AuditingLine = "lineA",
    List = "list", Movie = "movie", Note = "note", Picture = "pict", Radio = "radio",
    AuditingRectangle = "rectA", Scroll = "scroll", Spin = "spin", Shape = "shape", Group
    = "group", Rectangle = "rect", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BooleanEntryWithBlankValues {
    #[default]
    True,
    T,
    False,
    F,
    Empty,
}
crate::__string_enum! {
    BooleanEntryWithBlankValues { True = "true", T = "t", False = "false", F = "f", Empty
    = "", }
}
/// Attached Object Data.
/// When the object is serialized out as xml, it's qualified name is xvml:ClientData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:ClientData")]
pub struct ClientData {
    /// Object type
    /// Represents the following attribute in the schema: :ObjectType
    #[xml(attr = "ObjectType")]
    pub object_type: ObjectValues,
    #[xml(
        child = "xvml:MoveWithCells",
        child = "xvml:SizeWithCells",
        child = "xvml:Anchor",
        child = "xvml:Locked",
        child = "xvml:DefaultSize",
        child = "xvml:PrintObject",
        child = "xvml:Disabled",
        child = "xvml:AutoFill",
        child = "xvml:AutoLine",
        child = "xvml:AutoPict",
        child = "xvml:FmlaMacro",
        child = "xvml:TextHAlign",
        child = "xvml:TextVAlign",
        child = "xvml:LockText",
        child = "xvml:JustLastX",
        child = "xvml:SecretEdit",
        child = "xvml:Default",
        child = "xvml:Help",
        child = "xvml:Cancel",
        child = "xvml:Dismiss",
        child = "xvml:Accel",
        child = "xvml:Accel2",
        child = "xvml:Row",
        child = "xvml:Column",
        child = "xvml:Visible",
        child = "xvml:RowHidden",
        child = "xvml:ColHidden",
        child = "xvml:VTEdit",
        child = "xvml:MultiLine",
        child = "xvml:VScroll",
        child = "xvml:ValidIds",
        child = "xvml:FmlaRange",
        child = "xvml:WidthMin",
        child = "xvml:Sel",
        child = "xvml:NoThreeD2",
        child = "xvml:SelType",
        child = "xvml:MultiSel",
        child = "xvml:LCT",
        child = "xvml:ListItem",
        child = "xvml:DropStyle",
        child = "xvml:Colored",
        child = "xvml:DropLines",
        child = "xvml:Checked",
        child = "xvml:FmlaLink",
        child = "xvml:FmlaPict",
        child = "xvml:NoThreeD",
        child = "xvml:FirstButton",
        child = "xvml:FmlaGroup",
        child = "xvml:Val",
        child = "xvml:Min",
        child = "xvml:Max",
        child = "xvml:Inc",
        child = "xvml:Page",
        child = "xvml:Horiz",
        child = "xvml:Dx",
        child = "xvml:MapOCX",
        child = "xvml:CF",
        child = "xvml:Camera",
        child = "xvml:RecalcAlways",
        child = "xvml:AutoScale",
        child = "xvml:DDE",
        child = "xvml:UIObj",
        child = "xvml:ScriptText",
        child = "xvml:ScriptExtended",
        child = "xvml:ScriptLanguage",
        child = "xvml:ScriptLocation",
        child = "xvml:FmlaTxbx",
    )]
    pub children: Vec<ClientDataChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ClientDataChildChoice {
    #[xml(tag = "xvml:MoveWithCells")]
    XvmlMoveWithCells(MoveWithCells),
    #[xml(tag = "xvml:SizeWithCells")]
    XvmlSizeWithCells(ResizeWithCells),
    #[xml(tag = "xvml:Anchor")]
    XvmlAnchor(Anchor),
    #[xml(tag = "xvml:Locked")]
    XvmlLocked(Locked),
    #[xml(tag = "xvml:DefaultSize")]
    XvmlDefaultSize(DefaultSize),
    #[xml(tag = "xvml:PrintObject")]
    XvmlPrintObject(PrintObject),
    #[xml(tag = "xvml:Disabled")]
    XvmlDisabled(Disabled),
    #[xml(tag = "xvml:AutoFill")]
    XvmlAutoFill(AutoFill),
    #[xml(tag = "xvml:AutoLine")]
    XvmlAutoLine(AutoLine),
    #[xml(tag = "xvml:AutoPict")]
    XvmlAutoPict(AutoSizePicture),
    #[xml(tag = "xvml:FmlaMacro")]
    XvmlFmlaMacro(FormulaMacro),
    #[xml(tag = "xvml:TextHAlign")]
    XvmlTextHAlign(HorizontalTextAlignment),
    #[xml(tag = "xvml:TextVAlign")]
    XvmlTextVAlign(VerticalTextAlignment),
    #[xml(tag = "xvml:LockText")]
    XvmlLockText(LockText),
    #[xml(tag = "xvml:JustLastX")]
    XvmlJustLastX(JustifyLastLine),
    #[xml(tag = "xvml:SecretEdit")]
    XvmlSecretEdit(SecretEdit),
    #[xml(tag = "xvml:Default")]
    XvmlDefault(DefaultButton),
    #[xml(tag = "xvml:Help")]
    XvmlHelp(HelpButton),
    #[xml(tag = "xvml:Cancel")]
    XvmlCancel(CancelButton),
    #[xml(tag = "xvml:Dismiss")]
    XvmlDismiss(DismissButton),
    #[xml(tag = "xvml:Accel")]
    XvmlAccel(AcceleratorPrimary),
    #[xml(tag = "xvml:Accel2")]
    XvmlAccel2(AcceleratorSecondary),
    #[xml(tag = "xvml:Row")]
    XvmlRow(CommentRowTarget),
    #[xml(tag = "xvml:Column")]
    XvmlColumn(CommentColumnTarget),
    #[xml(tag = "xvml:Visible")]
    XvmlVisible(Visible),
    #[xml(tag = "xvml:RowHidden")]
    XvmlRowHidden(RowHidden),
    #[xml(tag = "xvml:ColHidden")]
    XvmlColHidden(ColumnHidden),
    #[xml(tag = "xvml:VTEdit")]
    XvmlVtEdit(InputValidationType),
    #[xml(tag = "xvml:MultiLine")]
    XvmlMultiLine(MultiLine),
    #[xml(tag = "xvml:VScroll")]
    XvmlVScroll(VerticalScrollBar),
    #[xml(tag = "xvml:ValidIds")]
    XvmlValidIds(ValidIds),
    #[xml(tag = "xvml:FmlaRange")]
    XvmlFmlaRange(FormulaRange),
    #[xml(tag = "xvml:WidthMin")]
    XvmlWidthMin(MinDropDownWidth),
    #[xml(tag = "xvml:Sel")]
    XvmlSel(SelectionEntry),
    #[xml(tag = "xvml:NoThreeD2")]
    XvmlNoThreeD2(Disable3DForListBoxAndDropDown),
    #[xml(tag = "xvml:SelType")]
    XvmlSelType(SelectionType),
    #[xml(tag = "xvml:MultiSel")]
    XvmlMultiSel(MultiSelections),
    #[xml(tag = "xvml:LCT")]
    XvmlLct(ListBoxCallbackType),
    #[xml(tag = "xvml:ListItem")]
    XvmlListItem(ListItem),
    #[xml(tag = "xvml:DropStyle")]
    XvmlDropStyle(DropStyle),
    #[xml(tag = "xvml:Colored")]
    XvmlColored(Colored),
    #[xml(tag = "xvml:DropLines")]
    XvmlDropLines(DropLines),
    #[xml(tag = "xvml:Checked")]
    XvmlChecked(Checked),
    #[xml(tag = "xvml:FmlaLink")]
    XvmlFmlaLink(FormulaLink),
    #[xml(tag = "xvml:FmlaPict")]
    XvmlFmlaPict(FormulaPicture),
    #[xml(tag = "xvml:NoThreeD")]
    XvmlNoThreeD(Disable3D),
    #[xml(tag = "xvml:FirstButton")]
    XvmlFirstButton(FirstButton),
    #[xml(tag = "xvml:FmlaGroup")]
    XvmlFmlaGroup(FormulaGroup),
    #[xml(tag = "xvml:Val")]
    XvmlVal(ScrollBarPosition),
    #[xml(tag = "xvml:Min")]
    XvmlMin(ScrollBarMin),
    #[xml(tag = "xvml:Max")]
    XvmlMax(ScrollBarMax),
    #[xml(tag = "xvml:Inc")]
    XvmlInc(ScrollBarIncrement),
    #[xml(tag = "xvml:Page")]
    XvmlPage(ScrollBarPageIncrement),
    #[xml(tag = "xvml:Horiz")]
    XvmlHoriz(HorizontalScrollBar),
    #[xml(tag = "xvml:Dx")]
    XvmlDx(ScrollBarWidth),
    #[xml(tag = "xvml:MapOCX")]
    XvmlMapOcx(MapOcxControl),
    #[xml(tag = "xvml:CF")]
    XvmlCf(ClipboardFormat),
    #[xml(tag = "xvml:Camera")]
    XvmlCamera(CameraObject),
    #[xml(tag = "xvml:RecalcAlways")]
    XvmlRecalcAlways(RecalculateAlways),
    #[xml(tag = "xvml:AutoScale")]
    XvmlAutoScale(AutoScaleFont),
    #[xml(tag = "xvml:DDE")]
    XvmlDde(DdeObject),
    #[xml(tag = "xvml:UIObj")]
    XvmlUiObj(UiObject),
    #[xml(tag = "xvml:ScriptText")]
    XvmlScriptText(ScriptText),
    #[xml(tag = "xvml:ScriptExtended")]
    XvmlScriptExtended(ScriptExtended),
    #[xml(tag = "xvml:ScriptLanguage")]
    XvmlScriptLanguage(ScriptLanguage),
    #[xml(tag = "xvml:ScriptLocation")]
    XvmlScriptLocation(ScriptLocation),
    #[xml(tag = "xvml:FmlaTxbx")]
    XvmlFmlaTxbx(FormulaTextBox),
}
/// Move with Cells.
/// When the object is serialized out as xml, it's qualified name is xvml:MoveWithCells.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:MoveWithCells")]
pub struct MoveWithCells {
    #[xml(attr = "xvml:MoveWithCells")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Resize with Cells.
/// When the object is serialized out as xml, it's qualified name is xvml:SizeWithCells.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:SizeWithCells")]
pub struct ResizeWithCells {
    #[xml(attr = "xvml:SizeWithCells")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Lock Toggle.
/// When the object is serialized out as xml, it's qualified name is xvml:Locked.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Locked")]
pub struct Locked {
    #[xml(attr = "xvml:Locked")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Default Size Toggle.
/// When the object is serialized out as xml, it's qualified name is xvml:DefaultSize.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:DefaultSize")]
pub struct DefaultSize {
    #[xml(attr = "xvml:DefaultSize")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Print Toggle.
/// When the object is serialized out as xml, it's qualified name is xvml:PrintObject.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:PrintObject")]
pub struct PrintObject {
    #[xml(attr = "xvml:PrintObject")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Macro Disable Toggle.
/// When the object is serialized out as xml, it's qualified name is xvml:Disabled.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Disabled")]
pub struct Disabled {
    #[xml(attr = "xvml:Disabled")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// AutoFill.
/// When the object is serialized out as xml, it's qualified name is xvml:AutoFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:AutoFill")]
pub struct AutoFill {
    #[xml(attr = "xvml:AutoFill")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// AutoLine.
/// When the object is serialized out as xml, it's qualified name is xvml:AutoLine.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:AutoLine")]
pub struct AutoLine {
    #[xml(attr = "xvml:AutoLine")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Automatically Size.
/// When the object is serialized out as xml, it's qualified name is xvml:AutoPict.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:AutoPict")]
pub struct AutoSizePicture {
    #[xml(attr = "xvml:AutoPict")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Text Lock.
/// When the object is serialized out as xml, it's qualified name is xvml:LockText.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:LockText")]
pub struct LockText {
    #[xml(attr = "xvml:LockText")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// East Asia Alignment Toggle.
/// When the object is serialized out as xml, it's qualified name is xvml:JustLastX.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:JustLastX")]
pub struct JustifyLastLine {
    #[xml(attr = "xvml:JustLastX")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Password Edit.
/// When the object is serialized out as xml, it's qualified name is xvml:SecretEdit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:SecretEdit")]
pub struct SecretEdit {
    #[xml(attr = "xvml:SecretEdit")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Default Button.
/// When the object is serialized out as xml, it's qualified name is xvml:Default.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Default")]
pub struct DefaultButton {
    #[xml(attr = "xvml:Default")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Help Button.
/// When the object is serialized out as xml, it's qualified name is xvml:Help.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Help")]
pub struct HelpButton {
    #[xml(attr = "xvml:Help")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Cancel Button.
/// When the object is serialized out as xml, it's qualified name is xvml:Cancel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Cancel")]
pub struct CancelButton {
    #[xml(attr = "xvml:Cancel")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Dismiss Button.
/// When the object is serialized out as xml, it's qualified name is xvml:Dismiss.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Dismiss")]
pub struct DismissButton {
    #[xml(attr = "xvml:Dismiss")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Comment Visibility Toggle.
/// When the object is serialized out as xml, it's qualified name is xvml:Visible.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Visible")]
pub struct Visible {
    #[xml(attr = "xvml:Visible")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Comment's Row is Hidden.
/// When the object is serialized out as xml, it's qualified name is xvml:RowHidden.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:RowHidden")]
pub struct RowHidden {
    #[xml(attr = "xvml:RowHidden")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Comment's Column is Hidden.
/// When the object is serialized out as xml, it's qualified name is xvml:ColHidden.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:ColHidden")]
pub struct ColumnHidden {
    #[xml(attr = "xvml:ColHidden")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Multi-line.
/// When the object is serialized out as xml, it's qualified name is xvml:MultiLine.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:MultiLine")]
pub struct MultiLine {
    #[xml(attr = "xvml:MultiLine")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Vertical Scroll.
/// When the object is serialized out as xml, it's qualified name is xvml:VScroll.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:VScroll")]
pub struct VerticalScrollBar {
    #[xml(attr = "xvml:VScroll")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Valid ID.
/// When the object is serialized out as xml, it's qualified name is xvml:ValidIds.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:ValidIds")]
pub struct ValidIds {
    #[xml(attr = "xvml:ValidIds")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Disable 3D.
/// When the object is serialized out as xml, it's qualified name is xvml:NoThreeD2.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:NoThreeD2")]
pub struct Disable3DForListBoxAndDropDown {
    #[xml(attr = "xvml:NoThreeD2")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Dropdown Color Toggle.
/// When the object is serialized out as xml, it's qualified name is xvml:Colored.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Colored")]
pub struct Colored {
    #[xml(attr = "xvml:Colored")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Disable 3D.
/// When the object is serialized out as xml, it's qualified name is xvml:NoThreeD.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:NoThreeD")]
pub struct Disable3D {
    #[xml(attr = "xvml:NoThreeD")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// First Radio Button.
/// When the object is serialized out as xml, it's qualified name is xvml:FirstButton.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:FirstButton")]
pub struct FirstButton {
    #[xml(attr = "xvml:FirstButton")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Scroll Bar Orientation.
/// When the object is serialized out as xml, it's qualified name is xvml:Horiz.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Horiz")]
pub struct HorizontalScrollBar {
    #[xml(attr = "xvml:Horiz")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// ActiveX Control.
/// When the object is serialized out as xml, it's qualified name is xvml:MapOCX.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:MapOCX")]
pub struct MapOcxControl {
    #[xml(attr = "xvml:MapOCX")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Camera Tool.
/// When the object is serialized out as xml, it's qualified name is xvml:Camera.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Camera")]
pub struct CameraObject {
    #[xml(attr = "xvml:Camera")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Recalculation Toggle.
/// When the object is serialized out as xml, it's qualified name is xvml:RecalcAlways.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:RecalcAlways")]
pub struct RecalculateAlways {
    #[xml(attr = "xvml:RecalcAlways")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Font AutoScale.
/// When the object is serialized out as xml, it's qualified name is xvml:AutoScale.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:AutoScale")]
pub struct AutoScaleFont {
    #[xml(attr = "xvml:AutoScale")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Dynamic Data Exchange.
/// When the object is serialized out as xml, it's qualified name is xvml:DDE.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:DDE")]
pub struct DdeObject {
    #[xml(attr = "xvml:DDE")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// UI Object Toggle.
/// When the object is serialized out as xml, it's qualified name is xvml:UIObj.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:UIObj")]
pub struct UiObject {
    #[xml(attr = "xvml:UIObj")]
    pub child: Option<BooleanEntryWithBlankValues>,
}
/// Anchor.
/// When the object is serialized out as xml, it's qualified name is xvml:Anchor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Anchor")]
pub struct Anchor {
    #[xml(text)]
    pub child: String,
}
/// Horizontal Text Alignment.
/// When the object is serialized out as xml, it's qualified name is xvml:TextHAlign.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:TextHAlign")]
pub struct HorizontalTextAlignment {
    #[xml(text)]
    pub child: String,
}
/// Vertical Text Alignment.
/// When the object is serialized out as xml, it's qualified name is xvml:TextVAlign.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:TextVAlign")]
pub struct VerticalTextAlignment {
    #[xml(text)]
    pub child: String,
}
/// List Items Source Range.
/// When the object is serialized out as xml, it's qualified name is xvml:FmlaRange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:FmlaRange")]
pub struct FormulaRange {
    #[xml(text)]
    pub child: String,
}
/// Selection Type.
/// When the object is serialized out as xml, it's qualified name is xvml:SelType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:SelType")]
pub struct SelectionType {
    #[xml(text)]
    pub child: String,
}
/// Multiple Selections.
/// When the object is serialized out as xml, it's qualified name is xvml:MultiSel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:MultiSel")]
pub struct MultiSelections {
    #[xml(text)]
    pub child: String,
}
/// Callback Type.
/// When the object is serialized out as xml, it's qualified name is xvml:LCT.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:LCT")]
pub struct ListBoxCallbackType {
    #[xml(text)]
    pub child: String,
}
/// Non-linked List Item.
/// When the object is serialized out as xml, it's qualified name is xvml:ListItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:ListItem")]
pub struct ListItem {
    #[xml(text)]
    pub child: String,
}
/// Dropdown Style.
/// When the object is serialized out as xml, it's qualified name is xvml:DropStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:DropStyle")]
pub struct DropStyle {
    #[xml(text)]
    pub child: String,
}
/// Linked Formula.
/// When the object is serialized out as xml, it's qualified name is xvml:FmlaLink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:FmlaLink")]
pub struct FormulaLink {
    #[xml(text)]
    pub child: String,
}
/// Camera Source Range.
/// When the object is serialized out as xml, it's qualified name is xvml:FmlaPict.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:FmlaPict")]
pub struct FormulaPicture {
    #[xml(text)]
    pub child: String,
}
/// Linked Formula - Group Box.
/// When the object is serialized out as xml, it's qualified name is xvml:FmlaGroup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:FmlaGroup")]
pub struct FormulaGroup {
    #[xml(text)]
    pub child: String,
}
/// HTML Script Text.
/// When the object is serialized out as xml, it's qualified name is xvml:ScriptText.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:ScriptText")]
pub struct ScriptText {
    #[xml(text)]
    pub child: String,
}
/// HTML Script Attributes.
/// When the object is serialized out as xml, it's qualified name is xvml:ScriptExtended.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:ScriptExtended")]
pub struct ScriptExtended {
    #[xml(text)]
    pub child: String,
}
/// Text Formula.
/// When the object is serialized out as xml, it's qualified name is xvml:FmlaTxbx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:FmlaTxbx")]
pub struct FormulaTextBox {
    #[xml(text)]
    pub child: String,
}
/// Reference to Custom Function.
/// When the object is serialized out as xml, it's qualified name is xvml:FmlaMacro.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:FmlaMacro")]
pub struct FormulaMacro {
    #[xml(text)]
    pub child: String,
}
/// Primary Keyboard Accelerator.
/// When the object is serialized out as xml, it's qualified name is xvml:Accel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Accel")]
pub struct AcceleratorPrimary {
    #[xml(text)]
    pub child: u8,
}
/// Secondary Keyboard Accelerator.
/// When the object is serialized out as xml, it's qualified name is xvml:Accel2.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Accel2")]
pub struct AcceleratorSecondary {
    #[xml(text)]
    pub child: u8,
}
/// Comment Row Target.
/// When the object is serialized out as xml, it's qualified name is xvml:Row.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Row")]
pub struct CommentRowTarget {
    #[xml(text)]
    pub child: i64,
}
/// Comment Column Target.
/// When the object is serialized out as xml, it's qualified name is xvml:Column.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Column")]
pub struct CommentColumnTarget {
    #[xml(text)]
    pub child: i64,
}
/// Validation Type.
/// When the object is serialized out as xml, it's qualified name is xvml:VTEdit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:VTEdit")]
pub struct InputValidationType {
    #[xml(text)]
    pub child: i64,
}
/// Minimum Width.
/// When the object is serialized out as xml, it's qualified name is xvml:WidthMin.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:WidthMin")]
pub struct MinDropDownWidth {
    #[xml(text)]
    pub child: i64,
}
/// Selected Entry.
/// When the object is serialized out as xml, it's qualified name is xvml:Sel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Sel")]
pub struct SelectionEntry {
    #[xml(text)]
    pub child: i64,
}
/// Dropdown Maximum Lines.
/// When the object is serialized out as xml, it's qualified name is xvml:DropLines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:DropLines")]
pub struct DropLines {
    #[xml(text)]
    pub child: i64,
}
/// Checked.
/// When the object is serialized out as xml, it's qualified name is xvml:Checked.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Checked")]
pub struct Checked {
    #[xml(text)]
    pub child: i64,
}
/// Scroll bar position.
/// When the object is serialized out as xml, it's qualified name is xvml:Val.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Val")]
pub struct ScrollBarPosition {
    #[xml(text)]
    pub child: i64,
}
/// Scroll Bar Minimum.
/// When the object is serialized out as xml, it's qualified name is xvml:Min.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Min")]
pub struct ScrollBarMin {
    #[xml(text)]
    pub child: i64,
}
/// Scroll Bar Maximum.
/// When the object is serialized out as xml, it's qualified name is xvml:Max.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Max")]
pub struct ScrollBarMax {
    #[xml(text)]
    pub child: i64,
}
/// Scroll Bar Increment.
/// When the object is serialized out as xml, it's qualified name is xvml:Inc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Inc")]
pub struct ScrollBarIncrement {
    #[xml(text)]
    pub child: i64,
}
/// Scroll Bar Page Increment.
/// When the object is serialized out as xml, it's qualified name is xvml:Page.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Page")]
pub struct ScrollBarPageIncrement {
    #[xml(text)]
    pub child: i64,
}
/// Scroll Bar Width.
/// When the object is serialized out as xml, it's qualified name is xvml:Dx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:Dx")]
pub struct ScrollBarWidth {
    #[xml(text)]
    pub child: i64,
}
/// Clipboard Format.
/// When the object is serialized out as xml, it's qualified name is xvml:CF.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:CF")]
pub struct ClipboardFormat {
    #[xml(attr = "xvml:CF")]
    pub child: Option<ClipboardFormatValues>,
}
/// HTML Script Language.
/// When the object is serialized out as xml, it's qualified name is xvml:ScriptLanguage.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:ScriptLanguage")]
pub struct ScriptLanguage {
    #[xml(text)]
    pub child: i64,
}
/// HTML Script Location.
/// When the object is serialized out as xml, it's qualified name is xvml:ScriptLocation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xvml:ScriptLocation")]
pub struct ScriptLocation {
    #[xml(text)]
    pub child: i64,
}
