#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FilterOperatorValues {
    #[default]
    Equal,
    LessThan,
    LessThanOrEqual,
    NotEqual,
    GreaterThanOrEqual,
    GreaterThan,
}
crate::__string_enum! {
    FilterOperatorValues { Equal = "equal", LessThan = "lessThan", LessThanOrEqual =
    "lessThanOrEqual", NotEqual = "notEqual", GreaterThanOrEqual = "greaterThanOrEqual",
    GreaterThan = "greaterThan", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DynamicFilterValues {
    #[default]
    Null,
    AboveAverage,
    BelowAverage,
    Tomorrow,
    Today,
    Yesterday,
    NextWeek,
    ThisWeek,
    LastWeek,
    NextMonth,
    ThisMonth,
    LastMonth,
    NextQuarter,
    ThisQuarter,
    LastQuarter,
    NextYear,
    ThisYear,
    LastYear,
    YearToDate,
    Quarter1,
    Quarter2,
    Quarter3,
    Quarter4,
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
crate::__string_enum! {
    DynamicFilterValues { Null = "null", AboveAverage = "aboveAverage", BelowAverage =
    "belowAverage", Tomorrow = "tomorrow", Today = "today", Yesterday = "yesterday",
    NextWeek = "nextWeek", ThisWeek = "thisWeek", LastWeek = "lastWeek", NextMonth =
    "nextMonth", ThisMonth = "thisMonth", LastMonth = "lastMonth", NextQuarter =
    "nextQuarter", ThisQuarter = "thisQuarter", LastQuarter = "lastQuarter", NextYear =
    "nextYear", ThisYear = "thisYear", LastYear = "lastYear", YearToDate = "yearToDate",
    Quarter1 = "q1", Quarter2 = "q2", Quarter3 = "q3", Quarter4 = "q4", January = "m1",
    February = "m2", March = "m3", April = "m4", May = "m5", June = "m6", July = "m7",
    August = "m8", September = "m9", October = "m10", November = "m11", December = "m12",
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum IconSetValues {
    #[default]
    ThreeArrows,
    ThreeArrowsGray,
    ThreeFlags,
    ThreeTrafficLights1,
    ThreeTrafficLights2,
    ThreeSigns,
    ThreeSymbols,
    ThreeSymbols2,
    FourArrows,
    FourArrowsGray,
    FourRedToBlack,
    FourRating,
    FourTrafficLights,
    FiveArrows,
    FiveArrowsGray,
    FiveRating,
    FiveQuarters,
}
crate::__string_enum! {
    IconSetValues { ThreeArrows = "3arrows", ThreeArrowsGray = "3arrowsGray", ThreeFlags
    = "3flags", ThreeTrafficLights1 = "3trafficLights1", ThreeTrafficLights2 =
    "3trafficLights2", ThreeSigns = "3signs", ThreeSymbols = "3symbols", ThreeSymbols2 =
    "3symbols2", FourArrows = "4arrows", FourArrowsGray = "4arrowsGray", FourRedToBlack =
    "4redToBlack", FourRating = "4rating", FourTrafficLights = "4trafficLights",
    FiveArrows = "5arrows", FiveArrowsGray = "5arrowsGray", FiveRating = "5rating",
    FiveQuarters = "5quarters", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SortByValues {
    #[default]
    Value,
    CellColor,
    FontColor,
    Icon,
}
crate::__string_enum! {
    SortByValues { Value = "value", CellColor = "cellColor", FontColor = "fontColor",
    Icon = "icon", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SortMethodValues {
    #[default]
    Stroke,
    PinYin,
    None,
}
crate::__string_enum! {
    SortMethodValues { Stroke = "stroke", PinYin = "pinYin", None = "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CalendarValues {
    #[default]
    None,
    Gregorian,
    GregorianUs,
    Japan,
    Taiwan,
    Korea,
    Hijri,
    Thai,
    Hebrew,
    GregorianMiddleEastFrench,
    GregorianArabic,
    GregorianTransliteratedEnglish,
    GregorianTransliteratedFrench,
}
crate::__string_enum! {
    CalendarValues { None = "none", Gregorian = "gregorian", GregorianUs = "gregorianUs",
    Japan = "japan", Taiwan = "taiwan", Korea = "korea", Hijri = "hijri", Thai = "thai",
    Hebrew = "hebrew", GregorianMiddleEastFrench = "gregorianMeFrench", GregorianArabic =
    "gregorianArabic", GregorianTransliteratedEnglish = "gregorianXlitEnglish",
    GregorianTransliteratedFrench = "gregorianXlitFrench", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DateTimeGroupingValues {
    #[default]
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
}
crate::__string_enum! {
    DateTimeGroupingValues { Year = "year", Month = "month", Day = "day", Hour = "hour",
    Minute = "minute", Second = "second", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HtmlFormattingValues {
    #[default]
    None,
    HonorRichText,
    All,
}
crate::__string_enum! {
    HtmlFormattingValues { None = "none", HonorRichText = "rtf", All = "all", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ParameterValues {
    #[default]
    Prompt,
    Value,
    Cell,
}
crate::__string_enum! {
    ParameterValues { Prompt = "prompt", Value = "value", Cell = "cell", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FileTypeValues {
    #[default]
    Mac,
    Win,
    Dos,
}
crate::__string_enum! {
    FileTypeValues { Mac = "mac", Win = "win", Dos = "dos", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum QualifierValues {
    #[default]
    DoubleQuote,
    SingleQuote,
    None,
}
crate::__string_enum! {
    QualifierValues { DoubleQuote = "doubleQuote", SingleQuote = "singleQuote", None =
    "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ExternalConnectionValues {
    #[default]
    General,
    Text,
    MonthDayYear,
    DayMonthYear,
    YearMonthDay,
    MonthYearDay,
    DayYearMonth,
    YearDayMonth,
    Skip,
    Emd,
}
crate::__string_enum! {
    ExternalConnectionValues { General = "general", Text = "text", MonthDayYear = "mdy",
    DayMonthYear = "dmy", YearMonthDay = "ymd", MonthYearDay = "myd", DayYearMonth =
    "dym", YearDayMonth = "ydm", Skip = "skip", Emd = "emd", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CredentialsMethodValues {
    #[default]
    Integrated,
    None,
    Stored,
}
crate::__string_enum! {
    CredentialsMethodValues { Integrated = "integrated", None = "none", Stored =
    "stored", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SourceValues {
    #[default]
    Worksheet,
    External,
    Consolidation,
    Scenario,
}
crate::__string_enum! {
    SourceValues { Worksheet = "worksheet", External = "external", Consolidation =
    "consolidation", Scenario = "scenario", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GroupByValues {
    #[default]
    Range,
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Quarters,
    Years,
}
crate::__string_enum! {
    GroupByValues { Range = "range", Seconds = "seconds", Minutes = "minutes", Hours =
    "hours", Days = "days", Months = "months", Quarters = "quarters", Years = "years", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SortValues {
    #[default]
    None,
    Ascending,
    Descending,
    AscendingAlpha,
    DescendingAlpha,
    AscendingNatural,
    DescendingNatural,
}
crate::__string_enum! {
    SortValues { None = "none", Ascending = "ascending", Descending = "descending",
    AscendingAlpha = "ascendingAlpha", DescendingAlpha = "descendingAlpha",
    AscendingNatural = "ascendingNatural", DescendingNatural = "descendingNatural", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ScopeValues {
    #[default]
    Selection,
    Data,
    Field,
}
crate::__string_enum! {
    ScopeValues { Selection = "selection", Data = "data", Field = "field", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RuleValues {
    #[default]
    None,
    All,
    Row,
    Column,
}
crate::__string_enum! {
    RuleValues { None = "none", All = "all", Row = "row", Column = "column", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ShowDataAsValues {
    #[default]
    Normal,
    Difference,
    Percent,
    PercentageDifference,
    RunTotal,
    PercentOfRaw,
    PercentOfColumn,
    PercentOfTotal,
    Index,
}
crate::__string_enum! {
    ShowDataAsValues { Normal = "normal", Difference = "difference", Percent = "percent",
    PercentageDifference = "percentDiff", RunTotal = "runTotal", PercentOfRaw =
    "percentOfRow", PercentOfColumn = "percentOfCol", PercentOfTotal = "percentOfTotal",
    Index = "index", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ItemValues {
    #[default]
    Data,
    Default,
    Sum,
    CountA,
    Average,
    Maximum,
    Minimum,
    Product,
    Count,
    StandardDeviation,
    StandardDeviationP,
    Variance,
    VarianceP,
    Grand,
    Blank,
}
crate::__string_enum! {
    ItemValues { Data = "data", Default = "default", Sum = "sum", CountA = "countA",
    Average = "avg", Maximum = "max", Minimum = "min", Product = "product", Count =
    "count", StandardDeviation = "stdDev", StandardDeviationP = "stdDevP", Variance =
    "var", VarianceP = "varP", Grand = "grand", Blank = "blank", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FieldSortValues {
    #[default]
    Manual,
    Ascending,
    Descending,
}
crate::__string_enum! {
    FieldSortValues { Manual = "manual", Ascending = "ascending", Descending =
    "descending", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PivotFilterValues {
    #[default]
    Unknown,
    Count,
    Percent,
    Sum,
    CaptionEqual,
    CaptionNotEqual,
    CaptionBeginsWith,
    CaptionNotBeginsWith,
    CaptionEndsWith,
    CaptionNotEndsWith,
    CaptionContains,
    CaptionNotContains,
    CaptionGreaterThan,
    CaptionGreaterThanOrEqual,
    CaptionLessThan,
    CaptionLessThanOrEqual,
    CaptionBetween,
    CaptionNotBetween,
    ValueEqual,
    ValueNotEqual,
    ValueGreaterThan,
    ValueGreaterThanOrEqual,
    ValueLessThan,
    ValueLessThanOrEqual,
    ValueBetween,
    ValueNotBetween,
    DateEqual,
    DateNotEqual,
    DateOlderThan,
    DateOlderThanOrEqual,
    DateNewerThan,
    DateNewerThanOrEqual,
    DateBetween,
    DateNotBetween,
    Tomorrow,
    Today,
    Yesterday,
    NextWeek,
    ThisWeek,
    LastWeek,
    NextMonth,
    ThisMonth,
    LastMonth,
    NextQuarter,
    ThisQuarter,
    LastQuarter,
    NextYear,
    ThisYear,
    LastYear,
    YearToDate,
    Quarter1,
    Quarter2,
    Quarter3,
    Quarter4,
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
crate::__string_enum! {
    PivotFilterValues { Unknown = "unknown", Count = "count", Percent = "percent", Sum =
    "sum", CaptionEqual = "captionEqual", CaptionNotEqual = "captionNotEqual",
    CaptionBeginsWith = "captionBeginsWith", CaptionNotBeginsWith =
    "captionNotBeginsWith", CaptionEndsWith = "captionEndsWith", CaptionNotEndsWith =
    "captionNotEndsWith", CaptionContains = "captionContains", CaptionNotContains =
    "captionNotContains", CaptionGreaterThan = "captionGreaterThan",
    CaptionGreaterThanOrEqual = "captionGreaterThanOrEqual", CaptionLessThan =
    "captionLessThan", CaptionLessThanOrEqual = "captionLessThanOrEqual", CaptionBetween
    = "captionBetween", CaptionNotBetween = "captionNotBetween", ValueEqual =
    "valueEqual", ValueNotEqual = "valueNotEqual", ValueGreaterThan = "valueGreaterThan",
    ValueGreaterThanOrEqual = "valueGreaterThanOrEqual", ValueLessThan = "valueLessThan",
    ValueLessThanOrEqual = "valueLessThanOrEqual", ValueBetween = "valueBetween",
    ValueNotBetween = "valueNotBetween", DateEqual = "dateEqual", DateNotEqual =
    "dateNotEqual", DateOlderThan = "dateOlderThan", DateOlderThanOrEqual =
    "dateOlderThanOrEqual", DateNewerThan = "dateNewerThan", DateNewerThanOrEqual =
    "dateNewerThanOrEqual", DateBetween = "dateBetween", DateNotBetween =
    "dateNotBetween", Tomorrow = "tomorrow", Today = "today", Yesterday = "yesterday",
    NextWeek = "nextWeek", ThisWeek = "thisWeek", LastWeek = "lastWeek", NextMonth =
    "nextMonth", ThisMonth = "thisMonth", LastMonth = "lastMonth", NextQuarter =
    "nextQuarter", ThisQuarter = "thisQuarter", LastQuarter = "lastQuarter", NextYear =
    "nextYear", ThisYear = "thisYear", LastYear = "lastYear", YearToDate = "yearToDate",
    Quarter1 = "q1", Quarter2 = "q2", Quarter3 = "q3", Quarter4 = "q4", January = "m1",
    February = "m2", March = "m3", April = "m4", May = "m5", June = "m6", July = "m7",
    August = "m8", September = "m9", October = "m10", November = "m11", December = "m12",
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FormatActionValues {
    #[default]
    Blank,
    Formatting,
}
crate::__string_enum! {
    FormatActionValues { Blank = "blank", Formatting = "formatting", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PivotTableAxisValues {
    #[default]
    AxisRow,
    AxisColumn,
    AxisPage,
    AxisValues,
}
crate::__string_enum! {
    PivotTableAxisValues { AxisRow = "axisRow", AxisColumn = "axisCol", AxisPage =
    "axisPage", AxisValues = "axisValues", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GrowShrinkValues {
    #[default]
    InsertDelete,
    InsertClear,
    OverwriteClear,
}
crate::__string_enum! {
    GrowShrinkValues { InsertDelete = "insertDelete", InsertClear = "insertClear",
    OverwriteClear = "overwriteClear", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PhoneticValues {
    #[default]
    HalfWidthKatakana,
    FullWidthKatakana,
    Hiragana,
    NoConversion,
}
crate::__string_enum! {
    PhoneticValues { HalfWidthKatakana = "halfwidthKatakana", FullWidthKatakana =
    "fullwidthKatakana", Hiragana = "hiragana", NoConversion = "noConversion", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PhoneticAlignmentValues {
    #[default]
    NoControl,
    Left,
    Center,
    Distributed,
}
crate::__string_enum! {
    PhoneticAlignmentValues { NoControl = "noControl", Left = "left", Center = "center",
    Distributed = "distributed", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RowColumnActionValues {
    #[default]
    InsertRow,
    DeleteRow,
    ColumnInsert,
    DeleteColumn,
}
crate::__string_enum! {
    RowColumnActionValues { InsertRow = "insertRow", DeleteRow = "deleteRow",
    ColumnInsert = "insertCol", DeleteColumn = "deleteCol", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RevisionActionValues {
    #[default]
    Add,
    Delete,
}
crate::__string_enum! {
    RevisionActionValues { Add = "add", Delete = "delete", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FormulaExpressionValues {
    #[default]
    Reference,
    ReferenceError,
    Area,
    AreaError,
    ComputedArea,
}
crate::__string_enum! {
    FormulaExpressionValues { Reference = "ref", ReferenceError = "refError", Area =
    "area", AreaError = "areaError", ComputedArea = "computedArea", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CellFormulaValues {
    #[default]
    Normal,
    Array,
    DataTable,
    Shared,
}
crate::__string_enum! {
    CellFormulaValues { Normal = "normal", Array = "array", DataTable = "dataTable",
    Shared = "shared", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PaneValues {
    #[default]
    BottomRight,
    TopRight,
    BottomLeft,
    TopLeft,
}
crate::__string_enum! {
    PaneValues { BottomRight = "bottomRight", TopRight = "topRight", BottomLeft =
    "bottomLeft", TopLeft = "topLeft", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SheetViewValues {
    #[default]
    Normal,
    PageBreakPreview,
    PageLayout,
}
crate::__string_enum! {
    SheetViewValues { Normal = "normal", PageBreakPreview = "pageBreakPreview",
    PageLayout = "pageLayout", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataConsolidateFunctionValues {
    #[default]
    Average,
    Count,
    CountNumbers,
    Maximum,
    Minimum,
    Product,
    StandardDeviation,
    StandardDeviationP,
    Sum,
    Variance,
    VarianceP,
}
crate::__string_enum! {
    DataConsolidateFunctionValues { Average = "average", Count = "count", CountNumbers =
    "countNums", Maximum = "max", Minimum = "min", Product = "product", StandardDeviation
    = "stdDev", StandardDeviationP = "stdDevp", Sum = "sum", Variance = "var", VarianceP
    = "varp", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataValidationValues {
    #[default]
    None,
    Whole,
    Decimal,
    List,
    Date,
    Time,
    TextLength,
    Custom,
}
crate::__string_enum! {
    DataValidationValues { None = "none", Whole = "whole", Decimal = "decimal", List =
    "list", Date = "date", Time = "time", TextLength = "textLength", Custom = "custom", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataValidationOperatorValues {
    #[default]
    Between,
    NotBetween,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}
crate::__string_enum! {
    DataValidationOperatorValues { Between = "between", NotBetween = "notBetween", Equal
    = "equal", NotEqual = "notEqual", LessThan = "lessThan", LessThanOrEqual =
    "lessThanOrEqual", GreaterThan = "greaterThan", GreaterThanOrEqual =
    "greaterThanOrEqual", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataValidationErrorStyleValues {
    #[default]
    Stop,
    Warning,
    Information,
}
crate::__string_enum! {
    DataValidationErrorStyleValues { Stop = "stop", Warning = "warning", Information =
    "information", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataValidationImeModeValues {
    #[default]
    NoControl,
    Off,
    On,
    Disabled,
    Hiragana,
    FullKatakana,
    HalfKatakana,
    FullAlpha,
    HalfAlpha,
    FullHangul,
    HalfHangul,
}
crate::__string_enum! {
    DataValidationImeModeValues { NoControl = "noControl", Off = "off", On = "on",
    Disabled = "disabled", Hiragana = "hiragana", FullKatakana = "fullKatakana",
    HalfKatakana = "halfKatakana", FullAlpha = "fullAlpha", HalfAlpha = "halfAlpha",
    FullHangul = "fullHangul", HalfHangul = "halfHangul", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConditionalFormatValues {
    #[default]
    Expression,
    CellIs,
    ColorScale,
    DataBar,
    IconSet,
    Top10,
    UniqueValues,
    DuplicateValues,
    ContainsText,
    NotContainsText,
    BeginsWith,
    EndsWith,
    ContainsBlanks,
    NotContainsBlanks,
    ContainsErrors,
    NotContainsErrors,
    TimePeriod,
    AboveAverage,
}
crate::__string_enum! {
    ConditionalFormatValues { Expression = "expression", CellIs = "cellIs", ColorScale =
    "colorScale", DataBar = "dataBar", IconSet = "iconSet", Top10 = "top10", UniqueValues
    = "uniqueValues", DuplicateValues = "duplicateValues", ContainsText = "containsText",
    NotContainsText = "notContainsText", BeginsWith = "beginsWith", EndsWith =
    "endsWith", ContainsBlanks = "containsBlanks", NotContainsBlanks =
    "notContainsBlanks", ContainsErrors = "containsErrors", NotContainsErrors =
    "notContainsErrors", TimePeriod = "timePeriod", AboveAverage = "aboveAverage", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TimePeriodValues {
    #[default]
    Today,
    Yesterday,
    Tomorrow,
    Last7Days,
    ThisMonth,
    LastMonth,
    NextMonth,
    ThisWeek,
    LastWeek,
    NextWeek,
}
crate::__string_enum! {
    TimePeriodValues { Today = "today", Yesterday = "yesterday", Tomorrow = "tomorrow",
    Last7Days = "last7Days", ThisMonth = "thisMonth", LastMonth = "lastMonth", NextMonth
    = "nextMonth", ThisWeek = "thisWeek", LastWeek = "lastWeek", NextWeek = "nextWeek", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConditionalFormattingOperatorValues {
    #[default]
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
    GreaterThanOrEqual,
    GreaterThan,
    Between,
    NotBetween,
    ContainsText,
    NotContains,
    BeginsWith,
    EndsWith,
}
crate::__string_enum! {
    ConditionalFormattingOperatorValues { LessThan = "lessThan", LessThanOrEqual =
    "lessThanOrEqual", Equal = "equal", NotEqual = "notEqual", GreaterThanOrEqual =
    "greaterThanOrEqual", GreaterThan = "greaterThan", Between = "between", NotBetween =
    "notBetween", ContainsText = "containsText", NotContains = "notContains", BeginsWith
    = "beginsWith", EndsWith = "endsWith", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConditionalFormatValueObjectValues {
    #[default]
    Number,
    Percent,
    Max,
    Min,
    Formula,
    Percentile,
}
crate::__string_enum! {
    ConditionalFormatValueObjectValues { Number = "num", Percent = "percent", Max =
    "max", Min = "min", Formula = "formula", Percentile = "percentile", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PageOrderValues {
    #[default]
    DownThenOver,
    OverThenDown,
}
crate::__string_enum! {
    PageOrderValues { DownThenOver = "downThenOver", OverThenDown = "overThenDown", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OrientationValues {
    #[default]
    Default,
    Portrait,
    Landscape,
}
crate::__string_enum! {
    OrientationValues { Default = "default", Portrait = "portrait", Landscape =
    "landscape", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CellCommentsValues {
    #[default]
    None,
    AsDisplayed,
    AtEnd,
}
crate::__string_enum! {
    CellCommentsValues { None = "none", AsDisplayed = "asDisplayed", AtEnd = "atEnd", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PrintErrorValues {
    #[default]
    Displayed,
    Blank,
    Dash,
    Na,
}
crate::__string_enum! {
    PrintErrorValues { Displayed = "displayed", Blank = "blank", Dash = "dash", Na =
    "na", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataViewAspectValues {
    #[default]
    DataViewAspectContent,
    DataViewAspectIcon,
}
crate::__string_enum! {
    DataViewAspectValues { DataViewAspectContent = "dvaspectContent", DataViewAspectIcon
    = "dvaspectIcon", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OleUpdateValues {
    #[default]
    OleUpdateAlways,
    OleUpdateOnCall,
}
crate::__string_enum! {
    OleUpdateValues { OleUpdateAlways = "oleupdateAlways", OleUpdateOnCall =
    "oleupdateOncall", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum WebSourceValues {
    #[default]
    Sheet,
    PrintArea,
    AutoFilter,
    Range,
    Chart,
    PivotTable,
    Query,
    Label,
}
crate::__string_enum! {
    WebSourceValues { Sheet = "sheet", PrintArea = "printArea", AutoFilter =
    "autoFilter", Range = "range", Chart = "chart", PivotTable = "pivotTable", Query =
    "query", Label = "label", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PaneStateValues {
    #[default]
    Split,
    Frozen,
    FrozenSplit,
}
crate::__string_enum! {
    PaneStateValues { Split = "split", Frozen = "frozen", FrozenSplit = "frozenSplit", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MdxFunctionValues {
    #[default]
    CubeMember,
    CubeValue,
    CubeSet,
    CubeSetCount,
    CubeRankedMember,
    CubeMemberProperty,
    CubeKpiMember,
}
crate::__string_enum! {
    MdxFunctionValues { CubeMember = "m", CubeValue = "v", CubeSet = "s", CubeSetCount =
    "c", CubeRankedMember = "r", CubeMemberProperty = "p", CubeKpiMember = "k", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MdxSetOrderValues {
    #[default]
    Unsorted,
    Ascending,
    Descending,
    AlphaAscendingSortOrder,
    AlphaDescendingSortOrder,
    NaturalAscending,
    NaturalDescending,
}
crate::__string_enum! {
    MdxSetOrderValues { Unsorted = "u", Ascending = "a", Descending = "d",
    AlphaAscendingSortOrder = "aa", AlphaDescendingSortOrder = "ad", NaturalAscending =
    "na", NaturalDescending = "nd", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MdxKpiPropertyValues {
    #[default]
    Value,
    Goal,
    Status,
    Trend,
    Weight,
    Time,
}
crate::__string_enum! {
    MdxKpiPropertyValues { Value = "v", Goal = "g", Status = "s", Trend = "t", Weight =
    "w", Time = "m", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BorderStyleValues {
    #[default]
    None,
    Thin,
    Medium,
    Dashed,
    Dotted,
    Thick,
    Double,
    Hair,
    MediumDashed,
    DashDot,
    MediumDashDot,
    DashDotDot,
    MediumDashDotDot,
    SlantDashDot,
}
crate::__string_enum! {
    BorderStyleValues { None = "none", Thin = "thin", Medium = "medium", Dashed =
    "dashed", Dotted = "dotted", Thick = "thick", Double = "double", Hair = "hair",
    MediumDashed = "mediumDashed", DashDot = "dashDot", MediumDashDot = "mediumDashDot",
    DashDotDot = "dashDotDot", MediumDashDotDot = "mediumDashDotDot", SlantDashDot =
    "slantDashDot", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PatternValues {
    #[default]
    None,
    Solid,
    MediumGray,
    DarkGray,
    LightGray,
    DarkHorizontal,
    DarkVertical,
    DarkDown,
    DarkUp,
    DarkGrid,
    DarkTrellis,
    LightHorizontal,
    LightVertical,
    LightDown,
    LightUp,
    LightGrid,
    LightTrellis,
    Gray125,
    Gray0625,
}
crate::__string_enum! {
    PatternValues { None = "none", Solid = "solid", MediumGray = "mediumGray", DarkGray =
    "darkGray", LightGray = "lightGray", DarkHorizontal = "darkHorizontal", DarkVertical
    = "darkVertical", DarkDown = "darkDown", DarkUp = "darkUp", DarkGrid = "darkGrid",
    DarkTrellis = "darkTrellis", LightHorizontal = "lightHorizontal", LightVertical =
    "lightVertical", LightDown = "lightDown", LightUp = "lightUp", LightGrid =
    "lightGrid", LightTrellis = "lightTrellis", Gray125 = "gray125", Gray0625 =
    "gray0625", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GradientValues {
    #[default]
    Linear,
    Path,
}
crate::__string_enum! {
    GradientValues { Linear = "linear", Path = "path", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HorizontalAlignmentValues {
    #[default]
    General,
    Left,
    Center,
    Right,
    Fill,
    Justify,
    CenterContinuous,
    Distributed,
}
crate::__string_enum! {
    HorizontalAlignmentValues { General = "general", Left = "left", Center = "center",
    Right = "right", Fill = "fill", Justify = "justify", CenterContinuous =
    "centerContinuous", Distributed = "distributed", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VerticalAlignmentValues {
    #[default]
    Top,
    Center,
    Bottom,
    Justify,
    Distributed,
}
crate::__string_enum! {
    VerticalAlignmentValues { Top = "top", Center = "center", Bottom = "bottom", Justify
    = "justify", Distributed = "distributed", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TableStyleValues {
    #[default]
    WholeTable,
    HeaderRow,
    TotalRow,
    FirstColumn,
    LastColumn,
    FirstRowStripe,
    SecondRowStripe,
    FirstColumnStripe,
    SecondColumnStripe,
    FirstHeaderCell,
    LastHeaderCell,
    FirstTotalCell,
    LastTotalCell,
    FirstSubtotalColumn,
    SecondSubtotalColumn,
    ThirdSubtotalColumn,
    FirstSubtotalRow,
    SecondSubtotalRow,
    ThirdSubtotalRow,
    BlankRow,
    FirstColumnSubheading,
    SecondColumnSubheading,
    ThirdColumnSubheading,
    FirstRowSubheading,
    SecondRowSubheading,
    ThirdRowSubheading,
    PageFieldLabels,
    PageFieldValues,
}
crate::__string_enum! {
    TableStyleValues { WholeTable = "wholeTable", HeaderRow = "headerRow", TotalRow =
    "totalRow", FirstColumn = "firstColumn", LastColumn = "lastColumn", FirstRowStripe =
    "firstRowStripe", SecondRowStripe = "secondRowStripe", FirstColumnStripe =
    "firstColumnStripe", SecondColumnStripe = "secondColumnStripe", FirstHeaderCell =
    "firstHeaderCell", LastHeaderCell = "lastHeaderCell", FirstTotalCell =
    "firstTotalCell", LastTotalCell = "lastTotalCell", FirstSubtotalColumn =
    "firstSubtotalColumn", SecondSubtotalColumn = "secondSubtotalColumn",
    ThirdSubtotalColumn = "thirdSubtotalColumn", FirstSubtotalRow = "firstSubtotalRow",
    SecondSubtotalRow = "secondSubtotalRow", ThirdSubtotalRow = "thirdSubtotalRow",
    BlankRow = "blankRow", FirstColumnSubheading = "firstColumnSubheading",
    SecondColumnSubheading = "secondColumnSubheading", ThirdColumnSubheading =
    "thirdColumnSubheading", FirstRowSubheading = "firstRowSubheading",
    SecondRowSubheading = "secondRowSubheading", ThirdRowSubheading =
    "thirdRowSubheading", PageFieldLabels = "pageFieldLabels", PageFieldValues =
    "pageFieldValues", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VerticalAlignmentRunValues {
    #[default]
    Baseline,
    Superscript,
    Subscript,
}
crate::__string_enum! {
    VerticalAlignmentRunValues { Baseline = "baseline", Superscript = "superscript",
    Subscript = "subscript", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FontSchemeValues {
    #[default]
    None,
    Major,
    Minor,
}
crate::__string_enum! {
    FontSchemeValues { None = "none", Major = "major", Minor = "minor", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum UnderlineValues {
    #[default]
    Single,
    Double,
    SingleAccounting,
    DoubleAccounting,
    None,
}
crate::__string_enum! {
    UnderlineValues { Single = "single", Double = "double", SingleAccounting =
    "singleAccounting", DoubleAccounting = "doubleAccounting", None = "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DdeValues {
    #[default]
    Nil,
    Boolean,
    RealNumber,
    Error,
    String,
}
crate::__string_enum! {
    DdeValues { Nil = "nil", Boolean = "b", RealNumber = "n", Error = "e", String =
    "str", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TableValues {
    #[default]
    Worksheet,
    Xml,
    QueryTable,
}
crate::__string_enum! {
    TableValues { Worksheet = "worksheet", Xml = "xml", QueryTable = "queryTable", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TotalsRowFunctionValues {
    #[default]
    None,
    Sum,
    Minimum,
    Maximum,
    Average,
    Count,
    CountNumbers,
    StandardDeviation,
    Variance,
    Custom,
}
crate::__string_enum! {
    TotalsRowFunctionValues { None = "none", Sum = "sum", Minimum = "min", Maximum =
    "max", Average = "average", Count = "count", CountNumbers = "countNums",
    StandardDeviation = "stdDev", Variance = "var", Custom = "custom", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum XmlDataValues {
    #[default]
    String,
    NormalizedString,
    Token,
    Byte,
    UnsignedByte,
    Base64Binary,
    HexBinary,
    Integer,
    PositiveInteger,
    NegativeInteger,
    NonPositiveInteger,
    NonNegativeInteger,
    Int,
    UnsignedInteger,
    Long,
    UnsignedLong,
    Short,
    UnsignedShort,
    Decimal,
    Float,
    Double,
    Boolean,
    Time,
    DateTime,
    Duration,
    Date,
    Gmonth,
    Gyear,
    GYearMonth,
    Gday,
    GMonthDay,
    Name,
    Qname,
    NcName,
    AnyUri,
    Language,
    Id,
    IdRef,
    IdRefs,
    Entity,
    Entities,
    Notation,
    NmToken,
    NmTokens,
    AnyType,
}
crate::__string_enum! {
    XmlDataValues { String = "string", NormalizedString = "normalizedString", Token =
    "token", Byte = "byte", UnsignedByte = "unsignedByte", Base64Binary = "base64Binary",
    HexBinary = "hexBinary", Integer = "integer", PositiveInteger = "positiveInteger",
    NegativeInteger = "negativeInteger", NonPositiveInteger = "nonPositiveInteger",
    NonNegativeInteger = "nonNegativeInteger", Int = "int", UnsignedInteger =
    "unsignedInt", Long = "long", UnsignedLong = "unsignedLong", Short = "short",
    UnsignedShort = "unsignedShort", Decimal = "decimal", Float = "float", Double =
    "double", Boolean = "boolean", Time = "time", DateTime = "dateTime", Duration =
    "duration", Date = "date", Gmonth = "gMonth", Gyear = "gYear", GYearMonth =
    "gYearMonth", Gday = "gDay", GMonthDay = "gMonthDay", Name = "name", Qname = "qName",
    NcName = "ncName", AnyUri = "anyUri", Language = "language", Id = "id", IdRef =
    "idref", IdRefs = "idrefs", Entity = "entity", Entities = "entities", Notation =
    "notation", NmToken = "nmtoken", NmTokens = "nmtokens", AnyType = "anyType", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VolatileDependencyValues {
    #[default]
    RealTimeData,
    OlapFunctions,
}
crate::__string_enum! {
    VolatileDependencyValues { RealTimeData = "realTimeData", OlapFunctions =
    "olapFunctions", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VolatileValues {
    #[default]
    Boolean,
    RealNumber,
    Error,
    String,
}
crate::__string_enum! {
    VolatileValues { Boolean = "b", RealNumber = "n", Error = "e", String = "s", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VisibilityValues {
    #[default]
    Visible,
    Hidden,
    VeryHidden,
}
crate::__string_enum! {
    VisibilityValues { Visible = "visible", Hidden = "hidden", VeryHidden = "veryHidden",
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CommentsValues {
    #[default]
    CommentNone,
    CommentIndicator,
    CommentIndicatorAndComment,
}
crate::__string_enum! {
    CommentsValues { CommentNone = "commNone", CommentIndicator = "commIndicator",
    CommentIndicatorAndComment = "commIndAndComment", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ObjectDisplayValues {
    #[default]
    All,
    Placeholders,
    None,
}
crate::__string_enum! {
    ObjectDisplayValues { All = "all", Placeholders = "placeholders", None = "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SheetStateValues {
    #[default]
    Visible,
    Hidden,
    VeryHidden,
}
crate::__string_enum! {
    SheetStateValues { Visible = "visible", Hidden = "hidden", VeryHidden = "veryHidden",
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum UpdateLinksBehaviorValues {
    #[default]
    UserSet,
    Never,
    Always,
}
crate::__string_enum! {
    UpdateLinksBehaviorValues { UserSet = "userSet", Never = "never", Always = "always",
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CalculateModeValues {
    #[default]
    Manual,
    Auto,
    AutoNoTable,
}
crate::__string_enum! {
    CalculateModeValues { Manual = "manual", Auto = "auto", AutoNoTable = "autoNoTable",
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ReferenceModeValues {
    #[default]
    A1,
    R1c1,
}
crate::__string_enum! {
    ReferenceModeValues { A1 = "a1", R1c1 = "r1c1", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TargetScreenSizeValues {
    #[default]
    Sz544x376,
    Sz640x480,
    Sz720x512,
    Sz800x600,
    Sz1024x768,
    Sz1152x882,
    Sz1152x900,
    Sz1280x1024,
    Sz1600x1200,
    Sz1800x1440,
    Sz1920x1200,
}
crate::__string_enum! {
    TargetScreenSizeValues { Sz544x376 = "544x376", Sz640x480 = "640x480", Sz720x512 =
    "720x512", Sz800x600 = "800x600", Sz1024x768 = "1024x768", Sz1152x882 = "1152x882",
    Sz1152x900 = "1152x900", Sz1280x1024 = "1280x1024", Sz1600x1200 = "1600x1200",
    Sz1800x1440 = "1800x1440", Sz1920x1200 = "1920x1200", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextHorizontalAlignmentValues {
    #[default]
    Left,
    Center,
    Right,
    Justify,
    Distributed,
}
crate::__string_enum! {
    TextHorizontalAlignmentValues { Left = "left", Center = "center", Right = "right",
    Justify = "justify", Distributed = "distributed", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextVerticalAlignmentValues {
    #[default]
    Top,
    Center,
    Bottom,
    Justify,
    Distributed,
}
crate::__string_enum! {
    TextVerticalAlignmentValues { Top = "top", Center = "center", Bottom = "bottom",
    Justify = "justify", Distributed = "distributed", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CellValues {
    #[default]
    Boolean,
    Number,
    Error,
    SharedString,
    String,
    InlineString,
    Date,
}
crate::__string_enum! {
    CellValues { Boolean = "b", Number = "n", Error = "e", SharedString = "s", String =
    "str", InlineString = "inlineStr", Date = "d", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PivotAreaValues {
    #[default]
    None,
    Normal,
    Data,
    All,
    Origin,
    Button,
    TopRight,
    TopEnd,
}
crate::__string_enum! {
    PivotAreaValues { None = "none", Normal = "normal", Data = "data", All = "all",
    Origin = "origin", Button = "button", TopRight = "topRight", TopEnd = "topEnd", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConformanceClass {
    #[default]
    Enumstrict,
    Enumtransitional,
}
crate::__string_enum! {
    ConformanceClass { Enumstrict = "strict", Enumtransitional = "transitional", }
}
/// Extension.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct Extension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
}
/// Calculation Chain Info.
/// When the object is serialized out as xml, it's qualified name is x:calcChain.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:calcChain")]
pub struct CalculationChain {
    #[xml(attr = "xmlns", with = "calculation_chain_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "x:c")]
    pub x_c: Vec<CalculationCell>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<ExtensionList>,
}
mod calculation_chain_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// Comments.
/// When the object is serialized out as xml, it's qualified name is x:comments.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:comments")]
pub struct Comments {
    #[xml(attr = "xmlns", with = "comments_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Authors
    #[xml(child = "x:authors")]
    pub authors: Authors,
    ///List of Comments
    #[xml(child = "x:commentList")]
    pub comment_list: CommentList,
    /// _
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
mod comments_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// XML Mapping.
/// When the object is serialized out as xml, it's qualified name is x:MapInfo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:MapInfo")]
pub struct MapInfo {
    #[xml(attr = "xmlns", with = "map_info_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Prefix Mappings for XPath Expressions
    /// Represents the following attribute in the schema: :SelectionNamespaces
    #[xml(attr = "SelectionNamespaces")]
    pub selection_namespaces: String,
    /// _
    #[xml(child = "x:Schema")]
    pub x_schema: Vec<Schema>,
    /// _
    #[xml(child = "x:Map")]
    pub x_map: Vec<Map>,
}
mod map_info_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// Connections.
/// When the object is serialized out as xml, it's qualified name is x:connections.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:connections")]
pub struct Connections {
    #[xml(attr = "xmlns", with = "connections_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "x:connection")]
    pub x_connection: Vec<Connection>,
}
mod connections_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// PivotCache Definition.
/// When the object is serialized out as xml, it's qualified name is x:pivotCacheDefinition.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pivotCacheDefinition")]
pub struct PivotCacheDefinition {
    #[xml(attr = "xmlns", with = "pivot_cache_definition_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    /// invalid
    /// Represents the following attribute in the schema: :invalid
    #[xml(attr = "invalid")]
    pub invalid: Option<bool>,
    /// saveData
    /// Represents the following attribute in the schema: :saveData
    #[xml(attr = "saveData")]
    pub save_data: Option<bool>,
    /// refreshOnLoad
    /// Represents the following attribute in the schema: :refreshOnLoad
    #[xml(attr = "refreshOnLoad")]
    pub refresh_on_load: Option<bool>,
    /// optimizeMemory
    /// Represents the following attribute in the schema: :optimizeMemory
    #[xml(attr = "optimizeMemory")]
    pub optimize_memory: Option<bool>,
    /// enableRefresh
    /// Represents the following attribute in the schema: :enableRefresh
    #[xml(attr = "enableRefresh")]
    pub enable_refresh: Option<bool>,
    /// refreshedBy
    /// Represents the following attribute in the schema: :refreshedBy
    #[xml(attr = "refreshedBy")]
    pub refreshed_by: Option<String>,
    /// refreshedDateIso
    /// Represents the following attribute in the schema: :refreshedDateIso
    #[xml(attr = "refreshedDateIso")]
    pub last_refreshed_date_iso: Option<String>,
    /// refreshedDate
    /// Represents the following attribute in the schema: :refreshedDate
    #[xml(attr = "refreshedDate")]
    pub refreshed_date: Option<f64>,
    /// backgroundQuery
    /// Represents the following attribute in the schema: :backgroundQuery
    #[xml(attr = "backgroundQuery")]
    pub background_query: Option<bool>,
    /// missingItemsLimit
    /// Represents the following attribute in the schema: :missingItemsLimit
    #[xml(attr = "missingItemsLimit")]
    pub missing_items_limit: Option<i32>,
    /// createdVersion
    /// Represents the following attribute in the schema: :createdVersion
    #[xml(attr = "createdVersion")]
    pub created_version: Option<u8>,
    /// refreshedVersion
    /// Represents the following attribute in the schema: :refreshedVersion
    #[xml(attr = "refreshedVersion")]
    pub refreshed_version: Option<u8>,
    /// minRefreshableVersion
    /// Represents the following attribute in the schema: :minRefreshableVersion
    #[xml(attr = "minRefreshableVersion")]
    pub min_refreshable_version: Option<u8>,
    /// recordCount
    /// Represents the following attribute in the schema: :recordCount
    #[xml(attr = "recordCount")]
    pub record_count: Option<i32>,
    /// upgradeOnRefresh
    /// Represents the following attribute in the schema: :upgradeOnRefresh
    #[xml(attr = "upgradeOnRefresh")]
    pub upgrade_on_refresh: Option<bool>,
    /// tupleCache
    /// Represents the following attribute in the schema: :tupleCache
    #[xml(attr = "tupleCache")]
    pub is_tuple_cache: Option<bool>,
    /// supportSubquery
    /// Represents the following attribute in the schema: :supportSubquery
    #[xml(attr = "supportSubquery")]
    pub support_subquery: Option<bool>,
    /// supportAdvancedDrill
    /// Represents the following attribute in the schema: :supportAdvancedDrill
    #[xml(attr = "supportAdvancedDrill")]
    pub support_advanced_drill: Option<bool>,
    /// _
    #[xml(child = "x:cacheSource")]
    pub cache_source: CacheSource,
    /// _
    #[xml(child = "x:cacheFields")]
    pub cache_fields: CacheFields,
    /// _
    #[xml(child = "x:cacheHierarchies")]
    pub cache_hierarchies: Option<CacheHierarchies>,
    /// _
    #[xml(child = "x:kpis")]
    pub kpis: Option<Kpis>,
    /// _
    #[xml(child = "x:tupleCache")]
    pub tuple_cache: Option<TupleCache>,
    /// _
    #[xml(child = "x:calculatedItems")]
    pub calculated_items: Option<CalculatedItems>,
    /// _
    #[xml(child = "x:calculatedMembers")]
    pub calculated_members: Option<CalculatedMembers>,
    /// _
    #[xml(child = "x:dimensions")]
    pub dimensions: Option<Dimensions>,
    /// _
    #[xml(child = "x:measureGroups")]
    pub measure_groups: Option<MeasureGroups>,
    /// _
    #[xml(child = "x:maps")]
    pub maps: Option<Maps>,
    /// _
    #[xml(child = "x:extLst")]
    pub pivot_cache_definition_extension_list: Option<PivotCacheDefinitionExtensionList>,
}
mod pivot_cache_definition_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// PivotCache Records.
/// When the object is serialized out as xml, it's qualified name is x:pivotCacheRecords.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pivotCacheRecords")]
pub struct PivotCacheRecords {
    #[xml(attr = "xmlns", with = "pivot_cache_records_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// PivotCache Records Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:r")]
    pub x_r: Vec<PivotCacheRecord>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<ExtensionList>,
}
mod pivot_cache_records_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// PivotTable Definition.
/// When the object is serialized out as xml, it's qualified name is x:pivotTableDefinition.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pivotTableDefinition")]
pub struct PivotTableDefinition {
    #[xml(attr = "xmlns", with = "pivot_table_definition_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// cacheId
    /// Represents the following attribute in the schema: :cacheId
    #[xml(attr = "cacheId")]
    pub cache_id: i32,
    /// dataOnRows
    /// Represents the following attribute in the schema: :dataOnRows
    #[xml(attr = "dataOnRows")]
    pub data_on_rows: Option<bool>,
    /// dataPosition
    /// Represents the following attribute in the schema: :dataPosition
    #[xml(attr = "dataPosition")]
    pub data_position: Option<i32>,
    /// Auto Format Id
    /// Represents the following attribute in the schema: :autoFormatId
    #[xml(attr = "autoFormatId")]
    pub auto_format_id: Option<i32>,
    /// Apply Number Formats
    /// Represents the following attribute in the schema: :applyNumberFormats
    #[xml(attr = "applyNumberFormats")]
    pub apply_number_formats: Option<bool>,
    /// Apply Border Formats
    /// Represents the following attribute in the schema: :applyBorderFormats
    #[xml(attr = "applyBorderFormats")]
    pub apply_border_formats: Option<bool>,
    /// Apply Font Formats
    /// Represents the following attribute in the schema: :applyFontFormats
    #[xml(attr = "applyFontFormats")]
    pub apply_font_formats: Option<bool>,
    /// Apply Pattern Formats
    /// Represents the following attribute in the schema: :applyPatternFormats
    #[xml(attr = "applyPatternFormats")]
    pub apply_pattern_formats: Option<bool>,
    /// Apply Alignment Formats
    /// Represents the following attribute in the schema: :applyAlignmentFormats
    #[xml(attr = "applyAlignmentFormats")]
    pub apply_alignment_formats: Option<bool>,
    /// Apply Width / Height Formats
    /// Represents the following attribute in the schema: :applyWidthHeightFormats
    #[xml(attr = "applyWidthHeightFormats")]
    pub apply_width_height_formats: Option<bool>,
    /// dataCaption
    /// Represents the following attribute in the schema: :dataCaption
    #[xml(attr = "dataCaption")]
    pub data_caption: String,
    /// grandTotalCaption
    /// Represents the following attribute in the schema: :grandTotalCaption
    #[xml(attr = "grandTotalCaption")]
    pub grand_total_caption: Option<String>,
    /// errorCaption
    /// Represents the following attribute in the schema: :errorCaption
    #[xml(attr = "errorCaption")]
    pub error_caption: Option<String>,
    /// showError
    /// Represents the following attribute in the schema: :showError
    #[xml(attr = "showError")]
    pub show_error: Option<bool>,
    /// missingCaption
    /// Represents the following attribute in the schema: :missingCaption
    #[xml(attr = "missingCaption")]
    pub missing_caption: Option<String>,
    /// showMissing
    /// Represents the following attribute in the schema: :showMissing
    #[xml(attr = "showMissing")]
    pub show_missing: Option<bool>,
    /// pageStyle
    /// Represents the following attribute in the schema: :pageStyle
    #[xml(attr = "pageStyle")]
    pub page_style: Option<String>,
    /// pivotTableStyle
    /// Represents the following attribute in the schema: :pivotTableStyle
    #[xml(attr = "pivotTableStyle")]
    pub pivot_table_style_name: Option<String>,
    /// vacatedStyle
    /// Represents the following attribute in the schema: :vacatedStyle
    #[xml(attr = "vacatedStyle")]
    pub vacated_style: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// updatedVersion
    /// Represents the following attribute in the schema: :updatedVersion
    #[xml(attr = "updatedVersion")]
    pub updated_version: Option<u8>,
    /// minRefreshableVersion
    /// Represents the following attribute in the schema: :minRefreshableVersion
    #[xml(attr = "minRefreshableVersion")]
    pub min_refreshable_version: Option<u8>,
    /// asteriskTotals
    /// Represents the following attribute in the schema: :asteriskTotals
    #[xml(attr = "asteriskTotals")]
    pub asterisk_totals: Option<bool>,
    /// showItems
    /// Represents the following attribute in the schema: :showItems
    #[xml(attr = "showItems")]
    pub show_items: Option<bool>,
    /// editData
    /// Represents the following attribute in the schema: :editData
    #[xml(attr = "editData")]
    pub edit_data: Option<bool>,
    /// disableFieldList
    /// Represents the following attribute in the schema: :disableFieldList
    #[xml(attr = "disableFieldList")]
    pub disable_field_list: Option<bool>,
    /// showCalcMbrs
    /// Represents the following attribute in the schema: :showCalcMbrs
    #[xml(attr = "showCalcMbrs")]
    pub show_calculated_members: Option<bool>,
    /// visualTotals
    /// Represents the following attribute in the schema: :visualTotals
    #[xml(attr = "visualTotals")]
    pub visual_totals: Option<bool>,
    /// showMultipleLabel
    /// Represents the following attribute in the schema: :showMultipleLabel
    #[xml(attr = "showMultipleLabel")]
    pub show_multiple_label: Option<bool>,
    /// showDataDropDown
    /// Represents the following attribute in the schema: :showDataDropDown
    #[xml(attr = "showDataDropDown")]
    pub show_data_drop_down: Option<bool>,
    /// showDrill
    /// Represents the following attribute in the schema: :showDrill
    #[xml(attr = "showDrill")]
    pub show_drill: Option<bool>,
    /// printDrill
    /// Represents the following attribute in the schema: :printDrill
    #[xml(attr = "printDrill")]
    pub print_drill: Option<bool>,
    /// showMemberPropertyTips
    /// Represents the following attribute in the schema: :showMemberPropertyTips
    #[xml(attr = "showMemberPropertyTips")]
    pub show_member_property_tips: Option<bool>,
    /// showDataTips
    /// Represents the following attribute in the schema: :showDataTips
    #[xml(attr = "showDataTips")]
    pub show_data_tips: Option<bool>,
    /// enableWizard
    /// Represents the following attribute in the schema: :enableWizard
    #[xml(attr = "enableWizard")]
    pub enable_wizard: Option<bool>,
    /// enableDrill
    /// Represents the following attribute in the schema: :enableDrill
    #[xml(attr = "enableDrill")]
    pub enable_drill: Option<bool>,
    /// enableFieldProperties
    /// Represents the following attribute in the schema: :enableFieldProperties
    #[xml(attr = "enableFieldProperties")]
    pub enable_field_properties: Option<bool>,
    /// preserveFormatting
    /// Represents the following attribute in the schema: :preserveFormatting
    #[xml(attr = "preserveFormatting")]
    pub preserve_formatting: Option<bool>,
    /// useAutoFormatting
    /// Represents the following attribute in the schema: :useAutoFormatting
    #[xml(attr = "useAutoFormatting")]
    pub use_auto_formatting: Option<bool>,
    /// pageWrap
    /// Represents the following attribute in the schema: :pageWrap
    #[xml(attr = "pageWrap")]
    pub page_wrap: Option<i32>,
    /// pageOverThenDown
    /// Represents the following attribute in the schema: :pageOverThenDown
    #[xml(attr = "pageOverThenDown")]
    pub page_over_then_down: Option<bool>,
    /// subtotalHiddenItems
    /// Represents the following attribute in the schema: :subtotalHiddenItems
    #[xml(attr = "subtotalHiddenItems")]
    pub subtotal_hidden_items: Option<bool>,
    /// rowGrandTotals
    /// Represents the following attribute in the schema: :rowGrandTotals
    #[xml(attr = "rowGrandTotals")]
    pub row_grand_totals: Option<bool>,
    /// colGrandTotals
    /// Represents the following attribute in the schema: :colGrandTotals
    #[xml(attr = "colGrandTotals")]
    pub column_grand_totals: Option<bool>,
    /// fieldPrintTitles
    /// Represents the following attribute in the schema: :fieldPrintTitles
    #[xml(attr = "fieldPrintTitles")]
    pub field_print_titles: Option<bool>,
    /// itemPrintTitles
    /// Represents the following attribute in the schema: :itemPrintTitles
    #[xml(attr = "itemPrintTitles")]
    pub item_print_titles: Option<bool>,
    /// mergeItem
    /// Represents the following attribute in the schema: :mergeItem
    #[xml(attr = "mergeItem")]
    pub merge_item: Option<bool>,
    /// showDropZones
    /// Represents the following attribute in the schema: :showDropZones
    #[xml(attr = "showDropZones")]
    pub show_drop_zones: Option<bool>,
    /// createdVersion
    /// Represents the following attribute in the schema: :createdVersion
    #[xml(attr = "createdVersion")]
    pub created_version: Option<u8>,
    /// indent
    /// Represents the following attribute in the schema: :indent
    #[xml(attr = "indent")]
    pub indent: Option<i32>,
    /// showEmptyRow
    /// Represents the following attribute in the schema: :showEmptyRow
    #[xml(attr = "showEmptyRow")]
    pub show_empty_row: Option<bool>,
    /// showEmptyCol
    /// Represents the following attribute in the schema: :showEmptyCol
    #[xml(attr = "showEmptyCol")]
    pub show_empty_column: Option<bool>,
    /// showHeaders
    /// Represents the following attribute in the schema: :showHeaders
    #[xml(attr = "showHeaders")]
    pub show_headers: Option<bool>,
    /// compact
    /// Represents the following attribute in the schema: :compact
    #[xml(attr = "compact")]
    pub compact: Option<bool>,
    /// outline
    /// Represents the following attribute in the schema: :outline
    #[xml(attr = "outline")]
    pub outline: Option<bool>,
    /// outlineData
    /// Represents the following attribute in the schema: :outlineData
    #[xml(attr = "outlineData")]
    pub outline_data: Option<bool>,
    /// compactData
    /// Represents the following attribute in the schema: :compactData
    #[xml(attr = "compactData")]
    pub compact_data: Option<bool>,
    /// published
    /// Represents the following attribute in the schema: :published
    #[xml(attr = "published")]
    pub published: Option<bool>,
    /// gridDropZones
    /// Represents the following attribute in the schema: :gridDropZones
    #[xml(attr = "gridDropZones")]
    pub grid_drop_zones: Option<bool>,
    /// immersive
    /// Represents the following attribute in the schema: :immersive
    #[xml(attr = "immersive")]
    pub stop_immersive_ui: Option<bool>,
    /// multipleFieldFilters
    /// Represents the following attribute in the schema: :multipleFieldFilters
    #[xml(attr = "multipleFieldFilters")]
    pub multiple_field_filters: Option<bool>,
    /// chartFormat
    /// Represents the following attribute in the schema: :chartFormat
    #[xml(attr = "chartFormat")]
    pub chart_format: Option<i32>,
    /// rowHeaderCaption
    /// Represents the following attribute in the schema: :rowHeaderCaption
    #[xml(attr = "rowHeaderCaption")]
    pub row_header_caption: Option<String>,
    /// colHeaderCaption
    /// Represents the following attribute in the schema: :colHeaderCaption
    #[xml(attr = "colHeaderCaption")]
    pub column_header_caption: Option<String>,
    /// fieldListSortAscending
    /// Represents the following attribute in the schema: :fieldListSortAscending
    #[xml(attr = "fieldListSortAscending")]
    pub field_list_sort_ascending: Option<bool>,
    /// mdxSubqueries
    /// Represents the following attribute in the schema: :mdxSubqueries
    #[xml(attr = "mdxSubqueries")]
    pub mdx_subqueries: Option<bool>,
    /// customListSort
    /// Represents the following attribute in the schema: :customListSort
    #[xml(attr = "customListSort")]
    pub custom_list_sort: Option<bool>,
    /// _
    #[xml(child = "x:location")]
    pub location: Location,
    /// _
    #[xml(child = "x:pivotFields")]
    pub pivot_fields: Option<PivotFields>,
    /// _
    #[xml(child = "x:rowFields")]
    pub row_fields: Option<RowFields>,
    /// _
    #[xml(child = "x:rowItems")]
    pub row_items: Option<RowItems>,
    /// _
    #[xml(child = "x:colFields")]
    pub column_fields: Option<ColumnFields>,
    /// _
    #[xml(child = "x:colItems")]
    pub column_items: Option<ColumnItems>,
    /// _
    #[xml(child = "x:pageFields")]
    pub page_fields: Option<PageFields>,
    /// _
    #[xml(child = "x:dataFields")]
    pub data_fields: Option<DataFields>,
    /// _
    #[xml(child = "x:formats")]
    pub formats: Option<Formats>,
    /// _
    #[xml(child = "x:conditionalFormats")]
    pub conditional_formats: Option<ConditionalFormats>,
    /// _
    #[xml(child = "x:chartFormats")]
    pub chart_formats: Option<ChartFormats>,
    /// _
    #[xml(child = "x:pivotHierarchies")]
    pub pivot_hierarchies: Option<PivotHierarchies>,
    /// _
    #[xml(child = "x:pivotTableStyleInfo")]
    pub pivot_table_style: Option<PivotTableStyle>,
    /// _
    #[xml(child = "x:filters")]
    pub pivot_filters: Option<PivotFilters>,
    /// _
    #[xml(child = "x:rowHierarchiesUsage")]
    pub row_hierarchies_usage: Option<RowHierarchiesUsage>,
    /// _
    #[xml(child = "x:colHierarchiesUsage")]
    pub column_hierarchies_usage: Option<ColumnHierarchiesUsage>,
    /// _
    #[xml(child = "x:extLst")]
    pub pivot_table_definition_extension_list: Option<PivotTableDefinitionExtensionList>,
}
mod pivot_table_definition_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// Query Table.
/// When the object is serialized out as xml, it's qualified name is x:queryTable.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:queryTable")]
pub struct QueryTable {
    #[xml(attr = "xmlns", with = "query_table_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// headers
    /// Represents the following attribute in the schema: :headers
    #[xml(attr = "headers")]
    pub headers: Option<bool>,
    /// rowNumbers
    /// Represents the following attribute in the schema: :rowNumbers
    #[xml(attr = "rowNumbers")]
    pub row_numbers: Option<bool>,
    /// disableRefresh
    /// Represents the following attribute in the schema: :disableRefresh
    #[xml(attr = "disableRefresh")]
    pub disable_refresh: Option<bool>,
    /// backgroundRefresh
    /// Represents the following attribute in the schema: :backgroundRefresh
    #[xml(attr = "backgroundRefresh")]
    pub background_refresh: Option<bool>,
    /// firstBackgroundRefresh
    /// Represents the following attribute in the schema: :firstBackgroundRefresh
    #[xml(attr = "firstBackgroundRefresh")]
    pub first_background_refresh: Option<bool>,
    /// refreshOnLoad
    /// Represents the following attribute in the schema: :refreshOnLoad
    #[xml(attr = "refreshOnLoad")]
    pub refresh_on_load: Option<bool>,
    /// growShrinkType
    /// Represents the following attribute in the schema: :growShrinkType
    #[xml(attr = "growShrinkType")]
    pub grow_shrink_type: Option<GrowShrinkValues>,
    /// fillFormulas
    /// Represents the following attribute in the schema: :fillFormulas
    #[xml(attr = "fillFormulas")]
    pub fill_formulas: Option<bool>,
    /// removeDataOnSave
    /// Represents the following attribute in the schema: :removeDataOnSave
    #[xml(attr = "removeDataOnSave")]
    pub remove_data_on_save: Option<bool>,
    /// disableEdit
    /// Represents the following attribute in the schema: :disableEdit
    #[xml(attr = "disableEdit")]
    pub disable_edit: Option<bool>,
    /// preserveFormatting
    /// Represents the following attribute in the schema: :preserveFormatting
    #[xml(attr = "preserveFormatting")]
    pub preserve_formatting: Option<bool>,
    /// adjustColumnWidth
    /// Represents the following attribute in the schema: :adjustColumnWidth
    #[xml(attr = "adjustColumnWidth")]
    pub adjust_column_width: Option<bool>,
    /// intermediate
    /// Represents the following attribute in the schema: :intermediate
    #[xml(attr = "intermediate")]
    pub intermediate: Option<bool>,
    /// connectionId
    /// Represents the following attribute in the schema: :connectionId
    #[xml(attr = "connectionId")]
    pub connection_id: i32,
    /// Auto Format Id
    /// Represents the following attribute in the schema: :autoFormatId
    #[xml(attr = "autoFormatId")]
    pub auto_format_id: Option<i32>,
    /// Apply Number Formats
    /// Represents the following attribute in the schema: :applyNumberFormats
    #[xml(attr = "applyNumberFormats")]
    pub apply_number_formats: Option<bool>,
    /// Apply Border Formats
    /// Represents the following attribute in the schema: :applyBorderFormats
    #[xml(attr = "applyBorderFormats")]
    pub apply_border_formats: Option<bool>,
    /// Apply Font Formats
    /// Represents the following attribute in the schema: :applyFontFormats
    #[xml(attr = "applyFontFormats")]
    pub apply_font_formats: Option<bool>,
    /// Apply Pattern Formats
    /// Represents the following attribute in the schema: :applyPatternFormats
    #[xml(attr = "applyPatternFormats")]
    pub apply_pattern_formats: Option<bool>,
    /// Apply Alignment Formats
    /// Represents the following attribute in the schema: :applyAlignmentFormats
    #[xml(attr = "applyAlignmentFormats")]
    pub apply_alignment_formats: Option<bool>,
    /// Apply Width / Height Formats
    /// Represents the following attribute in the schema: :applyWidthHeightFormats
    #[xml(attr = "applyWidthHeightFormats")]
    pub apply_width_height_formats: Option<bool>,
    /// _
    #[xml(child = "x:queryTableRefresh")]
    pub query_table_refresh: Option<QueryTableRefresh>,
    /// _
    #[xml(child = "x:extLst")]
    pub query_table_extension_list: Option<QueryTableExtensionList>,
}
mod query_table_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// Shared String Table.
/// When the object is serialized out as xml, it's qualified name is x:sst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sst")]
pub struct SharedStringTable {
    #[xml(attr = "xmlns", with = "shared_string_table_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// String Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// Unique String Count
    /// Represents the following attribute in the schema: :uniqueCount
    #[xml(attr = "uniqueCount")]
    pub unique_count: Option<i32>,
    /// _
    #[xml(child = "x:si")]
    pub x_si: Vec<SharedStringItem>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<ExtensionList>,
}
mod shared_string_table_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// Revision Headers.
/// When the object is serialized out as xml, it's qualified name is x:headers.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:headers")]
pub struct Headers {
    #[xml(attr = "xmlns", with = "headers_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Last Revision GUID
    /// Represents the following attribute in the schema: :guid
    #[xml(attr = "guid")]
    pub guid: String,
    /// Last GUID
    /// Represents the following attribute in the schema: :lastGuid
    #[xml(attr = "lastGuid")]
    pub last_guid: Option<String>,
    /// Shared Workbook
    /// Represents the following attribute in the schema: :shared
    #[xml(attr = "shared")]
    pub shared: Option<bool>,
    /// Disk Revisions
    /// Represents the following attribute in the schema: :diskRevisions
    #[xml(attr = "diskRevisions")]
    pub disk_revisions: Option<bool>,
    /// History
    /// Represents the following attribute in the schema: :history
    #[xml(attr = "history")]
    pub history: Option<bool>,
    /// Track Revisions
    /// Represents the following attribute in the schema: :trackRevisions
    #[xml(attr = "trackRevisions")]
    pub track_revisions: Option<bool>,
    /// Exclusive Mode
    /// Represents the following attribute in the schema: :exclusive
    #[xml(attr = "exclusive")]
    pub exclusive: Option<bool>,
    /// Revision Id
    /// Represents the following attribute in the schema: :revisionId
    #[xml(attr = "revisionId")]
    pub revision_id: Option<i32>,
    /// Version
    /// Represents the following attribute in the schema: :version
    #[xml(attr = "version")]
    pub version: Option<i32>,
    /// Keep Change History
    /// Represents the following attribute in the schema: :keepChangeHistory
    #[xml(attr = "keepChangeHistory")]
    pub keep_change_history: Option<bool>,
    /// Protected
    /// Represents the following attribute in the schema: :protected
    #[xml(attr = "protected")]
    pub protected: Option<bool>,
    /// Preserve History
    /// Represents the following attribute in the schema: :preserveHistory
    #[xml(attr = "preserveHistory")]
    pub preserve_history: Option<i32>,
    /// _
    #[xml(child = "x:header")]
    pub x_header: Vec<Header>,
}
mod headers_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// Revisions.
/// When the object is serialized out as xml, it's qualified name is x:revisions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:revisions")]
pub struct Revisions {
    #[xml(attr = "xmlns", with = "revisions_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "x:rrc",
        child = "x:rm",
        child = "x:rcv",
        child = "x:rsnm",
        child = "x:ris",
        child = "x:rcc",
        child = "x:rfmt",
        child = "x:raf",
        child = "x:rdn",
        child = "x:rcmt",
        child = "x:rqt",
        child = "x:rcft",
    )]
    pub children: Vec<RevisionsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RevisionsChildChoice {
    #[xml(tag = "x:rrc")]
    XRrc(RevisionRowColumn),
    #[xml(tag = "x:rm")]
    XRm(RevisionMove),
    #[xml(tag = "x:rcv")]
    XRcv(RevisionCustomView),
    #[xml(tag = "x:rsnm")]
    XRsnm(RevisionSheetName),
    #[xml(tag = "x:ris")]
    XRis(RevisionInsertSheet),
    #[xml(tag = "x:rcc")]
    XRcc(RevisionCellChange),
    #[xml(tag = "x:rfmt")]
    XRfmt(RevisionFormat),
    #[xml(tag = "x:raf")]
    XRaf(RevisionAutoFormat),
    #[xml(tag = "x:rdn")]
    XRdn(RevisionDefinedName),
    #[xml(tag = "x:rcmt")]
    XRcmt(RevisionComment),
    #[xml(tag = "x:rqt")]
    XRqt(RevisionQueryTable),
    #[xml(tag = "x:rcft")]
    XRcft(RevisionConflict),
}
mod revisions_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// User List.
/// When the object is serialized out as xml, it's qualified name is x:users.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:users")]
pub struct Users {
    #[xml(attr = "xmlns", with = "users_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Active User Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:userInfo")]
    pub x_user_info: Vec<UserInfo>,
}
mod users_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// Worksheet.
/// When the object is serialized out as xml, it's qualified name is x:worksheet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:worksheet")]
pub struct Worksheet {
    #[xml(attr = "xmlns", with = "worksheet_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "x:sheetPr")]
    pub sheet_properties: Option<SheetProperties>,
    /// _
    #[xml(child = "x:dimension")]
    pub sheet_dimension: Option<SheetDimension>,
    /// _
    #[xml(child = "x:sheetViews")]
    pub sheet_views: Option<SheetViews>,
    /// _
    #[xml(child = "x:sheetFormatPr")]
    pub sheet_format_properties: Option<SheetFormatProperties>,
    /// _
    #[xml(child = "x:cols")]
    pub x_cols: Vec<Columns>,
    /// _
    #[xml(child = "x:sheetData")]
    pub x_sheet_data: SheetData,
    /// _
    #[xml(child = "x:sheetCalcPr")]
    pub x_sheet_calc_pr: Option<SheetCalculationProperties>,
    /// _
    #[xml(child = "x:sheetProtection")]
    pub x_sheet_protection: Option<SheetProtection>,
    /// _
    #[xml(child = "x:protectedRanges")]
    pub x_protected_ranges: Option<ProtectedRanges>,
    /// _
    #[xml(child = "x:scenarios")]
    pub x_scenarios: Option<Scenarios>,
    /// _
    #[xml(child = "x:autoFilter")]
    pub x_auto_filter: Option<AutoFilter>,
    /// _
    #[xml(child = "x:sortState")]
    pub x_sort_state: Option<SortState>,
    /// _
    #[xml(child = "x:dataConsolidate")]
    pub x_data_consolidate: Option<DataConsolidate>,
    /// _
    #[xml(child = "x:customSheetViews")]
    pub x_custom_sheet_views: Option<CustomSheetViews>,
    /// _
    #[xml(child = "x:mergeCells")]
    pub x_merge_cells: Option<MergeCells>,
    /// _
    #[xml(child = "x:phoneticPr")]
    pub x_phonetic_pr: Option<PhoneticProperties>,
    /// _
    #[xml(child = "x:conditionalFormatting")]
    pub x_conditional_formatting: Vec<ConditionalFormatting>,
    /// _
    #[xml(child = "x:dataValidations")]
    pub x_data_validations: Option<DataValidations>,
    /// _
    #[xml(child = "x:hyperlinks")]
    pub x_hyperlinks: Option<Hyperlinks>,
    /// _
    #[xml(child = "x:printOptions")]
    pub x_print_options: Option<PrintOptions>,
    /// _
    #[xml(child = "x:pageMargins")]
    pub x_page_margins: Option<PageMargins>,
    /// _
    #[xml(child = "x:pageSetup")]
    pub x_page_setup: Option<PageSetup>,
    /// _
    #[xml(child = "x:headerFooter")]
    pub x_header_footer: Option<HeaderFooter>,
    /// _
    #[xml(child = "x:rowBreaks")]
    pub x_row_breaks: Option<RowBreaks>,
    /// _
    #[xml(child = "x:colBreaks")]
    pub x_col_breaks: Option<ColumnBreaks>,
    /// _
    #[xml(child = "x:customProperties")]
    pub x_custom_properties: Option<CustomProperties>,
    /// _
    #[xml(child = "x:cellWatches")]
    pub x_cell_watches: Option<CellWatches>,
    /// _
    #[xml(child = "x:ignoredErrors")]
    pub x_ignored_errors: Option<IgnoredErrors>,
    /// _
    #[xml(child = "x:drawing")]
    pub x_drawing: Option<Drawing>,
    /// _
    #[xml(child = "x:legacyDrawing")]
    pub x_legacy_drawing: Option<LegacyDrawing>,
    /// _
    #[xml(child = "x:legacyDrawingHF")]
    pub x_legacy_drawing_hf: Option<LegacyDrawingHeaderFooter>,
    /// _
    #[xml(child = "x:drawingHF")]
    pub x_drawing_hf: Option<DrawingHeaderFooter>,
    /// _
    #[xml(child = "x:picture")]
    pub x_picture: Option<Picture>,
    /// _
    #[xml(child = "x:oleObjects")]
    pub x_ole_objects: Option<OleObjects>,
    /// _
    #[xml(child = "x:controls")]
    pub x_controls: Option<Controls>,
    /// _
    #[xml(child = "x:webPublishItems")]
    pub x_web_publish_items: Option<WebPublishItems>,
    /// _
    #[xml(child = "x:tableParts")]
    pub x_table_parts: Option<TableParts>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<WorksheetExtensionList>,
}
mod worksheet_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// Chart Sheet.
/// When the object is serialized out as xml, it's qualified name is x:chartsheet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:chartsheet")]
pub struct Chartsheet {
    #[xml(attr = "xmlns", with = "chartsheet_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Chart Sheet Properties
    #[xml(child = "x:sheetPr")]
    pub chart_sheet_properties: Option<ChartSheetProperties>,
    ///Chart Sheet Views
    #[xml(child = "x:sheetViews")]
    pub chart_sheet_views: ChartSheetViews,
    ///Chart Sheet Protection
    #[xml(child = "x:sheetProtection")]
    pub chart_sheet_protection: Option<ChartSheetProtection>,
    ///Custom Chart Sheet Views
    #[xml(child = "x:customSheetViews")]
    pub custom_chartsheet_views: Option<CustomChartsheetViews>,
    /// _
    #[xml(child = "x:pageMargins")]
    pub page_margins: Option<PageMargins>,
    /// _
    #[xml(child = "x:pageSetup")]
    pub chart_sheet_page_setup: Option<ChartSheetPageSetup>,
    /// _
    #[xml(child = "x:headerFooter")]
    pub header_footer: Option<HeaderFooter>,
    ///Drawing
    #[xml(child = "x:drawing")]
    pub drawing: Drawing,
    /// _
    #[xml(child = "x:legacyDrawing")]
    pub legacy_drawing: Option<LegacyDrawing>,
    ///Legacy Drawing Reference in  Header Footer
    #[xml(child = "x:legacyDrawingHF")]
    pub legacy_drawing_header_footer: Option<LegacyDrawingHeaderFooter>,
    /// _
    #[xml(child = "x:drawingHF")]
    pub drawing_header_footer: Option<DrawingHeaderFooter>,
    /// _
    #[xml(child = "x:picture")]
    pub picture: Option<Picture>,
    /// _
    #[xml(child = "x:webPublishItems")]
    pub web_publish_items: Option<WebPublishItems>,
    /// _
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
mod chartsheet_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// Dialog Sheet.
/// When the object is serialized out as xml, it's qualified name is x:dialogsheet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dialogsheet")]
pub struct DialogSheet {
    #[xml(attr = "xmlns", with = "dialog_sheet_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Sheet Properties
    #[xml(child = "x:sheetPr")]
    pub sheet_properties: Option<SheetProperties>,
    ///Dialog Sheet Views
    #[xml(child = "x:sheetViews")]
    pub sheet_views: Option<SheetViews>,
    ///Dialog Sheet Format Properties
    #[xml(child = "x:sheetFormatPr")]
    pub sheet_format_properties: Option<SheetFormatProperties>,
    ///Sheet Protection
    #[xml(child = "x:sheetProtection")]
    pub sheet_protection: Option<SheetProtection>,
    ///Custom Sheet Views
    #[xml(child = "x:customSheetViews")]
    pub custom_sheet_views: Option<CustomSheetViews>,
    ///Print Options
    #[xml(child = "x:printOptions")]
    pub print_options: Option<PrintOptions>,
    ///Page Margins
    #[xml(child = "x:pageMargins")]
    pub page_margins: Option<PageMargins>,
    ///Page Setup Settings
    #[xml(child = "x:pageSetup")]
    pub page_setup: Option<PageSetup>,
    ///Header and Footer Settings
    #[xml(child = "x:headerFooter")]
    pub header_footer: Option<HeaderFooter>,
    ///Drawing
    #[xml(child = "x:drawing")]
    pub drawing: Option<Drawing>,
    ///Legacy Drawing
    #[xml(child = "x:legacyDrawing")]
    pub legacy_drawing: LegacyDrawing,
    ///Legacy Drawing Header Footer
    #[xml(child = "x:legacyDrawingHF")]
    pub legacy_drawing_header_footer: Option<LegacyDrawingHeaderFooter>,
    /// _
    #[xml(child = "x:drawingHF")]
    pub drawing_header_footer: Option<DrawingHeaderFooter>,
    /// _
    #[xml(child = "x:oleObjects")]
    pub ole_objects: Option<OleObjects>,
    /// _
    #[xml(child = "x:controls")]
    pub controls: Option<Controls>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
mod dialog_sheet_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// Metadata.
/// When the object is serialized out as xml, it's qualified name is x:metadata.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:metadata")]
pub struct Metadata {
    #[xml(attr = "xmlns", with = "metadata_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Metadata Types Collection
    #[xml(child = "x:metadataTypes")]
    pub metadata_types: Option<MetadataTypes>,
    ///Metadata String Store
    #[xml(child = "x:metadataStrings")]
    pub metadata_strings: Option<MetadataStrings>,
    ///MDX Metadata Information
    #[xml(child = "x:mdxMetadata")]
    pub mdx_metadata: Option<MdxMetadata>,
    /// _
    #[xml(child = "x:futureMetadata")]
    pub x_future_metadata: Vec<FutureMetadata>,
    /// _
    #[xml(child = "x:cellMetadata")]
    pub x_cell_metadata: Option<CellMetadata>,
    /// _
    #[xml(child = "x:valueMetadata")]
    pub x_value_metadata: Option<ValueMetadata>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<ExtensionList>,
}
mod metadata_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// Single Cells.
/// When the object is serialized out as xml, it's qualified name is x:singleXmlCells.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:singleXmlCells")]
pub struct SingleXmlCells {
    #[xml(attr = "xmlns", with = "single_xml_cells_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "x:singleXmlCell")]
    pub x_single_xml_cell: Vec<SingleXmlCell>,
}
mod single_xml_cells_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// Style Sheet.
/// When the object is serialized out as xml, it's qualified name is x:styleSheet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:styleSheet")]
pub struct Stylesheet {
    #[xml(attr = "xmlns", with = "stylesheet_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "x:numFmts")]
    pub numbering_formats: Option<NumberingFormats>,
    /// _
    #[xml(child = "x:fonts")]
    pub fonts: Option<Fonts>,
    /// _
    #[xml(child = "x:fills")]
    pub fills: Option<Fills>,
    /// _
    #[xml(child = "x:borders")]
    pub borders: Option<Borders>,
    /// _
    #[xml(child = "x:cellStyleXfs")]
    pub cell_style_formats: Option<CellStyleFormats>,
    /// _
    #[xml(child = "x:cellXfs")]
    pub cell_formats: Option<CellFormats>,
    /// _
    #[xml(child = "x:cellStyles")]
    pub cell_styles: Option<CellStyles>,
    /// _
    #[xml(child = "x:dxfs")]
    pub differential_formats: Option<DifferentialFormats>,
    /// _
    #[xml(child = "x:tableStyles")]
    pub table_styles: Option<TableStyles>,
    /// _
    #[xml(child = "x:colors")]
    pub colors: Option<Colors>,
    /// _
    #[xml(child = "x:extLst")]
    pub stylesheet_extension_list: Option<StylesheetExtensionList>,
}
mod stylesheet_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// External Reference.
/// When the object is serialized out as xml, it's qualified name is x:externalLink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:externalLink")]
pub struct ExternalLink {
    #[xml(attr = "xmlns", with = "external_link_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "x:externalBook",
        child = "x:ddeLink",
        child = "x:oleLink",
        child = "x:extLst",
    )]
    pub children: Vec<ExternalLinkChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExternalLinkChildChoice {
    #[xml(tag = "x:externalBook")]
    XExternalBook(ExternalBook),
    #[xml(tag = "x:ddeLink")]
    XDdeLink(DdeLink),
    #[xml(tag = "x:oleLink")]
    XOleLink(OleLink),
    #[xml(tag = "x:extLst")]
    XExtLst(ExtensionList),
}
mod external_link_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// Table.
/// When the object is serialized out as xml, it's qualified name is x:table.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:table")]
pub struct Table {
    #[xml(attr = "xmlns", with = "table_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Table Id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Table Name
    /// Represents the following attribute in the schema: :displayName
    #[xml(attr = "displayName")]
    pub display_name: String,
    /// Table Comment
    /// Represents the following attribute in the schema: :comment
    #[xml(attr = "comment")]
    pub comment: Option<String>,
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: String,
    /// Table Type
    /// Represents the following attribute in the schema: :tableType
    #[xml(attr = "tableType")]
    pub table_type: Option<TableValues>,
    /// Header Row Count
    /// Represents the following attribute in the schema: :headerRowCount
    #[xml(attr = "headerRowCount")]
    pub header_row_count: Option<i32>,
    /// Insert Row Showing
    /// Represents the following attribute in the schema: :insertRow
    #[xml(attr = "insertRow")]
    pub insert_row: Option<bool>,
    /// Insert Row Shift
    /// Represents the following attribute in the schema: :insertRowShift
    #[xml(attr = "insertRowShift")]
    pub insert_row_shift: Option<bool>,
    /// Totals Row Count
    /// Represents the following attribute in the schema: :totalsRowCount
    #[xml(attr = "totalsRowCount")]
    pub totals_row_count: Option<i32>,
    /// Totals Row Shown
    /// Represents the following attribute in the schema: :totalsRowShown
    #[xml(attr = "totalsRowShown")]
    pub totals_row_shown: Option<bool>,
    /// Published
    /// Represents the following attribute in the schema: :published
    #[xml(attr = "published")]
    pub published: Option<bool>,
    /// Header Row Format Id
    /// Represents the following attribute in the schema: :headerRowDxfId
    #[xml(attr = "headerRowDxfId")]
    pub header_row_format_id: Option<i32>,
    /// Data Area Format Id
    /// Represents the following attribute in the schema: :dataDxfId
    #[xml(attr = "dataDxfId")]
    pub data_format_id: Option<i32>,
    /// Totals Row Format Id
    /// Represents the following attribute in the schema: :totalsRowDxfId
    #[xml(attr = "totalsRowDxfId")]
    pub totals_row_format_id: Option<i32>,
    /// Header Row Border Format Id
    /// Represents the following attribute in the schema: :headerRowBorderDxfId
    #[xml(attr = "headerRowBorderDxfId")]
    pub header_row_border_format_id: Option<i32>,
    /// Table Border Format Id
    /// Represents the following attribute in the schema: :tableBorderDxfId
    #[xml(attr = "tableBorderDxfId")]
    pub border_format_id: Option<i32>,
    /// Totals Row Border Format Id
    /// Represents the following attribute in the schema: :totalsRowBorderDxfId
    #[xml(attr = "totalsRowBorderDxfId")]
    pub totals_row_border_format_id: Option<i32>,
    /// Header Row Style
    /// Represents the following attribute in the schema: :headerRowCellStyle
    #[xml(attr = "headerRowCellStyle")]
    pub header_row_cell_style: Option<String>,
    /// Data Style Name
    /// Represents the following attribute in the schema: :dataCellStyle
    #[xml(attr = "dataCellStyle")]
    pub data_cell_style: Option<String>,
    /// Totals Row Style
    /// Represents the following attribute in the schema: :totalsRowCellStyle
    #[xml(attr = "totalsRowCellStyle")]
    pub totals_row_cell_style: Option<String>,
    /// Connection ID
    /// Represents the following attribute in the schema: :connectionId
    #[xml(attr = "connectionId")]
    pub connection_id: Option<i32>,
    ///Table AutoFilter
    #[xml(child = "x:autoFilter")]
    pub auto_filter: Option<AutoFilter>,
    ///Sort State
    #[xml(child = "x:sortState")]
    pub sort_state: Option<SortState>,
    ///Table Columns
    #[xml(child = "x:tableColumns")]
    pub table_columns: TableColumns,
    ///Table Style
    #[xml(child = "x:tableStyleInfo")]
    pub table_style_info: Option<TableStyleInfo>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub table_extension_list: Option<TableExtensionList>,
}
mod table_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// Volatile Dependency Types.
/// When the object is serialized out as xml, it's qualified name is x:volTypes.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:volTypes")]
pub struct VolatileTypes {
    #[xml(attr = "xmlns", with = "volatile_types_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "x:volType")]
    pub x_vol_type: Vec<VolatileType>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<ExtensionList>,
}
mod volatile_types_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// Workbook.
/// When the object is serialized out as xml, it's qualified name is x:workbook.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:workbook")]
pub struct Workbook {
    #[xml(attr = "xmlns", with = "workbook_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// conformance
    /// Represents the following attribute in the schema: :conformance
    #[xml(attr = "conformance")]
    pub conformance: Option<ConformanceClass>,
    /// _
    #[xml(child = "x:fileVersion")]
    pub file_version: Option<FileVersion>,
    /// _
    #[xml(child = "x:fileSharing")]
    pub file_sharing: Option<FileSharing>,
    /// _
    #[xml(child = "x:workbookPr")]
    pub workbook_properties: Option<WorkbookProperties>,
    /// _
    #[xml(child = "x15ac:absPath")]
    pub absolute_path: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_ac::AbsolutePath,
    >,
    /// _
    #[xml(child = "x:workbookProtection")]
    pub workbook_protection: Option<WorkbookProtection>,
    /// _
    #[xml(child = "x:bookViews")]
    pub book_views: Option<BookViews>,
    /// _
    #[xml(child = "x:sheets")]
    pub sheets: Sheets,
    /// _
    #[xml(child = "x:functionGroups")]
    pub function_groups: Option<FunctionGroups>,
    /// _
    #[xml(child = "x:externalReferences")]
    pub external_references: Option<ExternalReferences>,
    /// _
    #[xml(child = "x:definedNames")]
    pub defined_names: Option<DefinedNames>,
    /// _
    #[xml(child = "x:calcPr")]
    pub calculation_properties: Option<CalculationProperties>,
    /// _
    #[xml(child = "x:oleSize")]
    pub ole_size: Option<OleSize>,
    /// _
    #[xml(child = "x:customWorkbookViews")]
    pub custom_workbook_views: Option<CustomWorkbookViews>,
    /// _
    #[xml(child = "x:pivotCaches")]
    pub pivot_caches: Option<PivotCaches>,
    /// _
    #[xml(child = "x:webPublishing")]
    pub web_publishing: Option<WebPublishing>,
    /// _
    #[xml(child = "x:fileRecoveryPr")]
    pub x_file_recovery_pr: Vec<FileRecoveryProperties>,
    /// _
    #[xml(child = "x:webPublishObjects")]
    pub x_web_publish_objects: Option<WebPublishObjects>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<WorkbookExtensionList>,
}
mod workbook_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
    }
}
/// AutoFilter Column.
/// When the object is serialized out as xml, it's qualified name is x:filterColumn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:filterColumn")]
pub struct FilterColumn {
    /// Filter Column Data
    /// Represents the following attribute in the schema: :colId
    #[xml(attr = "colId")]
    pub column_id: i32,
    /// Hidden AutoFilter Button
    /// Represents the following attribute in the schema: :hiddenButton
    #[xml(attr = "hiddenButton")]
    pub hidden_button: Option<bool>,
    /// Show Filter Button
    /// Represents the following attribute in the schema: :showButton
    #[xml(attr = "showButton")]
    pub show_button: Option<bool>,
    #[xml(
        child = "x:filters",
        child = "x:top10",
        child = "x14:customFilters",
        child = "x:customFilters",
        child = "x:dynamicFilter",
        child = "x:colorFilter",
        child = "x14:iconFilter",
        child = "x:iconFilter",
        child = "x:extLst",
    )]
    pub children: Vec<FilterColumnChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FilterColumnChildChoice {
    #[xml(tag = "x:filters")]
    XFilters(Filters),
    #[xml(tag = "x:top10")]
    XTop10(Top10),
    #[xml(tag = "x14:customFilters")]
    X14CustomFilters(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::CustomFilters,
    ),
    #[xml(tag = "x:customFilters")]
    XCustomFilters(CustomFilters),
    #[xml(tag = "x:dynamicFilter")]
    XDynamicFilter(DynamicFilter),
    #[xml(tag = "x:colorFilter")]
    XColorFilter(ColorFilter),
    #[xml(tag = "x14:iconFilter")]
    X14IconFilter(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::IconFilter,
    ),
    #[xml(tag = "x:iconFilter")]
    XIconFilter(IconFilter),
    #[xml(tag = "x:extLst")]
    XExtLst(ExtensionList),
}
/// Sort State for Auto Filter.
/// When the object is serialized out as xml, it's qualified name is x:sortState.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sortState")]
pub struct SortState {
    /// Sort by Columns
    /// Represents the following attribute in the schema: :columnSort
    #[xml(attr = "columnSort")]
    pub column_sort: Option<bool>,
    /// Case Sensitive
    /// Represents the following attribute in the schema: :caseSensitive
    #[xml(attr = "caseSensitive")]
    pub case_sensitive: Option<bool>,
    /// Sort Method
    /// Represents the following attribute in the schema: :sortMethod
    #[xml(attr = "sortMethod")]
    pub sort_method: Option<SortMethodValues>,
    /// Sort Range
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: String,
    #[xml(child = "x14:sortCondition", child = "x:sortCondition", child = "x:extLst")]
    pub children: Vec<SortStateChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SortStateChildChoice {
    #[xml(tag = "x14:sortCondition")]
    X14SortCondition(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SortCondition,
    ),
    #[xml(tag = "x:sortCondition")]
    XSortCondition(SortCondition),
    #[xml(tag = "x:extLst")]
    XExtLst(ExtensionList),
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct ExtensionList {
    #[xml(child = "x:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "x:ext")]
    XExt(Extension),
}
/// Custom Filter Criteria.
/// When the object is serialized out as xml, it's qualified name is x:customFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:customFilter")]
pub struct CustomFilter {
    /// Filter Comparison Operator
    /// Represents the following attribute in the schema: :operator
    #[xml(attr = "operator")]
    pub operator: Option<FilterOperatorValues>,
    /// Top or Bottom Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<String>,
}
/// Cell.
/// When the object is serialized out as xml, it's qualified name is x:c.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:c")]
pub struct CalculationCell {
    /// Cell Reference
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub cell_reference: String,
    /// Sheet Id
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub sheet_id: Option<i32>,
    /// Child Chain
    /// Represents the following attribute in the schema: :s
    #[xml(attr = "s")]
    pub in_child_chain: Option<bool>,
    /// New Dependency Level
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub new_level: Option<bool>,
    /// New Thread
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub new_thread: Option<bool>,
    /// Array
    /// Represents the following attribute in the schema: :a
    #[xml(attr = "a")]
    pub array: Option<bool>,
}
/// Authors.
/// When the object is serialized out as xml, it's qualified name is x:authors.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:authors")]
pub struct Authors {
    /// _
    #[xml(child = "x:author")]
    pub x_author: Vec<Author>,
}
/// List of Comments.
/// When the object is serialized out as xml, it's qualified name is x:commentList.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:commentList")]
pub struct CommentList {
    /// _
    #[xml(child = "x:comment")]
    pub x_comment: Vec<Comment>,
}
/// Comment.
/// When the object is serialized out as xml, it's qualified name is x:comment.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:comment")]
pub struct Comment {
    /// Cell Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: String,
    /// Author Id
    /// Represents the following attribute in the schema: :authorId
    #[xml(attr = "authorId")]
    pub author_id: i32,
    /// Unique Identifier for Comment
    /// Represents the following attribute in the schema: :guid
    #[xml(attr = "guid")]
    pub guid: Option<String>,
    /// shapeId
    /// Represents the following attribute in the schema: :shapeId
    #[xml(attr = "shapeId")]
    pub shape_id: Option<i32>,
    ///Comment Text
    #[xml(child = "x:text")]
    pub comment_text: CommentText,
    /// _
    #[xml(child = "x:commentPr")]
    pub comment_properties: Option<CommentProperties>,
}
/// Author.
/// When the object is serialized out as xml, it's qualified name is x:author.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:author")]
pub struct Author {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Text.
/// When the object is serialized out as xml, it's qualified name is x:t.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:t")]
pub struct Text {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Cell Value.
/// When the object is serialized out as xml, it's qualified name is x:v.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:v")]
pub struct CellValue {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Formula.
/// When the object is serialized out as xml, it's qualified name is x:formula.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:formula")]
pub struct Formula {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Old Formula.
/// When the object is serialized out as xml, it's qualified name is x:oldFormula.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:oldFormula")]
pub struct OldFormula {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Odd Header.
/// When the object is serialized out as xml, it's qualified name is x:oddHeader.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:oddHeader")]
pub struct OddHeader {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Odd Page Footer.
/// When the object is serialized out as xml, it's qualified name is x:oddFooter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:oddFooter")]
pub struct OddFooter {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Even Page Header.
/// When the object is serialized out as xml, it's qualified name is x:evenHeader.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:evenHeader")]
pub struct EvenHeader {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Even Page Footer.
/// When the object is serialized out as xml, it's qualified name is x:evenFooter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:evenFooter")]
pub struct EvenFooter {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// First Page Header.
/// When the object is serialized out as xml, it's qualified name is x:firstHeader.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:firstHeader")]
pub struct FirstHeader {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// First Page Footer.
/// When the object is serialized out as xml, it's qualified name is x:firstFooter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:firstFooter")]
pub struct FirstFooter {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// DDE Link Value.
/// When the object is serialized out as xml, it's qualified name is x:val.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:val")]
pub struct DdeLinkValue {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Strings in Subtopic.
/// When the object is serialized out as xml, it's qualified name is x:stp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:stp")]
pub struct Subtopic {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Defines the Formula1 Class.
/// When the object is serialized out as xml, it's qualified name is x:formula1.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:formula1")]
pub struct Formula1 {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Defines the Formula2 Class.
/// When the object is serialized out as xml, it's qualified name is x:formula2.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:formula2")]
pub struct Formula2 {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Defines the XstringType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct XstringType {
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// XML Schema.
/// When the object is serialized out as xml, it's qualified name is x:Schema.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:Schema")]
pub struct Schema {
    /// Schema ID
    /// Represents the following attribute in the schema: :ID
    #[xml(attr = "ID")]
    pub id: String,
    /// Schema Reference
    /// Represents the following attribute in the schema: :SchemaRef
    #[xml(attr = "SchemaRef")]
    pub schema_reference: Option<String>,
    /// Schema Root Namespace
    /// Represents the following attribute in the schema: :Namespace
    #[xml(attr = "Namespace")]
    pub namespace: Option<String>,
}
/// XML Mapping Properties.
/// When the object is serialized out as xml, it's qualified name is x:Map.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:Map")]
pub struct Map {
    /// XML Mapping ID
    /// Represents the following attribute in the schema: :ID
    #[xml(attr = "ID")]
    pub id: i32,
    /// XML Mapping Name
    /// Represents the following attribute in the schema: :Name
    #[xml(attr = "Name")]
    pub name: String,
    /// Root Element Name
    /// Represents the following attribute in the schema: :RootElement
    #[xml(attr = "RootElement")]
    pub root_element: String,
    /// Schema Name
    /// Represents the following attribute in the schema: :SchemaID
    #[xml(attr = "SchemaID")]
    pub schema_id: String,
    /// Show Validation Errors
    /// Represents the following attribute in the schema: :ShowImportExportValidationErrors
    #[xml(attr = "ShowImportExportValidationErrors")]
    pub show_import_export_errors: bool,
    /// AutoFit Table on Refresh
    /// Represents the following attribute in the schema: :AutoFit
    #[xml(attr = "AutoFit")]
    pub auto_fit: bool,
    /// Append Data to Table
    /// Represents the following attribute in the schema: :Append
    #[xml(attr = "Append")]
    pub append_data: bool,
    /// Preserve AutoFilter State
    /// Represents the following attribute in the schema: :PreserveSortAFLayout
    #[xml(attr = "PreserveSortAFLayout")]
    pub preserve_auto_filter_state: bool,
    /// Preserve Cell Formatting
    /// Represents the following attribute in the schema: :PreserveFormat
    #[xml(attr = "PreserveFormat")]
    pub preserve_format: bool,
    ///XML Mapping
    #[xml(child = "x:DataBinding")]
    pub data_binding: Option<DataBinding>,
}
/// XML Mapping.
/// When the object is serialized out as xml, it's qualified name is x:DataBinding.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:DataBinding")]
pub struct DataBinding {
    /// DataBindingName
    /// Represents the following attribute in the schema: :DataBindingName
    #[xml(attr = "DataBindingName")]
    pub data_binding_name: Option<String>,
    /// FileBinding
    /// Represents the following attribute in the schema: :FileBinding
    #[xml(attr = "FileBinding")]
    pub file_binding: Option<bool>,
    /// ConnectionID
    /// Represents the following attribute in the schema: :ConnectionID
    #[xml(attr = "ConnectionID")]
    pub connection_id: Option<i32>,
    /// FileBindingName
    /// Represents the following attribute in the schema: :FileBindingName
    #[xml(attr = "FileBindingName")]
    pub file_binding_name: Option<String>,
    /// DataBindingLoadMode
    /// Represents the following attribute in the schema: :DataBindingLoadMode
    #[xml(attr = "DataBindingLoadMode")]
    pub data_binding_load_mode: i32,
}
/// Connection.
/// When the object is serialized out as xml, it's qualified name is x:connection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:connection")]
pub struct Connection {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// sourceFile
    /// Represents the following attribute in the schema: :sourceFile
    #[xml(attr = "sourceFile")]
    pub source_file: Option<String>,
    /// odcFile
    /// Represents the following attribute in the schema: :odcFile
    #[xml(attr = "odcFile")]
    pub connection_file: Option<String>,
    /// keepAlive
    /// Represents the following attribute in the schema: :keepAlive
    #[xml(attr = "keepAlive")]
    pub keep_alive: Option<bool>,
    /// interval
    /// Represents the following attribute in the schema: :interval
    #[xml(attr = "interval")]
    pub interval: Option<i32>,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<i32>,
    /// reconnectionMethod
    /// Represents the following attribute in the schema: :reconnectionMethod
    #[xml(attr = "reconnectionMethod")]
    pub reconnection_method: Option<i32>,
    /// refreshedVersion
    /// Represents the following attribute in the schema: :refreshedVersion
    #[xml(attr = "refreshedVersion")]
    pub refreshed_version: u8,
    /// minRefreshableVersion
    /// Represents the following attribute in the schema: :minRefreshableVersion
    #[xml(attr = "minRefreshableVersion")]
    pub min_refreshable_version: Option<u8>,
    /// savePassword
    /// Represents the following attribute in the schema: :savePassword
    #[xml(attr = "savePassword")]
    pub save_password: Option<bool>,
    /// new
    /// Represents the following attribute in the schema: :new
    #[xml(attr = "new")]
    pub new: Option<bool>,
    /// deleted
    /// Represents the following attribute in the schema: :deleted
    #[xml(attr = "deleted")]
    pub deleted: Option<bool>,
    /// onlyUseConnectionFile
    /// Represents the following attribute in the schema: :onlyUseConnectionFile
    #[xml(attr = "onlyUseConnectionFile")]
    pub only_use_connection_file: Option<bool>,
    /// background
    /// Represents the following attribute in the schema: :background
    #[xml(attr = "background")]
    pub background: Option<bool>,
    /// refreshOnLoad
    /// Represents the following attribute in the schema: :refreshOnLoad
    #[xml(attr = "refreshOnLoad")]
    pub refresh_on_load: Option<bool>,
    /// saveData
    /// Represents the following attribute in the schema: :saveData
    #[xml(attr = "saveData")]
    pub save_data: Option<bool>,
    /// credentials
    /// Represents the following attribute in the schema: :credentials
    #[xml(attr = "credentials")]
    pub credentials: Option<CredentialsMethodValues>,
    /// singleSignOnId
    /// Represents the following attribute in the schema: :singleSignOnId
    #[xml(attr = "singleSignOnId")]
    pub single_sign_on_id: Option<String>,
    /// _
    #[xml(child = "x:dbPr")]
    pub database_properties: Option<DatabaseProperties>,
    /// _
    #[xml(child = "x:olapPr")]
    pub olap_properties: Option<OlapProperties>,
    /// _
    #[xml(child = "x:webPr")]
    pub web_query_properties: Option<WebQueryProperties>,
    /// _
    #[xml(child = "x:textPr")]
    pub text_properties: Option<TextProperties>,
    /// _
    #[xml(child = "x:parameters")]
    pub parameters: Option<Parameters>,
    /// _
    #[xml(child = "x:extLst")]
    pub connection_extension_list: Option<ConnectionExtensionList>,
}
/// Tables.
/// When the object is serialized out as xml, it's qualified name is x:tables.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:tables")]
pub struct Tables {
    /// Count of Tables
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    #[xml(child = "x:m", child = "x:s", child = "x:x")]
    pub children: Vec<TablesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TablesChildChoice {
    #[xml(tag = "x:m")]
    XM(MissingTable),
    #[xml(tag = "x:s")]
    XS(CharacterValue),
    #[xml(tag = "x:x")]
    XX(FieldItem),
}
/// Parameter Properties.
/// When the object is serialized out as xml, it's qualified name is x:parameter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:parameter")]
pub struct Parameter {
    /// Parameter Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// SQL Data Type
    /// Represents the following attribute in the schema: :sqlType
    #[xml(attr = "sqlType")]
    pub sql_type: Option<i32>,
    /// Parameter Type
    /// Represents the following attribute in the schema: :parameterType
    #[xml(attr = "parameterType")]
    pub parameter_type: Option<ParameterValues>,
    /// Refresh on Change
    /// Represents the following attribute in the schema: :refreshOnChange
    #[xml(attr = "refreshOnChange")]
    pub refresh_on_change: Option<bool>,
    /// Parameter Prompt String
    /// Represents the following attribute in the schema: :prompt
    #[xml(attr = "prompt")]
    pub prompt: Option<String>,
    /// Boolean
    /// Represents the following attribute in the schema: :boolean
    #[xml(attr = "boolean")]
    pub boolean: Option<bool>,
    /// Double
    /// Represents the following attribute in the schema: :double
    #[xml(attr = "double")]
    pub double: Option<f64>,
    /// Integer
    /// Represents the following attribute in the schema: :integer
    #[xml(attr = "integer")]
    pub integer: Option<i32>,
    /// String
    /// Represents the following attribute in the schema: :string
    #[xml(attr = "string")]
    pub string: Option<String>,
    /// Cell Reference
    /// Represents the following attribute in the schema: :cell
    #[xml(attr = "cell")]
    pub cell: Option<String>,
}
/// No Value.
/// When the object is serialized out as xml, it's qualified name is x:m.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:m")]
pub struct MissingTable {}
/// Character Value.
/// When the object is serialized out as xml, it's qualified name is x:s.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:s")]
pub struct CharacterValue {
    /// Value
    /// Represents the following attribute in the schema: :v
    #[xml(attr = "v")]
    pub val: String,
}
/// Index.
/// When the object is serialized out as xml, it's qualified name is x:x.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:x")]
pub struct FieldItem {
    /// Shared Items Index
    /// Represents the following attribute in the schema: :v
    #[xml(attr = "v")]
    pub val: i32,
}
/// Text Import Field Settings.
/// When the object is serialized out as xml, it's qualified name is x:textField.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:textField")]
pub struct TextField {
    /// Field Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<ExternalConnectionValues>,
    /// Position
    /// Represents the following attribute in the schema: :position
    #[xml(attr = "position")]
    pub position: Option<i32>,
}
/// PivotCache Field.
/// When the object is serialized out as xml, it's qualified name is x:cacheField.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cacheField")]
pub struct CacheField {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// caption
    /// Represents the following attribute in the schema: :caption
    #[xml(attr = "caption")]
    pub caption: Option<String>,
    /// propertyName
    /// Represents the following attribute in the schema: :propertyName
    #[xml(attr = "propertyName")]
    pub property_name: Option<String>,
    /// serverField
    /// Represents the following attribute in the schema: :serverField
    #[xml(attr = "serverField")]
    pub server_field: Option<bool>,
    /// uniqueList
    /// Represents the following attribute in the schema: :uniqueList
    #[xml(attr = "uniqueList")]
    pub unique_list: Option<bool>,
    /// numFmtId
    /// Represents the following attribute in the schema: :numFmtId
    #[xml(attr = "numFmtId")]
    pub number_format_id: Option<i32>,
    /// formula
    /// Represents the following attribute in the schema: :formula
    #[xml(attr = "formula")]
    pub formula: Option<String>,
    /// sqlType
    /// Represents the following attribute in the schema: :sqlType
    #[xml(attr = "sqlType")]
    pub sql_type: Option<i32>,
    /// hierarchy
    /// Represents the following attribute in the schema: :hierarchy
    #[xml(attr = "hierarchy")]
    pub hierarchy: Option<i32>,
    /// level
    /// Represents the following attribute in the schema: :level
    #[xml(attr = "level")]
    pub level: Option<i32>,
    /// databaseField
    /// Represents the following attribute in the schema: :databaseField
    #[xml(attr = "databaseField")]
    pub database_field: Option<bool>,
    /// mappingCount
    /// Represents the following attribute in the schema: :mappingCount
    #[xml(attr = "mappingCount")]
    pub mapping_count: Option<i32>,
    /// memberPropertyField
    /// Represents the following attribute in the schema: :memberPropertyField
    #[xml(attr = "memberPropertyField")]
    pub member_property_field: Option<bool>,
    /// _
    #[xml(child = "x:sharedItems")]
    pub shared_items: Option<SharedItems>,
    /// _
    #[xml(child = "x:fieldGroup")]
    pub field_group: Option<FieldGroup>,
    /// _
    #[xml(child = "x:mpMap")]
    pub x_mp_map: Vec<MemberPropertiesMap>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<CacheFieldExtensionList>,
}
/// Page Item Values.
/// When the object is serialized out as xml, it's qualified name is x:pages.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pages")]
pub struct Pages {
    /// Page Item String Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:page")]
    pub x_page: Vec<Page>,
}
/// Range Sets.
/// When the object is serialized out as xml, it's qualified name is x:rangeSets.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rangeSets")]
pub struct RangeSets {
    /// Reference and Page Item Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:rangeSet")]
    pub x_range_set: Vec<RangeSet>,
}
/// Page Items.
/// When the object is serialized out as xml, it's qualified name is x:page.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:page")]
pub struct Page {
    /// Page Item String Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:pageItem")]
    pub x_page_item: Vec<PageItem>,
}
/// Page Item.
/// When the object is serialized out as xml, it's qualified name is x:pageItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pageItem")]
pub struct PageItem {
    /// Page Item Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
}
/// Range Set.
/// When the object is serialized out as xml, it's qualified name is x:rangeSet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rangeSet")]
pub struct RangeSet {
    /// Field Item Index Page 1
    /// Represents the following attribute in the schema: :i1
    #[xml(attr = "i1")]
    pub field_item_index_page1: Option<i32>,
    /// Field Item Index Page 2
    /// Represents the following attribute in the schema: :i2
    #[xml(attr = "i2")]
    pub field_item_index_page2: Option<i32>,
    /// Field Item index Page 3
    /// Represents the following attribute in the schema: :i3
    #[xml(attr = "i3")]
    pub field_item_index_page3: Option<i32>,
    /// Field Item Index Page 4
    /// Represents the following attribute in the schema: :i4
    #[xml(attr = "i4")]
    pub field_item_index_page4: Option<i32>,
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: Option<String>,
    /// Named Range
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Sheet Name
    /// Represents the following attribute in the schema: :sheet
    #[xml(attr = "sheet")]
    pub sheet: Option<String>,
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
}
/// No Value.
/// When the object is serialized out as xml, it's qualified name is x:m.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:m")]
pub struct MissingItem {
    /// Unused Item
    /// Represents the following attribute in the schema: :u
    #[xml(attr = "u")]
    pub unused: Option<bool>,
    /// Calculated Item
    /// Represents the following attribute in the schema: :f
    #[xml(attr = "f")]
    pub calculated: Option<bool>,
    /// Caption
    /// Represents the following attribute in the schema: :c
    #[xml(attr = "c")]
    pub caption: Option<String>,
    /// Member Property Count
    /// Represents the following attribute in the schema: :cp
    #[xml(attr = "cp")]
    pub property_count: Option<i32>,
    /// Format Index
    /// Represents the following attribute in the schema: :in
    #[xml(attr = "in")]
    pub format_index: Option<i32>,
    /// background Color
    /// Represents the following attribute in the schema: :bc
    #[xml(attr = "bc")]
    pub background_color: Option<String>,
    /// Foreground Color
    /// Represents the following attribute in the schema: :fc
    #[xml(attr = "fc")]
    pub foreground_color: Option<String>,
    /// Italic
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub italic: Option<bool>,
    /// Underline
    /// Represents the following attribute in the schema: :un
    #[xml(attr = "un")]
    pub underline: Option<bool>,
    /// Strikethrough
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub strikethrough: Option<bool>,
    /// Bold
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bold: Option<bool>,
    /// _
    #[xml(child = "x:tpls")]
    pub x_tpls: Vec<Tuples>,
    /// _
    #[xml(child = "x:x")]
    pub x_x: Vec<MemberPropertyIndex>,
}
/// Numeric.
/// When the object is serialized out as xml, it's qualified name is x:n.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:n")]
pub struct NumberItem {
    /// Value
    /// Represents the following attribute in the schema: :v
    #[xml(attr = "v")]
    pub val: f64,
    /// Unused Item
    /// Represents the following attribute in the schema: :u
    #[xml(attr = "u")]
    pub unused: Option<bool>,
    /// Calculated Item
    /// Represents the following attribute in the schema: :f
    #[xml(attr = "f")]
    pub calculated: Option<bool>,
    /// Caption
    /// Represents the following attribute in the schema: :c
    #[xml(attr = "c")]
    pub caption: Option<String>,
    /// Member Property Count
    /// Represents the following attribute in the schema: :cp
    #[xml(attr = "cp")]
    pub property_count: Option<i32>,
    /// Format Index
    /// Represents the following attribute in the schema: :in
    #[xml(attr = "in")]
    pub format_index: Option<i32>,
    /// Background Color
    /// Represents the following attribute in the schema: :bc
    #[xml(attr = "bc")]
    pub background_color: Option<String>,
    /// Foreground Color
    /// Represents the following attribute in the schema: :fc
    #[xml(attr = "fc")]
    pub foreground_color: Option<String>,
    /// Italic
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub italic: Option<bool>,
    /// Underline
    /// Represents the following attribute in the schema: :un
    #[xml(attr = "un")]
    pub underline: Option<bool>,
    /// Strikethrough
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub strikethrough: Option<bool>,
    /// Bold
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bold: Option<bool>,
    /// _
    #[xml(child = "x:tpls")]
    pub x_tpls: Vec<Tuples>,
    /// _
    #[xml(child = "x:x")]
    pub x_x: Vec<MemberPropertyIndex>,
}
/// Boolean.
/// When the object is serialized out as xml, it's qualified name is x:b.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:b")]
pub struct BooleanItem {
    /// Value
    /// Represents the following attribute in the schema: :v
    #[xml(attr = "v")]
    pub val: bool,
    /// Unused Item
    /// Represents the following attribute in the schema: :u
    #[xml(attr = "u")]
    pub unused: Option<bool>,
    /// Calculated Item
    /// Represents the following attribute in the schema: :f
    #[xml(attr = "f")]
    pub calculated: Option<bool>,
    /// Caption
    /// Represents the following attribute in the schema: :c
    #[xml(attr = "c")]
    pub caption: Option<String>,
    /// Member Property Count
    /// Represents the following attribute in the schema: :cp
    #[xml(attr = "cp")]
    pub property_count: Option<i32>,
    /// _
    #[xml(child = "x:x")]
    pub x_x: Vec<MemberPropertyIndex>,
}
/// Error Value.
/// When the object is serialized out as xml, it's qualified name is x:e.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:e")]
pub struct ErrorItem {
    /// Value
    /// Represents the following attribute in the schema: :v
    #[xml(attr = "v")]
    pub val: String,
    /// Unused Item
    /// Represents the following attribute in the schema: :u
    #[xml(attr = "u")]
    pub unused: Option<bool>,
    /// Calculated Item
    /// Represents the following attribute in the schema: :f
    #[xml(attr = "f")]
    pub calculated: Option<bool>,
    /// Item Caption
    /// Represents the following attribute in the schema: :c
    #[xml(attr = "c")]
    pub caption: Option<String>,
    /// Member Property Count
    /// Represents the following attribute in the schema: :cp
    #[xml(attr = "cp")]
    pub property_count: Option<i32>,
    /// Format Index
    /// Represents the following attribute in the schema: :in
    #[xml(attr = "in")]
    pub format_index: Option<i32>,
    /// background Color
    /// Represents the following attribute in the schema: :bc
    #[xml(attr = "bc")]
    pub background_color: Option<String>,
    /// Foreground Color
    /// Represents the following attribute in the schema: :fc
    #[xml(attr = "fc")]
    pub foreground_color: Option<String>,
    /// Italic
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub italic: Option<bool>,
    /// Underline
    /// Represents the following attribute in the schema: :un
    #[xml(attr = "un")]
    pub underline: Option<bool>,
    /// Strikethrough
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub strikethrough: Option<bool>,
    /// Bold
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bold: Option<bool>,
    ///Tuples
    #[xml(child = "x:tpls")]
    pub tuples: Option<Tuples>,
    /// _
    #[xml(child = "x:x")]
    pub x_x: Vec<MemberPropertyIndex>,
}
/// Character Value.
/// When the object is serialized out as xml, it's qualified name is x:s.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:s")]
pub struct StringItem {
    /// Value
    /// Represents the following attribute in the schema: :v
    #[xml(attr = "v")]
    pub val: String,
    /// Unused Item
    /// Represents the following attribute in the schema: :u
    #[xml(attr = "u")]
    pub unused: Option<bool>,
    /// Calculated Item
    /// Represents the following attribute in the schema: :f
    #[xml(attr = "f")]
    pub calculated: Option<bool>,
    /// Item Caption
    /// Represents the following attribute in the schema: :c
    #[xml(attr = "c")]
    pub caption: Option<String>,
    /// Member Property Count
    /// Represents the following attribute in the schema: :cp
    #[xml(attr = "cp")]
    pub property_count: Option<i32>,
    /// Format Index
    /// Represents the following attribute in the schema: :in
    #[xml(attr = "in")]
    pub format_index: Option<i32>,
    /// Background Color
    /// Represents the following attribute in the schema: :bc
    #[xml(attr = "bc")]
    pub background_color: Option<String>,
    /// Foreground Color
    /// Represents the following attribute in the schema: :fc
    #[xml(attr = "fc")]
    pub foreground_color: Option<String>,
    /// Italic
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub italic: Option<bool>,
    /// Underline
    /// Represents the following attribute in the schema: :un
    #[xml(attr = "un")]
    pub underline: Option<bool>,
    /// Strikethrough
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub strikethrough: Option<bool>,
    /// Bold
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bold: Option<bool>,
    /// _
    #[xml(child = "x:tpls")]
    pub x_tpls: Vec<Tuples>,
    /// _
    #[xml(child = "x:x")]
    pub x_x: Vec<MemberPropertyIndex>,
}
/// Date Time.
/// When the object is serialized out as xml, it's qualified name is x:d.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:d")]
pub struct DateTimeItem {
    /// Value
    /// Represents the following attribute in the schema: :v
    #[xml(attr = "v")]
    pub val: String,
    /// Unused Item
    /// Represents the following attribute in the schema: :u
    #[xml(attr = "u")]
    pub unused: Option<bool>,
    /// Calculated Item Value
    /// Represents the following attribute in the schema: :f
    #[xml(attr = "f")]
    pub calculated: Option<bool>,
    /// Caption
    /// Represents the following attribute in the schema: :c
    #[xml(attr = "c")]
    pub caption: Option<String>,
    /// Member Property Count
    /// Represents the following attribute in the schema: :cp
    #[xml(attr = "cp")]
    pub property_count: Option<i32>,
    /// _
    #[xml(child = "x:x")]
    pub x_x: Vec<MemberPropertyIndex>,
}
/// Tuples.
/// When the object is serialized out as xml, it's qualified name is x:tpls.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:tpls")]
pub struct Tuples {
    /// Member Name Count
    /// Represents the following attribute in the schema: :c
    #[xml(attr = "c")]
    pub member_name_count: Option<i32>,
    #[xml(child = "x:tpl")]
    pub children: Vec<TuplesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TuplesChildChoice {
    #[xml(tag = "x:tpl")]
    XTpl(Tuple),
}
/// Sort By Tuple.
/// When the object is serialized out as xml, it's qualified name is x:sortByTuple.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sortByTuple")]
pub struct SortByTuple {
    /// Member Name Count
    /// Represents the following attribute in the schema: :c
    #[xml(attr = "c")]
    pub member_name_count: Option<i32>,
    #[xml(child = "x:tpl")]
    pub children: Vec<SortByTupleChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SortByTupleChildChoice {
    #[xml(tag = "x:tpl")]
    XTpl(Tuple),
}
/// Defines the TuplesType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TuplesType {
    /// Member Name Count
    /// Represents the following attribute in the schema: :c
    #[xml(attr = "c")]
    pub member_name_count: Option<i32>,
    #[xml(child = "x:tpl")]
    pub children: Vec<TuplesTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TuplesTypeChildChoice {
    #[xml(tag = "x:tpl")]
    XTpl(Tuple),
}
/// Member Property Indexes.
/// When the object is serialized out as xml, it's qualified name is x:x.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:x")]
pub struct MemberPropertyIndex {
    /// Shared Items Index
    /// Represents the following attribute in the schema: :v
    #[xml(attr = "v")]
    pub val: Option<i32>,
}
/// Defines the MemberPropertiesMap Class.
/// When the object is serialized out as xml, it's qualified name is x:mpMap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:mpMap")]
pub struct MemberPropertiesMap {
    /// Shared Items Index
    /// Represents the following attribute in the schema: :v
    #[xml(attr = "v")]
    pub val: Option<i32>,
}
/// Defines the XType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct XType {
    /// Shared Items Index
    /// Represents the following attribute in the schema: :v
    #[xml(attr = "v")]
    pub val: Option<i32>,
}
/// PivotCache Record.
/// When the object is serialized out as xml, it's qualified name is x:r.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:r")]
pub struct PivotCacheRecord {
    #[xml(
        child = "x:m",
        child = "x:n",
        child = "x:b",
        child = "x:e",
        child = "x:s",
        child = "x:d",
        child = "x:x",
    )]
    pub children: Vec<PivotCacheRecordChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PivotCacheRecordChildChoice {
    #[xml(tag = "x:m")]
    XM(MissingItem),
    #[xml(tag = "x:n")]
    XN(NumberItem),
    #[xml(tag = "x:b")]
    XB(BooleanItem),
    #[xml(tag = "x:e")]
    XE(ErrorItem),
    #[xml(tag = "x:s")]
    XS(StringItem),
    #[xml(tag = "x:d")]
    XD(DateTimeItem),
    #[xml(tag = "x:x")]
    XX(FieldItem),
}
/// OLAP KPI.
/// When the object is serialized out as xml, it's qualified name is x:kpi.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:kpi")]
pub struct Kpi {
    /// KPI Unique Name
    /// Represents the following attribute in the schema: :uniqueName
    #[xml(attr = "uniqueName")]
    pub unique_name: String,
    /// KPI Display Name
    /// Represents the following attribute in the schema: :caption
    #[xml(attr = "caption")]
    pub caption: String,
    /// KPI Display Folder
    /// Represents the following attribute in the schema: :displayFolder
    #[xml(attr = "displayFolder")]
    pub display_folder: Option<String>,
    /// KPI Measure Group Name
    /// Represents the following attribute in the schema: :measureGroup
    #[xml(attr = "measureGroup")]
    pub measure_group: Option<String>,
    /// Parent KPI
    /// Represents the following attribute in the schema: :parent
    #[xml(attr = "parent")]
    pub parent_kpi: Option<String>,
    /// KPI Value Unique Name
    /// Represents the following attribute in the schema: :value
    #[xml(attr = "value")]
    pub value: String,
    /// KPI Goal Unique Name
    /// Represents the following attribute in the schema: :goal
    #[xml(attr = "goal")]
    pub goal: Option<String>,
    /// KPI Status Unique Name
    /// Represents the following attribute in the schema: :status
    #[xml(attr = "status")]
    pub status: Option<String>,
    /// KPI Trend Unique Name
    /// Represents the following attribute in the schema: :trend
    #[xml(attr = "trend")]
    pub trend: Option<String>,
    /// KPI Weight Unique Name
    /// Represents the following attribute in the schema: :weight
    #[xml(attr = "weight")]
    pub weight: Option<String>,
}
/// PivotCache Field Id.
/// When the object is serialized out as xml, it's qualified name is x:fieldUsage.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:fieldUsage")]
pub struct FieldUsage {
    /// Field Index
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub index: i32,
}
/// OLAP Grouping Levels.
/// When the object is serialized out as xml, it's qualified name is x:groupLevel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:groupLevel")]
pub struct GroupLevel {
    /// Unique Name
    /// Represents the following attribute in the schema: :uniqueName
    #[xml(attr = "uniqueName")]
    pub unique_name: String,
    /// Grouping Level Display Name
    /// Represents the following attribute in the schema: :caption
    #[xml(attr = "caption")]
    pub caption: String,
    /// User-Defined Group Level
    /// Represents the following attribute in the schema: :user
    #[xml(attr = "user")]
    pub user: Option<bool>,
    /// Custom Roll Up
    /// Represents the following attribute in the schema: :customRollUp
    #[xml(attr = "customRollUp")]
    pub custom_roll_up: Option<bool>,
    ///OLAP Level Groups
    #[xml(child = "x:groups")]
    pub groups: Option<Groups>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// OLAP Level Groups.
/// When the object is serialized out as xml, it's qualified name is x:groups.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:groups")]
pub struct Groups {
    /// Level Group Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:group")]
    pub x_group: Vec<Group>,
}
/// OLAP Group.
/// When the object is serialized out as xml, it's qualified name is x:group.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:group")]
pub struct Group {
    /// Group Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Unique Group Name
    /// Represents the following attribute in the schema: :uniqueName
    #[xml(attr = "uniqueName")]
    pub unique_name: String,
    /// Group Caption
    /// Represents the following attribute in the schema: :caption
    #[xml(attr = "caption")]
    pub caption: String,
    /// Parent Unique Name
    /// Represents the following attribute in the schema: :uniqueParent
    #[xml(attr = "uniqueParent")]
    pub unique_parent: Option<String>,
    /// Group Id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<i32>,
    ///OLAP Group Members
    #[xml(child = "x:groupMembers")]
    pub group_members: GroupMembers,
}
/// OLAP Group Members.
/// When the object is serialized out as xml, it's qualified name is x:groupMembers.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:groupMembers")]
pub struct GroupMembers {
    /// Group Member Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:groupMember")]
    pub x_group_member: Vec<GroupMember>,
}
/// OLAP Group Member.
/// When the object is serialized out as xml, it's qualified name is x:groupMember.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:groupMember")]
pub struct GroupMember {
    /// Group Member Unique Name
    /// Represents the following attribute in the schema: :uniqueName
    #[xml(attr = "uniqueName")]
    pub unique_name: String,
    /// Group
    /// Represents the following attribute in the schema: :group
    #[xml(attr = "group")]
    pub group: Option<bool>,
}
/// Entries.
/// When the object is serialized out as xml, it's qualified name is x:entries.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:entries")]
pub struct Entries {
    /// Tuple Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    #[xml(child = "x:m", child = "x:n", child = "x:e", child = "x:s")]
    pub children: Vec<EntriesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EntriesChildChoice {
    #[xml(tag = "x:m")]
    XM(MissingItem),
    #[xml(tag = "x:n")]
    XN(NumberItem),
    #[xml(tag = "x:e")]
    XE(ErrorItem),
    #[xml(tag = "x:s")]
    XS(StringItem),
}
/// Sets.
/// When the object is serialized out as xml, it's qualified name is x:sets.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sets")]
pub struct Sets {
    /// Tuple Set Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:set")]
    pub x_set: Vec<TupleSet>,
}
/// OLAP Query Cache.
/// When the object is serialized out as xml, it's qualified name is x:queryCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:queryCache")]
pub struct QueryCache {
    /// Cached Query Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:query")]
    pub x_query: Vec<Query>,
}
/// Server Formats.
/// When the object is serialized out as xml, it's qualified name is x:serverFormats.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:serverFormats")]
pub struct ServerFormats {
    /// Format Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:serverFormat")]
    pub x_server_format: Vec<ServerFormat>,
}
/// Server Format.
/// When the object is serialized out as xml, it's qualified name is x:serverFormat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:serverFormat")]
pub struct ServerFormat {
    /// Culture
    /// Represents the following attribute in the schema: :culture
    #[xml(attr = "culture")]
    pub culture: Option<String>,
    /// Format
    /// Represents the following attribute in the schema: :format
    #[xml(attr = "format")]
    pub format: Option<String>,
}
/// Tuple.
/// When the object is serialized out as xml, it's qualified name is x:tpl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:tpl")]
pub struct Tuple {
    /// Field Index
    /// Represents the following attribute in the schema: :fld
    #[xml(attr = "fld")]
    pub field: Option<i32>,
    /// Hierarchy Index
    /// Represents the following attribute in the schema: :hier
    #[xml(attr = "hier")]
    pub hierarchy: Option<i32>,
    /// Item Index
    /// Represents the following attribute in the schema: :item
    #[xml(attr = "item")]
    pub item: i32,
}
/// OLAP Set.
/// When the object is serialized out as xml, it's qualified name is x:set.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:set")]
pub struct TupleSet {
    /// Number of Tuples
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// Maximum Rank Requested
    /// Represents the following attribute in the schema: :maxRank
    #[xml(attr = "maxRank")]
    pub max_rank: i32,
    /// MDX Set Definition
    /// Represents the following attribute in the schema: :setDefinition
    #[xml(attr = "setDefinition")]
    pub set_definition: String,
    /// Set Sort Order
    /// Represents the following attribute in the schema: :sortType
    #[xml(attr = "sortType")]
    pub sort_type: Option<SortValues>,
    /// Query Failed
    /// Represents the following attribute in the schema: :queryFailed
    #[xml(attr = "queryFailed")]
    pub query_failed: Option<bool>,
    /// _
    #[xml(child = "x:tpls")]
    pub x_tpls: Vec<Tuples>,
    /// _
    #[xml(child = "x:sortByTuple")]
    pub x_sort_by_tuple: Option<SortByTuple>,
}
/// Query.
/// When the object is serialized out as xml, it's qualified name is x:query.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:query")]
pub struct Query {
    /// MDX Query String
    /// Represents the following attribute in the schema: :mdx
    #[xml(attr = "mdx")]
    pub mdx: String,
    ///Tuples
    #[xml(child = "x:tpls")]
    pub tuples: Option<Tuples>,
}
/// Calculated Item.
/// When the object is serialized out as xml, it's qualified name is x:calculatedItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:calculatedItem")]
pub struct CalculatedItem {
    /// Field Index
    /// Represents the following attribute in the schema: :field
    #[xml(attr = "field")]
    pub field: Option<i32>,
    /// Calculated Item Formula
    /// Represents the following attribute in the schema: :formula
    #[xml(attr = "formula")]
    pub formula: Option<String>,
    ///Calculated Item Location
    #[xml(child = "x:pivotArea")]
    pub pivot_area: PivotArea,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Calculated Item Location.
/// When the object is serialized out as xml, it's qualified name is x:pivotArea.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pivotArea")]
pub struct PivotArea {
    /// Field Index
    /// Represents the following attribute in the schema: :field
    #[xml(attr = "field")]
    pub field: Option<i32>,
    /// Rule Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<PivotAreaValues>,
    /// Data Only
    /// Represents the following attribute in the schema: :dataOnly
    #[xml(attr = "dataOnly")]
    pub data_only: Option<bool>,
    /// Labels Only
    /// Represents the following attribute in the schema: :labelOnly
    #[xml(attr = "labelOnly")]
    pub label_only: Option<bool>,
    /// Include Row Grand Total
    /// Represents the following attribute in the schema: :grandRow
    #[xml(attr = "grandRow")]
    pub grand_row: Option<bool>,
    /// Include Column Grand Total
    /// Represents the following attribute in the schema: :grandCol
    #[xml(attr = "grandCol")]
    pub grand_column: Option<bool>,
    /// Cache Index
    /// Represents the following attribute in the schema: :cacheIndex
    #[xml(attr = "cacheIndex")]
    pub cache_index: Option<bool>,
    /// Outline
    /// Represents the following attribute in the schema: :outline
    #[xml(attr = "outline")]
    pub outline: Option<bool>,
    /// Offset Reference
    /// Represents the following attribute in the schema: :offset
    #[xml(attr = "offset")]
    pub offset: Option<String>,
    /// Collapsed Levels Are Subtotals
    /// Represents the following attribute in the schema: :collapsedLevelsAreSubtotals
    #[xml(attr = "collapsedLevelsAreSubtotals")]
    pub collapsed_levels_are_subtotals: Option<bool>,
    /// Axis
    /// Represents the following attribute in the schema: :axis
    #[xml(attr = "axis")]
    pub axis: Option<PivotTableAxisValues>,
    /// Field Position
    /// Represents the following attribute in the schema: :fieldPosition
    #[xml(attr = "fieldPosition")]
    pub field_position: Option<i32>,
    ///References
    #[xml(child = "x:references")]
    pub pivot_area_references: Option<PivotAreaReferences>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Calculated Member.
/// When the object is serialized out as xml, it's qualified name is x:calculatedMember.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:calculatedMember")]
pub struct CalculatedMember {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// mdx
    /// Represents the following attribute in the schema: :mdx
    #[xml(attr = "mdx")]
    pub mdx: String,
    /// memberName
    /// Represents the following attribute in the schema: :memberName
    #[xml(attr = "memberName")]
    pub member_name: Option<String>,
    /// hierarchy
    /// Represents the following attribute in the schema: :hierarchy
    #[xml(attr = "hierarchy")]
    pub hierarchy: Option<String>,
    /// parent
    /// Represents the following attribute in the schema: :parent
    #[xml(attr = "parent")]
    pub parent_name: Option<String>,
    /// solveOrder
    /// Represents the following attribute in the schema: :solveOrder
    #[xml(attr = "solveOrder")]
    pub solve_order: Option<i32>,
    /// set
    /// Represents the following attribute in the schema: :set
    #[xml(attr = "set")]
    pub set: Option<bool>,
    /// _
    #[xml(child = "x:extLst")]
    pub calculated_member_extension_list: Option<CalculatedMemberExtensionList>,
}
/// PivotTable Field.
/// When the object is serialized out as xml, it's qualified name is x:pivotField.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pivotField")]
pub struct PivotField {
    /// Field Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Axis
    /// Represents the following attribute in the schema: :axis
    #[xml(attr = "axis")]
    pub axis: Option<PivotTableAxisValues>,
    /// Data Field
    /// Represents the following attribute in the schema: :dataField
    #[xml(attr = "dataField")]
    pub data_field: Option<bool>,
    /// Custom Subtotal Caption
    /// Represents the following attribute in the schema: :subtotalCaption
    #[xml(attr = "subtotalCaption")]
    pub subtotal_caption: Option<String>,
    /// Show PivotField Header Drop Downs
    /// Represents the following attribute in the schema: :showDropDowns
    #[xml(attr = "showDropDowns")]
    pub show_drop_downs: Option<bool>,
    /// Hidden Level
    /// Represents the following attribute in the schema: :hiddenLevel
    #[xml(attr = "hiddenLevel")]
    pub hidden_level: Option<bool>,
    /// Unique Member Property
    /// Represents the following attribute in the schema: :uniqueMemberProperty
    #[xml(attr = "uniqueMemberProperty")]
    pub unique_member_property: Option<String>,
    /// Compact
    /// Represents the following attribute in the schema: :compact
    #[xml(attr = "compact")]
    pub compact: Option<bool>,
    /// All Items Expanded
    /// Represents the following attribute in the schema: :allDrilled
    #[xml(attr = "allDrilled")]
    pub all_drilled: Option<bool>,
    /// Number Format Id
    /// Represents the following attribute in the schema: :numFmtId
    #[xml(attr = "numFmtId")]
    pub number_format_id: Option<i32>,
    /// Outline Items
    /// Represents the following attribute in the schema: :outline
    #[xml(attr = "outline")]
    pub outline: Option<bool>,
    /// Subtotals At Top
    /// Represents the following attribute in the schema: :subtotalTop
    #[xml(attr = "subtotalTop")]
    pub subtotal_top: Option<bool>,
    /// Drag To Row
    /// Represents the following attribute in the schema: :dragToRow
    #[xml(attr = "dragToRow")]
    pub drag_to_row: Option<bool>,
    /// Drag To Column
    /// Represents the following attribute in the schema: :dragToCol
    #[xml(attr = "dragToCol")]
    pub drag_to_column: Option<bool>,
    /// Multiple Field Filters
    /// Represents the following attribute in the schema: :multipleItemSelectionAllowed
    #[xml(attr = "multipleItemSelectionAllowed")]
    pub multiple_item_selection_allowed: Option<bool>,
    /// Drag Field to Page
    /// Represents the following attribute in the schema: :dragToPage
    #[xml(attr = "dragToPage")]
    pub drag_to_page: Option<bool>,
    /// Field Can Drag to Data
    /// Represents the following attribute in the schema: :dragToData
    #[xml(attr = "dragToData")]
    pub drag_to_data: Option<bool>,
    /// Drag Off
    /// Represents the following attribute in the schema: :dragOff
    #[xml(attr = "dragOff")]
    pub drag_off: Option<bool>,
    /// Show All Items
    /// Represents the following attribute in the schema: :showAll
    #[xml(attr = "showAll")]
    pub show_all: Option<bool>,
    /// Insert Blank Row
    /// Represents the following attribute in the schema: :insertBlankRow
    #[xml(attr = "insertBlankRow")]
    pub insert_blank_row: Option<bool>,
    /// Server-based Page Field
    /// Represents the following attribute in the schema: :serverField
    #[xml(attr = "serverField")]
    pub server_field: Option<bool>,
    /// Insert Item Page Break
    /// Represents the following attribute in the schema: :insertPageBreak
    #[xml(attr = "insertPageBreak")]
    pub insert_page_break: Option<bool>,
    /// Auto Show
    /// Represents the following attribute in the schema: :autoShow
    #[xml(attr = "autoShow")]
    pub auto_show: Option<bool>,
    /// Top Auto Show
    /// Represents the following attribute in the schema: :topAutoShow
    #[xml(attr = "topAutoShow")]
    pub top_auto_show: Option<bool>,
    /// Hide New Items
    /// Represents the following attribute in the schema: :hideNewItems
    #[xml(attr = "hideNewItems")]
    pub hide_new_items: Option<bool>,
    /// Measure Filter
    /// Represents the following attribute in the schema: :measureFilter
    #[xml(attr = "measureFilter")]
    pub measure_filter: Option<bool>,
    /// Inclusive Manual Filter
    /// Represents the following attribute in the schema: :includeNewItemsInFilter
    #[xml(attr = "includeNewItemsInFilter")]
    pub include_new_items_in_filter: Option<bool>,
    /// Items Per Page Count
    /// Represents the following attribute in the schema: :itemPageCount
    #[xml(attr = "itemPageCount")]
    pub item_page_count: Option<i32>,
    /// Auto Sort Type
    /// Represents the following attribute in the schema: :sortType
    #[xml(attr = "sortType")]
    pub sort_type: Option<FieldSortValues>,
    /// Data Source Sort
    /// Represents the following attribute in the schema: :dataSourceSort
    #[xml(attr = "dataSourceSort")]
    pub data_source_sort: Option<bool>,
    /// Auto Sort
    /// Represents the following attribute in the schema: :nonAutoSortDefault
    #[xml(attr = "nonAutoSortDefault")]
    pub non_auto_sort_default: Option<bool>,
    /// Auto Show Rank By
    /// Represents the following attribute in the schema: :rankBy
    #[xml(attr = "rankBy")]
    pub rank_by: Option<i32>,
    /// Show Default Subtotal
    /// Represents the following attribute in the schema: :defaultSubtotal
    #[xml(attr = "defaultSubtotal")]
    pub default_subtotal: Option<bool>,
    /// Sum Subtotal
    /// Represents the following attribute in the schema: :sumSubtotal
    #[xml(attr = "sumSubtotal")]
    pub sum_subtotal: Option<bool>,
    /// CountA
    /// Represents the following attribute in the schema: :countASubtotal
    #[xml(attr = "countASubtotal")]
    pub count_a_subtotal: Option<bool>,
    /// Average
    /// Represents the following attribute in the schema: :avgSubtotal
    #[xml(attr = "avgSubtotal")]
    pub average_sub_total: Option<bool>,
    /// Max Subtotal
    /// Represents the following attribute in the schema: :maxSubtotal
    #[xml(attr = "maxSubtotal")]
    pub max_subtotal: Option<bool>,
    /// Min Subtotal
    /// Represents the following attribute in the schema: :minSubtotal
    #[xml(attr = "minSubtotal")]
    pub min_subtotal: Option<bool>,
    /// Product Subtotal
    /// Represents the following attribute in the schema: :productSubtotal
    #[xml(attr = "productSubtotal")]
    pub apply_product_in_subtotal: Option<bool>,
    /// Count
    /// Represents the following attribute in the schema: :countSubtotal
    #[xml(attr = "countSubtotal")]
    pub count_subtotal: Option<bool>,
    /// StdDev Subtotal
    /// Represents the following attribute in the schema: :stdDevSubtotal
    #[xml(attr = "stdDevSubtotal")]
    pub apply_standard_deviation_in_subtotal: Option<bool>,
    /// StdDevP Subtotal
    /// Represents the following attribute in the schema: :stdDevPSubtotal
    #[xml(attr = "stdDevPSubtotal")]
    pub apply_standard_deviation_p_in_subtotal: Option<bool>,
    /// Variance Subtotal
    /// Represents the following attribute in the schema: :varSubtotal
    #[xml(attr = "varSubtotal")]
    pub apply_variance_in_subtotal: Option<bool>,
    /// VarP Subtotal
    /// Represents the following attribute in the schema: :varPSubtotal
    #[xml(attr = "varPSubtotal")]
    pub apply_variance_p_in_subtotal: Option<bool>,
    /// Show Member Property in Cell
    /// Represents the following attribute in the schema: :showPropCell
    #[xml(attr = "showPropCell")]
    pub show_prop_cell: Option<bool>,
    /// Show Member Property ToolTip
    /// Represents the following attribute in the schema: :showPropTip
    #[xml(attr = "showPropTip")]
    pub show_property_tooltip: Option<bool>,
    /// Show As Caption
    /// Represents the following attribute in the schema: :showPropAsCaption
    #[xml(attr = "showPropAsCaption")]
    pub show_prop_as_caption: Option<bool>,
    /// Drill State
    /// Represents the following attribute in the schema: :defaultAttributeDrillState
    #[xml(attr = "defaultAttributeDrillState")]
    pub default_attribute_drill_state: Option<bool>,
    ///Field Items
    #[xml(child = "x:items")]
    pub items: Option<Items>,
    ///AutoSort Scope
    #[xml(child = "x:autoSortScope")]
    pub auto_sort_scope: Option<AutoSortScope>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub pivot_field_extension_list: Option<PivotFieldExtensionList>,
}
/// PivotTable Field Item.
/// When the object is serialized out as xml, it's qualified name is x:item.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:item")]
pub struct Item {
    /// Item User Caption
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub item_name: Option<String>,
    /// Item Type
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub item_type: Option<ItemValues>,
    /// Hidden
    /// Represents the following attribute in the schema: :h
    #[xml(attr = "h")]
    pub hidden: Option<bool>,
    /// Character
    /// Represents the following attribute in the schema: :s
    #[xml(attr = "s")]
    pub has_string_vlue: Option<bool>,
    /// Hide Details
    /// Represents the following attribute in the schema: :sd
    #[xml(attr = "sd")]
    pub hide_details: Option<bool>,
    /// Calculated Member
    /// Represents the following attribute in the schema: :f
    #[xml(attr = "f")]
    pub calculated: Option<bool>,
    /// Missing
    /// Represents the following attribute in the schema: :m
    #[xml(attr = "m")]
    pub missing: Option<bool>,
    /// Child Items
    /// Represents the following attribute in the schema: :c
    #[xml(attr = "c")]
    pub child_items: Option<bool>,
    /// Item Index
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub index: Option<i32>,
    /// Expanded
    /// Represents the following attribute in the schema: :d
    #[xml(attr = "d")]
    pub expanded: Option<bool>,
    /// Drill Across Attributes
    /// Represents the following attribute in the schema: :e
    #[xml(attr = "e")]
    pub drill_across_attributes: Option<bool>,
}
/// Data Field Item.
/// When the object is serialized out as xml, it's qualified name is x:dataField.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dataField")]
pub struct DataField {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// fld
    /// Represents the following attribute in the schema: :fld
    #[xml(attr = "fld")]
    pub field: i32,
    /// subtotal
    /// Represents the following attribute in the schema: :subtotal
    #[xml(attr = "subtotal")]
    pub subtotal: Option<DataConsolidateFunctionValues>,
    /// showDataAs
    /// Represents the following attribute in the schema: :showDataAs
    #[xml(attr = "showDataAs")]
    pub show_data_as: Option<ShowDataAsValues>,
    /// baseField
    /// Represents the following attribute in the schema: :baseField
    #[xml(attr = "baseField")]
    pub base_field: Option<i32>,
    /// baseItem
    /// Represents the following attribute in the schema: :baseItem
    #[xml(attr = "baseItem")]
    pub base_item: Option<i32>,
    /// numFmtId
    /// Represents the following attribute in the schema: :numFmtId
    #[xml(attr = "numFmtId")]
    pub number_format_id: Option<i32>,
    /// _
    #[xml(child = "x:extLst")]
    pub data_field_extension_list: Option<DataFieldExtensionList>,
}
/// Row Items.
/// When the object is serialized out as xml, it's qualified name is x:i.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:i")]
pub struct RowItem {
    /// Item Type
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub item_type: Option<ItemValues>,
    /// Repeated Items Count
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub repeated_item_count: Option<i32>,
    /// Data Field Index
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub index: Option<i32>,
    /// _
    #[xml(child = "x:x")]
    pub x_x: Vec<MemberPropertyIndex>,
}
/// Row Items.
/// When the object is serialized out as xml, it's qualified name is x:field.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:field")]
pub struct Field {
    /// Field Index
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub index: i32,
}
/// PivotTable Format.
/// When the object is serialized out as xml, it's qualified name is x:format.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:format")]
pub struct Format {
    /// Format Action
    /// Represents the following attribute in the schema: :action
    #[xml(attr = "action")]
    pub action: Option<FormatActionValues>,
    /// Format Id
    /// Represents the following attribute in the schema: :dxfId
    #[xml(attr = "dxfId")]
    pub format_id: Option<i32>,
    ///Pivot Table Location
    #[xml(child = "x:pivotArea")]
    pub pivot_area: PivotArea,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Conditional Formatting.
/// When the object is serialized out as xml, it's qualified name is x:conditionalFormat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:conditionalFormat")]
pub struct ConditionalFormat {
    /// Conditional Formatting Scope
    /// Represents the following attribute in the schema: :scope
    #[xml(attr = "scope")]
    pub scope: Option<ScopeValues>,
    /// Conditional Formatting Rule Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<RuleValues>,
    /// Priority
    /// Represents the following attribute in the schema: :priority
    #[xml(attr = "priority")]
    pub priority: i32,
    ///Pivot Areas
    #[xml(child = "x:pivotAreas")]
    pub pivot_areas: PivotAreas,
    /// _
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Pivot Areas.
/// When the object is serialized out as xml, it's qualified name is x:pivotAreas.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pivotAreas")]
pub struct PivotAreas {
    /// Pivot Area Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:pivotArea")]
    pub x_pivot_area: Vec<PivotArea>,
}
/// PivotChart Format.
/// When the object is serialized out as xml, it's qualified name is x:chartFormat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:chartFormat")]
pub struct ChartFormat {
    /// Chart Index
    /// Represents the following attribute in the schema: :chart
    #[xml(attr = "chart")]
    pub chart: i32,
    /// Pivot Format Id
    /// Represents the following attribute in the schema: :format
    #[xml(attr = "format")]
    pub format: i32,
    /// Series Format
    /// Represents the following attribute in the schema: :series
    #[xml(attr = "series")]
    pub series: Option<bool>,
    ///Pivot Table Location Rule
    #[xml(child = "x:pivotArea")]
    pub pivot_area: PivotArea,
}
/// OLAP Hierarchy.
/// When the object is serialized out as xml, it's qualified name is x:pivotHierarchy.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pivotHierarchy")]
pub struct PivotHierarchy {
    /// Outline New Levels
    /// Represents the following attribute in the schema: :outline
    #[xml(attr = "outline")]
    pub outline: Option<bool>,
    /// Multiple Field Filters
    /// Represents the following attribute in the schema: :multipleItemSelectionAllowed
    #[xml(attr = "multipleItemSelectionAllowed")]
    pub multiple_item_selection_allowed: Option<bool>,
    /// New Levels Subtotals At Top
    /// Represents the following attribute in the schema: :subtotalTop
    #[xml(attr = "subtotalTop")]
    pub subtotal_top: Option<bool>,
    /// Show In Field List
    /// Represents the following attribute in the schema: :showInFieldList
    #[xml(attr = "showInFieldList")]
    pub show_in_field_list: Option<bool>,
    /// Drag To Row
    /// Represents the following attribute in the schema: :dragToRow
    #[xml(attr = "dragToRow")]
    pub drag_to_row: Option<bool>,
    /// Drag To Column
    /// Represents the following attribute in the schema: :dragToCol
    #[xml(attr = "dragToCol")]
    pub drag_to_column: Option<bool>,
    /// Drag to Page
    /// Represents the following attribute in the schema: :dragToPage
    #[xml(attr = "dragToPage")]
    pub drag_to_page: Option<bool>,
    /// Drag To Data
    /// Represents the following attribute in the schema: :dragToData
    #[xml(attr = "dragToData")]
    pub drag_to_data: Option<bool>,
    /// Drag Off
    /// Represents the following attribute in the schema: :dragOff
    #[xml(attr = "dragOff")]
    pub drag_off: Option<bool>,
    /// Inclusive Manual Filter
    /// Represents the following attribute in the schema: :includeNewItemsInFilter
    #[xml(attr = "includeNewItemsInFilter")]
    pub include_new_items_in_filter: Option<bool>,
    /// Hierarchy Caption
    /// Represents the following attribute in the schema: :caption
    #[xml(attr = "caption")]
    pub caption: Option<String>,
    ///OLAP Member Properties
    #[xml(child = "x:mps")]
    pub member_properties: Option<MemberProperties>,
    /// _
    #[xml(child = "x:members")]
    pub x_members: Vec<Members>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<PivotHierarchyExtensionList>,
}
/// Row OLAP Hierarchies.
/// When the object is serialized out as xml, it's qualified name is x:rowHierarchyUsage.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rowHierarchyUsage")]
pub struct RowHierarchyUsage {
    /// Hierarchy Usage
    /// Represents the following attribute in the schema: :hierarchyUsage
    #[xml(attr = "hierarchyUsage")]
    pub value: i32,
}
/// Column OLAP Hierarchies.
/// When the object is serialized out as xml, it's qualified name is x:colHierarchyUsage.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:colHierarchyUsage")]
pub struct ColumnHierarchyUsage {
    /// Hierarchy Usage
    /// Represents the following attribute in the schema: :hierarchyUsage
    #[xml(attr = "hierarchyUsage")]
    pub value: i32,
}
/// Defines the HierarchyUsageType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct HierarchyUsageType {
    /// Hierarchy Usage
    /// Represents the following attribute in the schema: :hierarchyUsage
    #[xml(attr = "hierarchyUsage")]
    pub value: i32,
}
/// OLAP Member Property.
/// When the object is serialized out as xml, it's qualified name is x:mp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:mp")]
pub struct MemberProperty {
    /// OLAP Member Property Unique Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Show Cell
    /// Represents the following attribute in the schema: :showCell
    #[xml(attr = "showCell")]
    pub show_cell: Option<bool>,
    /// Show Tooltip
    /// Represents the following attribute in the schema: :showTip
    #[xml(attr = "showTip")]
    pub show_tip: Option<bool>,
    /// Show As Caption
    /// Represents the following attribute in the schema: :showAsCaption
    #[xml(attr = "showAsCaption")]
    pub show_as_caption: Option<bool>,
    /// Name Length
    /// Represents the following attribute in the schema: :nameLen
    #[xml(attr = "nameLen")]
    pub name_length: Option<i32>,
    /// Property Name Character Index
    /// Represents the following attribute in the schema: :pPos
    #[xml(attr = "pPos")]
    pub property_name_position: Option<i32>,
    /// Property Name Length
    /// Represents the following attribute in the schema: :pLen
    #[xml(attr = "pLen")]
    pub property_name_length: Option<i32>,
    /// Level Index
    /// Represents the following attribute in the schema: :level
    #[xml(attr = "level")]
    pub level: Option<i32>,
    /// Field Index
    /// Represents the following attribute in the schema: :field
    #[xml(attr = "field")]
    pub field: i32,
}
/// Member.
/// When the object is serialized out as xml, it's qualified name is x:member.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:member")]
pub struct Member {
    /// Hidden Item Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
}
/// OLAP Dimension.
/// When the object is serialized out as xml, it's qualified name is x:dimension.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dimension")]
pub struct Dimension {
    /// Measure
    /// Represents the following attribute in the schema: :measure
    #[xml(attr = "measure")]
    pub measure: Option<bool>,
    /// Dimension Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Dimension Unique Name
    /// Represents the following attribute in the schema: :uniqueName
    #[xml(attr = "uniqueName")]
    pub unique_name: String,
    /// Dimension Display Name
    /// Represents the following attribute in the schema: :caption
    #[xml(attr = "caption")]
    pub caption: String,
}
/// OLAP Measure Group.
/// When the object is serialized out as xml, it's qualified name is x:measureGroup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:measureGroup")]
pub struct MeasureGroup {
    /// Measure Group Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Measure Group Display Name
    /// Represents the following attribute in the schema: :caption
    #[xml(attr = "caption")]
    pub caption: String,
}
/// OLAP Measure Group.
/// When the object is serialized out as xml, it's qualified name is x:map.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:map")]
pub struct MeasureDimensionMap {
    /// Measure Group Id
    /// Represents the following attribute in the schema: :measureGroup
    #[xml(attr = "measureGroup")]
    pub measure_group: i32,
    /// Dimension Id
    /// Represents the following attribute in the schema: :dimension
    #[xml(attr = "dimension")]
    pub dimension: i32,
}
/// PivotTable Advanced Filter.
/// When the object is serialized out as xml, it's qualified name is x:filter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:filter")]
pub struct PivotFilter {
    /// fld
    /// Represents the following attribute in the schema: :fld
    #[xml(attr = "fld")]
    pub field: i32,
    /// mpFld
    /// Represents the following attribute in the schema: :mpFld
    #[xml(attr = "mpFld")]
    pub member_property_field_id: Option<i32>,
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: PivotFilterValues,
    /// evalOrder
    /// Represents the following attribute in the schema: :evalOrder
    #[xml(attr = "evalOrder")]
    pub evaluation_order: Option<i32>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// iMeasureHier
    /// Represents the following attribute in the schema: :iMeasureHier
    #[xml(attr = "iMeasureHier")]
    pub measure_hierarchy: Option<i32>,
    /// iMeasureFld
    /// Represents the following attribute in the schema: :iMeasureFld
    #[xml(attr = "iMeasureFld")]
    pub measure_field: Option<i32>,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// stringValue1
    /// Represents the following attribute in the schema: :stringValue1
    #[xml(attr = "stringValue1")]
    pub string_value1: Option<String>,
    /// stringValue2
    /// Represents the following attribute in the schema: :stringValue2
    #[xml(attr = "stringValue2")]
    pub string_value2: Option<String>,
    /// _
    #[xml(child = "x:autoFilter")]
    pub auto_filter: AutoFilter,
    /// _
    #[xml(child = "x:extLst")]
    pub pivot_filter_extension_list: Option<PivotFilterExtensionList>,
}
/// PivotCache Hierarchy.
/// When the object is serialized out as xml, it's qualified name is x:cacheHierarchy.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cacheHierarchy")]
pub struct CacheHierarchy {
    /// uniqueName
    /// Represents the following attribute in the schema: :uniqueName
    #[xml(attr = "uniqueName")]
    pub unique_name: String,
    /// caption
    /// Represents the following attribute in the schema: :caption
    #[xml(attr = "caption")]
    pub caption: Option<String>,
    /// measure
    /// Represents the following attribute in the schema: :measure
    #[xml(attr = "measure")]
    pub measure: Option<bool>,
    /// set
    /// Represents the following attribute in the schema: :set
    #[xml(attr = "set")]
    pub set: Option<bool>,
    /// parentSet
    /// Represents the following attribute in the schema: :parentSet
    #[xml(attr = "parentSet")]
    pub parent_set: Option<i32>,
    /// iconSet
    /// Represents the following attribute in the schema: :iconSet
    #[xml(attr = "iconSet")]
    pub icon_set: Option<i32>,
    /// attribute
    /// Represents the following attribute in the schema: :attribute
    #[xml(attr = "attribute")]
    pub attribute: Option<bool>,
    /// time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: Option<bool>,
    /// keyAttribute
    /// Represents the following attribute in the schema: :keyAttribute
    #[xml(attr = "keyAttribute")]
    pub key_attribute: Option<bool>,
    /// defaultMemberUniqueName
    /// Represents the following attribute in the schema: :defaultMemberUniqueName
    #[xml(attr = "defaultMemberUniqueName")]
    pub default_member_unique_name: Option<String>,
    /// allUniqueName
    /// Represents the following attribute in the schema: :allUniqueName
    #[xml(attr = "allUniqueName")]
    pub all_unique_name: Option<String>,
    /// allCaption
    /// Represents the following attribute in the schema: :allCaption
    #[xml(attr = "allCaption")]
    pub all_caption: Option<String>,
    /// dimensionUniqueName
    /// Represents the following attribute in the schema: :dimensionUniqueName
    #[xml(attr = "dimensionUniqueName")]
    pub dimension_unique_name: Option<String>,
    /// displayFolder
    /// Represents the following attribute in the schema: :displayFolder
    #[xml(attr = "displayFolder")]
    pub display_folder: Option<String>,
    /// measureGroup
    /// Represents the following attribute in the schema: :measureGroup
    #[xml(attr = "measureGroup")]
    pub measure_group: Option<String>,
    /// measures
    /// Represents the following attribute in the schema: :measures
    #[xml(attr = "measures")]
    pub measures: Option<bool>,
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: i32,
    /// oneField
    /// Represents the following attribute in the schema: :oneField
    #[xml(attr = "oneField")]
    pub one_field: Option<bool>,
    /// memberValueDatatype
    /// Represents the following attribute in the schema: :memberValueDatatype
    #[xml(attr = "memberValueDatatype")]
    pub member_value_datatype: Option<i16>,
    /// unbalanced
    /// Represents the following attribute in the schema: :unbalanced
    #[xml(attr = "unbalanced")]
    pub unbalanced: Option<bool>,
    /// unbalancedGroup
    /// Represents the following attribute in the schema: :unbalancedGroup
    #[xml(attr = "unbalancedGroup")]
    pub unbalanced_group: Option<bool>,
    /// hidden
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    /// _
    #[xml(child = "x:fieldsUsage")]
    pub fields_usage: Option<FieldsUsage>,
    /// _
    #[xml(child = "x:groupLevels")]
    pub group_levels: Option<GroupLevels>,
    /// _
    #[xml(child = "x:extLst")]
    pub cache_hierarchy_extension_list: Option<CacheHierarchyExtensionList>,
}
/// Range Grouping Properties.
/// When the object is serialized out as xml, it's qualified name is x:rangePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rangePr")]
pub struct RangeProperties {
    /// Source Data Set Beginning Range
    /// Represents the following attribute in the schema: :autoStart
    #[xml(attr = "autoStart")]
    pub auto_start: Option<bool>,
    /// Source Data Ending Range
    /// Represents the following attribute in the schema: :autoEnd
    #[xml(attr = "autoEnd")]
    pub auto_end: Option<bool>,
    /// Group By
    /// Represents the following attribute in the schema: :groupBy
    #[xml(attr = "groupBy")]
    pub group_by: Option<GroupByValues>,
    /// Numeric Grouping Start Value
    /// Represents the following attribute in the schema: :startNum
    #[xml(attr = "startNum")]
    pub start_number: Option<f64>,
    /// Numeric Grouping End Value
    /// Represents the following attribute in the schema: :endNum
    #[xml(attr = "endNum")]
    pub end_num: Option<f64>,
    /// Date Grouping Start Value
    /// Represents the following attribute in the schema: :startDate
    #[xml(attr = "startDate")]
    pub start_date: Option<String>,
    /// Date Grouping End Value
    /// Represents the following attribute in the schema: :endDate
    #[xml(attr = "endDate")]
    pub end_date: Option<String>,
    /// Grouping Interval
    /// Represents the following attribute in the schema: :groupInterval
    #[xml(attr = "groupInterval")]
    pub group_interval: Option<f64>,
}
/// Discrete Grouping Properties.
/// When the object is serialized out as xml, it's qualified name is x:discretePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:discretePr")]
pub struct DiscreteProperties {
    /// Mapping Index Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:x")]
    pub x_x: Vec<FieldItem>,
}
/// OLAP Group Items.
/// When the object is serialized out as xml, it's qualified name is x:groupItems.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:groupItems")]
pub struct GroupItems {
    /// Items Created Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    #[xml(
        child = "x:m",
        child = "x:n",
        child = "x:b",
        child = "x:e",
        child = "x:s",
        child = "x:d",
    )]
    pub children: Vec<GroupItemsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GroupItemsChildChoice {
    #[xml(tag = "x:m")]
    XM(MissingItem),
    #[xml(tag = "x:n")]
    XN(NumberItem),
    #[xml(tag = "x:b")]
    XB(BooleanItem),
    #[xml(tag = "x:e")]
    XE(ErrorItem),
    #[xml(tag = "x:s")]
    XS(StringItem),
    #[xml(tag = "x:d")]
    XD(DateTimeItem),
}
/// Page Field.
/// When the object is serialized out as xml, it's qualified name is x:pageField.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pageField")]
pub struct PageField {
    /// Field
    /// Represents the following attribute in the schema: :fld
    #[xml(attr = "fld")]
    pub field: i32,
    /// Item Index
    /// Represents the following attribute in the schema: :item
    #[xml(attr = "item")]
    pub item: Option<i32>,
    /// OLAP Hierarchy Index
    /// Represents the following attribute in the schema: :hier
    #[xml(attr = "hier")]
    pub hierarchy: Option<i32>,
    /// Hierarchy Unique Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Hierarchy Display Name
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub caption: Option<String>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// References.
/// When the object is serialized out as xml, it's qualified name is x:references.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:references")]
pub struct PivotAreaReferences {
    /// Pivot Filter Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:reference")]
    pub x_reference: Vec<PivotAreaReference>,
}
/// Reference.
/// When the object is serialized out as xml, it's qualified name is x:reference.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:reference")]
pub struct PivotAreaReference {
    /// Field Index
    /// Represents the following attribute in the schema: :field
    #[xml(attr = "field")]
    pub field: Option<i32>,
    /// Item Index Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// Selected
    /// Represents the following attribute in the schema: :selected
    #[xml(attr = "selected")]
    pub selected: Option<bool>,
    /// Positional Reference
    /// Represents the following attribute in the schema: :byPosition
    #[xml(attr = "byPosition")]
    pub by_position: Option<bool>,
    /// Relative Reference
    /// Represents the following attribute in the schema: :relative
    #[xml(attr = "relative")]
    pub relative: Option<bool>,
    /// Include Default Filter
    /// Represents the following attribute in the schema: :defaultSubtotal
    #[xml(attr = "defaultSubtotal")]
    pub default_subtotal: Option<bool>,
    /// Include Sum Filter
    /// Represents the following attribute in the schema: :sumSubtotal
    #[xml(attr = "sumSubtotal")]
    pub sum_subtotal: Option<bool>,
    /// Include CountA Filter
    /// Represents the following attribute in the schema: :countASubtotal
    #[xml(attr = "countASubtotal")]
    pub count_a_subtotal: Option<bool>,
    /// Include Average Filter
    /// Represents the following attribute in the schema: :avgSubtotal
    #[xml(attr = "avgSubtotal")]
    pub average_subtotal: Option<bool>,
    /// Include Maximum Filter
    /// Represents the following attribute in the schema: :maxSubtotal
    #[xml(attr = "maxSubtotal")]
    pub max_subtotal: Option<bool>,
    /// Include Minimum Filter
    /// Represents the following attribute in the schema: :minSubtotal
    #[xml(attr = "minSubtotal")]
    pub min_subtotal: Option<bool>,
    /// Include Product Filter
    /// Represents the following attribute in the schema: :productSubtotal
    #[xml(attr = "productSubtotal")]
    pub apply_product_in_subtotal: Option<bool>,
    /// Include Count Subtotal
    /// Represents the following attribute in the schema: :countSubtotal
    #[xml(attr = "countSubtotal")]
    pub count_subtotal: Option<bool>,
    /// Include StdDev Filter
    /// Represents the following attribute in the schema: :stdDevSubtotal
    #[xml(attr = "stdDevSubtotal")]
    pub apply_standard_deviation_in_subtotal: Option<bool>,
    /// Include StdDevP Filter
    /// Represents the following attribute in the schema: :stdDevPSubtotal
    #[xml(attr = "stdDevPSubtotal")]
    pub apply_standard_deviation_p_in_subtotal: Option<bool>,
    /// Include Var Filter
    /// Represents the following attribute in the schema: :varSubtotal
    #[xml(attr = "varSubtotal")]
    pub apply_variance_in_subtotal: Option<bool>,
    /// Include VarP Filter
    /// Represents the following attribute in the schema: :varPSubtotal
    #[xml(attr = "varPSubtotal")]
    pub apply_variance_p_in_subtotal: Option<bool>,
    /// _
    #[xml(child = "x:x")]
    pub x_x: Vec<FieldItem>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<ExtensionList>,
}
/// Query table fields.
/// When the object is serialized out as xml, it's qualified name is x:queryTableFields.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:queryTableFields")]
pub struct QueryTableFields {
    /// Column Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:queryTableField")]
    pub x_query_table_field: Vec<QueryTableField>,
}
/// Deleted Fields.
/// When the object is serialized out as xml, it's qualified name is x:queryTableDeletedFields.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:queryTableDeletedFields")]
pub struct QueryTableDeletedFields {
    /// Deleted Fields Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:deletedField")]
    pub x_deleted_field: Vec<DeletedField>,
}
/// Deleted Field.
/// When the object is serialized out as xml, it's qualified name is x:deletedField.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:deletedField")]
pub struct DeletedField {
    /// Deleted Fields Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
}
/// QueryTable Field.
/// When the object is serialized out as xml, it's qualified name is x:queryTableField.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:queryTableField")]
pub struct QueryTableField {
    /// Field Id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Data Bound Column
    /// Represents the following attribute in the schema: :dataBound
    #[xml(attr = "dataBound")]
    pub data_bound: Option<bool>,
    /// Row Numbers
    /// Represents the following attribute in the schema: :rowNumbers
    #[xml(attr = "rowNumbers")]
    pub row_numbers: Option<bool>,
    /// Fill This Formula On Refresh
    /// Represents the following attribute in the schema: :fillFormulas
    #[xml(attr = "fillFormulas")]
    pub fill_formulas: Option<bool>,
    /// Clipped Column
    /// Represents the following attribute in the schema: :clipped
    #[xml(attr = "clipped")]
    pub clipped: Option<bool>,
    /// Table Column Id
    /// Represents the following attribute in the schema: :tableColumnId
    #[xml(attr = "tableColumnId")]
    pub table_column_id: Option<i32>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// String Item.
/// When the object is serialized out as xml, it's qualified name is x:si.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:si")]
pub struct SharedStringItem {
    ///Text
    #[xml(child = "x:t")]
    pub text: Option<Text>,
    /// _
    #[xml(child = "x:r")]
    pub x_r: Vec<Run>,
    /// _
    #[xml(child = "x:rPh")]
    pub x_r_ph: Vec<PhoneticRun>,
    /// _
    #[xml(child = "x:phoneticPr")]
    pub x_phonetic_pr: Option<PhoneticProperties>,
}
/// Rich Text Inline.
/// When the object is serialized out as xml, it's qualified name is x:is.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:is")]
pub struct InlineString {
    ///Text
    #[xml(child = "x:t")]
    pub text: Option<Text>,
    /// _
    #[xml(child = "x:r")]
    pub x_r: Vec<Run>,
    /// _
    #[xml(child = "x:rPh")]
    pub x_r_ph: Vec<PhoneticRun>,
    /// _
    #[xml(child = "x:phoneticPr")]
    pub x_phonetic_pr: Option<PhoneticProperties>,
}
/// Comment Text.
/// When the object is serialized out as xml, it's qualified name is x:text.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:text")]
pub struct CommentText {
    ///Text
    #[xml(child = "x:t")]
    pub text: Option<Text>,
    /// _
    #[xml(child = "x:r")]
    pub x_r: Vec<Run>,
    /// _
    #[xml(child = "x:rPh")]
    pub x_r_ph: Vec<PhoneticRun>,
    /// _
    #[xml(child = "x:phoneticPr")]
    pub x_phonetic_pr: Option<PhoneticProperties>,
}
/// Defines the RstType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct RstType {}
/// Bold.
/// When the object is serialized out as xml, it's qualified name is x:b.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:b")]
pub struct Bold {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Italic.
/// When the object is serialized out as xml, it's qualified name is x:i.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:i")]
pub struct Italic {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Strike Through.
/// When the object is serialized out as xml, it's qualified name is x:strike.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:strike")]
pub struct Strike {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Condense.
/// When the object is serialized out as xml, it's qualified name is x:condense.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:condense")]
pub struct Condense {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Extend.
/// When the object is serialized out as xml, it's qualified name is x:extend.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extend")]
pub struct Extend {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Outline.
/// When the object is serialized out as xml, it's qualified name is x:outline.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:outline")]
pub struct Outline {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Shadow.
/// When the object is serialized out as xml, it's qualified name is x:shadow.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:shadow")]
pub struct Shadow {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the BooleanPropertyType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct BooleanPropertyType {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Underline.
/// When the object is serialized out as xml, it's qualified name is x:u.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:u")]
pub struct Underline {
    /// Underline Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<UnderlineValues>,
}
/// Vertical Alignment.
/// When the object is serialized out as xml, it's qualified name is x:vertAlign.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:vertAlign")]
pub struct VerticalTextAlignment {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: VerticalAlignmentRunValues,
}
/// Font Size.
/// When the object is serialized out as xml, it's qualified name is x:sz.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sz")]
pub struct FontSize {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Text Color.
/// When the object is serialized out as xml, it's qualified name is x:color.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:color")]
pub struct Color {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Sheet Tab Color.
/// When the object is serialized out as xml, it's qualified name is x:tabColor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:tabColor")]
pub struct TabColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Foreground Color.
/// When the object is serialized out as xml, it's qualified name is x:fgColor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:fgColor")]
pub struct ForegroundColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Background Color.
/// When the object is serialized out as xml, it's qualified name is x:bgColor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:bgColor")]
pub struct BackgroundColor {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Defines the ColorType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct ColorType {
    /// Automatic
    /// Represents the following attribute in the schema: :auto
    #[xml(attr = "auto")]
    pub auto: Option<bool>,
    /// Index
    /// Represents the following attribute in the schema: :indexed
    #[xml(attr = "indexed")]
    pub indexed: Option<i32>,
    /// Alpha Red Green Blue Color Value
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
    /// Theme Color
    /// Represents the following attribute in the schema: :theme
    #[xml(attr = "theme")]
    pub theme: Option<i32>,
    /// Tint
    /// Represents the following attribute in the schema: :tint
    #[xml(attr = "tint")]
    pub tint: Option<f64>,
}
/// Font.
/// When the object is serialized out as xml, it's qualified name is x:rFont.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rFont")]
pub struct RunFont {
    /// String Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Font Family.
/// When the object is serialized out as xml, it's qualified name is x:family.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:family")]
pub struct FontFamily {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Character Set.
/// When the object is serialized out as xml, it's qualified name is x:charset.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:charset")]
pub struct RunPropertyCharSet {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the InternationalPropertyType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct InternationalPropertyType {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Font Scheme.
/// When the object is serialized out as xml, it's qualified name is x:scheme.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:scheme")]
pub struct FontScheme {
    /// Font Scheme
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: FontSchemeValues,
}
/// Run Properties.
/// When the object is serialized out as xml, it's qualified name is x:rPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rPr")]
pub struct RunProperties {
    /// _
    #[xml(child = "x:b")]
    pub x_b: Option<Bold>,
    /// _
    #[xml(child = "x:i")]
    pub x_i: Option<Italic>,
    /// _
    #[xml(child = "x:strike")]
    pub x_strike: Option<Strike>,
    /// _
    #[xml(child = "x:condense")]
    pub x_condense: Option<Condense>,
    /// _
    #[xml(child = "x:extend")]
    pub x_extend: Option<Extend>,
    /// _
    #[xml(child = "x:outline")]
    pub x_outline: Option<Outline>,
    /// _
    #[xml(child = "x:shadow")]
    pub x_shadow: Option<Shadow>,
    /// _
    #[xml(child = "x:u")]
    pub x_u: Option<Underline>,
    /// _
    #[xml(child = "x:vertAlign")]
    pub x_vert_align: Option<VerticalTextAlignment>,
    /// _
    #[xml(child = "x:sz")]
    pub x_sz: Option<FontSize>,
    /// _
    #[xml(child = "x:color")]
    pub x_color: Option<Color>,
    /// _
    #[xml(child = "x:rFont")]
    pub x_r_font: Option<RunFont>,
    /// _
    #[xml(child = "x:family")]
    pub x_family: Option<FontFamily>,
    /// _
    #[xml(child = "x:charset")]
    pub x_charset: Option<RunPropertyCharSet>,
    /// _
    #[xml(child = "x:scheme")]
    pub x_scheme: Option<FontScheme>,
}
/// Rich Text Run.
/// When the object is serialized out as xml, it's qualified name is x:r.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:r")]
pub struct Run {
    ///Run Properties
    #[xml(child = "x:rPr")]
    pub run_properties: Option<RunProperties>,
    ///Text
    #[xml(child = "x:t")]
    pub text: Text,
}
/// Phonetic Run.
/// When the object is serialized out as xml, it's qualified name is x:rPh.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rPh")]
pub struct PhoneticRun {
    /// Base Text Start Index
    /// Represents the following attribute in the schema: :sb
    #[xml(attr = "sb")]
    pub base_text_start_index: i32,
    /// Base Text End Index
    /// Represents the following attribute in the schema: :eb
    #[xml(attr = "eb")]
    pub ending_base_index: i32,
    ///Text
    #[xml(child = "x:t")]
    pub text: Text,
}
/// Phonetic Properties.
/// When the object is serialized out as xml, it's qualified name is x:phoneticPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:phoneticPr")]
pub struct PhoneticProperties {
    /// Font Id
    /// Represents the following attribute in the schema: :fontId
    #[xml(attr = "fontId")]
    pub font_id: i32,
    /// Character Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<PhoneticValues>,
    /// Alignment
    /// Represents the following attribute in the schema: :alignment
    #[xml(attr = "alignment")]
    pub alignment: Option<PhoneticAlignmentValues>,
}
/// Header.
/// When the object is serialized out as xml, it's qualified name is x:header.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:header")]
pub struct Header {
    /// GUID
    /// Represents the following attribute in the schema: :guid
    #[xml(attr = "guid")]
    pub guid: String,
    /// Date Time
    /// Represents the following attribute in the schema: :dateTime
    #[xml(attr = "dateTime")]
    pub date_time: String,
    /// Last Sheet Id
    /// Represents the following attribute in the schema: :maxSheetId
    #[xml(attr = "maxSheetId")]
    pub max_sheet_id: i32,
    /// User Name
    /// Represents the following attribute in the schema: :userName
    #[xml(attr = "userName")]
    pub user_name: String,
    /// Relationship ID
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
    /// Minimum Revision Id
    /// Represents the following attribute in the schema: :minRId
    #[xml(attr = "minRId")]
    pub min_revision_id: Option<i32>,
    /// Max Revision Id
    /// Represents the following attribute in the schema: :maxRId
    #[xml(attr = "maxRId")]
    pub max_revision_id: Option<i32>,
    ///Sheet Id Map
    #[xml(child = "x:sheetIdMap")]
    pub sheet_id_map: SheetIdMap,
    ///Reviewed List
    #[xml(child = "x:reviewedList")]
    pub reviewed_list: Option<ReviewedList>,
    /// _
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Revision Row Column Insert Delete.
/// When the object is serialized out as xml, it's qualified name is x:rrc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rrc")]
pub struct RevisionRowColumn {
    /// Revision Id
    /// Represents the following attribute in the schema: :rId
    #[xml(attr = "rId")]
    pub revision_id: i32,
    /// Revision From Rejection
    /// Represents the following attribute in the schema: :ua
    #[xml(attr = "ua")]
    pub ua: Option<bool>,
    /// Revision Undo Rejected
    /// Represents the following attribute in the schema: :ra
    #[xml(attr = "ra")]
    pub ra: Option<bool>,
    /// Sheet Id
    /// Represents the following attribute in the schema: :sId
    #[xml(attr = "sId")]
    pub sheet_id: i32,
    /// End Of List
    /// Represents the following attribute in the schema: :eol
    #[xml(attr = "eol")]
    pub end_of_list: Option<bool>,
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: String,
    /// User Action
    /// Represents the following attribute in the schema: :action
    #[xml(attr = "action")]
    pub action: RowColumnActionValues,
    /// Edge Deleted
    /// Represents the following attribute in the schema: :edge
    #[xml(attr = "edge")]
    pub edge: Option<bool>,
    #[xml(child = "x:undo", child = "x:rcc", child = "x:rfmt")]
    pub children: Vec<RevisionRowColumnChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RevisionRowColumnChildChoice {
    #[xml(tag = "x:undo")]
    XUndo(Undo),
    #[xml(tag = "x:rcc")]
    XRcc(RevisionCellChange),
    #[xml(tag = "x:rfmt")]
    XRfmt(RevisionFormat),
}
/// Revision Cell Move.
/// When the object is serialized out as xml, it's qualified name is x:rm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rm")]
pub struct RevisionMove {
    /// Revision Id
    /// Represents the following attribute in the schema: :rId
    #[xml(attr = "rId")]
    pub revision_id: i32,
    /// Revision From Rejection
    /// Represents the following attribute in the schema: :ua
    #[xml(attr = "ua")]
    pub ua: Option<bool>,
    /// Revision Undo Rejected
    /// Represents the following attribute in the schema: :ra
    #[xml(attr = "ra")]
    pub ra: Option<bool>,
    /// Sheet Id
    /// Represents the following attribute in the schema: :sheetId
    #[xml(attr = "sheetId")]
    pub sheet_id: i32,
    /// Source
    /// Represents the following attribute in the schema: :source
    #[xml(attr = "source")]
    pub source: String,
    /// Destination
    /// Represents the following attribute in the schema: :destination
    #[xml(attr = "destination")]
    pub destination: String,
    /// Source Sheet Id
    /// Represents the following attribute in the schema: :sourceSheetId
    #[xml(attr = "sourceSheetId")]
    pub source_sheet_id: Option<i32>,
    #[xml(child = "x:undo", child = "x:rcc", child = "x:rfmt")]
    pub children: Vec<RevisionMoveChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RevisionMoveChildChoice {
    #[xml(tag = "x:undo")]
    XUndo(Undo),
    #[xml(tag = "x:rcc")]
    XRcc(RevisionCellChange),
    #[xml(tag = "x:rfmt")]
    XRfmt(RevisionFormat),
}
/// Revision Custom View.
/// When the object is serialized out as xml, it's qualified name is x:rcv.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rcv")]
pub struct RevisionCustomView {
    /// GUID
    /// Represents the following attribute in the schema: :guid
    #[xml(attr = "guid")]
    pub guid: String,
    /// User Action
    /// Represents the following attribute in the schema: :action
    #[xml(attr = "action")]
    pub action: RevisionActionValues,
}
/// Revision Sheet Name.
/// When the object is serialized out as xml, it's qualified name is x:rsnm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rsnm")]
pub struct RevisionSheetName {
    /// Revision Id
    /// Represents the following attribute in the schema: :rId
    #[xml(attr = "rId")]
    pub revision_id: i32,
    /// Revision From Rejection
    /// Represents the following attribute in the schema: :ua
    #[xml(attr = "ua")]
    pub ua: Option<bool>,
    /// Revision Undo Rejected
    /// Represents the following attribute in the schema: :ra
    #[xml(attr = "ra")]
    pub ra: Option<bool>,
    /// Sheet Id
    /// Represents the following attribute in the schema: :sheetId
    #[xml(attr = "sheetId")]
    pub sheet_id: i32,
    /// Old Sheet Name
    /// Represents the following attribute in the schema: :oldName
    #[xml(attr = "oldName")]
    pub old_name: String,
    /// New Sheet Name
    /// Represents the following attribute in the schema: :newName
    #[xml(attr = "newName")]
    pub new_name: String,
    /// _
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Revision Insert Sheet.
/// When the object is serialized out as xml, it's qualified name is x:ris.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ris")]
pub struct RevisionInsertSheet {
    /// Revision Id
    /// Represents the following attribute in the schema: :rId
    #[xml(attr = "rId")]
    pub revision_id: i32,
    /// Revision From Rejection
    /// Represents the following attribute in the schema: :ua
    #[xml(attr = "ua")]
    pub ua: Option<bool>,
    /// Revision Undo Rejected
    /// Represents the following attribute in the schema: :ra
    #[xml(attr = "ra")]
    pub ra: Option<bool>,
    /// Sheet Id
    /// Represents the following attribute in the schema: :sheetId
    #[xml(attr = "sheetId")]
    pub sheet_id: i32,
    /// Sheet Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Sheet Position
    /// Represents the following attribute in the schema: :sheetPosition
    #[xml(attr = "sheetPosition")]
    pub sheet_position: i32,
}
/// Revision Cell Change.
/// When the object is serialized out as xml, it's qualified name is x:rcc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rcc")]
pub struct RevisionCellChange {
    /// Revision Id
    /// Represents the following attribute in the schema: :rId
    #[xml(attr = "rId")]
    pub revision_id: i32,
    /// Revision From Rejection
    /// Represents the following attribute in the schema: :ua
    #[xml(attr = "ua")]
    pub ua: Option<bool>,
    /// Revision Undo Rejected
    /// Represents the following attribute in the schema: :ra
    #[xml(attr = "ra")]
    pub ra: Option<bool>,
    /// Sheet Id
    /// Represents the following attribute in the schema: :sId
    #[xml(attr = "sId")]
    pub sheet_id: i32,
    /// Old Formatting
    /// Represents the following attribute in the schema: :odxf
    #[xml(attr = "odxf")]
    pub old_formatting: Option<bool>,
    /// Row Column Formatting Change
    /// Represents the following attribute in the schema: :xfDxf
    #[xml(attr = "xfDxf")]
    pub row_column_formatting_affected: Option<bool>,
    /// Style Revision
    /// Represents the following attribute in the schema: :s
    #[xml(attr = "s")]
    pub style_revision: Option<bool>,
    /// Formatting
    /// Represents the following attribute in the schema: :dxf
    #[xml(attr = "dxf")]
    pub format: Option<bool>,
    /// Number Format Id
    /// Represents the following attribute in the schema: :numFmtId
    #[xml(attr = "numFmtId")]
    pub number_format_id: Option<i32>,
    /// Quote Prefix
    /// Represents the following attribute in the schema: :quotePrefix
    #[xml(attr = "quotePrefix")]
    pub quote_prefix: Option<bool>,
    /// Old Quote Prefix
    /// Represents the following attribute in the schema: :oldQuotePrefix
    #[xml(attr = "oldQuotePrefix")]
    pub old_quote_prefix: Option<bool>,
    /// Phonetic Text
    /// Represents the following attribute in the schema: :ph
    #[xml(attr = "ph")]
    pub has_phonetic_text: Option<bool>,
    /// Old Phonetic Text
    /// Represents the following attribute in the schema: :oldPh
    #[xml(attr = "oldPh")]
    pub old_phonetic_text: Option<bool>,
    /// End of List  Formula Update
    /// Represents the following attribute in the schema: :endOfListFormulaUpdate
    #[xml(attr = "endOfListFormulaUpdate")]
    pub end_of_list_formula_update: Option<bool>,
    ///Old Cell Data
    #[xml(child = "x:oc")]
    pub old_cell: Option<OldCell>,
    ///New Cell Data
    #[xml(child = "x:nc")]
    pub new_cell: NewCell,
    ///Old Formatting Information
    #[xml(child = "x:odxf")]
    pub old_differential_format: Option<OldDifferentialFormat>,
    ///New Formatting Information
    #[xml(child = "x:ndxf")]
    pub new_differential_format: Option<NewDifferentialFormat>,
    /// _
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Revision Format.
/// When the object is serialized out as xml, it's qualified name is x:rfmt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rfmt")]
pub struct RevisionFormat {
    /// Sheet Id
    /// Represents the following attribute in the schema: :sheetId
    #[xml(attr = "sheetId")]
    pub sheet_id: i32,
    /// Row or Column Formatting Change
    /// Represents the following attribute in the schema: :xfDxf
    #[xml(attr = "xfDxf")]
    pub row_or_column_affected: Option<bool>,
    /// Style
    /// Represents the following attribute in the schema: :s
    #[xml(attr = "s")]
    pub style_affected: Option<bool>,
    /// Sequence Of References
    /// Represents the following attribute in the schema: :sqref
    #[xml(attr = "sqref")]
    pub sequence_of_references: String,
    /// Start index
    /// Represents the following attribute in the schema: :start
    #[xml(attr = "start")]
    pub start: Option<i32>,
    /// Length
    /// Represents the following attribute in the schema: :length
    #[xml(attr = "length")]
    pub length: Option<i32>,
    ///Formatting
    #[xml(child = "x:dxf")]
    pub differential_format: Option<DifferentialFormat>,
    /// _
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Revision AutoFormat.
/// When the object is serialized out as xml, it's qualified name is x:raf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:raf")]
pub struct RevisionAutoFormat {
    /// Sheet Id
    /// Represents the following attribute in the schema: :sheetId
    #[xml(attr = "sheetId")]
    pub sheet_id: i32,
    /// Auto Format Id
    /// Represents the following attribute in the schema: :autoFormatId
    #[xml(attr = "autoFormatId")]
    pub auto_format_id: Option<i32>,
    /// Apply Number Formats
    /// Represents the following attribute in the schema: :applyNumberFormats
    #[xml(attr = "applyNumberFormats")]
    pub apply_number_formats: Option<bool>,
    /// Apply Border Formats
    /// Represents the following attribute in the schema: :applyBorderFormats
    #[xml(attr = "applyBorderFormats")]
    pub apply_border_formats: Option<bool>,
    /// Apply Font Formats
    /// Represents the following attribute in the schema: :applyFontFormats
    #[xml(attr = "applyFontFormats")]
    pub apply_font_formats: Option<bool>,
    /// Apply Pattern Formats
    /// Represents the following attribute in the schema: :applyPatternFormats
    #[xml(attr = "applyPatternFormats")]
    pub apply_pattern_formats: Option<bool>,
    /// Apply Alignment Formats
    /// Represents the following attribute in the schema: :applyAlignmentFormats
    #[xml(attr = "applyAlignmentFormats")]
    pub apply_alignment_formats: Option<bool>,
    /// Apply Width / Height Formats
    /// Represents the following attribute in the schema: :applyWidthHeightFormats
    #[xml(attr = "applyWidthHeightFormats")]
    pub apply_width_height_formats: Option<bool>,
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: String,
}
/// Revision Defined Name.
/// When the object is serialized out as xml, it's qualified name is x:rdn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rdn")]
pub struct RevisionDefinedName {
    /// Revision Id
    /// Represents the following attribute in the schema: :rId
    #[xml(attr = "rId")]
    pub revision_id: i32,
    /// Revision From Rejection
    /// Represents the following attribute in the schema: :ua
    #[xml(attr = "ua")]
    pub ua: Option<bool>,
    /// Revision Undo Rejected
    /// Represents the following attribute in the schema: :ra
    #[xml(attr = "ra")]
    pub ra: Option<bool>,
    /// Local Name Sheet Id
    /// Represents the following attribute in the schema: :localSheetId
    #[xml(attr = "localSheetId")]
    pub local_sheet_id: Option<i32>,
    /// Custom View
    /// Represents the following attribute in the schema: :customView
    #[xml(attr = "customView")]
    pub custom_view: Option<bool>,
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Function
    /// Represents the following attribute in the schema: :function
    #[xml(attr = "function")]
    pub function: Option<bool>,
    /// Old Function
    /// Represents the following attribute in the schema: :oldFunction
    #[xml(attr = "oldFunction")]
    pub old_function: Option<bool>,
    /// Function Group Id
    /// Represents the following attribute in the schema: :functionGroupId
    #[xml(attr = "functionGroupId")]
    pub function_group_id: Option<u8>,
    /// Old Function Group Id
    /// Represents the following attribute in the schema: :oldFunctionGroupId
    #[xml(attr = "oldFunctionGroupId")]
    pub old_function_group_id: Option<u8>,
    /// Shortcut Key
    /// Represents the following attribute in the schema: :shortcutKey
    #[xml(attr = "shortcutKey")]
    pub shortcut_key: Option<u8>,
    /// Old Short Cut Key
    /// Represents the following attribute in the schema: :oldShortcutKey
    #[xml(attr = "oldShortcutKey")]
    pub old_shortcut_key: Option<u8>,
    /// Named Range Hidden
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    /// Old Hidden
    /// Represents the following attribute in the schema: :oldHidden
    #[xml(attr = "oldHidden")]
    pub old_hidden: Option<bool>,
    /// New Custom Menu
    /// Represents the following attribute in the schema: :customMenu
    #[xml(attr = "customMenu")]
    pub custom_menu: Option<String>,
    /// Old Custom Menu Text
    /// Represents the following attribute in the schema: :oldCustomMenu
    #[xml(attr = "oldCustomMenu")]
    pub old_custom_menu: Option<String>,
    /// Description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// Old Description
    /// Represents the following attribute in the schema: :oldDescription
    #[xml(attr = "oldDescription")]
    pub old_description: Option<String>,
    /// New Help Topic
    /// Represents the following attribute in the schema: :help
    #[xml(attr = "help")]
    pub help: Option<String>,
    /// Old Help Topic
    /// Represents the following attribute in the schema: :oldHelp
    #[xml(attr = "oldHelp")]
    pub old_help: Option<String>,
    /// Status Bar
    /// Represents the following attribute in the schema: :statusBar
    #[xml(attr = "statusBar")]
    pub status_bar: Option<String>,
    /// Old Status Bar
    /// Represents the following attribute in the schema: :oldStatusBar
    #[xml(attr = "oldStatusBar")]
    pub old_status_bar: Option<String>,
    /// Name Comment
    /// Represents the following attribute in the schema: :comment
    #[xml(attr = "comment")]
    pub comment: Option<String>,
    /// Old Name Comment
    /// Represents the following attribute in the schema: :oldComment
    #[xml(attr = "oldComment")]
    pub old_comment: Option<String>,
    ///Formula
    #[xml(child = "x:formula")]
    pub formula: Option<Formula>,
    ///Old Formula
    #[xml(child = "x:oldFormula")]
    pub old_formula: Option<OldFormula>,
    /// _
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Revision Cell Comment.
/// When the object is serialized out as xml, it's qualified name is x:rcmt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rcmt")]
pub struct RevisionComment {
    /// Sheet Id
    /// Represents the following attribute in the schema: :sheetId
    #[xml(attr = "sheetId")]
    pub sheet_id: i32,
    /// Cell
    /// Represents the following attribute in the schema: :cell
    #[xml(attr = "cell")]
    pub cell: String,
    /// GUID
    /// Represents the following attribute in the schema: :guid
    #[xml(attr = "guid")]
    pub guid: String,
    /// User Action
    /// Represents the following attribute in the schema: :action
    #[xml(attr = "action")]
    pub action: Option<RevisionActionValues>,
    /// Always Show Comment
    /// Represents the following attribute in the schema: :alwaysShow
    #[xml(attr = "alwaysShow")]
    pub always_show: Option<bool>,
    /// Old Comment
    /// Represents the following attribute in the schema: :old
    #[xml(attr = "old")]
    pub old: Option<bool>,
    /// Comment In Hidden Row
    /// Represents the following attribute in the schema: :hiddenRow
    #[xml(attr = "hiddenRow")]
    pub hidden_row: Option<bool>,
    /// Hidden Column
    /// Represents the following attribute in the schema: :hiddenColumn
    #[xml(attr = "hiddenColumn")]
    pub hidden_column: Option<bool>,
    /// Author
    /// Represents the following attribute in the schema: :author
    #[xml(attr = "author")]
    pub author: String,
    /// Original Comment Length
    /// Represents the following attribute in the schema: :oldLength
    #[xml(attr = "oldLength")]
    pub old_length: Option<i32>,
    /// New Comment Length
    /// Represents the following attribute in the schema: :newLength
    #[xml(attr = "newLength")]
    pub new_length: Option<i32>,
}
/// Revision Query Table.
/// When the object is serialized out as xml, it's qualified name is x:rqt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rqt")]
pub struct RevisionQueryTable {
    /// Sheet Id
    /// Represents the following attribute in the schema: :sheetId
    #[xml(attr = "sheetId")]
    pub sheet_id: i32,
    /// QueryTable Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: String,
    /// Field Id
    /// Represents the following attribute in the schema: :fieldId
    #[xml(attr = "fieldId")]
    pub field_id: i32,
}
/// Revision Merge Conflict.
/// When the object is serialized out as xml, it's qualified name is x:rcft.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rcft")]
pub struct RevisionConflict {
    /// Revision Id
    /// Represents the following attribute in the schema: :rId
    #[xml(attr = "rId")]
    pub revision_id: i32,
    /// Revision From Rejection
    /// Represents the following attribute in the schema: :ua
    #[xml(attr = "ua")]
    pub ua: Option<bool>,
    /// Revision Undo Rejected
    /// Represents the following attribute in the schema: :ra
    #[xml(attr = "ra")]
    pub ra: Option<bool>,
    /// Sheet Id
    /// Represents the following attribute in the schema: :sheetId
    #[xml(attr = "sheetId")]
    pub sheet_id: Option<i32>,
}
/// Sheet Id Map.
/// When the object is serialized out as xml, it's qualified name is x:sheetIdMap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetIdMap")]
pub struct SheetIdMap {
    /// Sheet Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:sheetId")]
    pub x_sheet_id: Vec<SheetId>,
}
/// Reviewed List.
/// When the object is serialized out as xml, it's qualified name is x:reviewedList.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:reviewedList")]
pub struct ReviewedList {
    /// Reviewed Revisions Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:reviewed")]
    pub x_reviewed: Vec<Reviewed>,
}
/// Reviewed.
/// When the object is serialized out as xml, it's qualified name is x:reviewed.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:reviewed")]
pub struct Reviewed {
    /// revision Id
    /// Represents the following attribute in the schema: :rId
    #[xml(attr = "rId")]
    pub revision_id: i32,
}
/// Undo.
/// When the object is serialized out as xml, it's qualified name is x:undo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:undo")]
pub struct Undo {
    /// Index
    /// Represents the following attribute in the schema: :index
    #[xml(attr = "index")]
    pub index: i32,
    /// Expression
    /// Represents the following attribute in the schema: :exp
    #[xml(attr = "exp")]
    pub expression: FormulaExpressionValues,
    /// Reference 3D
    /// Represents the following attribute in the schema: :ref3D
    #[xml(attr = "ref3D")]
    pub reference3_d: Option<bool>,
    /// Array Entered
    /// Represents the following attribute in the schema: :array
    #[xml(attr = "array")]
    pub array: Option<bool>,
    /// Value Needed
    /// Represents the following attribute in the schema: :v
    #[xml(attr = "v")]
    pub val: Option<bool>,
    /// Defined Name Formula
    /// Represents the following attribute in the schema: :nf
    #[xml(attr = "nf")]
    pub defined_name_formula: Option<bool>,
    /// Cross Sheet Move
    /// Represents the following attribute in the schema: :cs
    #[xml(attr = "cs")]
    pub cross_sheet_move: Option<bool>,
    /// Range
    /// Represents the following attribute in the schema: :dr
    #[xml(attr = "dr")]
    pub deleted_range: String,
    /// Defined Name
    /// Represents the following attribute in the schema: :dn
    #[xml(attr = "dn")]
    pub defined_name: Option<String>,
    /// Cell Reference
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub cell_reference: Option<String>,
    /// Sheet Id
    /// Represents the following attribute in the schema: :sId
    #[xml(attr = "sId")]
    pub sheet_id: Option<i32>,
}
/// Old Cell Data.
/// When the object is serialized out as xml, it's qualified name is x:oc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:oc")]
pub struct OldCell {
    /// Reference
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub cell_reference: Option<String>,
    /// Style Index
    /// Represents the following attribute in the schema: :s
    #[xml(attr = "s")]
    pub style_index: Option<i32>,
    /// Cell Data Type
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub data_type: Option<CellValues>,
    /// Cell Metadata Index
    /// Represents the following attribute in the schema: :cm
    #[xml(attr = "cm")]
    pub cell_meta_index: Option<i32>,
    /// Value Metadata Index
    /// Represents the following attribute in the schema: :vm
    #[xml(attr = "vm")]
    pub value_meta_index: Option<i32>,
    /// Show Phonetic
    /// Represents the following attribute in the schema: :ph
    #[xml(attr = "ph")]
    pub show_phonetic: Option<bool>,
    ///Formula
    #[xml(child = "x:f")]
    pub cell_formula: Option<CellFormula>,
    ///Cell Value
    #[xml(child = "x:v")]
    pub cell_value: Option<CellValue>,
    ///Rich Text Inline
    #[xml(child = "x:is")]
    pub inline_string: Option<InlineString>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Cell.
/// When the object is serialized out as xml, it's qualified name is x:c.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:c")]
pub struct Cell {
    /// Reference
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub cell_reference: Option<String>,
    /// Style Index
    /// Represents the following attribute in the schema: :s
    #[xml(attr = "s")]
    pub style_index: Option<i32>,
    /// Cell Data Type
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub data_type: Option<CellValues>,
    /// Cell Metadata Index
    /// Represents the following attribute in the schema: :cm
    #[xml(attr = "cm")]
    pub cell_meta_index: Option<i32>,
    /// Value Metadata Index
    /// Represents the following attribute in the schema: :vm
    #[xml(attr = "vm")]
    pub value_meta_index: Option<i32>,
    /// Show Phonetic
    /// Represents the following attribute in the schema: :ph
    #[xml(attr = "ph")]
    pub show_phonetic: Option<bool>,
    ///Formula
    #[xml(child = "x:f")]
    pub cell_formula: Option<CellFormula>,
    ///Cell Value
    #[xml(child = "x:v")]
    pub cell_value: Option<CellValue>,
    ///Rich Text Inline
    #[xml(child = "x:is")]
    pub inline_string: Option<InlineString>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the CellType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct CellType {
    /// Reference
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub cell_reference: Option<String>,
    /// Style Index
    /// Represents the following attribute in the schema: :s
    #[xml(attr = "s")]
    pub style_index: Option<i32>,
    /// Cell Data Type
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub data_type: Option<CellValues>,
    /// Cell Metadata Index
    /// Represents the following attribute in the schema: :cm
    #[xml(attr = "cm")]
    pub cell_meta_index: Option<i32>,
    /// Value Metadata Index
    /// Represents the following attribute in the schema: :vm
    #[xml(attr = "vm")]
    pub value_meta_index: Option<i32>,
    /// Show Phonetic
    /// Represents the following attribute in the schema: :ph
    #[xml(attr = "ph")]
    pub show_phonetic: Option<bool>,
}
/// New Cell Data.
/// When the object is serialized out as xml, it's qualified name is x:nc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:nc")]
pub struct NewCell {
    /// Reference
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub cell_reference: String,
    /// Style Index
    /// Represents the following attribute in the schema: :s
    #[xml(attr = "s")]
    pub style_index: Option<i32>,
    /// Cell Data Type
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub data_type: Option<CellValues>,
    /// Cell Metadata Index
    /// Represents the following attribute in the schema: :cm
    #[xml(attr = "cm")]
    pub cell_meta_index: Option<i32>,
    /// Value Metadata Index
    /// Represents the following attribute in the schema: :vm
    #[xml(attr = "vm")]
    pub value_meta_index: Option<i32>,
    /// Show Phonetic
    /// Represents the following attribute in the schema: :ph
    #[xml(attr = "ph")]
    pub show_phonetic: Option<bool>,
    ///Formula
    #[xml(child = "x:f")]
    pub cell_formula: Option<CellFormula>,
    ///Cell Value
    #[xml(child = "x:v")]
    pub cell_value: Option<CellValue>,
    ///Rich Text Inline
    #[xml(child = "x:is")]
    pub inline_string: Option<InlineString>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Old Formatting Information.
/// When the object is serialized out as xml, it's qualified name is x:odxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:odxf")]
pub struct OldDifferentialFormat {
    ///Font Properties
    #[xml(child = "x:font")]
    pub font: Option<Font>,
    ///Number Format
    #[xml(child = "x:numFmt")]
    pub numbering_format: Option<NumberingFormat>,
    ///Fill
    #[xml(child = "x:fill")]
    pub fill: Option<Fill>,
    ///Alignment
    #[xml(child = "x:alignment")]
    pub alignment: Option<Alignment>,
    ///Border Properties
    #[xml(child = "x:border")]
    pub border: Option<Border>,
    ///Protection Properties
    #[xml(child = "x:protection")]
    pub protection: Option<Protection>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// New Formatting Information.
/// When the object is serialized out as xml, it's qualified name is x:ndxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ndxf")]
pub struct NewDifferentialFormat {
    ///Font Properties
    #[xml(child = "x:font")]
    pub font: Option<Font>,
    ///Number Format
    #[xml(child = "x:numFmt")]
    pub numbering_format: Option<NumberingFormat>,
    ///Fill
    #[xml(child = "x:fill")]
    pub fill: Option<Fill>,
    ///Alignment
    #[xml(child = "x:alignment")]
    pub alignment: Option<Alignment>,
    ///Border Properties
    #[xml(child = "x:border")]
    pub border: Option<Border>,
    ///Protection Properties
    #[xml(child = "x:protection")]
    pub protection: Option<Protection>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Formatting.
/// When the object is serialized out as xml, it's qualified name is x:dxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dxf")]
pub struct DifferentialFormat {
    ///Font Properties
    #[xml(child = "x:font")]
    pub font: Option<Font>,
    ///Number Format
    #[xml(child = "x:numFmt")]
    pub numbering_format: Option<NumberingFormat>,
    ///Fill
    #[xml(child = "x:fill")]
    pub fill: Option<Fill>,
    ///Alignment
    #[xml(child = "x:alignment")]
    pub alignment: Option<Alignment>,
    ///Border Properties
    #[xml(child = "x:border")]
    pub border: Option<Border>,
    ///Protection Properties
    #[xml(child = "x:protection")]
    pub protection: Option<Protection>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the DifferentialFormatType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct DifferentialFormatType {}
/// Sheet Id.
/// When the object is serialized out as xml, it's qualified name is x:sheetId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetId")]
pub struct SheetId {
    /// Sheet Id
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Formula.
/// When the object is serialized out as xml, it's qualified name is x:f.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:f")]
pub struct CellFormula {
    /// Formula Type
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub formula_type: Option<CellFormulaValues>,
    /// Always Calculate Array
    /// Represents the following attribute in the schema: :aca
    #[xml(attr = "aca")]
    pub always_calculate_array: Option<bool>,
    /// Range of Cells
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: Option<String>,
    /// Data Table 2-D
    /// Represents the following attribute in the schema: :dt2D
    #[xml(attr = "dt2D")]
    pub data_table2_d: Option<bool>,
    /// Data Table Row
    /// Represents the following attribute in the schema: :dtr
    #[xml(attr = "dtr")]
    pub data_table_row: Option<bool>,
    /// Input 1 Deleted
    /// Represents the following attribute in the schema: :del1
    #[xml(attr = "del1")]
    pub input1_deleted: Option<bool>,
    /// Input 2 Deleted
    /// Represents the following attribute in the schema: :del2
    #[xml(attr = "del2")]
    pub input2_deleted: Option<bool>,
    /// Data Table Cell 1
    /// Represents the following attribute in the schema: :r1
    #[xml(attr = "r1")]
    pub r1: Option<String>,
    /// Input Cell 2
    /// Represents the following attribute in the schema: :r2
    #[xml(attr = "r2")]
    pub r2: Option<String>,
    /// Calculate Cell
    /// Represents the following attribute in the schema: :ca
    #[xml(attr = "ca")]
    pub calculate_cell: Option<bool>,
    /// Shared Group Index
    /// Represents the following attribute in the schema: :si
    #[xml(attr = "si")]
    pub shared_index: Option<i32>,
    /// Assigns Value to Name
    /// Represents the following attribute in the schema: :bx
    #[xml(attr = "bx")]
    pub bx: Option<bool>,
    /// Content Contains Significant Whitespace
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// User Information.
/// When the object is serialized out as xml, it's qualified name is x:userInfo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:userInfo")]
pub struct UserInfo {
    /// User Revisions GUID
    /// Represents the following attribute in the schema: :guid
    #[xml(attr = "guid")]
    pub guid: String,
    /// User Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// User Id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// Date Time
    /// Represents the following attribute in the schema: :dateTime
    #[xml(attr = "dateTime")]
    pub date_time: String,
    /// _
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Row.
/// When the object is serialized out as xml, it's qualified name is x:row.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:row")]
pub struct Row {
    /// Row Index
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub row_index: Option<i32>,
    /// Spans
    /// Represents the following attribute in the schema: :spans
    #[xml(attr = "spans")]
    pub spans: Option<String>,
    /// Style Index
    /// Represents the following attribute in the schema: :s
    #[xml(attr = "s")]
    pub style_index: Option<i32>,
    /// Custom Format
    /// Represents the following attribute in the schema: :customFormat
    #[xml(attr = "customFormat")]
    pub custom_format: Option<bool>,
    /// Row Height
    /// Represents the following attribute in the schema: :ht
    #[xml(attr = "ht")]
    pub height: Option<f64>,
    /// Hidden
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    /// Custom Height
    /// Represents the following attribute in the schema: :customHeight
    #[xml(attr = "customHeight")]
    pub custom_height: Option<bool>,
    /// Outline Level
    /// Represents the following attribute in the schema: :outlineLevel
    #[xml(attr = "outlineLevel")]
    pub outline_level: Option<u8>,
    /// Collapsed
    /// Represents the following attribute in the schema: :collapsed
    #[xml(attr = "collapsed")]
    pub collapsed: Option<bool>,
    /// Thick Top Border
    /// Represents the following attribute in the schema: :thickTop
    #[xml(attr = "thickTop")]
    pub thick_top: Option<bool>,
    /// Thick Bottom
    /// Represents the following attribute in the schema: :thickBot
    #[xml(attr = "thickBot")]
    pub thick_bot: Option<bool>,
    /// Show Phonetic
    /// Represents the following attribute in the schema: :ph
    #[xml(attr = "ph")]
    pub show_phonetic: Option<bool>,
    /// dyDescent
    /// Represents the following attribute in the schema: x14ac:dyDescent
    #[xml(attr = "x14ac:dyDescent")]
    pub dy_descent: Option<f64>,
    /// _
    #[xml(child = "x:c")]
    pub x_c: Vec<Cell>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<ExtensionList>,
}
/// Column Width and Formatting.
/// When the object is serialized out as xml, it's qualified name is x:col.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:col")]
pub struct Column {
    /// Minimum Column
    /// Represents the following attribute in the schema: :min
    #[xml(attr = "min")]
    pub min: i32,
    /// Maximum Column
    /// Represents the following attribute in the schema: :max
    #[xml(attr = "max")]
    pub max: i32,
    /// Column Width
    /// Represents the following attribute in the schema: :width
    #[xml(attr = "width")]
    pub width: Option<f64>,
    /// Style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<i32>,
    /// Hidden Columns
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    /// Best Fit Column Width
    /// Represents the following attribute in the schema: :bestFit
    #[xml(attr = "bestFit")]
    pub best_fit: Option<bool>,
    /// Custom Width
    /// Represents the following attribute in the schema: :customWidth
    #[xml(attr = "customWidth")]
    pub custom_width: Option<bool>,
    /// Show Phonetic Information
    /// Represents the following attribute in the schema: :phonetic
    #[xml(attr = "phonetic")]
    pub phonetic: Option<bool>,
    /// Outline Level
    /// Represents the following attribute in the schema: :outlineLevel
    #[xml(attr = "outlineLevel")]
    pub outline_level: Option<u8>,
    /// Collapsed
    /// Represents the following attribute in the schema: :collapsed
    #[xml(attr = "collapsed")]
    pub collapsed: Option<bool>,
}
/// Outline Properties.
/// When the object is serialized out as xml, it's qualified name is x:outlinePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:outlinePr")]
pub struct OutlineProperties {
    /// Apply Styles in Outline
    /// Represents the following attribute in the schema: :applyStyles
    #[xml(attr = "applyStyles")]
    pub apply_styles: Option<bool>,
    /// Summary Below
    /// Represents the following attribute in the schema: :summaryBelow
    #[xml(attr = "summaryBelow")]
    pub summary_below: Option<bool>,
    /// Summary Right
    /// Represents the following attribute in the schema: :summaryRight
    #[xml(attr = "summaryRight")]
    pub summary_right: Option<bool>,
    /// Show Outline Symbols
    /// Represents the following attribute in the schema: :showOutlineSymbols
    #[xml(attr = "showOutlineSymbols")]
    pub show_outline_symbols: Option<bool>,
}
/// Page Setup Properties.
/// When the object is serialized out as xml, it's qualified name is x:pageSetUpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pageSetUpPr")]
pub struct PageSetupProperties {
    /// Show Auto Page Breaks
    /// Represents the following attribute in the schema: :autoPageBreaks
    #[xml(attr = "autoPageBreaks")]
    pub auto_page_breaks: Option<bool>,
    /// Fit To Page
    /// Represents the following attribute in the schema: :fitToPage
    #[xml(attr = "fitToPage")]
    pub fit_to_page: Option<bool>,
}
/// View Pane.
/// When the object is serialized out as xml, it's qualified name is x:pane.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pane")]
pub struct Pane {
    /// Horizontal Split Position
    /// Represents the following attribute in the schema: :xSplit
    #[xml(attr = "xSplit")]
    pub horizontal_split: Option<f64>,
    /// Vertical Split Position
    /// Represents the following attribute in the schema: :ySplit
    #[xml(attr = "ySplit")]
    pub vertical_split: Option<f64>,
    /// Top Left Visible Cell
    /// Represents the following attribute in the schema: :topLeftCell
    #[xml(attr = "topLeftCell")]
    pub top_left_cell: Option<String>,
    /// Active Pane
    /// Represents the following attribute in the schema: :activePane
    #[xml(attr = "activePane")]
    pub active_pane: Option<PaneValues>,
    /// Split State
    /// Represents the following attribute in the schema: :state
    #[xml(attr = "state")]
    pub state: Option<PaneStateValues>,
}
/// Selection.
/// When the object is serialized out as xml, it's qualified name is x:selection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:selection")]
pub struct Selection {
    /// Pane
    /// Represents the following attribute in the schema: :pane
    #[xml(attr = "pane")]
    pub pane: Option<PaneValues>,
    /// Active Cell Location
    /// Represents the following attribute in the schema: :activeCell
    #[xml(attr = "activeCell")]
    pub active_cell: Option<String>,
    /// Active Cell Index
    /// Represents the following attribute in the schema: :activeCellId
    #[xml(attr = "activeCellId")]
    pub active_cell_id: Option<i32>,
    /// Sequence of References
    /// Represents the following attribute in the schema: :sqref
    #[xml(attr = "sqref")]
    pub sequence_of_references: Option<String>,
}
/// PivotTable Selection.
/// When the object is serialized out as xml, it's qualified name is x:pivotSelection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pivotSelection")]
pub struct PivotSelection {
    /// Pane
    /// Represents the following attribute in the schema: :pane
    #[xml(attr = "pane")]
    pub pane: Option<PaneValues>,
    /// Show Header
    /// Represents the following attribute in the schema: :showHeader
    #[xml(attr = "showHeader")]
    pub show_header: Option<bool>,
    /// Label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: Option<bool>,
    /// Data Selection
    /// Represents the following attribute in the schema: :data
    #[xml(attr = "data")]
    pub data: Option<bool>,
    /// Extendable
    /// Represents the following attribute in the schema: :extendable
    #[xml(attr = "extendable")]
    pub extendable: Option<bool>,
    /// Selection Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// Axis
    /// Represents the following attribute in the schema: :axis
    #[xml(attr = "axis")]
    pub axis: Option<PivotTableAxisValues>,
    /// Dimension
    /// Represents the following attribute in the schema: :dimension
    #[xml(attr = "dimension")]
    pub dimension: Option<i32>,
    /// Start
    /// Represents the following attribute in the schema: :start
    #[xml(attr = "start")]
    pub start: Option<i32>,
    /// Minimum
    /// Represents the following attribute in the schema: :min
    #[xml(attr = "min")]
    pub min: Option<i32>,
    /// Maximum
    /// Represents the following attribute in the schema: :max
    #[xml(attr = "max")]
    pub max: Option<i32>,
    /// Active Row
    /// Represents the following attribute in the schema: :activeRow
    #[xml(attr = "activeRow")]
    pub active_row: Option<i32>,
    /// Active Column
    /// Represents the following attribute in the schema: :activeCol
    #[xml(attr = "activeCol")]
    pub active_column: Option<i32>,
    /// Previous Row
    /// Represents the following attribute in the schema: :previousRow
    #[xml(attr = "previousRow")]
    pub previous_row: Option<i32>,
    /// Previous Column Selection
    /// Represents the following attribute in the schema: :previousCol
    #[xml(attr = "previousCol")]
    pub previous_column: Option<i32>,
    /// Click Count
    /// Represents the following attribute in the schema: :click
    #[xml(attr = "click")]
    pub click: Option<i32>,
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
    ///Pivot Area
    #[xml(child = "x:pivotArea")]
    pub pivot_area: PivotArea,
}
/// Break.
/// When the object is serialized out as xml, it's qualified name is x:brk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:brk")]
pub struct Break {
    /// Id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<i32>,
    /// Minimum
    /// Represents the following attribute in the schema: :min
    #[xml(attr = "min")]
    pub min: Option<i32>,
    /// Maximum
    /// Represents the following attribute in the schema: :max
    #[xml(attr = "max")]
    pub max: Option<i32>,
    /// Manual Page Break
    /// Represents the following attribute in the schema: :man
    #[xml(attr = "man")]
    pub manual_page_break: Option<bool>,
    /// Pivot-Created Page Break
    /// Represents the following attribute in the schema: :pt
    #[xml(attr = "pt")]
    pub pivot_table_page_break: Option<bool>,
}
/// Data Consolidation Reference.
/// When the object is serialized out as xml, it's qualified name is x:dataRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dataRef")]
pub struct DataReference {
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: Option<String>,
    /// Named Range
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Sheet Name
    /// Represents the following attribute in the schema: :sheet
    #[xml(attr = "sheet")]
    pub sheet: Option<String>,
    /// relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
}
/// Horizontal Page Breaks.
/// When the object is serialized out as xml, it's qualified name is x:rowBreaks.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rowBreaks")]
pub struct RowBreaks {
    /// Page Break Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// Manual Break Count
    /// Represents the following attribute in the schema: :manualBreakCount
    #[xml(attr = "manualBreakCount")]
    pub manual_break_count: Option<i32>,
    #[xml(child = "x:brk")]
    pub children: Vec<RowBreaksChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RowBreaksChildChoice {
    #[xml(tag = "x:brk")]
    XBrk(Break),
}
/// Vertical Page Breaks.
/// When the object is serialized out as xml, it's qualified name is x:colBreaks.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:colBreaks")]
pub struct ColumnBreaks {
    /// Page Break Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// Manual Break Count
    /// Represents the following attribute in the schema: :manualBreakCount
    #[xml(attr = "manualBreakCount")]
    pub manual_break_count: Option<i32>,
    #[xml(child = "x:brk")]
    pub children: Vec<ColumnBreaksChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ColumnBreaksChildChoice {
    #[xml(tag = "x:brk")]
    XBrk(Break),
}
/// Defines the PageBreakType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct PageBreakType {
    /// Page Break Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// Manual Break Count
    /// Represents the following attribute in the schema: :manualBreakCount
    #[xml(attr = "manualBreakCount")]
    pub manual_break_count: Option<i32>,
    #[xml(child = "x:brk")]
    pub children: Vec<PageBreakTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PageBreakTypeChildChoice {
    #[xml(tag = "x:brk")]
    XBrk(Break),
}
/// Page Margins.
/// When the object is serialized out as xml, it's qualified name is x:pageMargins.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pageMargins")]
pub struct PageMargins {
    /// Left Page Margin
    /// Represents the following attribute in the schema: :left
    #[xml(attr = "left")]
    pub left: f64,
    /// Right Page Margin
    /// Represents the following attribute in the schema: :right
    #[xml(attr = "right")]
    pub right: f64,
    /// Top Page Margin
    /// Represents the following attribute in the schema: :top
    #[xml(attr = "top")]
    pub top: f64,
    /// Bottom Page Margin
    /// Represents the following attribute in the schema: :bottom
    #[xml(attr = "bottom")]
    pub bottom: f64,
    /// Header Page Margin
    /// Represents the following attribute in the schema: :header
    #[xml(attr = "header")]
    pub header: f64,
    /// Footer Page Margin
    /// Represents the following attribute in the schema: :footer
    #[xml(attr = "footer")]
    pub footer: f64,
}
/// Print Options.
/// When the object is serialized out as xml, it's qualified name is x:printOptions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:printOptions")]
pub struct PrintOptions {
    /// Horizontal Centered
    /// Represents the following attribute in the schema: :horizontalCentered
    #[xml(attr = "horizontalCentered")]
    pub horizontal_centered: Option<bool>,
    /// Vertical Centered
    /// Represents the following attribute in the schema: :verticalCentered
    #[xml(attr = "verticalCentered")]
    pub vertical_centered: Option<bool>,
    /// Print Headings
    /// Represents the following attribute in the schema: :headings
    #[xml(attr = "headings")]
    pub headings: Option<bool>,
    /// Print Grid Lines
    /// Represents the following attribute in the schema: :gridLines
    #[xml(attr = "gridLines")]
    pub grid_lines: Option<bool>,
    /// Grid Lines Set
    /// Represents the following attribute in the schema: :gridLinesSet
    #[xml(attr = "gridLinesSet")]
    pub grid_lines_set: Option<bool>,
}
/// Page Setup Settings.
/// When the object is serialized out as xml, it's qualified name is x:pageSetup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pageSetup")]
pub struct PageSetup {
    /// Paper Size
    /// Represents the following attribute in the schema: :paperSize
    #[xml(attr = "paperSize")]
    pub paper_size: Option<i32>,
    /// Print Scale
    /// Represents the following attribute in the schema: :scale
    #[xml(attr = "scale")]
    pub scale: Option<i32>,
    /// First Page Number
    /// Represents the following attribute in the schema: :firstPageNumber
    #[xml(attr = "firstPageNumber")]
    pub first_page_number: Option<i32>,
    /// Fit To Width
    /// Represents the following attribute in the schema: :fitToWidth
    #[xml(attr = "fitToWidth")]
    pub fit_to_width: Option<i32>,
    /// Fit To Height
    /// Represents the following attribute in the schema: :fitToHeight
    #[xml(attr = "fitToHeight")]
    pub fit_to_height: Option<i32>,
    /// Page Order
    /// Represents the following attribute in the schema: :pageOrder
    #[xml(attr = "pageOrder")]
    pub page_order: Option<PageOrderValues>,
    /// Orientation
    /// Represents the following attribute in the schema: :orientation
    #[xml(attr = "orientation")]
    pub orientation: Option<OrientationValues>,
    /// Use Printer Defaults
    /// Represents the following attribute in the schema: :usePrinterDefaults
    #[xml(attr = "usePrinterDefaults")]
    pub use_printer_defaults: Option<bool>,
    /// Black And White
    /// Represents the following attribute in the schema: :blackAndWhite
    #[xml(attr = "blackAndWhite")]
    pub black_and_white: Option<bool>,
    /// Draft
    /// Represents the following attribute in the schema: :draft
    #[xml(attr = "draft")]
    pub draft: Option<bool>,
    /// Print Cell Comments
    /// Represents the following attribute in the schema: :cellComments
    #[xml(attr = "cellComments")]
    pub cell_comments: Option<CellCommentsValues>,
    /// Use First Page Number
    /// Represents the following attribute in the schema: :useFirstPageNumber
    #[xml(attr = "useFirstPageNumber")]
    pub use_first_page_number: Option<bool>,
    /// Print Error Handling
    /// Represents the following attribute in the schema: :errors
    #[xml(attr = "errors")]
    pub errors: Option<PrintErrorValues>,
    /// Horizontal DPI
    /// Represents the following attribute in the schema: :horizontalDpi
    #[xml(attr = "horizontalDpi")]
    pub horizontal_dpi: Option<i32>,
    /// Vertical DPI
    /// Represents the following attribute in the schema: :verticalDpi
    #[xml(attr = "verticalDpi")]
    pub vertical_dpi: Option<i32>,
    /// Number Of Copies
    /// Represents the following attribute in the schema: :copies
    #[xml(attr = "copies")]
    pub copies: Option<i32>,
    /// Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
}
/// Header Footer Settings.
/// When the object is serialized out as xml, it's qualified name is x:headerFooter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:headerFooter")]
pub struct HeaderFooter {
    /// Different Odd Even Header Footer
    /// Represents the following attribute in the schema: :differentOddEven
    #[xml(attr = "differentOddEven")]
    pub different_odd_even: Option<bool>,
    /// Different First Page
    /// Represents the following attribute in the schema: :differentFirst
    #[xml(attr = "differentFirst")]
    pub different_first: Option<bool>,
    /// Scale Header and Footer With Document
    /// Represents the following attribute in the schema: :scaleWithDoc
    #[xml(attr = "scaleWithDoc")]
    pub scale_with_doc: Option<bool>,
    /// Align Margins
    /// Represents the following attribute in the schema: :alignWithMargins
    #[xml(attr = "alignWithMargins")]
    pub align_with_margins: Option<bool>,
    ///Odd Header
    #[xml(child = "x:oddHeader")]
    pub odd_header: Option<OddHeader>,
    ///Odd Page Footer
    #[xml(child = "x:oddFooter")]
    pub odd_footer: Option<OddFooter>,
    ///Even Page Header
    #[xml(child = "x:evenHeader")]
    pub even_header: Option<EvenHeader>,
    ///Even Page Footer
    #[xml(child = "x:evenFooter")]
    pub even_footer: Option<EvenFooter>,
    ///First Page Header
    #[xml(child = "x:firstHeader")]
    pub first_header: Option<FirstHeader>,
    ///First Page Footer
    #[xml(child = "x:firstFooter")]
    pub first_footer: Option<FirstFooter>,
}
/// AutoFilter Settings.
/// When the object is serialized out as xml, it's qualified name is x:autoFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:autoFilter")]
pub struct AutoFilter {
    /// Cell or Range Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: Option<String>,
    /// _
    #[xml(child = "x:filterColumn")]
    pub x_filter_column: Vec<FilterColumn>,
    /// _
    #[xml(child = "x:sortState")]
    pub x_sort_state: Option<SortState>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<ExtensionList>,
}
/// Conditional Formatting Rule.
/// When the object is serialized out as xml, it's qualified name is x:cfRule.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cfRule")]
pub struct ConditionalFormattingRule {
    /// Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: ConditionalFormatValues,
    /// Differential Formatting Id
    /// Represents the following attribute in the schema: :dxfId
    #[xml(attr = "dxfId")]
    pub format_id: Option<i32>,
    /// Priority
    /// Represents the following attribute in the schema: :priority
    #[xml(attr = "priority")]
    pub priority: i32,
    /// Stop If True
    /// Represents the following attribute in the schema: :stopIfTrue
    #[xml(attr = "stopIfTrue")]
    pub stop_if_true: Option<bool>,
    /// Above Or Below Average
    /// Represents the following attribute in the schema: :aboveAverage
    #[xml(attr = "aboveAverage")]
    pub above_average: Option<bool>,
    /// Top 10 Percent
    /// Represents the following attribute in the schema: :percent
    #[xml(attr = "percent")]
    pub percent: Option<bool>,
    /// Bottom N
    /// Represents the following attribute in the schema: :bottom
    #[xml(attr = "bottom")]
    pub bottom: Option<bool>,
    /// Operator
    /// Represents the following attribute in the schema: :operator
    #[xml(attr = "operator")]
    pub operator: Option<ConditionalFormattingOperatorValues>,
    /// Text
    /// Represents the following attribute in the schema: :text
    #[xml(attr = "text")]
    pub text: Option<String>,
    /// Time Period
    /// Represents the following attribute in the schema: :timePeriod
    #[xml(attr = "timePeriod")]
    pub time_period: Option<TimePeriodValues>,
    /// Rank
    /// Represents the following attribute in the schema: :rank
    #[xml(attr = "rank")]
    pub rank: Option<i32>,
    /// StdDev
    /// Represents the following attribute in the schema: :stdDev
    #[xml(attr = "stdDev")]
    pub std_dev: Option<i32>,
    /// Equal Average
    /// Represents the following attribute in the schema: :equalAverage
    #[xml(attr = "equalAverage")]
    pub equal_average: Option<bool>,
    /// _
    #[xml(child = "x:formula")]
    pub x_formula: Vec<Formula>,
    /// _
    #[xml(child = "x:colorScale")]
    pub x_color_scale: Option<ColorScale>,
    /// _
    #[xml(child = "x:dataBar")]
    pub x_data_bar: Option<DataBar>,
    /// _
    #[xml(child = "x:iconSet")]
    pub x_icon_set: Option<IconSet>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<ConditionalFormattingRuleExtensionList>,
}
/// Hyperlink.
/// When the object is serialized out as xml, it's qualified name is x:hyperlink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:hyperlink")]
pub struct Hyperlink {
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: String,
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    /// Location
    /// Represents the following attribute in the schema: :location
    #[xml(attr = "location")]
    pub location: Option<String>,
    /// Tool Tip
    /// Represents the following attribute in the schema: :tooltip
    #[xml(attr = "tooltip")]
    pub tooltip: Option<String>,
    /// Display String
    /// Represents the following attribute in the schema: :display
    #[xml(attr = "display")]
    pub display: Option<String>,
}
/// Conditional Format Value Object.
/// When the object is serialized out as xml, it's qualified name is x:cfvo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cfvo")]
pub struct ConditionalFormatValueObject {
    /// Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: ConditionalFormatValueObjectValues,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<String>,
    /// Greater Than Or Equal
    /// Represents the following attribute in the schema: :gte
    #[xml(attr = "gte")]
    pub greater_than_or_equal: Option<bool>,
    /// _
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Scenario.
/// When the object is serialized out as xml, it's qualified name is x:scenario.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:scenario")]
pub struct Scenario {
    /// Scenario Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Scenario Locked
    /// Represents the following attribute in the schema: :locked
    #[xml(attr = "locked")]
    pub locked: Option<bool>,
    /// Hidden Scenario
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    /// Changing Cell Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// User Name
    /// Represents the following attribute in the schema: :user
    #[xml(attr = "user")]
    pub user: Option<String>,
    /// Scenario Comment
    /// Represents the following attribute in the schema: :comment
    #[xml(attr = "comment")]
    pub comment: Option<String>,
    /// _
    #[xml(child = "x:inputCells")]
    pub x_input_cells: Vec<InputCells>,
}
/// Protected Range.
/// When the object is serialized out as xml, it's qualified name is x:protectedRange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:protectedRange")]
pub struct ProtectedRange {
    /// password
    /// Represents the following attribute in the schema: :password
    #[xml(attr = "password")]
    pub password: Option<String>,
    /// algorithmName
    /// Represents the following attribute in the schema: :algorithmName
    #[xml(attr = "algorithmName")]
    pub algorithm_name: Option<String>,
    /// hashValue
    /// Represents the following attribute in the schema: :hashValue
    #[xml(attr = "hashValue")]
    pub hash_value: Option<String>,
    /// saltValue
    /// Represents the following attribute in the schema: :saltValue
    #[xml(attr = "saltValue")]
    pub salt_value: Option<String>,
    /// spinCount
    /// Represents the following attribute in the schema: :spinCount
    #[xml(attr = "spinCount")]
    pub spin_count: Option<i32>,
    /// sqref
    /// Represents the following attribute in the schema: :sqref
    #[xml(attr = "sqref")]
    pub sequence_of_references: String,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// securityDescriptor
    /// Represents the following attribute in the schema: :securityDescriptor
    #[xml(attr = "securityDescriptor")]
    pub security_descriptor: Option<String>,
}
/// Cell Watch Item.
/// When the object is serialized out as xml, it's qualified name is x:cellWatch.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cellWatch")]
pub struct CellWatch {
    /// Reference
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub cell_reference: String,
}
/// Chart Sheet Page Setup.
/// When the object is serialized out as xml, it's qualified name is x:pageSetup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pageSetup")]
pub struct ChartSheetPageSetup {
    /// Paper Size
    /// Represents the following attribute in the schema: :paperSize
    #[xml(attr = "paperSize")]
    pub paper_size: Option<i32>,
    /// First Page Number
    /// Represents the following attribute in the schema: :firstPageNumber
    #[xml(attr = "firstPageNumber")]
    pub first_page_number: Option<i32>,
    /// Orientation
    /// Represents the following attribute in the schema: :orientation
    #[xml(attr = "orientation")]
    pub orientation: Option<OrientationValues>,
    /// Use Printer Defaults
    /// Represents the following attribute in the schema: :usePrinterDefaults
    #[xml(attr = "usePrinterDefaults")]
    pub use_printer_defaults: Option<bool>,
    /// Black And White
    /// Represents the following attribute in the schema: :blackAndWhite
    #[xml(attr = "blackAndWhite")]
    pub black_and_white: Option<bool>,
    /// Draft
    /// Represents the following attribute in the schema: :draft
    #[xml(attr = "draft")]
    pub draft: Option<bool>,
    /// Use First Page Number
    /// Represents the following attribute in the schema: :useFirstPageNumber
    #[xml(attr = "useFirstPageNumber")]
    pub use_first_page_number: Option<bool>,
    /// Horizontal DPI
    /// Represents the following attribute in the schema: :horizontalDpi
    #[xml(attr = "horizontalDpi")]
    pub horizontal_dpi: Option<i32>,
    /// Vertical DPI
    /// Represents the following attribute in the schema: :verticalDpi
    #[xml(attr = "verticalDpi")]
    pub vertical_dpi: Option<i32>,
    /// Number Of Copies
    /// Represents the following attribute in the schema: :copies
    #[xml(attr = "copies")]
    pub copies: Option<i32>,
    /// Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
}
/// Custom Property.
/// When the object is serialized out as xml, it's qualified name is x:customPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:customPr")]
pub struct CustomProperty {
    /// Custom Property Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Web Publishing Item.
/// When the object is serialized out as xml, it's qualified name is x:webPublishItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:webPublishItem")]
pub struct WebPublishItem {
    /// Id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// Destination Bookmark
    /// Represents the following attribute in the schema: :divId
    #[xml(attr = "divId")]
    pub div_id: String,
    /// Web Source Type
    /// Represents the following attribute in the schema: :sourceType
    #[xml(attr = "sourceType")]
    pub source_type: WebSourceValues,
    /// Source Id
    /// Represents the following attribute in the schema: :sourceRef
    #[xml(attr = "sourceRef")]
    pub source_ref: Option<String>,
    /// Source Object Name
    /// Represents the following attribute in the schema: :sourceObject
    #[xml(attr = "sourceObject")]
    pub source_object: Option<String>,
    /// Destination File Name
    /// Represents the following attribute in the schema: :destinationFile
    #[xml(attr = "destinationFile")]
    pub destination_file: String,
    /// Title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// Automatically Publish
    /// Represents the following attribute in the schema: :autoRepublish
    #[xml(attr = "autoRepublish")]
    pub auto_republish: Option<bool>,
}
/// Table Part.
/// When the object is serialized out as xml, it's qualified name is x:tablePart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:tablePart")]
pub struct TablePart {
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Chart Sheet View.
/// When the object is serialized out as xml, it's qualified name is x:sheetView.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetView")]
pub struct ChartSheetView {
    /// Sheet Tab Selected
    /// Represents the following attribute in the schema: :tabSelected
    #[xml(attr = "tabSelected")]
    pub tab_selected: Option<bool>,
    /// Window Zoom Scale
    /// Represents the following attribute in the schema: :zoomScale
    #[xml(attr = "zoomScale")]
    pub zoom_scale: Option<i32>,
    /// Workbook View Id
    /// Represents the following attribute in the schema: :workbookViewId
    #[xml(attr = "workbookViewId")]
    pub workbook_view_id: i32,
    /// Zoom To Fit
    /// Represents the following attribute in the schema: :zoomToFit
    #[xml(attr = "zoomToFit")]
    pub zoom_to_fit: Option<bool>,
    /// _
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Custom Chart Sheet View.
/// When the object is serialized out as xml, it's qualified name is x:customSheetView.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:customSheetView")]
pub struct CustomChartsheetView {
    /// GUID
    /// Represents the following attribute in the schema: :guid
    #[xml(attr = "guid")]
    pub guid: String,
    /// Print Scale
    /// Represents the following attribute in the schema: :scale
    #[xml(attr = "scale")]
    pub scale: Option<i32>,
    /// Visible State
    /// Represents the following attribute in the schema: :state
    #[xml(attr = "state")]
    pub state: Option<SheetStateValues>,
    /// Zoom To Fit
    /// Represents the following attribute in the schema: :zoomToFit
    #[xml(attr = "zoomToFit")]
    pub zoom_to_fit: Option<bool>,
    /// _
    #[xml(child = "x:pageMargins")]
    pub page_margins: Option<PageMargins>,
    ///Chart Sheet Page Setup
    #[xml(child = "x:pageSetup")]
    pub chart_sheet_page_setup: Option<ChartSheetPageSetup>,
    /// _
    #[xml(child = "x:headerFooter")]
    pub header_footer: Option<HeaderFooter>,
}
/// Input Cells.
/// When the object is serialized out as xml, it's qualified name is x:inputCells.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:inputCells")]
pub struct InputCells {
    /// Reference
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub cell_reference: String,
    /// Deleted
    /// Represents the following attribute in the schema: :deleted
    #[xml(attr = "deleted")]
    pub deleted: Option<bool>,
    /// Undone
    /// Represents the following attribute in the schema: :undone
    #[xml(attr = "undone")]
    pub undone: Option<bool>,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
    /// Number Format Id
    /// Represents the following attribute in the schema: :numFmtId
    #[xml(attr = "numFmtId")]
    pub number_format_id: Option<i32>,
}
/// Embedded Control.
/// When the object is serialized out as xml, it's qualified name is x:control.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:control")]
pub struct Control {
    /// Shape Id
    /// Represents the following attribute in the schema: :shapeId
    #[xml(attr = "shapeId")]
    pub shape_id: i32,
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
    /// Control Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// _
    #[xml(child = "x:controlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Ignored Error.
/// When the object is serialized out as xml, it's qualified name is x:ignoredError.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ignoredError")]
pub struct IgnoredError {
    /// Sequence of References
    /// Represents the following attribute in the schema: :sqref
    #[xml(attr = "sqref")]
    pub sequence_of_references: String,
    /// Evaluation Error
    /// Represents the following attribute in the schema: :evalError
    #[xml(attr = "evalError")]
    pub eval_error: Option<bool>,
    /// Two Digit Text Year
    /// Represents the following attribute in the schema: :twoDigitTextYear
    #[xml(attr = "twoDigitTextYear")]
    pub two_digit_text_year: Option<bool>,
    /// Number Stored As Text
    /// Represents the following attribute in the schema: :numberStoredAsText
    #[xml(attr = "numberStoredAsText")]
    pub number_stored_as_text: Option<bool>,
    /// Formula
    /// Represents the following attribute in the schema: :formula
    #[xml(attr = "formula")]
    pub formula: Option<bool>,
    /// Formula Range
    /// Represents the following attribute in the schema: :formulaRange
    #[xml(attr = "formulaRange")]
    pub formula_range: Option<bool>,
    /// Unlocked Formula
    /// Represents the following attribute in the schema: :unlockedFormula
    #[xml(attr = "unlockedFormula")]
    pub unlocked_formula: Option<bool>,
    /// Empty Cell Reference
    /// Represents the following attribute in the schema: :emptyCellReference
    #[xml(attr = "emptyCellReference")]
    pub empty_cell_reference: Option<bool>,
    /// List Data Validation
    /// Represents the following attribute in the schema: :listDataValidation
    #[xml(attr = "listDataValidation")]
    pub list_data_validation: Option<bool>,
    /// Calculated Column
    /// Represents the following attribute in the schema: :calculatedColumn
    #[xml(attr = "calculatedColumn")]
    pub calculated_column: Option<bool>,
}
/// Merged Cell.
/// When the object is serialized out as xml, it's qualified name is x:mergeCell.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:mergeCell")]
pub struct MergeCell {
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: String,
}
/// Data Validation.
/// When the object is serialized out as xml, it's qualified name is x:dataValidation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dataValidation")]
pub struct DataValidation {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<DataValidationValues>,
    /// errorStyle
    /// Represents the following attribute in the schema: :errorStyle
    #[xml(attr = "errorStyle")]
    pub error_style: Option<DataValidationErrorStyleValues>,
    /// imeMode
    /// Represents the following attribute in the schema: :imeMode
    #[xml(attr = "imeMode")]
    pub ime_mode: Option<DataValidationImeModeValues>,
    /// operator
    /// Represents the following attribute in the schema: :operator
    #[xml(attr = "operator")]
    pub operator: Option<DataValidationOperatorValues>,
    /// allowBlank
    /// Represents the following attribute in the schema: :allowBlank
    #[xml(attr = "allowBlank")]
    pub allow_blank: Option<bool>,
    /// showDropDown
    /// Represents the following attribute in the schema: :showDropDown
    #[xml(attr = "showDropDown")]
    pub show_drop_down: Option<bool>,
    /// showInputMessage
    /// Represents the following attribute in the schema: :showInputMessage
    #[xml(attr = "showInputMessage")]
    pub show_input_message: Option<bool>,
    /// showErrorMessage
    /// Represents the following attribute in the schema: :showErrorMessage
    #[xml(attr = "showErrorMessage")]
    pub show_error_message: Option<bool>,
    /// errorTitle
    /// Represents the following attribute in the schema: :errorTitle
    #[xml(attr = "errorTitle")]
    pub error_title: Option<String>,
    /// error
    /// Represents the following attribute in the schema: :error
    #[xml(attr = "error")]
    pub error: Option<String>,
    /// promptTitle
    /// Represents the following attribute in the schema: :promptTitle
    #[xml(attr = "promptTitle")]
    pub prompt_title: Option<String>,
    /// prompt
    /// Represents the following attribute in the schema: :prompt
    #[xml(attr = "prompt")]
    pub prompt: Option<String>,
    /// sqref
    /// Represents the following attribute in the schema: :sqref
    #[xml(attr = "sqref")]
    pub sequence_of_references: String,
    /// _
    #[xml(child = "x12ac:list")]
    pub list: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2011_1_ac::List,
    >,
    /// _
    #[xml(child = "x:formula1")]
    pub formula1: Option<Formula1>,
    /// _
    #[xml(child = "x:formula2")]
    pub formula2: Option<Formula2>,
}
/// Worksheet View.
/// When the object is serialized out as xml, it's qualified name is x:sheetView.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetView")]
pub struct SheetView {
    /// Window Protection
    /// Represents the following attribute in the schema: :windowProtection
    #[xml(attr = "windowProtection")]
    pub window_protection: Option<bool>,
    /// Show Formulas
    /// Represents the following attribute in the schema: :showFormulas
    #[xml(attr = "showFormulas")]
    pub show_formulas: Option<bool>,
    /// Show Grid Lines
    /// Represents the following attribute in the schema: :showGridLines
    #[xml(attr = "showGridLines")]
    pub show_grid_lines: Option<bool>,
    /// Show Headers
    /// Represents the following attribute in the schema: :showRowColHeaders
    #[xml(attr = "showRowColHeaders")]
    pub show_row_col_headers: Option<bool>,
    /// Show Zero Values
    /// Represents the following attribute in the schema: :showZeros
    #[xml(attr = "showZeros")]
    pub show_zeros: Option<bool>,
    /// Right To Left
    /// Represents the following attribute in the schema: :rightToLeft
    #[xml(attr = "rightToLeft")]
    pub right_to_left: Option<bool>,
    /// Sheet Tab Selected
    /// Represents the following attribute in the schema: :tabSelected
    #[xml(attr = "tabSelected")]
    pub tab_selected: Option<bool>,
    /// Show Ruler
    /// Represents the following attribute in the schema: :showRuler
    #[xml(attr = "showRuler")]
    pub show_ruler: Option<bool>,
    /// Show Outline Symbols
    /// Represents the following attribute in the schema: :showOutlineSymbols
    #[xml(attr = "showOutlineSymbols")]
    pub show_outline_symbols: Option<bool>,
    /// Default Grid Color
    /// Represents the following attribute in the schema: :defaultGridColor
    #[xml(attr = "defaultGridColor")]
    pub default_grid_color: Option<bool>,
    /// Show White Space
    /// Represents the following attribute in the schema: :showWhiteSpace
    #[xml(attr = "showWhiteSpace")]
    pub show_white_space: Option<bool>,
    /// View Type
    /// Represents the following attribute in the schema: :view
    #[xml(attr = "view")]
    pub view: Option<SheetViewValues>,
    /// Top Left Visible Cell
    /// Represents the following attribute in the schema: :topLeftCell
    #[xml(attr = "topLeftCell")]
    pub top_left_cell: Option<String>,
    /// Color Id
    /// Represents the following attribute in the schema: :colorId
    #[xml(attr = "colorId")]
    pub color_id: Option<i32>,
    /// Zoom Scale
    /// Represents the following attribute in the schema: :zoomScale
    #[xml(attr = "zoomScale")]
    pub zoom_scale: Option<i32>,
    /// Zoom Scale Normal View
    /// Represents the following attribute in the schema: :zoomScaleNormal
    #[xml(attr = "zoomScaleNormal")]
    pub zoom_scale_normal: Option<i32>,
    /// Zoom Scale Page Break Preview
    /// Represents the following attribute in the schema: :zoomScaleSheetLayoutView
    #[xml(attr = "zoomScaleSheetLayoutView")]
    pub zoom_scale_sheet_layout_view: Option<i32>,
    /// Zoom Scale Page Layout View
    /// Represents the following attribute in the schema: :zoomScalePageLayoutView
    #[xml(attr = "zoomScalePageLayoutView")]
    pub zoom_scale_page_layout_view: Option<i32>,
    /// Workbook View Index
    /// Represents the following attribute in the schema: :workbookViewId
    #[xml(attr = "workbookViewId")]
    pub workbook_view_id: i32,
    ///View Pane
    #[xml(child = "x:pane")]
    pub pane: Option<Pane>,
    /// _
    #[xml(child = "x:selection")]
    pub x_selection: Vec<Selection>,
    /// _
    #[xml(child = "x:pivotSelection")]
    pub x_pivot_selection: Vec<PivotSelection>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<ExtensionList>,
}
/// Custom Sheet View.
/// When the object is serialized out as xml, it's qualified name is x:customSheetView.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:customSheetView")]
pub struct CustomSheetView {
    /// GUID
    /// Represents the following attribute in the schema: :guid
    #[xml(attr = "guid")]
    pub guid: String,
    /// Print Scale
    /// Represents the following attribute in the schema: :scale
    #[xml(attr = "scale")]
    pub scale: Option<i32>,
    /// Color Id
    /// Represents the following attribute in the schema: :colorId
    #[xml(attr = "colorId")]
    pub color_id: Option<i32>,
    /// Show Page Breaks
    /// Represents the following attribute in the schema: :showPageBreaks
    #[xml(attr = "showPageBreaks")]
    pub show_page_breaks: Option<bool>,
    /// Show Formulas
    /// Represents the following attribute in the schema: :showFormulas
    #[xml(attr = "showFormulas")]
    pub show_formulas: Option<bool>,
    /// Show Grid Lines
    /// Represents the following attribute in the schema: :showGridLines
    #[xml(attr = "showGridLines")]
    pub show_grid_lines: Option<bool>,
    /// Show Headers
    /// Represents the following attribute in the schema: :showRowCol
    #[xml(attr = "showRowCol")]
    pub show_row_column: Option<bool>,
    /// Show Outline Symbols
    /// Represents the following attribute in the schema: :outlineSymbols
    #[xml(attr = "outlineSymbols")]
    pub outline_symbols: Option<bool>,
    /// Show Zero Values
    /// Represents the following attribute in the schema: :zeroValues
    #[xml(attr = "zeroValues")]
    pub zero_values: Option<bool>,
    /// Fit To Page
    /// Represents the following attribute in the schema: :fitToPage
    #[xml(attr = "fitToPage")]
    pub fit_to_page: Option<bool>,
    /// Print Area Defined
    /// Represents the following attribute in the schema: :printArea
    #[xml(attr = "printArea")]
    pub print_area: Option<bool>,
    /// Filtered List
    /// Represents the following attribute in the schema: :filter
    #[xml(attr = "filter")]
    pub filter: Option<bool>,
    /// Show AutoFitler Drop Down Controls
    /// Represents the following attribute in the schema: :showAutoFilter
    #[xml(attr = "showAutoFilter")]
    pub show_auto_filter: Option<bool>,
    /// Hidden Rows
    /// Represents the following attribute in the schema: :hiddenRows
    #[xml(attr = "hiddenRows")]
    pub hidden_rows: Option<bool>,
    /// Hidden Columns
    /// Represents the following attribute in the schema: :hiddenColumns
    #[xml(attr = "hiddenColumns")]
    pub hidden_columns: Option<bool>,
    /// Visible State
    /// Represents the following attribute in the schema: :state
    #[xml(attr = "state")]
    pub state: Option<SheetStateValues>,
    /// Filter
    /// Represents the following attribute in the schema: :filterUnique
    #[xml(attr = "filterUnique")]
    pub filter_unique: Option<bool>,
    /// View Type
    /// Represents the following attribute in the schema: :view
    #[xml(attr = "view")]
    pub view: Option<SheetViewValues>,
    /// Show Ruler
    /// Represents the following attribute in the schema: :showRuler
    #[xml(attr = "showRuler")]
    pub show_ruler: Option<bool>,
    /// Top Left Visible Cell
    /// Represents the following attribute in the schema: :topLeftCell
    #[xml(attr = "topLeftCell")]
    pub top_left_cell: Option<String>,
    ///Pane Split Information
    #[xml(child = "x:pane")]
    pub pane: Option<Pane>,
    ///Selection
    #[xml(child = "x:selection")]
    pub selection: Option<Selection>,
    ///Horizontal Page Breaks
    #[xml(child = "x:rowBreaks")]
    pub row_breaks: Option<RowBreaks>,
    ///Vertical Page Breaks
    #[xml(child = "x:colBreaks")]
    pub column_breaks: Option<ColumnBreaks>,
    ///Page Margins
    #[xml(child = "x:pageMargins")]
    pub page_margins: Option<PageMargins>,
    ///Print Options
    #[xml(child = "x:printOptions")]
    pub print_options: Option<PrintOptions>,
    ///Page Setup Settings
    #[xml(child = "x:pageSetup")]
    pub page_setup: Option<PageSetup>,
    ///Header Footer Settings
    #[xml(child = "x:headerFooter")]
    pub header_footer: Option<HeaderFooter>,
    ///AutoFilter Settings
    #[xml(child = "x:autoFilter")]
    pub auto_filter: Option<AutoFilter>,
    /// _
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// OLE Object.
/// When the object is serialized out as xml, it's qualified name is x:oleObject.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:oleObject")]
pub struct OleObject {
    /// OLE ProgId
    /// Represents the following attribute in the schema: :progId
    #[xml(attr = "progId")]
    pub prog_id: Option<String>,
    /// Data or View Aspect
    /// Represents the following attribute in the schema: :dvAspect
    #[xml(attr = "dvAspect")]
    pub data_or_view_aspect: Option<DataViewAspectValues>,
    /// OLE Link Moniker
    /// Represents the following attribute in the schema: :link
    #[xml(attr = "link")]
    pub link: Option<String>,
    /// OLE Update
    /// Represents the following attribute in the schema: :oleUpdate
    #[xml(attr = "oleUpdate")]
    pub ole_update: Option<OleUpdateValues>,
    /// Auto Load
    /// Represents the following attribute in the schema: :autoLoad
    #[xml(attr = "autoLoad")]
    pub auto_load: Option<bool>,
    /// Shape Id
    /// Represents the following attribute in the schema: :shapeId
    #[xml(attr = "shapeId")]
    pub shape_id: i32,
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    /// _
    #[xml(child = "x:objectPr")]
    pub embedded_object_properties: Option<EmbeddedObjectProperties>,
}
/// Metadata Types Collection.
/// When the object is serialized out as xml, it's qualified name is x:metadataTypes.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:metadataTypes")]
pub struct MetadataTypes {
    /// Metadata Type Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:metadataType")]
    pub x_metadata_type: Vec<MetadataType>,
}
/// Metadata String Store.
/// When the object is serialized out as xml, it's qualified name is x:metadataStrings.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:metadataStrings")]
pub struct MetadataStrings {
    /// MDX Metadata String Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:s")]
    pub x_s: Vec<CharacterValue>,
}
/// MDX Metadata Information.
/// When the object is serialized out as xml, it's qualified name is x:mdxMetadata.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:mdxMetadata")]
pub struct MdxMetadata {
    /// MDX Metadata Record Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:mdx")]
    pub x_mdx: Vec<Mdx>,
}
/// Future Metadata.
/// When the object is serialized out as xml, it's qualified name is x:futureMetadata.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:futureMetadata")]
pub struct FutureMetadata {
    /// Metadata Type Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Future Metadata Block Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:bk")]
    pub x_bk: Vec<FutureMetadataBlock>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<ExtensionList>,
}
/// Cell Metadata.
/// When the object is serialized out as xml, it's qualified name is x:cellMetadata.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cellMetadata")]
pub struct CellMetadata {
    /// Metadata Block Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    #[xml(child = "x:bk")]
    pub children: Vec<CellMetadataChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CellMetadataChildChoice {
    #[xml(tag = "x:bk")]
    XBk(MetadataBlock),
}
/// Value Metadata.
/// When the object is serialized out as xml, it's qualified name is x:valueMetadata.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:valueMetadata")]
pub struct ValueMetadata {
    /// Metadata Block Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    #[xml(child = "x:bk")]
    pub children: Vec<ValueMetadataChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ValueMetadataChildChoice {
    #[xml(tag = "x:bk")]
    XBk(MetadataBlock),
}
/// Defines the MetadataBlocksType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct MetadataBlocksType {
    /// Metadata Block Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    #[xml(child = "x:bk")]
    pub children: Vec<MetadataBlocksTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MetadataBlocksTypeChildChoice {
    #[xml(tag = "x:bk")]
    XBk(MetadataBlock),
}
/// Metadata Type Information.
/// When the object is serialized out as xml, it's qualified name is x:metadataType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:metadataType")]
pub struct MetadataType {
    /// Metadata Type Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Minimum Supported Version
    /// Represents the following attribute in the schema: :minSupportedVersion
    #[xml(attr = "minSupportedVersion")]
    pub min_supported_version: i32,
    /// Metadata Ghost Row
    /// Represents the following attribute in the schema: :ghostRow
    #[xml(attr = "ghostRow")]
    pub ghost_row: Option<bool>,
    /// Metadata Ghost Column
    /// Represents the following attribute in the schema: :ghostCol
    #[xml(attr = "ghostCol")]
    pub ghost_column: Option<bool>,
    /// Metadata Edit
    /// Represents the following attribute in the schema: :edit
    #[xml(attr = "edit")]
    pub edit: Option<bool>,
    /// Metadata Cell Value Delete
    /// Represents the following attribute in the schema: :delete
    #[xml(attr = "delete")]
    pub delete: Option<bool>,
    /// Metadata Copy
    /// Represents the following attribute in the schema: :copy
    #[xml(attr = "copy")]
    pub copy: Option<bool>,
    /// Metadata Paste All
    /// Represents the following attribute in the schema: :pasteAll
    #[xml(attr = "pasteAll")]
    pub paste_all: Option<bool>,
    /// Metadata Paste Formulas
    /// Represents the following attribute in the schema: :pasteFormulas
    #[xml(attr = "pasteFormulas")]
    pub paste_formulas: Option<bool>,
    /// Metadata Paste Special Values
    /// Represents the following attribute in the schema: :pasteValues
    #[xml(attr = "pasteValues")]
    pub paste_values: Option<bool>,
    /// Metadata Paste Formats
    /// Represents the following attribute in the schema: :pasteFormats
    #[xml(attr = "pasteFormats")]
    pub paste_formats: Option<bool>,
    /// Metadata Paste Comments
    /// Represents the following attribute in the schema: :pasteComments
    #[xml(attr = "pasteComments")]
    pub paste_comments: Option<bool>,
    /// Metadata Paste Data Validation
    /// Represents the following attribute in the schema: :pasteDataValidation
    #[xml(attr = "pasteDataValidation")]
    pub paste_data_validation: Option<bool>,
    /// Metadata Paste Borders
    /// Represents the following attribute in the schema: :pasteBorders
    #[xml(attr = "pasteBorders")]
    pub paste_borders: Option<bool>,
    /// Metadata Paste Column Widths
    /// Represents the following attribute in the schema: :pasteColWidths
    #[xml(attr = "pasteColWidths")]
    pub paste_col_widths: Option<bool>,
    /// Metadata Paste Number Formats
    /// Represents the following attribute in the schema: :pasteNumberFormats
    #[xml(attr = "pasteNumberFormats")]
    pub paste_number_formats: Option<bool>,
    /// Metadata Merge
    /// Represents the following attribute in the schema: :merge
    #[xml(attr = "merge")]
    pub merge: Option<bool>,
    /// Meatadata Split First
    /// Represents the following attribute in the schema: :splitFirst
    #[xml(attr = "splitFirst")]
    pub split_first: Option<bool>,
    /// Metadata Split All
    /// Represents the following attribute in the schema: :splitAll
    #[xml(attr = "splitAll")]
    pub split_all: Option<bool>,
    /// Metadata Insert Delete
    /// Represents the following attribute in the schema: :rowColShift
    #[xml(attr = "rowColShift")]
    pub row_column_shift: Option<bool>,
    /// Metadata Clear All
    /// Represents the following attribute in the schema: :clearAll
    #[xml(attr = "clearAll")]
    pub clear_all: Option<bool>,
    /// Metadata Clear Formats
    /// Represents the following attribute in the schema: :clearFormats
    #[xml(attr = "clearFormats")]
    pub clear_formats: Option<bool>,
    /// Metadata Clear Contents
    /// Represents the following attribute in the schema: :clearContents
    #[xml(attr = "clearContents")]
    pub clear_contents: Option<bool>,
    /// Metadata Clear Comments
    /// Represents the following attribute in the schema: :clearComments
    #[xml(attr = "clearComments")]
    pub clear_comments: Option<bool>,
    /// Metadata Formula Assignment
    /// Represents the following attribute in the schema: :assign
    #[xml(attr = "assign")]
    pub assign: Option<bool>,
    /// Metadata Coercion
    /// Represents the following attribute in the schema: :coerce
    #[xml(attr = "coerce")]
    pub coerce: Option<bool>,
    /// Adjust Metadata
    /// Represents the following attribute in the schema: :adjust
    #[xml(attr = "adjust")]
    pub adjust: Option<bool>,
    /// Cell Metadata
    /// Represents the following attribute in the schema: :cellMeta
    #[xml(attr = "cellMeta")]
    pub cell_meta: Option<bool>,
}
/// Metadata Block.
/// When the object is serialized out as xml, it's qualified name is x:bk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:bk")]
pub struct MetadataBlock {
    /// _
    #[xml(child = "x:rc")]
    pub x_rc: Vec<MetadataRecord>,
}
/// Metadata Record.
/// When the object is serialized out as xml, it's qualified name is x:rc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rc")]
pub struct MetadataRecord {
    /// Metadata Record Type Index
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub type_index: i32,
    /// Metadata Record Value Index
    /// Represents the following attribute in the schema: :v
    #[xml(attr = "v")]
    pub val: i32,
}
/// Future Metadata Block.
/// When the object is serialized out as xml, it's qualified name is x:bk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:bk")]
pub struct FutureMetadataBlock {
    ///Future Feature Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// MDX Metadata Record.
/// When the object is serialized out as xml, it's qualified name is x:mdx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:mdx")]
pub struct Mdx {
    /// Connection Name Index
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub name_index: i32,
    /// Cube Function Tag
    /// Represents the following attribute in the schema: :f
    #[xml(attr = "f")]
    pub cube_function: MdxFunctionValues,
    #[xml(child = "x:t", child = "x:ms", child = "x:p", child = "x:k")]
    pub children: Vec<MdxChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MdxChildChoice {
    #[xml(tag = "x:t")]
    XT(MdxTuple),
    #[xml(tag = "x:ms")]
    XMs(MdxSet),
    #[xml(tag = "x:p")]
    XP(MdxMemberProp),
    #[xml(tag = "x:k")]
    XK(MdxKpi),
}
/// Tuple MDX Metadata.
/// When the object is serialized out as xml, it's qualified name is x:t.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:t")]
pub struct MdxTuple {
    /// Member Index Count
    /// Represents the following attribute in the schema: :c
    #[xml(attr = "c")]
    pub member_index_count: Option<i32>,
    /// Server Formatting Culture Currency
    /// Represents the following attribute in the schema: :ct
    #[xml(attr = "ct")]
    pub culture_currency: Option<String>,
    /// Server Formatting String Index
    /// Represents the following attribute in the schema: :si
    #[xml(attr = "si")]
    pub formatting_string_index: Option<i32>,
    /// Server Formatting Built-In Number Format Index
    /// Represents the following attribute in the schema: :fi
    #[xml(attr = "fi")]
    pub format_index: Option<i32>,
    /// Server Formatting Background Color
    /// Represents the following attribute in the schema: :bc
    #[xml(attr = "bc")]
    pub background_color: Option<String>,
    /// Server Formatting Foreground Color
    /// Represents the following attribute in the schema: :fc
    #[xml(attr = "fc")]
    pub foreground_color: Option<String>,
    /// Server Formatting Italic Font
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub italic: Option<bool>,
    /// Server Formatting Underline Font
    /// Represents the following attribute in the schema: :u
    #[xml(attr = "u")]
    pub underline: Option<bool>,
    /// Server Formatting Strikethrough Font
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub strikethrough: Option<bool>,
    /// Server Formatting Bold Font
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bold: Option<bool>,
    /// _
    #[xml(child = "x:n")]
    pub x_n: Vec<NameIndex>,
}
/// Set MDX Metadata.
/// When the object is serialized out as xml, it's qualified name is x:ms.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ms")]
pub struct MdxSet {
    /// Set Definition Index
    /// Represents the following attribute in the schema: :ns
    #[xml(attr = "ns")]
    pub set_definition_index: i32,
    /// Sort By Member Index Count
    /// Represents the following attribute in the schema: :c
    #[xml(attr = "c")]
    pub member_index_count: Option<i32>,
    /// Set Sort Order
    /// Represents the following attribute in the schema: :o
    #[xml(attr = "o")]
    pub sorting_order: Option<MdxSetOrderValues>,
    /// _
    #[xml(child = "x:n")]
    pub x_n: Vec<NameIndex>,
}
/// Member Property MDX Metadata.
/// When the object is serialized out as xml, it's qualified name is x:p.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:p")]
pub struct MdxMemberProp {
    /// Member Unique Name Index
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub name_index: i32,
    /// Property Name Index
    /// Represents the following attribute in the schema: :np
    #[xml(attr = "np")]
    pub property_name_index: i32,
}
/// KPI MDX Metadata.
/// When the object is serialized out as xml, it's qualified name is x:k.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:k")]
pub struct MdxKpi {
    /// Member Unique Name Index
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub name_index: i32,
    /// KPI Index
    /// Represents the following attribute in the schema: :np
    #[xml(attr = "np")]
    pub kpi_index: i32,
    /// KPI Property
    /// Represents the following attribute in the schema: :p
    #[xml(attr = "p")]
    pub kpi_property: MdxKpiPropertyValues,
}
/// Member Unique Name Index.
/// When the object is serialized out as xml, it's qualified name is x:n.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:n")]
pub struct NameIndex {
    /// Index Value
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub index: i32,
    /// String is a Set
    /// Represents the following attribute in the schema: :s
    #[xml(attr = "s")]
    pub is_a_set: Option<bool>,
}
/// Table Properties.
/// When the object is serialized out as xml, it's qualified name is x:singleXmlCell.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:singleXmlCell")]
pub struct SingleXmlCell {
    /// Table Id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// Reference
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub cell_reference: String,
    /// Connection ID
    /// Represents the following attribute in the schema: :connectionId
    #[xml(attr = "connectionId")]
    pub connection_id: i32,
    ///Cell Properties
    #[xml(child = "x:xmlCellPr")]
    pub xml_cell_properties: XmlCellProperties,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Cell Properties.
/// When the object is serialized out as xml, it's qualified name is x:xmlCellPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:xmlCellPr")]
pub struct XmlCellProperties {
    /// Table Field Id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// Unique Table Name
    /// Represents the following attribute in the schema: :uniqueName
    #[xml(attr = "uniqueName")]
    pub unique_name: String,
    ///Column XML Properties
    #[xml(child = "x:xmlPr")]
    pub xml_properties: XmlProperties,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Column XML Properties.
/// When the object is serialized out as xml, it's qualified name is x:xmlPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:xmlPr")]
pub struct XmlProperties {
    /// XML Map Id
    /// Represents the following attribute in the schema: :mapId
    #[xml(attr = "mapId")]
    pub map_id: i32,
    /// XPath
    /// Represents the following attribute in the schema: :xpath
    #[xml(attr = "xpath")]
    pub x_path: String,
    /// XML Data Type
    /// Represents the following attribute in the schema: :xmlDataType
    #[xml(attr = "xmlDataType")]
    pub xml_data_type: XmlDataValues,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Pattern.
/// When the object is serialized out as xml, it's qualified name is x:patternFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:patternFill")]
pub struct PatternFill {
    /// Pattern Type
    /// Represents the following attribute in the schema: :patternType
    #[xml(attr = "patternType")]
    pub pattern_type: Option<PatternValues>,
    ///Foreground Color
    #[xml(child = "x:fgColor")]
    pub foreground_color: Option<ForegroundColor>,
    ///Background Color
    #[xml(child = "x:bgColor")]
    pub background_color: Option<BackgroundColor>,
}
/// Gradient.
/// When the object is serialized out as xml, it's qualified name is x:gradientFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:gradientFill")]
pub struct GradientFill {
    /// Gradient Fill Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<GradientValues>,
    /// Linear Gradient Degree
    /// Represents the following attribute in the schema: :degree
    #[xml(attr = "degree")]
    pub degree: Option<f64>,
    /// Left Convergence
    /// Represents the following attribute in the schema: :left
    #[xml(attr = "left")]
    pub left: Option<f64>,
    /// Right Convergence
    /// Represents the following attribute in the schema: :right
    #[xml(attr = "right")]
    pub right: Option<f64>,
    /// Top Gradient Convergence
    /// Represents the following attribute in the schema: :top
    #[xml(attr = "top")]
    pub top: Option<f64>,
    /// Bottom Convergence
    /// Represents the following attribute in the schema: :bottom
    #[xml(attr = "bottom")]
    pub bottom: Option<f64>,
    /// _
    #[xml(child = "x:stop")]
    pub x_stop: Vec<GradientStop>,
}
/// Gradient Stop.
/// When the object is serialized out as xml, it's qualified name is x:stop.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:stop")]
pub struct GradientStop {
    /// Gradient Stop Position
    /// Represents the following attribute in the schema: :position
    #[xml(attr = "position")]
    pub position: f64,
    ///Color
    #[xml(child = "x:color")]
    pub color: Color,
}
/// Number Formats.
/// When the object is serialized out as xml, it's qualified name is x:numFmt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:numFmt")]
pub struct NumberingFormat {
    /// Number Format Id
    /// Represents the following attribute in the schema: :numFmtId
    #[xml(attr = "numFmtId")]
    pub number_format_id: i32,
    /// Number Format Code
    /// Represents the following attribute in the schema: :formatCode
    #[xml(attr = "formatCode")]
    pub format_code: String,
}
/// Alignment.
/// When the object is serialized out as xml, it's qualified name is x:alignment.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:alignment")]
pub struct Alignment {
    /// Horizontal Alignment
    /// Represents the following attribute in the schema: :horizontal
    #[xml(attr = "horizontal")]
    pub horizontal: Option<HorizontalAlignmentValues>,
    /// Vertical Alignment
    /// Represents the following attribute in the schema: :vertical
    #[xml(attr = "vertical")]
    pub vertical: Option<VerticalAlignmentValues>,
    /// Text Rotation
    /// Represents the following attribute in the schema: :textRotation
    #[xml(attr = "textRotation")]
    pub text_rotation: Option<i32>,
    /// Wrap Text
    /// Represents the following attribute in the schema: :wrapText
    #[xml(attr = "wrapText")]
    pub wrap_text: Option<bool>,
    /// Indent
    /// Represents the following attribute in the schema: :indent
    #[xml(attr = "indent")]
    pub indent: Option<i32>,
    /// Relative Indent
    /// Represents the following attribute in the schema: :relativeIndent
    #[xml(attr = "relativeIndent")]
    pub relative_indent: Option<i32>,
    /// Justify Last Line
    /// Represents the following attribute in the schema: :justifyLastLine
    #[xml(attr = "justifyLastLine")]
    pub justify_last_line: Option<bool>,
    /// Shrink To Fit
    /// Represents the following attribute in the schema: :shrinkToFit
    #[xml(attr = "shrinkToFit")]
    pub shrink_to_fit: Option<bool>,
    /// Reading Order
    /// Represents the following attribute in the schema: :readingOrder
    #[xml(attr = "readingOrder")]
    pub reading_order: Option<i32>,
    /// mergeCell
    /// Represents the following attribute in the schema: :mergeCell
    #[xml(attr = "mergeCell")]
    pub merge_cell: Option<String>,
}
/// Protection.
/// When the object is serialized out as xml, it's qualified name is x:protection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:protection")]
pub struct Protection {
    /// Cell Locked
    /// Represents the following attribute in the schema: :locked
    #[xml(attr = "locked")]
    pub locked: Option<bool>,
    /// Hidden Cell
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
}
/// Font Properties.
/// When the object is serialized out as xml, it's qualified name is x:font.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:font")]
pub struct Font {
    ///Bold
    #[xml(child = "x:b")]
    pub bold: Option<Bold>,
    ///Italic
    #[xml(child = "x:i")]
    pub italic: Option<Italic>,
    ///Strike Through
    #[xml(child = "x:strike")]
    pub strike: Option<Strike>,
    ///Condense
    #[xml(child = "x:condense")]
    pub condense: Option<Condense>,
    ///Extend
    #[xml(child = "x:extend")]
    pub extend: Option<Extend>,
    ///Outline
    #[xml(child = "x:outline")]
    pub outline: Option<Outline>,
    ///Shadow
    #[xml(child = "x:shadow")]
    pub shadow: Option<Shadow>,
    ///Underline
    #[xml(child = "x:u")]
    pub underline: Option<Underline>,
    ///Text Vertical Alignment
    #[xml(child = "x:vertAlign")]
    pub vertical_text_alignment: Option<VerticalTextAlignment>,
    ///Font Size
    #[xml(child = "x:sz")]
    pub font_size: Option<FontSize>,
    ///Text Color
    #[xml(child = "x:color")]
    pub color: Option<Color>,
    ///Font Name
    #[xml(child = "x:name")]
    pub font_name: Option<FontName>,
    ///Font Family
    #[xml(child = "x:family")]
    pub font_family_numbering: Option<FontFamilyNumbering>,
    ///Character Set
    #[xml(child = "x:charset")]
    pub font_char_set: Option<FontCharSet>,
    ///Scheme
    #[xml(child = "x:scheme")]
    pub font_scheme: Option<FontScheme>,
}
/// Fill.
/// When the object is serialized out as xml, it's qualified name is x:fill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:fill")]
pub struct Fill {
    #[xml(child = "x:patternFill", child = "x:gradientFill")]
    pub children: Vec<FillChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FillChildChoice {
    #[xml(tag = "x:patternFill")]
    XPatternFill(PatternFill),
    #[xml(tag = "x:gradientFill")]
    XGradientFill(GradientFill),
}
/// Border Properties.
/// When the object is serialized out as xml, it's qualified name is x:border.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:border")]
pub struct Border {
    /// Diagonal Up
    /// Represents the following attribute in the schema: :diagonalUp
    #[xml(attr = "diagonalUp")]
    pub diagonal_up: Option<bool>,
    /// Diagonal Down
    /// Represents the following attribute in the schema: :diagonalDown
    #[xml(attr = "diagonalDown")]
    pub diagonal_down: Option<bool>,
    /// Outline
    /// Represents the following attribute in the schema: :outline
    #[xml(attr = "outline")]
    pub outline: Option<bool>,
    /// _
    #[xml(child = "x:start")]
    pub start_border: Option<StartBorder>,
    /// _
    #[xml(child = "x:end")]
    pub end_border: Option<EndBorder>,
    ///Left Border
    #[xml(child = "x:left")]
    pub left_border: Option<LeftBorder>,
    ///Right Border
    #[xml(child = "x:right")]
    pub right_border: Option<RightBorder>,
    ///Top Border
    #[xml(child = "x:top")]
    pub top_border: Option<TopBorder>,
    ///Bottom Border
    #[xml(child = "x:bottom")]
    pub bottom_border: Option<BottomBorder>,
    ///Diagonal
    #[xml(child = "x:diagonal")]
    pub diagonal_border: Option<DiagonalBorder>,
    ///Vertical Inner Border
    #[xml(child = "x:vertical")]
    pub vertical_border: Option<VerticalBorder>,
    ///Horizontal Inner Borders
    #[xml(child = "x:horizontal")]
    pub horizontal_border: Option<HorizontalBorder>,
}
/// Color Indexes.
/// When the object is serialized out as xml, it's qualified name is x:indexedColors.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:indexedColors")]
pub struct IndexedColors {
    /// _
    #[xml(child = "x:rgbColor")]
    pub x_rgb_color: Vec<RgbColor>,
}
/// MRU Colors.
/// When the object is serialized out as xml, it's qualified name is x:mruColors.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:mruColors")]
pub struct MruColors {
    /// _
    #[xml(child = "x:color")]
    pub x_color: Vec<Color>,
}
/// Table Style.
/// When the object is serialized out as xml, it's qualified name is x:tableStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:tableStyle")]
pub struct TableStyle {
    /// Table Style Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Pivot Style
    /// Represents the following attribute in the schema: :pivot
    #[xml(attr = "pivot")]
    pub pivot: Option<bool>,
    /// Table
    /// Represents the following attribute in the schema: :table
    #[xml(attr = "table")]
    pub table: Option<bool>,
    /// Table Style Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:tableStyleElement")]
    pub x_table_style_element: Vec<TableStyleElement>,
}
/// RGB Color.
/// When the object is serialized out as xml, it's qualified name is x:rgbColor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rgbColor")]
pub struct RgbColor {
    /// Alpha Red Green Blue
    /// Represents the following attribute in the schema: :rgb
    #[xml(attr = "rgb")]
    pub rgb: Option<String>,
}
/// Cell Style.
/// When the object is serialized out as xml, it's qualified name is x:cellStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cellStyle")]
pub struct CellStyle {
    /// User Defined Cell Style
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Format Id
    /// Represents the following attribute in the schema: :xfId
    #[xml(attr = "xfId")]
    pub format_id: i32,
    /// Built-In Style Id
    /// Represents the following attribute in the schema: :builtinId
    #[xml(attr = "builtinId")]
    pub builtin_id: Option<i32>,
    /// Outline Style
    /// Represents the following attribute in the schema: :iLevel
    #[xml(attr = "iLevel")]
    pub outline_level: Option<i32>,
    /// Hidden Style
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    /// Custom Built In
    /// Represents the following attribute in the schema: :customBuiltin
    #[xml(attr = "customBuiltin")]
    pub custom_builtin: Option<bool>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Formatting Elements.
/// When the object is serialized out as xml, it's qualified name is x:xf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:xf")]
pub struct CellFormat {
    /// Number Format Id
    /// Represents the following attribute in the schema: :numFmtId
    #[xml(attr = "numFmtId")]
    pub number_format_id: Option<i32>,
    /// Font Id
    /// Represents the following attribute in the schema: :fontId
    #[xml(attr = "fontId")]
    pub font_id: Option<i32>,
    /// Fill Id
    /// Represents the following attribute in the schema: :fillId
    #[xml(attr = "fillId")]
    pub fill_id: Option<i32>,
    /// Border Id
    /// Represents the following attribute in the schema: :borderId
    #[xml(attr = "borderId")]
    pub border_id: Option<i32>,
    /// Format Id
    /// Represents the following attribute in the schema: :xfId
    #[xml(attr = "xfId")]
    pub format_id: Option<i32>,
    /// Quote Prefix
    /// Represents the following attribute in the schema: :quotePrefix
    #[xml(attr = "quotePrefix")]
    pub quote_prefix: Option<bool>,
    /// Pivot Button
    /// Represents the following attribute in the schema: :pivotButton
    #[xml(attr = "pivotButton")]
    pub pivot_button: Option<bool>,
    /// Apply Number Format
    /// Represents the following attribute in the schema: :applyNumberFormat
    #[xml(attr = "applyNumberFormat")]
    pub apply_number_format: Option<bool>,
    /// Apply Font
    /// Represents the following attribute in the schema: :applyFont
    #[xml(attr = "applyFont")]
    pub apply_font: Option<bool>,
    /// Apply Fill
    /// Represents the following attribute in the schema: :applyFill
    #[xml(attr = "applyFill")]
    pub apply_fill: Option<bool>,
    /// Apply Border
    /// Represents the following attribute in the schema: :applyBorder
    #[xml(attr = "applyBorder")]
    pub apply_border: Option<bool>,
    /// Apply Alignment
    /// Represents the following attribute in the schema: :applyAlignment
    #[xml(attr = "applyAlignment")]
    pub apply_alignment: Option<bool>,
    /// Apply Protection
    /// Represents the following attribute in the schema: :applyProtection
    #[xml(attr = "applyProtection")]
    pub apply_protection: Option<bool>,
    ///Alignment
    #[xml(child = "x:alignment")]
    pub alignment: Option<Alignment>,
    ///Protection
    #[xml(child = "x:protection")]
    pub protection: Option<Protection>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Font Name.
/// When the object is serialized out as xml, it's qualified name is x:name.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:name")]
pub struct FontName {
    /// String Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Font Family.
/// When the object is serialized out as xml, it's qualified name is x:family.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:family")]
pub struct FontFamilyNumbering {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Character Set.
/// When the object is serialized out as xml, it's qualified name is x:charset.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:charset")]
pub struct FontCharSet {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Table Style.
/// When the object is serialized out as xml, it's qualified name is x:tableStyleElement.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:tableStyleElement")]
pub struct TableStyleElement {
    /// Table Style Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: TableStyleValues,
    /// Band Size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: Option<i32>,
    /// Formatting Id
    /// Represents the following attribute in the schema: :dxfId
    #[xml(attr = "dxfId")]
    pub format_id: Option<i32>,
}
/// Defined Name.
/// When the object is serialized out as xml, it's qualified name is x:definedName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:definedName")]
pub struct ExternalDefinedName {
    /// Defined Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Refers To
    /// Represents the following attribute in the schema: :refersTo
    #[xml(attr = "refersTo")]
    pub refers_to: Option<String>,
    /// Sheet Id
    /// Represents the following attribute in the schema: :sheetId
    #[xml(attr = "sheetId")]
    pub sheet_id: Option<i32>,
}
/// External Sheet Data Set.
/// When the object is serialized out as xml, it's qualified name is x:sheetData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetData")]
pub struct ExternalSheetData {
    /// Sheet Id
    /// Represents the following attribute in the schema: :sheetId
    #[xml(attr = "sheetId")]
    pub sheet_id: i32,
    /// Last Refresh Resulted in Error
    /// Represents the following attribute in the schema: :refreshError
    #[xml(attr = "refreshError")]
    pub refresh_error: Option<bool>,
    /// _
    #[xml(child = "x:row")]
    pub x_row: Vec<ExternalRow>,
}
/// Row.
/// When the object is serialized out as xml, it's qualified name is x:row.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:row")]
pub struct ExternalRow {
    /// Row
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub row_index: i32,
    /// _
    #[xml(child = "x:cell")]
    pub x_cell: Vec<ExternalCell>,
}
/// External Cell Data.
/// When the object is serialized out as xml, it's qualified name is x:cell.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cell")]
pub struct ExternalCell {
    /// Reference
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub cell_reference: String,
    /// Type
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub data_type: Option<CellValues>,
    /// Value Metadata
    /// Represents the following attribute in the schema: :vm
    #[xml(attr = "vm")]
    pub value_meta_index: Option<i32>,
    ///Value
    #[xml(child = "x:v")]
    pub xstring: Option<Xstring>,
}
/// DDE Items Collection.
/// When the object is serialized out as xml, it's qualified name is x:ddeItems.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ddeItems")]
pub struct DdeItems {
    /// _
    #[xml(child = "x:ddeItem")]
    pub x_dde_item: Vec<DdeItem>,
}
/// DDE Item definition.
/// When the object is serialized out as xml, it's qualified name is x:ddeItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ddeItem")]
pub struct DdeItem {
    /// DDE Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// OLE
    /// Represents the following attribute in the schema: :ole
    #[xml(attr = "ole")]
    pub use_ole: Option<bool>,
    /// Advise
    /// Represents the following attribute in the schema: :advise
    #[xml(attr = "advise")]
    pub advise: Option<bool>,
    /// Data is an Image
    /// Represents the following attribute in the schema: :preferPic
    #[xml(attr = "preferPic")]
    pub prefer_picture: Option<bool>,
    ///DDE Name Values
    #[xml(child = "x:values")]
    pub values: Option<Values>,
}
/// DDE Name Values.
/// When the object is serialized out as xml, it's qualified name is x:values.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:values")]
pub struct Values {
    /// Rows
    /// Represents the following attribute in the schema: :rows
    #[xml(attr = "rows")]
    pub rows: Option<i32>,
    /// Columns
    /// Represents the following attribute in the schema: :cols
    #[xml(attr = "cols")]
    pub columns: Option<i32>,
    /// _
    #[xml(child = "x:value")]
    pub x_value: Vec<Value>,
}
/// Value.
/// When the object is serialized out as xml, it's qualified name is x:value.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:value")]
pub struct Value {
    /// DDE Value Type
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub value_type: Option<DdeValues>,
    ///DDE Link Value
    #[xml(child = "x:val")]
    pub dde_link_value: DdeLinkValue,
}
/// OLE Link Items.
/// When the object is serialized out as xml, it's qualified name is x:oleItems.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:oleItems")]
pub struct OleItems {
    #[xml(child = "x:oleItem", child = "x14:oleItem")]
    pub children: Vec<OleItemsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OleItemsChildChoice {
    #[xml(tag = "x:oleItem")]
    XOleItem(OleItem),
    #[xml(tag = "x14:oleItem")]
    X14OleItem(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::OleItem,
    ),
}
/// External Workbook.
/// When the object is serialized out as xml, it's qualified name is x:externalBook.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:externalBook")]
pub struct ExternalBook {
    /// Relationship to supporting book file path
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
    ///Alternate URLs and identifiers of the external book
    #[xml(child = "xxl21:alternateUrls")]
    pub external_book_alternate_urls: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2021_extlinks2021::ExternalBookAlternateUrls,
    >,
    ///Sheet names of supporting book
    #[xml(child = "x:sheetNames")]
    pub sheet_names: Option<SheetNames>,
    ///Defined names associated with supporting book.
    #[xml(child = "x:definedNames")]
    pub external_defined_names: Option<ExternalDefinedNames>,
    ///Cached worksheet data associated with supporting book
    #[xml(child = "x:sheetDataSet")]
    pub sheet_data_set: Option<SheetDataSet>,
}
/// DDE Connection.
/// When the object is serialized out as xml, it's qualified name is x:ddeLink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ddeLink")]
pub struct DdeLink {
    /// Service name
    /// Represents the following attribute in the schema: :ddeService
    #[xml(attr = "ddeService")]
    pub dde_service: String,
    /// Topic for DDE server
    /// Represents the following attribute in the schema: :ddeTopic
    #[xml(attr = "ddeTopic")]
    pub dde_topic: String,
    ///DDE Items Collection
    #[xml(child = "x:ddeItems")]
    pub dde_items: Option<DdeItems>,
}
/// OLE Link.
/// When the object is serialized out as xml, it's qualified name is x:oleLink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:oleLink")]
pub struct OleLink {
    /// OLE Link Relationship
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
    /// OLE Link ProgID
    /// Represents the following attribute in the schema: :progId
    #[xml(attr = "progId")]
    pub prog_id: String,
    ///OLE Link Items
    #[xml(child = "x:oleItems")]
    pub ole_items: Option<OleItems>,
}
/// Sheet Name.
/// When the object is serialized out as xml, it's qualified name is x:sheetName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetName")]
pub struct SheetName {
    /// Sheet Name Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<String>,
}
/// Value.
/// When the object is serialized out as xml, it's qualified name is x:v.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:v")]
pub struct Xstring {
    #[xml(text)]
    pub child: String,
}
/// Table Column.
/// When the object is serialized out as xml, it's qualified name is x:tableColumn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:tableColumn")]
pub struct TableColumn {
    /// Table Field Id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// Unique Name
    /// Represents the following attribute in the schema: :uniqueName
    #[xml(attr = "uniqueName")]
    pub unique_name: Option<String>,
    /// Column name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Totals Row Function
    /// Represents the following attribute in the schema: :totalsRowFunction
    #[xml(attr = "totalsRowFunction")]
    pub totals_row_function: Option<TotalsRowFunctionValues>,
    /// Totals Row Label
    /// Represents the following attribute in the schema: :totalsRowLabel
    #[xml(attr = "totalsRowLabel")]
    pub totals_row_label: Option<String>,
    /// Query Table Field Id
    /// Represents the following attribute in the schema: :queryTableFieldId
    #[xml(attr = "queryTableFieldId")]
    pub query_table_field_id: Option<i32>,
    /// Header Row Cell Format Id
    /// Represents the following attribute in the schema: :headerRowDxfId
    #[xml(attr = "headerRowDxfId")]
    pub header_row_differential_formatting_id: Option<i32>,
    /// Data and Insert Row Format Id
    /// Represents the following attribute in the schema: :dataDxfId
    #[xml(attr = "dataDxfId")]
    pub data_format_id: Option<i32>,
    /// Totals Row Format Id
    /// Represents the following attribute in the schema: :totalsRowDxfId
    #[xml(attr = "totalsRowDxfId")]
    pub totals_row_differential_formatting_id: Option<i32>,
    /// Header Row Cell Style
    /// Represents the following attribute in the schema: :headerRowCellStyle
    #[xml(attr = "headerRowCellStyle")]
    pub header_row_cell_style: Option<String>,
    /// Data Area Style Name
    /// Represents the following attribute in the schema: :dataCellStyle
    #[xml(attr = "dataCellStyle")]
    pub data_cell_style: Option<String>,
    /// Totals Row Style Name
    /// Represents the following attribute in the schema: :totalsRowCellStyle
    #[xml(attr = "totalsRowCellStyle")]
    pub totals_row_cell_style: Option<String>,
    ///Calculated Column Formula
    #[xml(child = "x:calculatedColumnFormula")]
    pub calculated_column_formula: Option<CalculatedColumnFormula>,
    ///Totals Row Formula
    #[xml(child = "x:totalsRowFormula")]
    pub totals_row_formula: Option<TotalsRowFormula>,
    ///XML Column Properties
    #[xml(child = "x:xmlColumnPr")]
    pub xml_column_properties: Option<XmlColumnProperties>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Calculated Column Formula.
/// When the object is serialized out as xml, it's qualified name is x:calculatedColumnFormula.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:calculatedColumnFormula")]
pub struct CalculatedColumnFormula {
    /// Array
    /// Represents the following attribute in the schema: :array
    #[xml(attr = "array")]
    pub array: Option<bool>,
    /// space
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Totals Row Formula.
/// When the object is serialized out as xml, it's qualified name is x:totalsRowFormula.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:totalsRowFormula")]
pub struct TotalsRowFormula {
    /// Array
    /// Represents the following attribute in the schema: :array
    #[xml(attr = "array")]
    pub array: Option<bool>,
    /// space
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Defines the TableFormulaType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TableFormulaType {
    /// Array
    /// Represents the following attribute in the schema: :array
    #[xml(attr = "array")]
    pub array: Option<bool>,
    /// space
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// XML Column Properties.
/// When the object is serialized out as xml, it's qualified name is x:xmlColumnPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:xmlColumnPr")]
pub struct XmlColumnProperties {
    /// XML Map Id
    /// Represents the following attribute in the schema: :mapId
    #[xml(attr = "mapId")]
    pub map_id: i32,
    /// XPath
    /// Represents the following attribute in the schema: :xpath
    #[xml(attr = "xpath")]
    pub x_path: String,
    /// Denormalized
    /// Represents the following attribute in the schema: :denormalized
    #[xml(attr = "denormalized")]
    pub denormalized: Option<bool>,
    /// XML Data Type
    /// Represents the following attribute in the schema: :xmlDataType
    #[xml(attr = "xmlDataType")]
    pub xml_data_type: XmlDataValues,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Volatile Dependency Type.
/// When the object is serialized out as xml, it's qualified name is x:volType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:volType")]
pub struct VolatileType {
    /// Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: VolatileDependencyValues,
    /// _
    #[xml(child = "x:main")]
    pub x_main: Vec<Main>,
}
/// Main.
/// When the object is serialized out as xml, it's qualified name is x:main.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:main")]
pub struct Main {
    /// First String
    /// Represents the following attribute in the schema: :first
    #[xml(attr = "first")]
    pub first: String,
    /// _
    #[xml(child = "x:tp")]
    pub x_tp: Vec<Topic>,
}
/// Topic.
/// When the object is serialized out as xml, it's qualified name is x:tp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:tp")]
pub struct Topic {
    /// Type
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub value_type: Option<VolatileValues>,
    ///Topic Value
    #[xml(child = "x:v")]
    pub xstring: Xstring,
    /// _
    #[xml(child = "x:stp")]
    pub x_stp: Vec<Subtopic>,
    /// _
    #[xml(child = "x:tr")]
    pub x_tr: Vec<TopicReferences>,
}
/// References.
/// When the object is serialized out as xml, it's qualified name is x:tr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:tr")]
pub struct TopicReferences {
    /// Reference
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub cell_reference: String,
    /// Sheet Id
    /// Represents the following attribute in the schema: :s
    #[xml(attr = "s")]
    pub sheet_id: i32,
}
/// PivotCache.
/// When the object is serialized out as xml, it's qualified name is x:pivotCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pivotCache")]
pub struct PivotCache {
    /// PivotCache Id
    /// Represents the following attribute in the schema: :cacheId
    #[xml(attr = "cacheId")]
    pub cache_id: i32,
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Web Publishing Object.
/// When the object is serialized out as xml, it's qualified name is x:webPublishObject.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:webPublishObject")]
pub struct WebPublishObject {
    /// Id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// Div Id
    /// Represents the following attribute in the schema: :divId
    #[xml(attr = "divId")]
    pub div_id: String,
    /// Source Object
    /// Represents the following attribute in the schema: :sourceObject
    #[xml(attr = "sourceObject")]
    pub source_object: Option<String>,
    /// Destination File
    /// Represents the following attribute in the schema: :destinationFile
    #[xml(attr = "destinationFile")]
    pub destination_file: String,
    /// Title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// Auto Republish
    /// Represents the following attribute in the schema: :autoRepublish
    #[xml(attr = "autoRepublish")]
    pub auto_republish: Option<bool>,
}
/// External Reference.
/// When the object is serialized out as xml, it's qualified name is x:externalReference.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:externalReference")]
pub struct ExternalReference {
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Custom Workbook View.
/// When the object is serialized out as xml, it's qualified name is x:customWorkbookView.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:customWorkbookView")]
pub struct CustomWorkbookView {
    /// Custom View Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Custom View GUID
    /// Represents the following attribute in the schema: :guid
    #[xml(attr = "guid")]
    pub guid: String,
    /// Auto Update
    /// Represents the following attribute in the schema: :autoUpdate
    #[xml(attr = "autoUpdate")]
    pub auto_update: Option<bool>,
    /// Merge Interval
    /// Represents the following attribute in the schema: :mergeInterval
    #[xml(attr = "mergeInterval")]
    pub merge_interval: Option<i32>,
    /// Changes Saved Win
    /// Represents the following attribute in the schema: :changesSavedWin
    #[xml(attr = "changesSavedWin")]
    pub changes_saved_win: Option<bool>,
    /// Only Synch
    /// Represents the following attribute in the schema: :onlySync
    #[xml(attr = "onlySync")]
    pub only_sync: Option<bool>,
    /// Personal View
    /// Represents the following attribute in the schema: :personalView
    #[xml(attr = "personalView")]
    pub personal_view: Option<bool>,
    /// Include Print Settings
    /// Represents the following attribute in the schema: :includePrintSettings
    #[xml(attr = "includePrintSettings")]
    pub include_print_settings: Option<bool>,
    /// Include Hidden Rows and Columns
    /// Represents the following attribute in the schema: :includeHiddenRowCol
    #[xml(attr = "includeHiddenRowCol")]
    pub include_hidden_row_column: Option<bool>,
    /// Maximized
    /// Represents the following attribute in the schema: :maximized
    #[xml(attr = "maximized")]
    pub maximized: Option<bool>,
    /// Minimized
    /// Represents the following attribute in the schema: :minimized
    #[xml(attr = "minimized")]
    pub minimized: Option<bool>,
    /// Show Horizontal Scroll
    /// Represents the following attribute in the schema: :showHorizontalScroll
    #[xml(attr = "showHorizontalScroll")]
    pub show_horizontal_scroll: Option<bool>,
    /// Show Vertical Scroll
    /// Represents the following attribute in the schema: :showVerticalScroll
    #[xml(attr = "showVerticalScroll")]
    pub show_vertical_scroll: Option<bool>,
    /// Show Sheet Tabs
    /// Represents the following attribute in the schema: :showSheetTabs
    #[xml(attr = "showSheetTabs")]
    pub show_sheet_tabs: Option<bool>,
    /// Top Left Corner (X Coordinate)
    /// Represents the following attribute in the schema: :xWindow
    #[xml(attr = "xWindow")]
    pub x_window: Option<i32>,
    /// Top Left Corner (Y Coordinate)
    /// Represents the following attribute in the schema: :yWindow
    #[xml(attr = "yWindow")]
    pub y_window: Option<i32>,
    /// Window Width
    /// Represents the following attribute in the schema: :windowWidth
    #[xml(attr = "windowWidth")]
    pub window_width: Option<i32>,
    /// Window Height
    /// Represents the following attribute in the schema: :windowHeight
    #[xml(attr = "windowHeight")]
    pub window_height: Option<i32>,
    /// Sheet Tab Ratio
    /// Represents the following attribute in the schema: :tabRatio
    #[xml(attr = "tabRatio")]
    pub tab_ratio: Option<i32>,
    /// Active Sheet in Book View
    /// Represents the following attribute in the schema: :activeSheetId
    #[xml(attr = "activeSheetId")]
    pub active_sheet_id: i32,
    /// Show Formula Bar
    /// Represents the following attribute in the schema: :showFormulaBar
    #[xml(attr = "showFormulaBar")]
    pub show_formula_bar: Option<bool>,
    /// Show Status Bar
    /// Represents the following attribute in the schema: :showStatusbar
    #[xml(attr = "showStatusbar")]
    pub show_statusbar: Option<bool>,
    /// Show Comments
    /// Represents the following attribute in the schema: :showComments
    #[xml(attr = "showComments")]
    pub show_comments: Option<CommentsValues>,
    /// Show Objects
    /// Represents the following attribute in the schema: :showObjects
    #[xml(attr = "showObjects")]
    pub show_objects: Option<ObjectDisplayValues>,
    /// _
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Sheet Information.
/// When the object is serialized out as xml, it's qualified name is x:sheet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheet")]
pub struct Sheet {
    /// Sheet Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Sheet Tab Id
    /// Represents the following attribute in the schema: :sheetId
    #[xml(attr = "sheetId")]
    pub sheet_id: i32,
    /// Visible State
    /// Represents the following attribute in the schema: :state
    #[xml(attr = "state")]
    pub state: Option<SheetStateValues>,
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Workbook View.
/// When the object is serialized out as xml, it's qualified name is x:workbookView.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:workbookView")]
pub struct WorkbookView {
    /// Visibility
    /// Represents the following attribute in the schema: :visibility
    #[xml(attr = "visibility")]
    pub visibility: Option<VisibilityValues>,
    /// Minimized
    /// Represents the following attribute in the schema: :minimized
    #[xml(attr = "minimized")]
    pub minimized: Option<bool>,
    /// Show Horizontal Scroll
    /// Represents the following attribute in the schema: :showHorizontalScroll
    #[xml(attr = "showHorizontalScroll")]
    pub show_horizontal_scroll: Option<bool>,
    /// Show Vertical Scroll
    /// Represents the following attribute in the schema: :showVerticalScroll
    #[xml(attr = "showVerticalScroll")]
    pub show_vertical_scroll: Option<bool>,
    /// Show Sheet Tabs
    /// Represents the following attribute in the schema: :showSheetTabs
    #[xml(attr = "showSheetTabs")]
    pub show_sheet_tabs: Option<bool>,
    /// Upper Left Corner (X Coordinate)
    /// Represents the following attribute in the schema: :xWindow
    #[xml(attr = "xWindow")]
    pub x_window: Option<i32>,
    /// Upper Left Corner (Y Coordinate)
    /// Represents the following attribute in the schema: :yWindow
    #[xml(attr = "yWindow")]
    pub y_window: Option<i32>,
    /// Window Width
    /// Represents the following attribute in the schema: :windowWidth
    #[xml(attr = "windowWidth")]
    pub window_width: Option<i32>,
    /// Window Height
    /// Represents the following attribute in the schema: :windowHeight
    #[xml(attr = "windowHeight")]
    pub window_height: Option<i32>,
    /// Sheet Tab Ratio
    /// Represents the following attribute in the schema: :tabRatio
    #[xml(attr = "tabRatio")]
    pub tab_ratio: Option<i32>,
    /// First Sheet
    /// Represents the following attribute in the schema: :firstSheet
    #[xml(attr = "firstSheet")]
    pub first_sheet: Option<i32>,
    /// Active Sheet Index
    /// Represents the following attribute in the schema: :activeTab
    #[xml(attr = "activeTab")]
    pub active_tab: Option<i32>,
    /// AutoFilter Date Grouping
    /// Represents the following attribute in the schema: :autoFilterDateGrouping
    #[xml(attr = "autoFilterDateGrouping")]
    pub auto_filter_date_grouping: Option<bool>,
    /// _
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defined Name.
/// When the object is serialized out as xml, it's qualified name is x:definedName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:definedName")]
pub struct DefinedName {
    /// Defined Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Comment
    /// Represents the following attribute in the schema: :comment
    #[xml(attr = "comment")]
    pub comment: Option<String>,
    /// Custom Menu Text
    /// Represents the following attribute in the schema: :customMenu
    #[xml(attr = "customMenu")]
    pub custom_menu: Option<String>,
    /// Description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// Help
    /// Represents the following attribute in the schema: :help
    #[xml(attr = "help")]
    pub help: Option<String>,
    /// Status Bar
    /// Represents the following attribute in the schema: :statusBar
    #[xml(attr = "statusBar")]
    pub status_bar: Option<String>,
    /// Local Name Sheet Id
    /// Represents the following attribute in the schema: :localSheetId
    #[xml(attr = "localSheetId")]
    pub local_sheet_id: Option<i32>,
    /// Hidden Name
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    /// Function
    /// Represents the following attribute in the schema: :function
    #[xml(attr = "function")]
    pub function: Option<bool>,
    /// Procedure
    /// Represents the following attribute in the schema: :vbProcedure
    #[xml(attr = "vbProcedure")]
    pub vb_procedure: Option<bool>,
    /// External Function
    /// Represents the following attribute in the schema: :xlm
    #[xml(attr = "xlm")]
    pub xlm: Option<bool>,
    /// Function Group Id
    /// Represents the following attribute in the schema: :functionGroupId
    #[xml(attr = "functionGroupId")]
    pub function_group_id: Option<i32>,
    /// Shortcut Key
    /// Represents the following attribute in the schema: :shortcutKey
    #[xml(attr = "shortcutKey")]
    pub shortcut_key: Option<String>,
    /// Publish To Server
    /// Represents the following attribute in the schema: :publishToServer
    #[xml(attr = "publishToServer")]
    pub publish_to_server: Option<bool>,
    /// Workbook Parameter (Server)
    /// Represents the following attribute in the schema: :workbookParameter
    #[xml(attr = "workbookParameter")]
    pub workbook_parameter: Option<bool>,
    #[xml(text)]
    pub child: String,
}
/// Function Group.
/// When the object is serialized out as xml, it's qualified name is x:functionGroup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:functionGroup")]
pub struct FunctionGroup {
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
}
/// Defines the ObjectAnchor Class.
/// When the object is serialized out as xml, it's qualified name is x:anchor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:anchor")]
pub struct ObjectAnchor {
    /// moveWithCells
    /// Represents the following attribute in the schema: :moveWithCells
    #[xml(attr = "moveWithCells")]
    pub move_with_cells: Option<bool>,
    /// sizeWithCells
    /// Represents the following attribute in the schema: :sizeWithCells
    #[xml(attr = "sizeWithCells")]
    pub size_with_cells: Option<bool>,
    /// z-order
    /// Represents the following attribute in the schema: :z-order
    #[xml(attr = "z-order")]
    pub z_order: Option<i32>,
    /// _
    #[xml(child = "x:from")]
    pub from_marker: FromMarker,
    /// _
    #[xml(child = "x:to")]
    pub to_marker: ToMarker,
}
/// Defines the FromMarker Class.
/// When the object is serialized out as xml, it's qualified name is x:from.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:from")]
pub struct FromMarker {
    ///Column)
    #[xml(child = "xdr:col")]
    pub column_id: crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::ColumnId,
    ///Column Offset
    #[xml(child = "xdr:colOff")]
    pub column_offset: crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::ColumnOffset,
    ///Row
    #[xml(child = "xdr:row")]
    pub row_id: crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::RowId,
    ///Row Offset
    #[xml(child = "xdr:rowOff")]
    pub row_offset: crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::RowOffset,
}
/// Defines the ToMarker Class.
/// When the object is serialized out as xml, it's qualified name is x:to.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:to")]
pub struct ToMarker {
    ///Column)
    #[xml(child = "xdr:col")]
    pub column_id: crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::ColumnId,
    ///Column Offset
    #[xml(child = "xdr:colOff")]
    pub column_offset: crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::ColumnOffset,
    ///Row
    #[xml(child = "xdr:row")]
    pub row_id: crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::RowId,
    ///Row Offset
    #[xml(child = "xdr:rowOff")]
    pub row_offset: crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::RowOffset,
}
/// Defines the MarkerType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct MarkerType {}
/// Defines the ConditionalFormattingRuleExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct ConditionalFormattingRuleExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "x14:id")]
    pub children: Vec<ConditionalFormattingRuleExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ConditionalFormattingRuleExtensionChildChoice {
    #[xml(tag = "x14:id")]
    X14Id(crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Id),
}
/// Defines the PivotHierarchyExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct PivotHierarchyExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "x14:pivotHierarchy")]
    pub children: Vec<PivotHierarchyExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PivotHierarchyExtensionChildChoice {
    #[xml(tag = "x14:pivotHierarchy")]
    X14PivotHierarchy(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::PivotHierarchy,
    ),
}
/// Defines the PivotFieldExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct PivotFieldExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "x14:pivotField")]
    pub children: Vec<PivotFieldExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PivotFieldExtensionChildChoice {
    #[xml(tag = "x14:pivotField")]
    X14PivotField(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::PivotField,
    ),
}
/// Defines the CacheSourceExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct CacheSourceExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "x14:sourceConnection")]
    pub children: Vec<CacheSourceExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CacheSourceExtensionChildChoice {
    #[xml(tag = "x14:sourceConnection")]
    X14SourceConnection(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SourceConnection,
    ),
}
/// OLE Link Item.
/// When the object is serialized out as xml, it's qualified name is x:oleItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:oleItem")]
pub struct OleItem {
    /// OLE Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Icon
    /// Represents the following attribute in the schema: :icon
    #[xml(attr = "icon")]
    pub icon: Option<bool>,
    /// Advise
    /// Represents the following attribute in the schema: :advise
    #[xml(attr = "advise")]
    pub advise: Option<bool>,
    /// Object is an Image
    /// Represents the following attribute in the schema: :preferPic
    #[xml(attr = "preferPic")]
    pub prefer_picture: Option<bool>,
}
/// Defines the StartBorder Class.
/// When the object is serialized out as xml, it's qualified name is x:start.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:start")]
pub struct StartBorder {
    /// Line Style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<BorderStyleValues>,
    ///Color
    #[xml(child = "x:color")]
    pub color: Option<Color>,
}
/// Defines the EndBorder Class.
/// When the object is serialized out as xml, it's qualified name is x:end.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:end")]
pub struct EndBorder {
    /// Line Style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<BorderStyleValues>,
    ///Color
    #[xml(child = "x:color")]
    pub color: Option<Color>,
}
/// Left Border.
/// When the object is serialized out as xml, it's qualified name is x:left.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:left")]
pub struct LeftBorder {
    /// Line Style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<BorderStyleValues>,
    ///Color
    #[xml(child = "x:color")]
    pub color: Option<Color>,
}
/// Right Border.
/// When the object is serialized out as xml, it's qualified name is x:right.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:right")]
pub struct RightBorder {
    /// Line Style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<BorderStyleValues>,
    ///Color
    #[xml(child = "x:color")]
    pub color: Option<Color>,
}
/// Top Border.
/// When the object is serialized out as xml, it's qualified name is x:top.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:top")]
pub struct TopBorder {
    /// Line Style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<BorderStyleValues>,
    ///Color
    #[xml(child = "x:color")]
    pub color: Option<Color>,
}
/// Bottom Border.
/// When the object is serialized out as xml, it's qualified name is x:bottom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:bottom")]
pub struct BottomBorder {
    /// Line Style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<BorderStyleValues>,
    ///Color
    #[xml(child = "x:color")]
    pub color: Option<Color>,
}
/// Diagonal.
/// When the object is serialized out as xml, it's qualified name is x:diagonal.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:diagonal")]
pub struct DiagonalBorder {
    /// Line Style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<BorderStyleValues>,
    ///Color
    #[xml(child = "x:color")]
    pub color: Option<Color>,
}
/// Vertical Inner Border.
/// When the object is serialized out as xml, it's qualified name is x:vertical.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:vertical")]
pub struct VerticalBorder {
    /// Line Style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<BorderStyleValues>,
    ///Color
    #[xml(child = "x:color")]
    pub color: Option<Color>,
}
/// Horizontal Inner Borders.
/// When the object is serialized out as xml, it's qualified name is x:horizontal.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:horizontal")]
pub struct HorizontalBorder {
    /// Line Style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<BorderStyleValues>,
    ///Color
    #[xml(child = "x:color")]
    pub color: Option<Color>,
}
/// Defines the BorderPropertiesType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct BorderPropertiesType {
    /// Line Style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<BorderStyleValues>,
}
/// Defines the ControlProperties Class.
/// When the object is serialized out as xml, it's qualified name is x:controlPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:controlPr")]
pub struct ControlProperties {
    /// locked
    /// Represents the following attribute in the schema: :locked
    #[xml(attr = "locked")]
    pub locked: Option<bool>,
    /// defaultSize
    /// Represents the following attribute in the schema: :defaultSize
    #[xml(attr = "defaultSize")]
    pub default_size: Option<bool>,
    /// print
    /// Represents the following attribute in the schema: :print
    #[xml(attr = "print")]
    pub print: Option<bool>,
    /// disabled
    /// Represents the following attribute in the schema: :disabled
    #[xml(attr = "disabled")]
    pub disabled: Option<bool>,
    /// recalcAlways
    /// Represents the following attribute in the schema: :recalcAlways
    #[xml(attr = "recalcAlways")]
    pub recalc_always: Option<bool>,
    /// uiObject
    /// Represents the following attribute in the schema: :uiObject
    #[xml(attr = "uiObject")]
    pub ui_object: Option<bool>,
    /// autoFill
    /// Represents the following attribute in the schema: :autoFill
    #[xml(attr = "autoFill")]
    pub auto_fill: Option<bool>,
    /// autoLine
    /// Represents the following attribute in the schema: :autoLine
    #[xml(attr = "autoLine")]
    pub auto_line: Option<bool>,
    /// autoPict
    /// Represents the following attribute in the schema: :autoPict
    #[xml(attr = "autoPict")]
    pub auto_pict: Option<bool>,
    /// macro
    /// Represents the following attribute in the schema: :macro
    #[xml(attr = "macro")]
    pub r#macro: Option<String>,
    /// altText
    /// Represents the following attribute in the schema: :altText
    #[xml(attr = "altText")]
    pub alt_text: Option<String>,
    /// linkedCell
    /// Represents the following attribute in the schema: :linkedCell
    #[xml(attr = "linkedCell")]
    pub linked_cell: Option<String>,
    /// listFillRange
    /// Represents the following attribute in the schema: :listFillRange
    #[xml(attr = "listFillRange")]
    pub list_fill_range: Option<String>,
    /// cf
    /// Represents the following attribute in the schema: :cf
    #[xml(attr = "cf")]
    pub cf: Option<String>,
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: Option<String>,
    /// _
    #[xml(child = "x:anchor")]
    pub object_anchor: ObjectAnchor,
}
/// Defines the EmbeddedObjectProperties Class.
/// When the object is serialized out as xml, it's qualified name is x:objectPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:objectPr")]
pub struct EmbeddedObjectProperties {
    /// locked
    /// Represents the following attribute in the schema: :locked
    #[xml(attr = "locked")]
    pub locked: Option<bool>,
    /// defaultSize
    /// Represents the following attribute in the schema: :defaultSize
    #[xml(attr = "defaultSize")]
    pub default_size: Option<bool>,
    /// print
    /// Represents the following attribute in the schema: :print
    #[xml(attr = "print")]
    pub print: Option<bool>,
    /// disabled
    /// Represents the following attribute in the schema: :disabled
    #[xml(attr = "disabled")]
    pub disabled: Option<bool>,
    /// uiObject
    /// Represents the following attribute in the schema: :uiObject
    #[xml(attr = "uiObject")]
    pub ui_object: Option<bool>,
    /// autoFill
    /// Represents the following attribute in the schema: :autoFill
    #[xml(attr = "autoFill")]
    pub auto_fill: Option<bool>,
    /// autoLine
    /// Represents the following attribute in the schema: :autoLine
    #[xml(attr = "autoLine")]
    pub auto_line: Option<bool>,
    /// autoPict
    /// Represents the following attribute in the schema: :autoPict
    #[xml(attr = "autoPict")]
    pub auto_pict: Option<bool>,
    /// macro
    /// Represents the following attribute in the schema: :macro
    #[xml(attr = "macro")]
    pub r#macro: Option<String>,
    /// altText
    /// Represents the following attribute in the schema: :altText
    #[xml(attr = "altText")]
    pub alt_text: Option<String>,
    /// dde
    /// Represents the following attribute in the schema: :dde
    #[xml(attr = "dde")]
    pub dde: Option<bool>,
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: Option<String>,
    /// _
    #[xml(child = "x:anchor")]
    pub object_anchor: ObjectAnchor,
}
/// Chart Sheet Properties.
/// When the object is serialized out as xml, it's qualified name is x:sheetPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetPr")]
pub struct ChartSheetProperties {
    /// Published
    /// Represents the following attribute in the schema: :published
    #[xml(attr = "published")]
    pub published: Option<bool>,
    /// Code Name
    /// Represents the following attribute in the schema: :codeName
    #[xml(attr = "codeName")]
    pub code_name: Option<String>,
    /// _
    #[xml(child = "x:tabColor")]
    pub tab_color: Option<TabColor>,
}
/// Chart Sheet Views.
/// When the object is serialized out as xml, it's qualified name is x:sheetViews.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetViews")]
pub struct ChartSheetViews {
    /// _
    #[xml(child = "x:sheetView")]
    pub x_sheet_view: Vec<ChartSheetView>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<ExtensionList>,
}
/// Chart Sheet Protection.
/// When the object is serialized out as xml, it's qualified name is x:sheetProtection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetProtection")]
pub struct ChartSheetProtection {
    /// Password
    /// Represents the following attribute in the schema: :password
    #[xml(attr = "password")]
    pub password: Option<String>,
    /// Cryptographic Algorithm Name
    /// Represents the following attribute in the schema: :algorithmName
    #[xml(attr = "algorithmName")]
    pub algorithm_name: Option<String>,
    /// Password Hash Value
    /// Represents the following attribute in the schema: :hashValue
    #[xml(attr = "hashValue")]
    pub hash_value: Option<String>,
    /// Salt Value for Password Verifier
    /// Represents the following attribute in the schema: :saltValue
    #[xml(attr = "saltValue")]
    pub salt_value: Option<String>,
    /// Iterations to Run Hashing Algorithm
    /// Represents the following attribute in the schema: :spinCount
    #[xml(attr = "spinCount")]
    pub spin_count: Option<i32>,
    /// Contents
    /// Represents the following attribute in the schema: :content
    #[xml(attr = "content")]
    pub content: Option<bool>,
    /// Objects Locked
    /// Represents the following attribute in the schema: :objects
    #[xml(attr = "objects")]
    pub objects: Option<bool>,
}
/// Custom Chart Sheet Views.
/// When the object is serialized out as xml, it's qualified name is x:customSheetViews.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:customSheetViews")]
pub struct CustomChartsheetViews {
    /// _
    #[xml(child = "x:customSheetView")]
    pub x_custom_sheet_view: Vec<CustomChartsheetView>,
}
/// Drawing.
/// When the object is serialized out as xml, it's qualified name is x:drawing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:drawing")]
pub struct Drawing {
    /// Relationship id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the LegacyDrawing Class.
/// When the object is serialized out as xml, it's qualified name is x:legacyDrawing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:legacyDrawing")]
pub struct LegacyDrawing {
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Legacy Drawing Reference in  Header Footer.
/// When the object is serialized out as xml, it's qualified name is x:legacyDrawingHF.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:legacyDrawingHF")]
pub struct LegacyDrawingHeaderFooter {
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the LegacyDrawingType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct LegacyDrawingType {
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the DrawingHeaderFooter Class.
/// When the object is serialized out as xml, it's qualified name is x:drawingHF.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:drawingHF")]
pub struct DrawingHeaderFooter {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
    /// lho
    /// Represents the following attribute in the schema: :lho
    #[xml(attr = "lho")]
    pub lho: Option<i32>,
    /// lhe
    /// Represents the following attribute in the schema: :lhe
    #[xml(attr = "lhe")]
    pub lhe: Option<i32>,
    /// lhf
    /// Represents the following attribute in the schema: :lhf
    #[xml(attr = "lhf")]
    pub lhf: Option<i32>,
    /// cho
    /// Represents the following attribute in the schema: :cho
    #[xml(attr = "cho")]
    pub cho: Option<i32>,
    /// che
    /// Represents the following attribute in the schema: :che
    #[xml(attr = "che")]
    pub che: Option<i32>,
    /// chf
    /// Represents the following attribute in the schema: :chf
    #[xml(attr = "chf")]
    pub chf: Option<i32>,
    /// rho
    /// Represents the following attribute in the schema: :rho
    #[xml(attr = "rho")]
    pub rho: Option<i32>,
    /// rhe
    /// Represents the following attribute in the schema: :rhe
    #[xml(attr = "rhe")]
    pub rhe: Option<i32>,
    /// rhf
    /// Represents the following attribute in the schema: :rhf
    #[xml(attr = "rhf")]
    pub rhf: Option<i32>,
    /// lfo
    /// Represents the following attribute in the schema: :lfo
    #[xml(attr = "lfo")]
    pub lfo: Option<i32>,
    /// lfe
    /// Represents the following attribute in the schema: :lfe
    #[xml(attr = "lfe")]
    pub lfe: Option<i32>,
    /// lff
    /// Represents the following attribute in the schema: :lff
    #[xml(attr = "lff")]
    pub lff: Option<i32>,
    /// cfo
    /// Represents the following attribute in the schema: :cfo
    #[xml(attr = "cfo")]
    pub cfo: Option<i32>,
    /// cfe
    /// Represents the following attribute in the schema: :cfe
    #[xml(attr = "cfe")]
    pub cfe: Option<i32>,
    /// cff
    /// Represents the following attribute in the schema: :cff
    #[xml(attr = "cff")]
    pub cff: Option<i32>,
    /// rfo
    /// Represents the following attribute in the schema: :rfo
    #[xml(attr = "rfo")]
    pub rfo: Option<i32>,
    /// rfe
    /// Represents the following attribute in the schema: :rfe
    #[xml(attr = "rfe")]
    pub rfe: Option<i32>,
    /// rff
    /// Represents the following attribute in the schema: :rff
    #[xml(attr = "rff")]
    pub rff: Option<i32>,
}
/// Defines the Picture Class.
/// When the object is serialized out as xml, it's qualified name is x:picture.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:picture")]
pub struct Picture {
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the WebPublishItems Class.
/// When the object is serialized out as xml, it's qualified name is x:webPublishItems.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:webPublishItems")]
pub struct WebPublishItems {
    /// Web Publishing Items Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:webPublishItem")]
    pub x_web_publish_item: Vec<WebPublishItem>,
}
/// Color Scale.
/// When the object is serialized out as xml, it's qualified name is x:colorScale.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:colorScale")]
pub struct ColorScale {
    /// _
    #[xml(child = "x:cfvo")]
    pub x_cfvo: Vec<ConditionalFormatValueObject>,
    /// _
    #[xml(child = "x:color")]
    pub x_color: Vec<Color>,
}
/// Data Bar.
/// When the object is serialized out as xml, it's qualified name is x:dataBar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dataBar")]
pub struct DataBar {
    /// Minimum Length
    /// Represents the following attribute in the schema: :minLength
    #[xml(attr = "minLength")]
    pub min_length: Option<i32>,
    /// Maximum Length
    /// Represents the following attribute in the schema: :maxLength
    #[xml(attr = "maxLength")]
    pub max_length: Option<i32>,
    /// Show Values
    /// Represents the following attribute in the schema: :showValue
    #[xml(attr = "showValue")]
    pub show_value: Option<bool>,
    /// _
    #[xml(child = "x:cfvo")]
    pub x_cfvo: Vec<ConditionalFormatValueObject>,
    /// _
    #[xml(child = "x:color")]
    pub x_color: Color,
}
/// Icon Set.
/// When the object is serialized out as xml, it's qualified name is x:iconSet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:iconSet")]
pub struct IconSet {
    /// Icon Set
    /// Represents the following attribute in the schema: :iconSet
    #[xml(attr = "iconSet")]
    pub icon_set_value: Option<IconSetValues>,
    /// Show Value
    /// Represents the following attribute in the schema: :showValue
    #[xml(attr = "showValue")]
    pub show_value: Option<bool>,
    /// Percent
    /// Represents the following attribute in the schema: :percent
    #[xml(attr = "percent")]
    pub percent: Option<bool>,
    /// Reverse Icons
    /// Represents the following attribute in the schema: :reverse
    #[xml(attr = "reverse")]
    pub reverse: Option<bool>,
    /// _
    #[xml(child = "x:cfvo")]
    pub x_cfvo: Vec<ConditionalFormatValueObject>,
}
/// Defines the ConditionalFormattingRuleExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct ConditionalFormattingRuleExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<ConditionalFormattingRuleExtension>,
}
/// Data Consolidation References.
/// When the object is serialized out as xml, it's qualified name is x:dataRefs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dataRefs")]
pub struct DataReferences {
    /// Data Consolidation Reference Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:dataRef")]
    pub x_data_ref: Vec<DataReference>,
}
/// Sheet Properties.
/// When the object is serialized out as xml, it's qualified name is x:sheetPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetPr")]
pub struct SheetProperties {
    /// Synch Horizontal
    /// Represents the following attribute in the schema: :syncHorizontal
    #[xml(attr = "syncHorizontal")]
    pub sync_horizontal: Option<bool>,
    /// Synch Vertical
    /// Represents the following attribute in the schema: :syncVertical
    #[xml(attr = "syncVertical")]
    pub sync_vertical: Option<bool>,
    /// Synch Reference
    /// Represents the following attribute in the schema: :syncRef
    #[xml(attr = "syncRef")]
    pub sync_reference: Option<String>,
    /// Transition Formula Evaluation
    /// Represents the following attribute in the schema: :transitionEvaluation
    #[xml(attr = "transitionEvaluation")]
    pub transition_evaluation: Option<bool>,
    /// Transition Formula Entry
    /// Represents the following attribute in the schema: :transitionEntry
    #[xml(attr = "transitionEntry")]
    pub transition_entry: Option<bool>,
    /// Published
    /// Represents the following attribute in the schema: :published
    #[xml(attr = "published")]
    pub published: Option<bool>,
    /// Code Name
    /// Represents the following attribute in the schema: :codeName
    #[xml(attr = "codeName")]
    pub code_name: Option<String>,
    /// Filter Mode
    /// Represents the following attribute in the schema: :filterMode
    #[xml(attr = "filterMode")]
    pub filter_mode: Option<bool>,
    /// Enable Conditional Formatting Calculations
    /// Represents the following attribute in the schema: :enableFormatConditionsCalculation
    #[xml(attr = "enableFormatConditionsCalculation")]
    pub enable_format_conditions_calculation: Option<bool>,
    ///Sheet Tab Color
    #[xml(child = "x:tabColor")]
    pub tab_color: Option<TabColor>,
    ///Outline Properties
    #[xml(child = "x:outlinePr")]
    pub outline_properties: Option<OutlineProperties>,
    ///Page Setup Properties
    #[xml(child = "x:pageSetUpPr")]
    pub page_setup_properties: Option<PageSetupProperties>,
}
/// Dialog Sheet Views.
/// When the object is serialized out as xml, it's qualified name is x:sheetViews.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetViews")]
pub struct SheetViews {
    /// _
    #[xml(child = "x:sheetView")]
    pub x_sheet_view: Vec<SheetView>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<ExtensionList>,
}
/// Dialog Sheet Format Properties.
/// When the object is serialized out as xml, it's qualified name is x:sheetFormatPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetFormatPr")]
pub struct SheetFormatProperties {
    /// Base Column Width
    /// Represents the following attribute in the schema: :baseColWidth
    #[xml(attr = "baseColWidth")]
    pub base_column_width: Option<i32>,
    /// Default Column Width
    /// Represents the following attribute in the schema: :defaultColWidth
    #[xml(attr = "defaultColWidth")]
    pub default_column_width: Option<f64>,
    /// Default Row Height
    /// Represents the following attribute in the schema: :defaultRowHeight
    #[xml(attr = "defaultRowHeight")]
    pub default_row_height: f64,
    /// Custom Height
    /// Represents the following attribute in the schema: :customHeight
    #[xml(attr = "customHeight")]
    pub custom_height: Option<bool>,
    /// Hidden By Default
    /// Represents the following attribute in the schema: :zeroHeight
    #[xml(attr = "zeroHeight")]
    pub zero_height: Option<bool>,
    /// Thick Top Border
    /// Represents the following attribute in the schema: :thickTop
    #[xml(attr = "thickTop")]
    pub thick_top: Option<bool>,
    /// Thick Bottom Border
    /// Represents the following attribute in the schema: :thickBottom
    #[xml(attr = "thickBottom")]
    pub thick_bottom: Option<bool>,
    /// Maximum Outline Row
    /// Represents the following attribute in the schema: :outlineLevelRow
    #[xml(attr = "outlineLevelRow")]
    pub outline_level_row: Option<u8>,
    /// Column Outline Level
    /// Represents the following attribute in the schema: :outlineLevelCol
    #[xml(attr = "outlineLevelCol")]
    pub outline_level_column: Option<u8>,
    /// dyDescent
    /// Represents the following attribute in the schema: x14ac:dyDescent
    #[xml(attr = "x14ac:dyDescent")]
    pub dy_descent: Option<f64>,
}
/// Sheet Protection.
/// When the object is serialized out as xml, it's qualified name is x:sheetProtection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetProtection")]
pub struct SheetProtection {
    /// Password
    /// Represents the following attribute in the schema: :password
    #[xml(attr = "password")]
    pub password: Option<String>,
    /// Cryptographic Algorithm Name
    /// Represents the following attribute in the schema: :algorithmName
    #[xml(attr = "algorithmName")]
    pub algorithm_name: Option<String>,
    /// Password Hash Value
    /// Represents the following attribute in the schema: :hashValue
    #[xml(attr = "hashValue")]
    pub hash_value: Option<String>,
    /// Salt Value for Password Verifier
    /// Represents the following attribute in the schema: :saltValue
    #[xml(attr = "saltValue")]
    pub salt_value: Option<String>,
    /// Iterations to Run Hashing Algorithm
    /// Represents the following attribute in the schema: :spinCount
    #[xml(attr = "spinCount")]
    pub spin_count: Option<i32>,
    /// Sheet Locked
    /// Represents the following attribute in the schema: :sheet
    #[xml(attr = "sheet")]
    pub sheet: Option<bool>,
    /// Objects Locked
    /// Represents the following attribute in the schema: :objects
    #[xml(attr = "objects")]
    pub objects: Option<bool>,
    /// Scenarios Locked
    /// Represents the following attribute in the schema: :scenarios
    #[xml(attr = "scenarios")]
    pub scenarios: Option<bool>,
    /// Format Cells Locked
    /// Represents the following attribute in the schema: :formatCells
    #[xml(attr = "formatCells")]
    pub format_cells: Option<bool>,
    /// Format Columns Locked
    /// Represents the following attribute in the schema: :formatColumns
    #[xml(attr = "formatColumns")]
    pub format_columns: Option<bool>,
    /// Format Rows Locked
    /// Represents the following attribute in the schema: :formatRows
    #[xml(attr = "formatRows")]
    pub format_rows: Option<bool>,
    /// Insert Columns Locked
    /// Represents the following attribute in the schema: :insertColumns
    #[xml(attr = "insertColumns")]
    pub insert_columns: Option<bool>,
    /// Insert Rows Locked
    /// Represents the following attribute in the schema: :insertRows
    #[xml(attr = "insertRows")]
    pub insert_rows: Option<bool>,
    /// Insert Hyperlinks Locked
    /// Represents the following attribute in the schema: :insertHyperlinks
    #[xml(attr = "insertHyperlinks")]
    pub insert_hyperlinks: Option<bool>,
    /// Delete Columns Locked
    /// Represents the following attribute in the schema: :deleteColumns
    #[xml(attr = "deleteColumns")]
    pub delete_columns: Option<bool>,
    /// Delete Rows Locked
    /// Represents the following attribute in the schema: :deleteRows
    #[xml(attr = "deleteRows")]
    pub delete_rows: Option<bool>,
    /// Select Locked Cells Locked
    /// Represents the following attribute in the schema: :selectLockedCells
    #[xml(attr = "selectLockedCells")]
    pub select_locked_cells: Option<bool>,
    /// Sort Locked
    /// Represents the following attribute in the schema: :sort
    #[xml(attr = "sort")]
    pub sort: Option<bool>,
    /// AutoFilter Locked
    /// Represents the following attribute in the schema: :autoFilter
    #[xml(attr = "autoFilter")]
    pub auto_filter: Option<bool>,
    /// Pivot Tables Locked
    /// Represents the following attribute in the schema: :pivotTables
    #[xml(attr = "pivotTables")]
    pub pivot_tables: Option<bool>,
    /// Select Unlocked Cells Locked
    /// Represents the following attribute in the schema: :selectUnlockedCells
    #[xml(attr = "selectUnlockedCells")]
    pub select_unlocked_cells: Option<bool>,
}
/// Custom Sheet Views.
/// When the object is serialized out as xml, it's qualified name is x:customSheetViews.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:customSheetViews")]
pub struct CustomSheetViews {
    /// _
    #[xml(child = "x:customSheetView")]
    pub x_custom_sheet_view: Vec<CustomSheetView>,
}
/// Defines the OleObjects Class.
/// When the object is serialized out as xml, it's qualified name is x:oleObjects.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:oleObjects")]
pub struct OleObjects {
    /// _
    #[xml(child = "x:oleObject")]
    pub x_ole_object: Vec<OleObject>,
}
/// Defines the Controls Class.
/// When the object is serialized out as xml, it's qualified name is x:controls.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:controls")]
pub struct Controls {
    /// _
    #[xml(child = "x:control")]
    pub x_control: Vec<Control>,
}
/// Macro Sheet Dimensions.
/// When the object is serialized out as xml, it's qualified name is x:dimension.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dimension")]
pub struct SheetDimension {
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: String,
}
/// Column Information.
/// When the object is serialized out as xml, it's qualified name is x:cols.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cols")]
pub struct Columns {
    /// _
    #[xml(child = "x:col")]
    pub x_col: Vec<Column>,
}
/// Sheet Data.
/// When the object is serialized out as xml, it's qualified name is x:sheetData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetData")]
pub struct SheetData {
    /// _
    #[xml(child = "x:row")]
    pub x_row: Vec<Row>,
}
/// Data Consolidation.
/// When the object is serialized out as xml, it's qualified name is x:dataConsolidate.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dataConsolidate")]
pub struct DataConsolidate {
    /// Function Index
    /// Represents the following attribute in the schema: :function
    #[xml(attr = "function")]
    pub function: Option<DataConsolidateFunctionValues>,
    /// Use Left Column Labels
    /// Represents the following attribute in the schema: :leftLabels
    #[xml(attr = "leftLabels")]
    pub left_labels: Option<bool>,
    /// startLabels
    /// Represents the following attribute in the schema: :startLabels
    #[xml(attr = "startLabels")]
    pub start_labels: Option<bool>,
    /// Labels In Top Row
    /// Represents the following attribute in the schema: :topLabels
    #[xml(attr = "topLabels")]
    pub top_labels: Option<bool>,
    /// Link
    /// Represents the following attribute in the schema: :link
    #[xml(attr = "link")]
    pub link: Option<bool>,
    ///Data Consolidation References
    #[xml(child = "x:dataRefs")]
    pub data_references: Option<DataReferences>,
}
/// Conditional Formatting.
/// When the object is serialized out as xml, it's qualified name is x:conditionalFormatting.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:conditionalFormatting")]
pub struct ConditionalFormatting {
    /// PivotTable Conditional Formatting
    /// Represents the following attribute in the schema: :pivot
    #[xml(attr = "pivot")]
    pub pivot: Option<bool>,
    /// Sequence of References
    /// Represents the following attribute in the schema: :sqref
    #[xml(attr = "sqref")]
    pub sequence_of_references: Option<String>,
    /// _
    #[xml(child = "x:cfRule")]
    pub x_cf_rule: Vec<ConditionalFormattingRule>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<ExtensionList>,
}
/// Custom Properties.
/// When the object is serialized out as xml, it's qualified name is x:customProperties.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:customProperties")]
pub struct CustomProperties {
    /// _
    #[xml(child = "x:customPr")]
    pub x_custom_pr: Vec<CustomProperty>,
}
/// OLAP Member Properties.
/// When the object is serialized out as xml, it's qualified name is x:mps.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:mps")]
pub struct MemberProperties {
    /// OLAP Member Properties Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:mp")]
    pub x_mp: Vec<MemberProperty>,
}
/// Members.
/// When the object is serialized out as xml, it's qualified name is x:members.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:members")]
pub struct Members {
    /// Item Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// Hierarchy Level
    /// Represents the following attribute in the schema: :level
    #[xml(attr = "level")]
    pub level: Option<i32>,
    /// _
    #[xml(child = "x:member")]
    pub x_member: Vec<Member>,
}
/// Future Feature Data Storage Area.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct PivotHierarchyExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<PivotHierarchyExtension>,
}
/// Field Items.
/// When the object is serialized out as xml, it's qualified name is x:items.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:items")]
pub struct Items {
    /// Field Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:item")]
    pub x_item: Vec<Item>,
}
/// AutoSort Scope.
/// When the object is serialized out as xml, it's qualified name is x:autoSortScope.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:autoSortScope")]
pub struct AutoSortScope {
    ///Auto Sort Scope
    #[xml(child = "x:pivotArea")]
    pub pivot_area: PivotArea,
}
/// Future Feature Data Storage Area.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct PivotFieldExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<PivotFieldExtension>,
}
/// Defines the WorksheetSource Class.
/// When the object is serialized out as xml, it's qualified name is x:worksheetSource.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:worksheetSource")]
pub struct WorksheetSource {
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: Option<String>,
    /// Named Range
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Sheet Name
    /// Represents the following attribute in the schema: :sheet
    #[xml(attr = "sheet")]
    pub sheet: Option<String>,
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
}
/// Defines the Consolidation Class.
/// When the object is serialized out as xml, it's qualified name is x:consolidation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:consolidation")]
pub struct Consolidation {
    /// Auto Page
    /// Represents the following attribute in the schema: :autoPage
    #[xml(attr = "autoPage")]
    pub auto_page: Option<bool>,
    ///Page Item Values
    #[xml(child = "x:pages")]
    pub pages: Option<Pages>,
    ///Range Sets
    #[xml(child = "x:rangeSets")]
    pub range_sets: RangeSets,
}
/// Defines the CacheSourceExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct CacheSourceExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<CacheSourceExtension>,
}
/// Defines the CommentProperties Class.
/// When the object is serialized out as xml, it's qualified name is x:commentPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:commentPr")]
pub struct CommentProperties {
    /// locked
    /// Represents the following attribute in the schema: :locked
    #[xml(attr = "locked")]
    pub locked: Option<bool>,
    /// defaultSize
    /// Represents the following attribute in the schema: :defaultSize
    #[xml(attr = "defaultSize")]
    pub default_size: Option<bool>,
    /// print
    /// Represents the following attribute in the schema: :print
    #[xml(attr = "print")]
    pub print: Option<bool>,
    /// disabled
    /// Represents the following attribute in the schema: :disabled
    #[xml(attr = "disabled")]
    pub disabled: Option<bool>,
    /// uiObject
    /// Represents the following attribute in the schema: :uiObject
    #[xml(attr = "uiObject")]
    pub ui_object: Option<bool>,
    /// autoFill
    /// Represents the following attribute in the schema: :autoFill
    #[xml(attr = "autoFill")]
    pub auto_fill: Option<bool>,
    /// autoLine
    /// Represents the following attribute in the schema: :autoLine
    #[xml(attr = "autoLine")]
    pub auto_line: Option<bool>,
    /// altText
    /// Represents the following attribute in the schema: :altText
    #[xml(attr = "altText")]
    pub alt_text: Option<String>,
    /// textHAlign
    /// Represents the following attribute in the schema: :textHAlign
    #[xml(attr = "textHAlign")]
    pub text_h_align: Option<TextHorizontalAlignmentValues>,
    /// textVAlign
    /// Represents the following attribute in the schema: :textVAlign
    #[xml(attr = "textVAlign")]
    pub text_v_align: Option<TextVerticalAlignmentValues>,
    /// lockText
    /// Represents the following attribute in the schema: :lockText
    #[xml(attr = "lockText")]
    pub lock_text: Option<bool>,
    /// justLastX
    /// Represents the following attribute in the schema: :justLastX
    #[xml(attr = "justLastX")]
    pub just_last_x: Option<bool>,
    /// autoScale
    /// Represents the following attribute in the schema: :autoScale
    #[xml(attr = "autoScale")]
    pub auto_scale: Option<bool>,
    /// rowHidden
    /// Represents the following attribute in the schema: :rowHidden
    #[xml(attr = "rowHidden")]
    pub row_hidden: Option<bool>,
    /// colHidden
    /// Represents the following attribute in the schema: :colHidden
    #[xml(attr = "colHidden")]
    pub col_hidden: Option<bool>,
    /// _
    #[xml(child = "x:anchor")]
    pub object_anchor: ObjectAnchor,
}
/// Defines the SortCondition Class.
/// When the object is serialized out as xml, it's qualified name is x:sortCondition.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sortCondition")]
pub struct SortCondition {
    /// Descending
    /// Represents the following attribute in the schema: :descending
    #[xml(attr = "descending")]
    pub descending: Option<bool>,
    /// Sort By
    /// Represents the following attribute in the schema: :sortBy
    #[xml(attr = "sortBy")]
    pub sort_by: Option<SortByValues>,
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: String,
    /// Custom List
    /// Represents the following attribute in the schema: :customList
    #[xml(attr = "customList")]
    pub custom_list: Option<String>,
    /// Format Id
    /// Represents the following attribute in the schema: :dxfId
    #[xml(attr = "dxfId")]
    pub format_id: Option<i32>,
    /// Icon Set
    /// Represents the following attribute in the schema: :iconSet
    #[xml(attr = "iconSet")]
    pub icon_set: Option<IconSetValues>,
    /// Icon Id
    /// Represents the following attribute in the schema: :iconId
    #[xml(attr = "iconId")]
    pub icon_id: Option<i32>,
}
/// Filter.
/// When the object is serialized out as xml, it's qualified name is x:filter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:filter")]
pub struct Filter {
    /// Filter Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Date Grouping.
/// When the object is serialized out as xml, it's qualified name is x:dateGroupItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dateGroupItem")]
pub struct DateGroupItem {
    /// Year
    /// Represents the following attribute in the schema: :year
    #[xml(attr = "year")]
    pub year: i16,
    /// Month
    /// Represents the following attribute in the schema: :month
    #[xml(attr = "month")]
    pub month: Option<i16>,
    /// Day
    /// Represents the following attribute in the schema: :day
    #[xml(attr = "day")]
    pub day: Option<i16>,
    /// Hour
    /// Represents the following attribute in the schema: :hour
    #[xml(attr = "hour")]
    pub hour: Option<i16>,
    /// Minute
    /// Represents the following attribute in the schema: :minute
    #[xml(attr = "minute")]
    pub minute: Option<i16>,
    /// Second
    /// Represents the following attribute in the schema: :second
    #[xml(attr = "second")]
    pub second: Option<i16>,
    /// Date Time Grouping
    /// Represents the following attribute in the schema: :dateTimeGrouping
    #[xml(attr = "dateTimeGrouping")]
    pub date_time_grouping: DateTimeGroupingValues,
}
/// Filter Criteria.
/// When the object is serialized out as xml, it's qualified name is x:filters.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:filters")]
pub struct Filters {
    /// Filter by Blank
    /// Represents the following attribute in the schema: :blank
    #[xml(attr = "blank")]
    pub blank: Option<bool>,
    /// Calendar Type
    /// Represents the following attribute in the schema: :calendarType
    #[xml(attr = "calendarType")]
    pub calendar_type: Option<CalendarValues>,
    #[xml(child = "x14:filter", child = "x:filter", child = "x:dateGroupItem")]
    pub children: Vec<FiltersChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FiltersChildChoice {
    #[xml(tag = "x14:filter")]
    X14Filter(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Filter,
    ),
    #[xml(tag = "x:filter")]
    XFilter(Filter),
    #[xml(tag = "x:dateGroupItem")]
    XDateGroupItem(DateGroupItem),
}
/// Top 10.
/// When the object is serialized out as xml, it's qualified name is x:top10.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:top10")]
pub struct Top10 {
    /// Top
    /// Represents the following attribute in the schema: :top
    #[xml(attr = "top")]
    pub top: Option<bool>,
    /// Filter by Percent
    /// Represents the following attribute in the schema: :percent
    #[xml(attr = "percent")]
    pub percent: Option<bool>,
    /// Top or Bottom Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
    /// Filter Value
    /// Represents the following attribute in the schema: :filterVal
    #[xml(attr = "filterVal")]
    pub filter_value: Option<f64>,
}
/// Custom Filters.
/// When the object is serialized out as xml, it's qualified name is x:customFilters.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:customFilters")]
pub struct CustomFilters {
    /// And
    /// Represents the following attribute in the schema: :and
    #[xml(attr = "and")]
    pub and: Option<bool>,
    /// _
    #[xml(child = "x:customFilter")]
    pub x_custom_filter: Vec<CustomFilter>,
}
/// Dynamic Filter.
/// When the object is serialized out as xml, it's qualified name is x:dynamicFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dynamicFilter")]
pub struct DynamicFilter {
    /// Dynamic filter type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: DynamicFilterValues,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<f64>,
    /// Max Value
    /// Represents the following attribute in the schema: :maxVal
    #[xml(attr = "maxVal")]
    pub max_val: Option<f64>,
    /// valIso
    /// Represents the following attribute in the schema: :valIso
    #[xml(attr = "valIso")]
    pub val_iso: Option<String>,
    /// maxValIso
    /// Represents the following attribute in the schema: :maxValIso
    #[xml(attr = "maxValIso")]
    pub max_val_iso: Option<String>,
}
/// Color Filter Criteria.
/// When the object is serialized out as xml, it's qualified name is x:colorFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:colorFilter")]
pub struct ColorFilter {
    /// Differential Format Record Id
    /// Represents the following attribute in the schema: :dxfId
    #[xml(attr = "dxfId")]
    pub format_id: i32,
    /// Filter By Cell Color
    /// Represents the following attribute in the schema: :cellColor
    #[xml(attr = "cellColor")]
    pub cell_color: Option<bool>,
}
/// Icon Filter.
/// When the object is serialized out as xml, it's qualified name is x:iconFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:iconFilter")]
pub struct IconFilter {
    /// Icon Set
    /// Represents the following attribute in the schema: :iconSet
    #[xml(attr = "iconSet")]
    pub icon_set: IconSetValues,
    /// Icon Id
    /// Represents the following attribute in the schema: :iconId
    #[xml(attr = "iconId")]
    pub icon_id: Option<i32>,
}
/// Defines the SlicerCacheDefinitionExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct SlicerCacheDefinitionExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "x15:slicerCachePivotTables",
        child = "x15:tableSlicerCache",
        child = "x15:slicerCacheHideItemsWithNoData",
    )]
    pub children: Vec<SlicerCacheDefinitionExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SlicerCacheDefinitionExtensionChildChoice {
    #[xml(tag = "x15:slicerCachePivotTables")]
    X15SlicerCachePivotTables(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::SlicerCachePivotTables,
    ),
    #[xml(tag = "x15:tableSlicerCache")]
    X15TableSlicerCache(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TableSlicerCache,
    ),
    #[xml(tag = "x15:slicerCacheHideItemsWithNoData")]
    X15SlicerCacheHideItemsWithNoData(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::SlicerCacheHideItemsWithNoData,
    ),
}
/// Defines the PivotFilterExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct PivotFilterExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "x15:pivotFilter", child = "x15:movingPeriodState")]
    pub children: Vec<PivotFilterExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PivotFilterExtensionChildChoice {
    #[xml(tag = "x15:pivotFilter")]
    X15PivotFilter(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::PivotFilter,
    ),
    #[xml(tag = "x15:movingPeriodState")]
    X15MovingPeriodState(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::MovingPeriodState,
    ),
}
/// Defines the QueryTableExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct QueryTableExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "x15:queryTable")]
    pub children: Vec<QueryTableExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum QueryTableExtensionChildChoice {
    #[xml(tag = "x15:queryTable")]
    X15QueryTable(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::QueryTable,
    ),
}
/// Defines the DatabaseProperties Class.
/// When the object is serialized out as xml, it's qualified name is x:dbPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dbPr")]
pub struct DatabaseProperties {
    /// Connection String
    /// Represents the following attribute in the schema: :connection
    #[xml(attr = "connection")]
    pub connection: String,
    /// Command Text
    /// Represents the following attribute in the schema: :command
    #[xml(attr = "command")]
    pub command: Option<String>,
    /// Command Text
    /// Represents the following attribute in the schema: :serverCommand
    #[xml(attr = "serverCommand")]
    pub server_command: Option<String>,
    /// OLE DB Command Type
    /// Represents the following attribute in the schema: :commandType
    #[xml(attr = "commandType")]
    pub command_type: Option<i32>,
}
/// Defines the OlapProperties Class.
/// When the object is serialized out as xml, it's qualified name is x:olapPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:olapPr")]
pub struct OlapProperties {
    /// Local Cube
    /// Represents the following attribute in the schema: :local
    #[xml(attr = "local")]
    pub local: Option<bool>,
    /// Local Cube Connection
    /// Represents the following attribute in the schema: :localConnection
    #[xml(attr = "localConnection")]
    pub local_connection: Option<String>,
    /// Local Refresh
    /// Represents the following attribute in the schema: :localRefresh
    #[xml(attr = "localRefresh")]
    pub local_refresh: Option<bool>,
    /// Send Locale to OLAP
    /// Represents the following attribute in the schema: :sendLocale
    #[xml(attr = "sendLocale")]
    pub send_locale: Option<bool>,
    /// Drill Through Count
    /// Represents the following attribute in the schema: :rowDrillCount
    #[xml(attr = "rowDrillCount")]
    pub row_drill_count: Option<i32>,
    /// OLAP Fill Formatting
    /// Represents the following attribute in the schema: :serverFill
    #[xml(attr = "serverFill")]
    pub server_fill: Option<bool>,
    /// OLAP Number Format
    /// Represents the following attribute in the schema: :serverNumberFormat
    #[xml(attr = "serverNumberFormat")]
    pub server_number_format: Option<bool>,
    /// OLAP Server Font
    /// Represents the following attribute in the schema: :serverFont
    #[xml(attr = "serverFont")]
    pub server_font: Option<bool>,
    /// OLAP Font Formatting
    /// Represents the following attribute in the schema: :serverFontColor
    #[xml(attr = "serverFontColor")]
    pub server_font_color: Option<bool>,
}
/// Defines the WebQueryProperties Class.
/// When the object is serialized out as xml, it's qualified name is x:webPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:webPr")]
pub struct WebQueryProperties {
    /// XML Source
    /// Represents the following attribute in the schema: :xml
    #[xml(attr = "xml")]
    pub xml_source: Option<bool>,
    /// Import XML Source Data
    /// Represents the following attribute in the schema: :sourceData
    #[xml(attr = "sourceData")]
    pub source_data: Option<bool>,
    /// Parse PRE
    /// Represents the following attribute in the schema: :parsePre
    #[xml(attr = "parsePre")]
    pub parse_pre_tag: Option<bool>,
    /// Consecutive Delimiters
    /// Represents the following attribute in the schema: :consecutive
    #[xml(attr = "consecutive")]
    pub consecutive: Option<bool>,
    /// Use First Row
    /// Represents the following attribute in the schema: :firstRow
    #[xml(attr = "firstRow")]
    pub first_row: Option<bool>,
    /// Created in Excel 97
    /// Represents the following attribute in the schema: :xl97
    #[xml(attr = "xl97")]
    pub created_in_excel97: Option<bool>,
    /// Dates as Text
    /// Represents the following attribute in the schema: :textDates
    #[xml(attr = "textDates")]
    pub text_dates: Option<bool>,
    /// Refreshed in Excel 2000
    /// Represents the following attribute in the schema: :xl2000
    #[xml(attr = "xl2000")]
    pub refreshed_in_excel2000: Option<bool>,
    /// URL
    /// Represents the following attribute in the schema: :url
    #[xml(attr = "url")]
    pub url: Option<String>,
    /// Web Post
    /// Represents the following attribute in the schema: :post
    #[xml(attr = "post")]
    pub post: Option<String>,
    /// HTML Tables Only
    /// Represents the following attribute in the schema: :htmlTables
    #[xml(attr = "htmlTables")]
    pub html_tables: Option<bool>,
    /// HTML Formatting Handling
    /// Represents the following attribute in the schema: :htmlFormat
    #[xml(attr = "htmlFormat")]
    pub html_format: Option<HtmlFormattingValues>,
    /// Edit Query URL
    /// Represents the following attribute in the schema: :editPage
    #[xml(attr = "editPage")]
    pub edit_page: Option<String>,
    ///Tables
    #[xml(child = "x:tables")]
    pub tables: Option<Tables>,
}
/// Defines the TextProperties Class.
/// When the object is serialized out as xml, it's qualified name is x:textPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:textPr")]
pub struct TextProperties {
    /// prompt
    /// Represents the following attribute in the schema: :prompt
    #[xml(attr = "prompt")]
    pub prompt: Option<bool>,
    /// fileType
    /// Represents the following attribute in the schema: :fileType
    #[xml(attr = "fileType")]
    pub file_type: Option<FileTypeValues>,
    /// codePage
    /// Represents the following attribute in the schema: :codePage
    #[xml(attr = "codePage")]
    pub code_page: Option<i32>,
    /// characterSet
    /// Represents the following attribute in the schema: :characterSet
    #[xml(attr = "characterSet")]
    pub text_character_set: Option<String>,
    /// firstRow
    /// Represents the following attribute in the schema: :firstRow
    #[xml(attr = "firstRow")]
    pub first_row: Option<i32>,
    /// sourceFile
    /// Represents the following attribute in the schema: :sourceFile
    #[xml(attr = "sourceFile")]
    pub source_file: Option<String>,
    /// delimited
    /// Represents the following attribute in the schema: :delimited
    #[xml(attr = "delimited")]
    pub delimited: Option<bool>,
    /// decimal
    /// Represents the following attribute in the schema: :decimal
    #[xml(attr = "decimal")]
    pub decimal: Option<String>,
    /// thousands
    /// Represents the following attribute in the schema: :thousands
    #[xml(attr = "thousands")]
    pub thousands: Option<String>,
    /// tab
    /// Represents the following attribute in the schema: :tab
    #[xml(attr = "tab")]
    pub tab_as_delimiter: Option<bool>,
    /// space
    /// Represents the following attribute in the schema: :space
    #[xml(attr = "space")]
    pub space: Option<bool>,
    /// comma
    /// Represents the following attribute in the schema: :comma
    #[xml(attr = "comma")]
    pub comma: Option<bool>,
    /// semicolon
    /// Represents the following attribute in the schema: :semicolon
    #[xml(attr = "semicolon")]
    pub semicolon: Option<bool>,
    /// consecutive
    /// Represents the following attribute in the schema: :consecutive
    #[xml(attr = "consecutive")]
    pub consecutive: Option<bool>,
    /// qualifier
    /// Represents the following attribute in the schema: :qualifier
    #[xml(attr = "qualifier")]
    pub qualifier: Option<QualifierValues>,
    /// delimiter
    /// Represents the following attribute in the schema: :delimiter
    #[xml(attr = "delimiter")]
    pub delimiter: Option<String>,
    /// _
    #[xml(child = "x:textFields")]
    pub text_fields: Option<TextFields>,
}
/// Defines the Parameters Class.
/// When the object is serialized out as xml, it's qualified name is x:parameters.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:parameters")]
pub struct Parameters {
    /// Parameter Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:parameter")]
    pub x_parameter: Vec<Parameter>,
}
/// Defines the ConnectionExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct ConnectionExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<ConnectionExtension>,
}
/// Defines the ConnectionExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct ConnectionExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "x14:connection", child = "x15:connection")]
    pub children: Vec<ConnectionExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ConnectionExtensionChildChoice {
    #[xml(tag = "x14:connection")]
    X14Connection(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Connection,
    ),
    #[xml(tag = "x15:connection")]
    X15Connection(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::Connection,
    ),
}
/// Defines the TextFields Class.
/// When the object is serialized out as xml, it's qualified name is x:textFields.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:textFields")]
pub struct TextFields {
    /// Count of Fields
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:textField")]
    pub x_text_field: Vec<TextField>,
}
/// Defines the SharedItems Class.
/// When the object is serialized out as xml, it's qualified name is x:sharedItems.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sharedItems")]
pub struct SharedItems {
    /// Contains Semi Mixed Data Types
    /// Represents the following attribute in the schema: :containsSemiMixedTypes
    #[xml(attr = "containsSemiMixedTypes")]
    pub contains_semi_mixed_types: Option<bool>,
    /// Contains Non Date
    /// Represents the following attribute in the schema: :containsNonDate
    #[xml(attr = "containsNonDate")]
    pub contains_non_date: Option<bool>,
    /// Contains Date
    /// Represents the following attribute in the schema: :containsDate
    #[xml(attr = "containsDate")]
    pub contains_date: Option<bool>,
    /// Contains String
    /// Represents the following attribute in the schema: :containsString
    #[xml(attr = "containsString")]
    pub contains_string: Option<bool>,
    /// Contains Blank
    /// Represents the following attribute in the schema: :containsBlank
    #[xml(attr = "containsBlank")]
    pub contains_blank: Option<bool>,
    /// Contains Mixed Data Types
    /// Represents the following attribute in the schema: :containsMixedTypes
    #[xml(attr = "containsMixedTypes")]
    pub contains_mixed_types: Option<bool>,
    /// Contains Numbers
    /// Represents the following attribute in the schema: :containsNumber
    #[xml(attr = "containsNumber")]
    pub contains_number: Option<bool>,
    /// Contains Integer
    /// Represents the following attribute in the schema: :containsInteger
    #[xml(attr = "containsInteger")]
    pub contains_integer: Option<bool>,
    /// Minimum Numeric Value
    /// Represents the following attribute in the schema: :minValue
    #[xml(attr = "minValue")]
    pub min_value: Option<f64>,
    /// Maximum Numeric Value
    /// Represents the following attribute in the schema: :maxValue
    #[xml(attr = "maxValue")]
    pub max_value: Option<f64>,
    /// Minimum Date Time
    /// Represents the following attribute in the schema: :minDate
    #[xml(attr = "minDate")]
    pub min_date: Option<String>,
    /// Maximum Date Time Value
    /// Represents the following attribute in the schema: :maxDate
    #[xml(attr = "maxDate")]
    pub max_date: Option<String>,
    /// Shared Items Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// Long Text
    /// Represents the following attribute in the schema: :longText
    #[xml(attr = "longText")]
    pub long_text: Option<bool>,
    #[xml(
        child = "x:m",
        child = "x:n",
        child = "x:b",
        child = "x:e",
        child = "x:s",
        child = "x:d",
    )]
    pub children: Vec<SharedItemsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SharedItemsChildChoice {
    #[xml(tag = "x:m")]
    XM(MissingItem),
    #[xml(tag = "x:n")]
    XN(NumberItem),
    #[xml(tag = "x:b")]
    XB(BooleanItem),
    #[xml(tag = "x:e")]
    XE(ErrorItem),
    #[xml(tag = "x:s")]
    XS(StringItem),
    #[xml(tag = "x:d")]
    XD(DateTimeItem),
}
/// Defines the FieldGroup Class.
/// When the object is serialized out as xml, it's qualified name is x:fieldGroup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:fieldGroup")]
pub struct FieldGroup {
    /// Parent
    /// Represents the following attribute in the schema: :par
    #[xml(attr = "par")]
    pub parent_id: Option<i32>,
    /// Field Base
    /// Represents the following attribute in the schema: :base
    #[xml(attr = "base")]
    pub base: Option<i32>,
    #[xml(child = "x:rangePr", child = "x:discretePr", child = "x:groupItems")]
    pub children: Vec<FieldGroupChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FieldGroupChildChoice {
    #[xml(tag = "x:rangePr")]
    XRangePr(RangeProperties),
    #[xml(tag = "x:discretePr")]
    XDiscretePr(DiscreteProperties),
    #[xml(tag = "x:groupItems")]
    XGroupItems(GroupItems),
}
/// Defines the CacheFieldExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct CacheFieldExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<CacheFieldExtension>,
}
/// Defines the CacheFieldExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct CacheFieldExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "x14:cacheField", child = "x15:cachedUniqueNames")]
    pub children: Vec<CacheFieldExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CacheFieldExtensionChildChoice {
    #[xml(tag = "x14:cacheField")]
    X14CacheField(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::CacheField,
    ),
    #[xml(tag = "x15:cachedUniqueNames")]
    X15CachedUniqueNames(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::CachedUniqueNames,
    ),
}
/// Defines the FieldsUsage Class.
/// When the object is serialized out as xml, it's qualified name is x:fieldsUsage.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:fieldsUsage")]
pub struct FieldsUsage {
    /// Field Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:fieldUsage")]
    pub x_field_usage: Vec<FieldUsage>,
}
/// Defines the GroupLevels Class.
/// When the object is serialized out as xml, it's qualified name is x:groupLevels.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:groupLevels")]
pub struct GroupLevels {
    /// Grouping Level Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:groupLevel")]
    pub x_group_level: Vec<GroupLevel>,
}
/// Defines the CacheHierarchyExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct CacheHierarchyExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<CacheHierarchyExtension>,
}
/// Defines the CacheHierarchyExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct CacheHierarchyExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "x14:cacheHierarchy", child = "x15:cacheHierarchy")]
    pub children: Vec<CacheHierarchyExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CacheHierarchyExtensionChildChoice {
    #[xml(tag = "x14:cacheHierarchy")]
    X14CacheHierarchy(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::CacheHierarchy,
    ),
    #[xml(tag = "x15:cacheHierarchy")]
    X15CacheHierarchy(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::CacheHierarchy,
    ),
}
/// Defines the CalculatedMemberExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct CalculatedMemberExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<CalculatedMemberExtension>,
}
/// Defines the CalculatedMemberExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct CalculatedMemberExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "x14:calculatedMember", child = "x15:calculatedMember")]
    pub children: Vec<CalculatedMemberExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CalculatedMemberExtensionChildChoice {
    #[xml(tag = "x14:calculatedMember")]
    X14CalculatedMember(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::CalculatedMember,
    ),
    #[xml(tag = "x15:calculatedMember")]
    X15CalculatedMember(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::CalculatedMember,
    ),
}
/// Defines the DataFieldExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct DataFieldExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<DataFieldExtension>,
}
/// Defines the DataFieldExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct DataFieldExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "x14:dataField", child = "x15:dataField")]
    pub children: Vec<DataFieldExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DataFieldExtensionChildChoice {
    #[xml(tag = "x14:dataField")]
    X14DataField(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DataField,
    ),
    #[xml(tag = "x15:dataField")]
    X15DataField(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::DataField,
    ),
}
/// Defines the PivotFilterExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct PivotFilterExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<PivotFilterExtension>,
}
/// Defines the QueryTableRefresh Class.
/// When the object is serialized out as xml, it's qualified name is x:queryTableRefresh.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:queryTableRefresh")]
pub struct QueryTableRefresh {
    /// Preserve Sort and Filter Layout
    /// Represents the following attribute in the schema: :preserveSortFilterLayout
    #[xml(attr = "preserveSortFilterLayout")]
    pub preserve_sort_filter_layout: Option<bool>,
    /// Next Field Id Wrapped
    /// Represents the following attribute in the schema: :fieldIdWrapped
    #[xml(attr = "fieldIdWrapped")]
    pub field_id_wrapped: Option<bool>,
    /// Headers In Last Refresh
    /// Represents the following attribute in the schema: :headersInLastRefresh
    #[xml(attr = "headersInLastRefresh")]
    pub headers_in_last_refresh: Option<bool>,
    /// Minimum Refresh Version
    /// Represents the following attribute in the schema: :minimumVersion
    #[xml(attr = "minimumVersion")]
    pub minimum_version: Option<u8>,
    /// Next field id
    /// Represents the following attribute in the schema: :nextId
    #[xml(attr = "nextId")]
    pub next_id: Option<i32>,
    /// Columns Left
    /// Represents the following attribute in the schema: :unboundColumnsLeft
    #[xml(attr = "unboundColumnsLeft")]
    pub unbound_columns_left: Option<i32>,
    /// Columns Right
    /// Represents the following attribute in the schema: :unboundColumnsRight
    #[xml(attr = "unboundColumnsRight")]
    pub unbound_columns_right: Option<i32>,
    ///Query table fields
    #[xml(child = "x:queryTableFields")]
    pub query_table_fields: QueryTableFields,
    ///Deleted Fields
    #[xml(child = "x:queryTableDeletedFields")]
    pub query_table_deleted_fields: Option<QueryTableDeletedFields>,
    ///Sort State
    #[xml(child = "x:sortState")]
    pub sort_state: Option<SortState>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the QueryTableExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct QueryTableExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<QueryTableExtension>,
}
/// Defines the SheetCalculationProperties Class.
/// When the object is serialized out as xml, it's qualified name is x:sheetCalcPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetCalcPr")]
pub struct SheetCalculationProperties {
    /// Full Calculation On Load
    /// Represents the following attribute in the schema: :fullCalcOnLoad
    #[xml(attr = "fullCalcOnLoad")]
    pub full_calculation_on_load: Option<bool>,
}
/// Defines the ProtectedRanges Class.
/// When the object is serialized out as xml, it's qualified name is x:protectedRanges.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:protectedRanges")]
pub struct ProtectedRanges {
    /// _
    #[xml(child = "x:protectedRange")]
    pub x_protected_range: Vec<ProtectedRange>,
}
/// Defines the Scenarios Class.
/// When the object is serialized out as xml, it's qualified name is x:scenarios.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:scenarios")]
pub struct Scenarios {
    /// Current Scenario
    /// Represents the following attribute in the schema: :current
    #[xml(attr = "current")]
    pub current: Option<i32>,
    /// Last Shown Scenario
    /// Represents the following attribute in the schema: :show
    #[xml(attr = "show")]
    pub show: Option<i32>,
    /// Sequence of References
    /// Represents the following attribute in the schema: :sqref
    #[xml(attr = "sqref")]
    pub sequence_of_references: Option<String>,
    /// _
    #[xml(child = "x:scenario")]
    pub x_scenario: Vec<Scenario>,
}
/// Defines the MergeCells Class.
/// When the object is serialized out as xml, it's qualified name is x:mergeCells.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:mergeCells")]
pub struct MergeCells {
    /// Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:mergeCell")]
    pub x_merge_cell: Vec<MergeCell>,
}
/// Defines the DataValidations Class.
/// When the object is serialized out as xml, it's qualified name is x:dataValidations.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dataValidations")]
pub struct DataValidations {
    /// Disable Prompts
    /// Represents the following attribute in the schema: :disablePrompts
    #[xml(attr = "disablePrompts")]
    pub disable_prompts: Option<bool>,
    /// Top Left Corner (X Coodrinate)
    /// Represents the following attribute in the schema: :xWindow
    #[xml(attr = "xWindow")]
    pub x_window: Option<i32>,
    /// Top Left Corner (Y Coordinate)
    /// Represents the following attribute in the schema: :yWindow
    #[xml(attr = "yWindow")]
    pub y_window: Option<i32>,
    /// Data Validation Item Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:dataValidation")]
    pub x_data_validation: Vec<DataValidation>,
}
/// Defines the Hyperlinks Class.
/// When the object is serialized out as xml, it's qualified name is x:hyperlinks.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:hyperlinks")]
pub struct Hyperlinks {
    /// _
    #[xml(child = "x:hyperlink")]
    pub x_hyperlink: Vec<Hyperlink>,
}
/// Defines the CellWatches Class.
/// When the object is serialized out as xml, it's qualified name is x:cellWatches.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cellWatches")]
pub struct CellWatches {
    /// _
    #[xml(child = "x:cellWatch")]
    pub x_cell_watch: Vec<CellWatch>,
}
/// Defines the IgnoredErrors Class.
/// When the object is serialized out as xml, it's qualified name is x:ignoredErrors.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ignoredErrors")]
pub struct IgnoredErrors {
    /// _
    #[xml(child = "x:ignoredError")]
    pub x_ignored_error: Vec<IgnoredError>,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<ExtensionList>,
}
/// Defines the TableParts Class.
/// When the object is serialized out as xml, it's qualified name is x:tableParts.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:tableParts")]
pub struct TableParts {
    /// Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:tablePart")]
    pub x_table_part: Vec<TablePart>,
}
/// Defines the WorksheetExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct WorksheetExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<WorksheetExtension>,
}
/// Defines the WorksheetExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct WorksheetExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "x14:conditionalFormattings",
        child = "x14:dataValidations",
        child = "x14:sparklineGroups",
        child = "x14:slicerList",
        child = "x14:protectedRanges",
        child = "x14:ignoredErrors",
        child = "x15:webExtensions",
        child = "x15:timelineRefs",
    )]
    pub children: Vec<WorksheetExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum WorksheetExtensionChildChoice {
    #[xml(tag = "x14:conditionalFormattings")]
    X14ConditionalFormattings(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::ConditionalFormattings,
    ),
    #[xml(tag = "x14:dataValidations")]
    X14DataValidations(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DataValidations,
    ),
    #[xml(tag = "x14:sparklineGroups")]
    X14SparklineGroups(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SparklineGroups,
    ),
    #[xml(tag = "x14:slicerList")]
    X14SlicerList(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerList,
    ),
    #[xml(tag = "x14:protectedRanges")]
    X14ProtectedRanges(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::ProtectedRanges,
    ),
    #[xml(tag = "x14:ignoredErrors")]
    X14IgnoredErrors(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::IgnoredErrors,
    ),
    #[xml(tag = "x15:webExtensions")]
    X15WebExtensions(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::WebExtensions,
    ),
    #[xml(tag = "x15:timelineRefs")]
    X15TimelineRefs(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineReferences,
    ),
}
/// Defines the NumberingFormats Class.
/// When the object is serialized out as xml, it's qualified name is x:numFmts.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:numFmts")]
pub struct NumberingFormats {
    /// Number Format Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:numFmt")]
    pub x_num_fmt: Vec<NumberingFormat>,
}
/// Defines the Fonts Class.
/// When the object is serialized out as xml, it's qualified name is x:fonts.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:fonts")]
pub struct Fonts {
    /// Font Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// knownFonts
    /// Represents the following attribute in the schema: x14ac:knownFonts
    #[xml(attr = "x14ac:knownFonts")]
    pub known_fonts: Option<bool>,
    /// _
    #[xml(child = "x:font")]
    pub x_font: Vec<Font>,
}
/// Defines the Fills Class.
/// When the object is serialized out as xml, it's qualified name is x:fills.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:fills")]
pub struct Fills {
    /// Fill Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:fill")]
    pub x_fill: Vec<Fill>,
}
/// Defines the Borders Class.
/// When the object is serialized out as xml, it's qualified name is x:borders.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:borders")]
pub struct Borders {
    /// Border Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:border")]
    pub x_border: Vec<Border>,
}
/// Defines the CellStyleFormats Class.
/// When the object is serialized out as xml, it's qualified name is x:cellStyleXfs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cellStyleXfs")]
pub struct CellStyleFormats {
    /// Style Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:xf")]
    pub x_xf: Vec<CellFormat>,
}
/// Defines the CellFormats Class.
/// When the object is serialized out as xml, it's qualified name is x:cellXfs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cellXfs")]
pub struct CellFormats {
    /// Format Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:xf")]
    pub x_xf: Vec<CellFormat>,
}
/// Defines the CellStyles Class.
/// When the object is serialized out as xml, it's qualified name is x:cellStyles.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cellStyles")]
pub struct CellStyles {
    /// Style Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:cellStyle")]
    pub x_cell_style: Vec<CellStyle>,
}
/// Defines the DifferentialFormats Class.
/// When the object is serialized out as xml, it's qualified name is x:dxfs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dxfs")]
pub struct DifferentialFormats {
    /// Format Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:dxf")]
    pub x_dxf: Vec<DifferentialFormat>,
}
/// Defines the TableStyles Class.
/// When the object is serialized out as xml, it's qualified name is x:tableStyles.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:tableStyles")]
pub struct TableStyles {
    /// Table Style Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// Default Table Style
    /// Represents the following attribute in the schema: :defaultTableStyle
    #[xml(attr = "defaultTableStyle")]
    pub default_table_style: Option<String>,
    /// Default Pivot Style
    /// Represents the following attribute in the schema: :defaultPivotStyle
    #[xml(attr = "defaultPivotStyle")]
    pub default_pivot_style: Option<String>,
    /// _
    #[xml(child = "x:tableStyle")]
    pub x_table_style: Vec<TableStyle>,
}
/// Defines the Colors Class.
/// When the object is serialized out as xml, it's qualified name is x:colors.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:colors")]
pub struct Colors {
    ///Color Indexes
    #[xml(child = "x:indexedColors")]
    pub indexed_colors: Option<IndexedColors>,
    ///MRU Colors
    #[xml(child = "x:mruColors")]
    pub mru_colors: Option<MruColors>,
}
/// Defines the StylesheetExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct StylesheetExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<StylesheetExtension>,
}
/// Defines the StylesheetExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct StylesheetExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "x14:dxfs",
        child = "x14:slicerStyles",
        child = "x15:dxfs",
        child = "x15:timelineStyles",
    )]
    pub children: Vec<StylesheetExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum StylesheetExtensionChildChoice {
    #[xml(tag = "x14:dxfs")]
    X14Dxfs(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DifferentialFormats,
    ),
    #[xml(tag = "x14:slicerStyles")]
    X14SlicerStyles(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerStyles,
    ),
    #[xml(tag = "x15:dxfs")]
    X15Dxfs(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::DifferentialFormats,
    ),
    #[xml(tag = "x15:timelineStyles")]
    X15TimelineStyles(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineStyles,
    ),
}
/// Defines the Location Class.
/// When the object is serialized out as xml, it's qualified name is x:location.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:location")]
pub struct Location {
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: String,
    /// First Header Row
    /// Represents the following attribute in the schema: :firstHeaderRow
    #[xml(attr = "firstHeaderRow")]
    pub first_header_row: i32,
    /// PivotTable Data First Row
    /// Represents the following attribute in the schema: :firstDataRow
    #[xml(attr = "firstDataRow")]
    pub first_data_row: i32,
    /// First Data Column
    /// Represents the following attribute in the schema: :firstDataCol
    #[xml(attr = "firstDataCol")]
    pub first_data_column: i32,
    /// Rows Per Page Count
    /// Represents the following attribute in the schema: :rowPageCount
    #[xml(attr = "rowPageCount")]
    pub row_page_count: Option<i32>,
    /// Columns Per Page
    /// Represents the following attribute in the schema: :colPageCount
    #[xml(attr = "colPageCount")]
    pub columns_per_page: Option<i32>,
}
/// Defines the PivotFields Class.
/// When the object is serialized out as xml, it's qualified name is x:pivotFields.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pivotFields")]
pub struct PivotFields {
    /// Field Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:pivotField")]
    pub x_pivot_field: Vec<PivotField>,
}
/// Defines the RowFields Class.
/// When the object is serialized out as xml, it's qualified name is x:rowFields.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rowFields")]
pub struct RowFields {
    /// Repeated Items Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:field")]
    pub x_field: Vec<Field>,
}
/// Defines the RowItems Class.
/// When the object is serialized out as xml, it's qualified name is x:rowItems.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rowItems")]
pub struct RowItems {
    /// Items in a Row Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:i")]
    pub x_i: Vec<RowItem>,
}
/// Defines the ColumnFields Class.
/// When the object is serialized out as xml, it's qualified name is x:colFields.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:colFields")]
pub struct ColumnFields {
    /// Repeated Items Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:field")]
    pub x_field: Vec<Field>,
}
/// Defines the ColumnItems Class.
/// When the object is serialized out as xml, it's qualified name is x:colItems.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:colItems")]
pub struct ColumnItems {
    /// Column Item Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:i")]
    pub x_i: Vec<RowItem>,
}
/// Defines the PageFields Class.
/// When the object is serialized out as xml, it's qualified name is x:pageFields.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pageFields")]
pub struct PageFields {
    /// Page Item Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:pageField")]
    pub x_page_field: Vec<PageField>,
}
/// Defines the DataFields Class.
/// When the object is serialized out as xml, it's qualified name is x:dataFields.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dataFields")]
pub struct DataFields {
    /// Data Items Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:dataField")]
    pub x_data_field: Vec<DataField>,
}
/// Defines the Formats Class.
/// When the object is serialized out as xml, it's qualified name is x:formats.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:formats")]
pub struct Formats {
    /// Formats Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:format")]
    pub x_format: Vec<Format>,
}
/// Defines the ConditionalFormats Class.
/// When the object is serialized out as xml, it's qualified name is x:conditionalFormats.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:conditionalFormats")]
pub struct ConditionalFormats {
    /// Conditional Format Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:conditionalFormat")]
    pub x_conditional_format: Vec<ConditionalFormat>,
}
/// Defines the ChartFormats Class.
/// When the object is serialized out as xml, it's qualified name is x:chartFormats.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:chartFormats")]
pub struct ChartFormats {
    /// Format Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:chartFormat")]
    pub x_chart_format: Vec<ChartFormat>,
}
/// Defines the PivotHierarchies Class.
/// When the object is serialized out as xml, it's qualified name is x:pivotHierarchies.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pivotHierarchies")]
pub struct PivotHierarchies {
    /// OLAP Hierarchy Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:pivotHierarchy")]
    pub x_pivot_hierarchy: Vec<PivotHierarchy>,
}
/// Defines the PivotTableStyle Class.
/// When the object is serialized out as xml, it's qualified name is x:pivotTableStyleInfo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pivotTableStyleInfo")]
pub struct PivotTableStyle {
    /// Table Style Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Show Row Header Formatting
    /// Represents the following attribute in the schema: :showRowHeaders
    #[xml(attr = "showRowHeaders")]
    pub show_row_headers: Option<bool>,
    /// Show Table Style Column Header Formatting
    /// Represents the following attribute in the schema: :showColHeaders
    #[xml(attr = "showColHeaders")]
    pub show_column_headers: Option<bool>,
    /// Show Row Stripes
    /// Represents the following attribute in the schema: :showRowStripes
    #[xml(attr = "showRowStripes")]
    pub show_row_stripes: Option<bool>,
    /// Show Column Stripes
    /// Represents the following attribute in the schema: :showColStripes
    #[xml(attr = "showColStripes")]
    pub show_column_stripes: Option<bool>,
    /// Show Last Column
    /// Represents the following attribute in the schema: :showLastColumn
    #[xml(attr = "showLastColumn")]
    pub show_last_column: Option<bool>,
}
/// Defines the PivotFilters Class.
/// When the object is serialized out as xml, it's qualified name is x:filters.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:filters")]
pub struct PivotFilters {
    /// Pivot Filter Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:filter")]
    pub x_filter: Vec<PivotFilter>,
}
/// Defines the RowHierarchiesUsage Class.
/// When the object is serialized out as xml, it's qualified name is x:rowHierarchiesUsage.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:rowHierarchiesUsage")]
pub struct RowHierarchiesUsage {
    /// Item Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:rowHierarchyUsage")]
    pub x_row_hierarchy_usage: Vec<RowHierarchyUsage>,
}
/// Defines the ColumnHierarchiesUsage Class.
/// When the object is serialized out as xml, it's qualified name is x:colHierarchiesUsage.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:colHierarchiesUsage")]
pub struct ColumnHierarchiesUsage {
    /// Items Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:colHierarchyUsage")]
    pub x_col_hierarchy_usage: Vec<ColumnHierarchyUsage>,
}
/// Defines the PivotTableDefinitionExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct PivotTableDefinitionExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<PivotTableDefinitionExtension>,
}
/// Defines the PivotTableDefinitionExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct PivotTableDefinitionExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "x14:pivotTableDefinition",
        child = "x15:pivotTableData",
        child = "x15:pivotTableUISettings",
        child = "xxpvi:pivotVersionInfo",
    )]
    pub children: Vec<PivotTableDefinitionExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PivotTableDefinitionExtensionChildChoice {
    #[xml(tag = "x14:pivotTableDefinition")]
    X14PivotTableDefinition(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::PivotTableDefinition,
    ),
    #[xml(tag = "x15:pivotTableData")]
    X15PivotTableData(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::PivotTableData,
    ),
    #[xml(tag = "x15:pivotTableUISettings")]
    X15PivotTableUiSettings(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::PivotTableUiSettings,
    ),
    #[xml(tag = "xxpvi:pivotVersionInfo")]
    XxpviPivotVersionInfo(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2022_pivot_version_info::PivotVersionInfo,
    ),
}
/// Defines the CacheSource Class.
/// When the object is serialized out as xml, it's qualified name is x:cacheSource.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cacheSource")]
pub struct CacheSource {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: SourceValues,
    /// connectionId
    /// Represents the following attribute in the schema: :connectionId
    #[xml(attr = "connectionId")]
    pub connection_id: Option<i32>,
    #[xml(child = "x:worksheetSource", child = "x:consolidation", child = "x:extLst")]
    pub children: Vec<CacheSourceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CacheSourceChildChoice {
    #[xml(tag = "x:worksheetSource")]
    XWorksheetSource(WorksheetSource),
    #[xml(tag = "x:consolidation")]
    XConsolidation(Consolidation),
    #[xml(tag = "x:extLst")]
    XExtLst(CacheSourceExtensionList),
}
/// Defines the CacheFields Class.
/// When the object is serialized out as xml, it's qualified name is x:cacheFields.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cacheFields")]
pub struct CacheFields {
    /// Field Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:cacheField")]
    pub x_cache_field: Vec<CacheField>,
}
/// Defines the CacheHierarchies Class.
/// When the object is serialized out as xml, it's qualified name is x:cacheHierarchies.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:cacheHierarchies")]
pub struct CacheHierarchies {
    /// Hierarchy Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:cacheHierarchy")]
    pub x_cache_hierarchy: Vec<CacheHierarchy>,
}
/// Defines the Kpis Class.
/// When the object is serialized out as xml, it's qualified name is x:kpis.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:kpis")]
pub struct Kpis {
    /// KPI Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:kpi")]
    pub x_kpi: Vec<Kpi>,
}
/// Defines the TupleCache Class.
/// When the object is serialized out as xml, it's qualified name is x:tupleCache.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:tupleCache")]
pub struct TupleCache {
    ///Entries
    #[xml(child = "x:entries")]
    pub entries: Option<Entries>,
    ///Sets
    #[xml(child = "x:sets")]
    pub sets: Option<Sets>,
    ///OLAP Query Cache
    #[xml(child = "x:queryCache")]
    pub query_cache: Option<QueryCache>,
    ///Server Formats
    #[xml(child = "x:serverFormats")]
    pub server_formats: Option<ServerFormats>,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the CalculatedItems Class.
/// When the object is serialized out as xml, it's qualified name is x:calculatedItems.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:calculatedItems")]
pub struct CalculatedItems {
    /// Calculated Item Formula Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:calculatedItem")]
    pub x_calculated_item: Vec<CalculatedItem>,
}
/// Defines the CalculatedMembers Class.
/// When the object is serialized out as xml, it's qualified name is x:calculatedMembers.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:calculatedMembers")]
pub struct CalculatedMembers {
    /// Calculated Members Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:calculatedMember")]
    pub x_calculated_member: Vec<CalculatedMember>,
}
/// Defines the Dimensions Class.
/// When the object is serialized out as xml, it's qualified name is x:dimensions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:dimensions")]
pub struct Dimensions {
    /// OLAP Dimensions Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:dimension")]
    pub x_dimension: Vec<Dimension>,
}
/// Defines the MeasureGroups Class.
/// When the object is serialized out as xml, it's qualified name is x:measureGroups.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:measureGroups")]
pub struct MeasureGroups {
    /// Measure Group Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:measureGroup")]
    pub x_measure_group: Vec<MeasureGroup>,
}
/// Defines the Maps Class.
/// When the object is serialized out as xml, it's qualified name is x:maps.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:maps")]
pub struct Maps {
    /// Measure Group Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:map")]
    pub x_map: Vec<MeasureDimensionMap>,
}
/// Defines the PivotCacheDefinitionExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct PivotCacheDefinitionExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<PivotCacheDefinitionExtension>,
}
/// Defines the PivotCacheDefinitionExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct PivotCacheDefinitionExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "x14:pivotCacheDefinition",
        child = "x15:pivotCacheDecoupled",
        child = "x15:timelinePivotCacheDefinition",
        child = "x15:pivotCacheIdVersion",
        child = "xxpim:implicitMeasureSupport",
        child = "xxpvi:cacheVersionInfo",
    )]
    pub children: Vec<PivotCacheDefinitionExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PivotCacheDefinitionExtensionChildChoice {
    #[xml(tag = "x14:pivotCacheDefinition")]
    X14PivotCacheDefinition(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::PivotCacheDefinition,
    ),
    #[xml(tag = "x15:pivotCacheDecoupled")]
    X15PivotCacheDecoupled(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::PivotCacheDecoupled,
    ),
    #[xml(tag = "x15:timelinePivotCacheDefinition")]
    X15TimelinePivotCacheDefinition(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelinePivotCacheDefinition,
    ),
    #[xml(tag = "x15:pivotCacheIdVersion")]
    X15PivotCacheIdVersion(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::PivotCacheIdVersion,
    ),
    #[xml(tag = "xxpim:implicitMeasureSupport")]
    XxpimImplicitMeasureSupport(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2020_pivot_nov2020::Xsdboolean,
    ),
    #[xml(tag = "xxpvi:cacheVersionInfo")]
    XxpviCacheVersionInfo(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2022_pivot_version_info::CacheVersionInfo,
    ),
}
/// Sheet names of supporting book.
/// When the object is serialized out as xml, it's qualified name is x:sheetNames.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetNames")]
pub struct SheetNames {
    /// _
    #[xml(child = "x:sheetName")]
    pub x_sheet_name: Vec<SheetName>,
}
/// Defined names associated with supporting book..
/// When the object is serialized out as xml, it's qualified name is x:definedNames.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:definedNames")]
pub struct ExternalDefinedNames {
    /// _
    #[xml(child = "x:definedName")]
    pub x_defined_name: Vec<ExternalDefinedName>,
}
/// Cached worksheet data associated with supporting book.
/// When the object is serialized out as xml, it's qualified name is x:sheetDataSet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheetDataSet")]
pub struct SheetDataSet {
    /// _
    #[xml(child = "x:sheetData")]
    pub x_sheet_data: Vec<ExternalSheetData>,
}
/// Table Columns.
/// When the object is serialized out as xml, it's qualified name is x:tableColumns.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:tableColumns")]
pub struct TableColumns {
    /// Column Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:tableColumn")]
    pub x_table_column: Vec<TableColumn>,
}
/// Table Style.
/// When the object is serialized out as xml, it's qualified name is x:tableStyleInfo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:tableStyleInfo")]
pub struct TableStyleInfo {
    /// Style Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Show First Column
    /// Represents the following attribute in the schema: :showFirstColumn
    #[xml(attr = "showFirstColumn")]
    pub show_first_column: Option<bool>,
    /// Show Last Column
    /// Represents the following attribute in the schema: :showLastColumn
    #[xml(attr = "showLastColumn")]
    pub show_last_column: Option<bool>,
    /// Show Row Stripes
    /// Represents the following attribute in the schema: :showRowStripes
    #[xml(attr = "showRowStripes")]
    pub show_row_stripes: Option<bool>,
    /// Show Column Stripes
    /// Represents the following attribute in the schema: :showColumnStripes
    #[xml(attr = "showColumnStripes")]
    pub show_column_stripes: Option<bool>,
}
/// Future Feature Data Storage Area.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct TableExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<TableExtension>,
}
/// Defines the TableExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct TableExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "x14:table", child = "xlmsforms:msForm")]
    pub children: Vec<TableExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableExtensionChildChoice {
    #[xml(tag = "x14:table")]
    X14Table(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Table,
    ),
    #[xml(tag = "xlmsforms:msForm")]
    XlmsformsMsForm(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2023_ms_forms::MsForm,
    ),
}
/// Defines the FileVersion Class.
/// When the object is serialized out as xml, it's qualified name is x:fileVersion.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:fileVersion")]
pub struct FileVersion {
    /// Application Name
    /// Represents the following attribute in the schema: :appName
    #[xml(attr = "appName")]
    pub application_name: Option<String>,
    /// Last Edited Version
    /// Represents the following attribute in the schema: :lastEdited
    #[xml(attr = "lastEdited")]
    pub last_edited: Option<String>,
    /// Lowest Edited Version
    /// Represents the following attribute in the schema: :lowestEdited
    #[xml(attr = "lowestEdited")]
    pub lowest_edited: Option<String>,
    /// Build Version
    /// Represents the following attribute in the schema: :rupBuild
    #[xml(attr = "rupBuild")]
    pub build_version: Option<String>,
    /// Code Name
    /// Represents the following attribute in the schema: :codeName
    #[xml(attr = "codeName")]
    pub code_name: Option<String>,
}
/// Defines the FileSharing Class.
/// When the object is serialized out as xml, it's qualified name is x:fileSharing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:fileSharing")]
pub struct FileSharing {
    /// Read Only Recommended
    /// Represents the following attribute in the schema: :readOnlyRecommended
    #[xml(attr = "readOnlyRecommended")]
    pub read_only_recommended: Option<bool>,
    /// User Name
    /// Represents the following attribute in the schema: :userName
    #[xml(attr = "userName")]
    pub user_name: Option<String>,
    /// Write Reservation Password
    /// Represents the following attribute in the schema: :reservationPassword
    #[xml(attr = "reservationPassword")]
    pub reservation_password: Option<String>,
    /// Password hash algorithm
    /// Represents the following attribute in the schema: :algorithmName
    #[xml(attr = "algorithmName")]
    pub algorithm_name: Option<String>,
    /// Password hash
    /// Represents the following attribute in the schema: :hashValue
    #[xml(attr = "hashValue")]
    pub hash_value: Option<String>,
    /// Salt for password hash
    /// Represents the following attribute in the schema: :saltValue
    #[xml(attr = "saltValue")]
    pub salt_value: Option<String>,
    /// Spin count for password hash
    /// Represents the following attribute in the schema: :spinCount
    #[xml(attr = "spinCount")]
    pub spin_count: Option<i32>,
}
/// Defines the WorkbookProperties Class.
/// When the object is serialized out as xml, it's qualified name is x:workbookPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:workbookPr")]
pub struct WorkbookProperties {
    /// Date 1904
    /// Represents the following attribute in the schema: :date1904
    #[xml(attr = "date1904")]
    pub date1904: Option<bool>,
    /// dateCompatibility
    /// Represents the following attribute in the schema: :dateCompatibility
    #[xml(attr = "dateCompatibility")]
    pub date_compatibility: Option<bool>,
    /// Show Objects
    /// Represents the following attribute in the schema: :showObjects
    #[xml(attr = "showObjects")]
    pub show_objects: Option<ObjectDisplayValues>,
    /// Show Border Unselected Table
    /// Represents the following attribute in the schema: :showBorderUnselectedTables
    #[xml(attr = "showBorderUnselectedTables")]
    pub show_border_unselected_tables: Option<bool>,
    /// Filter Privacy
    /// Represents the following attribute in the schema: :filterPrivacy
    #[xml(attr = "filterPrivacy")]
    pub filter_privacy: Option<bool>,
    /// Prompted Solutions
    /// Represents the following attribute in the schema: :promptedSolutions
    #[xml(attr = "promptedSolutions")]
    pub prompted_solutions: Option<bool>,
    /// Show Ink Annotations
    /// Represents the following attribute in the schema: :showInkAnnotation
    #[xml(attr = "showInkAnnotation")]
    pub show_ink_annotation: Option<bool>,
    /// Create Backup File
    /// Represents the following attribute in the schema: :backupFile
    #[xml(attr = "backupFile")]
    pub backup_file: Option<bool>,
    /// Save External Link Values
    /// Represents the following attribute in the schema: :saveExternalLinkValues
    #[xml(attr = "saveExternalLinkValues")]
    pub save_external_link_values: Option<bool>,
    /// Update Links Behavior
    /// Represents the following attribute in the schema: :updateLinks
    #[xml(attr = "updateLinks")]
    pub update_links: Option<UpdateLinksBehaviorValues>,
    /// Code Name
    /// Represents the following attribute in the schema: :codeName
    #[xml(attr = "codeName")]
    pub code_name: Option<String>,
    /// Hide Pivot Field List
    /// Represents the following attribute in the schema: :hidePivotFieldList
    #[xml(attr = "hidePivotFieldList")]
    pub hide_pivot_field_list: Option<bool>,
    /// Show Pivot Chart Filter
    /// Represents the following attribute in the schema: :showPivotChartFilter
    #[xml(attr = "showPivotChartFilter")]
    pub show_pivot_chart_filter: Option<bool>,
    /// Allow Refresh Query
    /// Represents the following attribute in the schema: :allowRefreshQuery
    #[xml(attr = "allowRefreshQuery")]
    pub allow_refresh_query: Option<bool>,
    /// Publish Items
    /// Represents the following attribute in the schema: :publishItems
    #[xml(attr = "publishItems")]
    pub publish_items: Option<bool>,
    /// Check Compatibility On Save
    /// Represents the following attribute in the schema: :checkCompatibility
    #[xml(attr = "checkCompatibility")]
    pub check_compatibility: Option<bool>,
    /// Auto Compress Pictures
    /// Represents the following attribute in the schema: :autoCompressPictures
    #[xml(attr = "autoCompressPictures")]
    pub auto_compress_pictures: Option<bool>,
    /// Refresh all Connections on Open
    /// Represents the following attribute in the schema: :refreshAllConnections
    #[xml(attr = "refreshAllConnections")]
    pub refresh_all_connections: Option<bool>,
    /// Default Theme Version
    /// Represents the following attribute in the schema: :defaultThemeVersion
    #[xml(attr = "defaultThemeVersion")]
    pub default_theme_version: Option<i32>,
}
/// Defines the WorkbookProtection Class.
/// When the object is serialized out as xml, it's qualified name is x:workbookProtection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:workbookProtection")]
pub struct WorkbookProtection {
    /// Workbook Password
    /// Represents the following attribute in the schema: :workbookPassword
    #[xml(attr = "workbookPassword")]
    pub workbook_password: Option<String>,
    /// Revisions Password
    /// Represents the following attribute in the schema: :revisionsPassword
    #[xml(attr = "revisionsPassword")]
    pub revisions_password: Option<String>,
    /// Lock Structure
    /// Represents the following attribute in the schema: :lockStructure
    #[xml(attr = "lockStructure")]
    pub lock_structure: Option<bool>,
    /// Lock Windows
    /// Represents the following attribute in the schema: :lockWindows
    #[xml(attr = "lockWindows")]
    pub lock_windows: Option<bool>,
    /// Lock Revisions
    /// Represents the following attribute in the schema: :lockRevision
    #[xml(attr = "lockRevision")]
    pub lock_revision: Option<bool>,
    /// Cryptographic Algorithm Name
    /// Represents the following attribute in the schema: :revisionsAlgorithmName
    #[xml(attr = "revisionsAlgorithmName")]
    pub revisions_algorithm_name: Option<String>,
    /// Password Hash Value
    /// Represents the following attribute in the schema: :revisionsHashValue
    #[xml(attr = "revisionsHashValue")]
    pub revisions_hash_value: Option<String>,
    /// Salt Value for Password Verifier
    /// Represents the following attribute in the schema: :revisionsSaltValue
    #[xml(attr = "revisionsSaltValue")]
    pub revisions_salt_value: Option<String>,
    /// Iterations to Run Hashing Algorithm
    /// Represents the following attribute in the schema: :revisionsSpinCount
    #[xml(attr = "revisionsSpinCount")]
    pub revisions_spin_count: Option<i32>,
    /// Cryptographic Algorithm Name
    /// Represents the following attribute in the schema: :workbookAlgorithmName
    #[xml(attr = "workbookAlgorithmName")]
    pub workbook_algorithm_name: Option<String>,
    /// Password Hash Value
    /// Represents the following attribute in the schema: :workbookHashValue
    #[xml(attr = "workbookHashValue")]
    pub workbook_hash_value: Option<String>,
    /// Salt Value for Password Verifier
    /// Represents the following attribute in the schema: :workbookSaltValue
    #[xml(attr = "workbookSaltValue")]
    pub workbook_salt_value: Option<String>,
    /// Iterations to Run Hashing Algorithm
    /// Represents the following attribute in the schema: :workbookSpinCount
    #[xml(attr = "workbookSpinCount")]
    pub workbook_spin_count: Option<i32>,
}
/// Defines the BookViews Class.
/// When the object is serialized out as xml, it's qualified name is x:bookViews.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:bookViews")]
pub struct BookViews {
    /// _
    #[xml(child = "x:workbookView")]
    pub x_workbook_view: Vec<WorkbookView>,
}
/// Defines the Sheets Class.
/// When the object is serialized out as xml, it's qualified name is x:sheets.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:sheets")]
pub struct Sheets {
    /// _
    #[xml(child = "x:sheet")]
    pub x_sheet: Vec<Sheet>,
}
/// Defines the FunctionGroups Class.
/// When the object is serialized out as xml, it's qualified name is x:functionGroups.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:functionGroups")]
pub struct FunctionGroups {
    /// Built-in Function Group Count
    /// Represents the following attribute in the schema: :builtInGroupCount
    #[xml(attr = "builtInGroupCount")]
    pub built_in_group_count: Option<i32>,
    /// _
    #[xml(child = "x:functionGroup")]
    pub x_function_group: Vec<FunctionGroup>,
}
/// Defines the ExternalReferences Class.
/// When the object is serialized out as xml, it's qualified name is x:externalReferences.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:externalReferences")]
pub struct ExternalReferences {
    /// _
    #[xml(child = "x:externalReference")]
    pub x_external_reference: Vec<ExternalReference>,
}
/// Defines the DefinedNames Class.
/// When the object is serialized out as xml, it's qualified name is x:definedNames.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:definedNames")]
pub struct DefinedNames {
    /// _
    #[xml(child = "x:definedName")]
    pub x_defined_name: Vec<DefinedName>,
}
/// Defines the CalculationProperties Class.
/// When the object is serialized out as xml, it's qualified name is x:calcPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:calcPr")]
pub struct CalculationProperties {
    /// Calculation Id
    /// Represents the following attribute in the schema: :calcId
    #[xml(attr = "calcId")]
    pub calculation_id: Option<i32>,
    /// Calculation Mode
    /// Represents the following attribute in the schema: :calcMode
    #[xml(attr = "calcMode")]
    pub calculation_mode: Option<CalculateModeValues>,
    /// Full Calculation On Load
    /// Represents the following attribute in the schema: :fullCalcOnLoad
    #[xml(attr = "fullCalcOnLoad")]
    pub full_calculation_on_load: Option<bool>,
    /// Reference Mode
    /// Represents the following attribute in the schema: :refMode
    #[xml(attr = "refMode")]
    pub reference_mode: Option<ReferenceModeValues>,
    /// Calculation Iteration
    /// Represents the following attribute in the schema: :iterate
    #[xml(attr = "iterate")]
    pub iterate: Option<bool>,
    /// Iteration Count
    /// Represents the following attribute in the schema: :iterateCount
    #[xml(attr = "iterateCount")]
    pub iterate_count: Option<i32>,
    /// Iterative Calculation Delta
    /// Represents the following attribute in the schema: :iterateDelta
    #[xml(attr = "iterateDelta")]
    pub iterate_delta: Option<f64>,
    /// Full Precision Calculation
    /// Represents the following attribute in the schema: :fullPrecision
    #[xml(attr = "fullPrecision")]
    pub full_precision: Option<bool>,
    /// Calc Completed
    /// Represents the following attribute in the schema: :calcCompleted
    #[xml(attr = "calcCompleted")]
    pub calculation_completed: Option<bool>,
    /// Calculate On Save
    /// Represents the following attribute in the schema: :calcOnSave
    #[xml(attr = "calcOnSave")]
    pub calculation_on_save: Option<bool>,
    /// Concurrent Calculations
    /// Represents the following attribute in the schema: :concurrentCalc
    #[xml(attr = "concurrentCalc")]
    pub concurrent_calculation: Option<bool>,
    /// Concurrent Thread Manual Count
    /// Represents the following attribute in the schema: :concurrentManualCount
    #[xml(attr = "concurrentManualCount")]
    pub concurrent_manual_count: Option<i32>,
    /// Force Full Calculation
    /// Represents the following attribute in the schema: :forceFullCalc
    #[xml(attr = "forceFullCalc")]
    pub force_full_calculation: Option<bool>,
}
/// Defines the OleSize Class.
/// When the object is serialized out as xml, it's qualified name is x:oleSize.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:oleSize")]
pub struct OleSize {
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: String,
}
/// Defines the CustomWorkbookViews Class.
/// When the object is serialized out as xml, it's qualified name is x:customWorkbookViews.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:customWorkbookViews")]
pub struct CustomWorkbookViews {
    /// _
    #[xml(child = "x:customWorkbookView")]
    pub x_custom_workbook_view: Vec<CustomWorkbookView>,
}
/// Defines the PivotCaches Class.
/// When the object is serialized out as xml, it's qualified name is x:pivotCaches.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:pivotCaches")]
pub struct PivotCaches {
    /// _
    #[xml(child = "x:pivotCache")]
    pub x_pivot_cache: Vec<PivotCache>,
}
/// Defines the WebPublishing Class.
/// When the object is serialized out as xml, it's qualified name is x:webPublishing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:webPublishing")]
pub struct WebPublishing {
    /// css
    /// Represents the following attribute in the schema: :css
    #[xml(attr = "css")]
    pub use_css: Option<bool>,
    /// thicket
    /// Represents the following attribute in the schema: :thicket
    #[xml(attr = "thicket")]
    pub thicket: Option<bool>,
    /// longFileNames
    /// Represents the following attribute in the schema: :longFileNames
    #[xml(attr = "longFileNames")]
    pub long_file_names: Option<bool>,
    /// vml
    /// Represents the following attribute in the schema: :vml
    #[xml(attr = "vml")]
    pub use_vml: Option<bool>,
    /// allowPng
    /// Represents the following attribute in the schema: :allowPng
    #[xml(attr = "allowPng")]
    pub allow_png: Option<bool>,
    /// targetScreenSize
    /// Represents the following attribute in the schema: :targetScreenSize
    #[xml(attr = "targetScreenSize")]
    pub target_screen_size: Option<TargetScreenSizeValues>,
    /// dpi
    /// Represents the following attribute in the schema: :dpi
    #[xml(attr = "dpi")]
    pub dpi: Option<i32>,
    /// codePage
    /// Represents the following attribute in the schema: :codePage
    #[xml(attr = "codePage")]
    pub code_page: Option<i32>,
    /// characterSet
    /// Represents the following attribute in the schema: :characterSet
    #[xml(attr = "characterSet")]
    pub character_set: Option<String>,
}
/// Defines the FileRecoveryProperties Class.
/// When the object is serialized out as xml, it's qualified name is x:fileRecoveryPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:fileRecoveryPr")]
pub struct FileRecoveryProperties {
    /// Auto Recover
    /// Represents the following attribute in the schema: :autoRecover
    #[xml(attr = "autoRecover")]
    pub auto_recover: Option<bool>,
    /// Crash Save
    /// Represents the following attribute in the schema: :crashSave
    #[xml(attr = "crashSave")]
    pub crash_save: Option<bool>,
    /// Data Extract Load
    /// Represents the following attribute in the schema: :dataExtractLoad
    #[xml(attr = "dataExtractLoad")]
    pub data_extract_load: Option<bool>,
    /// Repair Load
    /// Represents the following attribute in the schema: :repairLoad
    #[xml(attr = "repairLoad")]
    pub repair_load: Option<bool>,
}
/// Defines the WebPublishObjects Class.
/// When the object is serialized out as xml, it's qualified name is x:webPublishObjects.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:webPublishObjects")]
pub struct WebPublishObjects {
    /// Count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "x:webPublishObject")]
    pub x_web_publish_object: Vec<WebPublishObject>,
}
/// Defines the WorkbookExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:extLst")]
pub struct WorkbookExtensionList {
    /// _
    #[xml(child = "x:ext")]
    pub x_ext: Vec<WorkbookExtension>,
}
/// Defines the WorkbookExtension Class.
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x:ext")]
pub struct WorkbookExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "x14:definedNames",
        child = "x14:pivotCaches",
        child = "x14:slicerCaches",
        child = "x15:slicerCaches",
        child = "x14:workbookPr",
        child = "x15:pivotCaches",
        child = "x15:pivotTableReferences",
        child = "x15:timelineCachePivotCaches",
        child = "x15:timelineCacheRefs",
        child = "x15:workbookPr",
        child = "x15:dataModel",
        child = "xlecs:externalCodeService",
    )]
    pub children: Vec<WorkbookExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum WorkbookExtensionChildChoice {
    #[xml(tag = "x14:definedNames")]
    X14DefinedNames(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DefinedNames,
    ),
    #[xml(tag = "x14:pivotCaches")]
    X14PivotCaches(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::PivotCaches,
    ),
    #[xml(tag = "x14:slicerCaches")]
    X14SlicerCaches(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCaches,
    ),
    #[xml(tag = "x15:slicerCaches")]
    X15SlicerCaches(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::SlicerCaches,
    ),
    #[xml(tag = "x14:workbookPr")]
    X14WorkbookPr(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::WorkbookProperties,
    ),
    #[xml(tag = "x15:pivotCaches")]
    X15PivotCaches(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::PivotCaches,
    ),
    #[xml(tag = "x15:pivotTableReferences")]
    X15PivotTableReferences(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::PivotTableReferences,
    ),
    #[xml(tag = "x15:timelineCachePivotCaches")]
    X15TimelineCachePivotCaches(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineCachePivotCaches,
    ),
    #[xml(tag = "x15:timelineCacheRefs")]
    X15TimelineCacheRefs(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineCacheReferences,
    ),
    #[xml(tag = "x15:workbookPr")]
    X15WorkbookPr(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::WorkbookProperties,
    ),
    #[xml(tag = "x15:dataModel")]
    X15DataModel(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::DataModel,
    ),
    #[xml(tag = "xlecs:externalCodeService")]
    XlecsExternalCodeService(
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2023_external_code_service::ExternalCodeService,
    ),
}
