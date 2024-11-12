#[macro_export]
#[doc(hidden)]
macro_rules! __define_struct_vec {
    ( ($tag:expr, $name:ident, $choicename:ident) { $($value2:expr, $variant2:ident, $ty2: ty)* } { $($value:expr, $variant:ident)* }) => {
        #[derive(Debug, XmlRead, XmlWrite, Clone)]
        #[cfg_attr(test, derive(PartialEq))]
        pub enum $choicename {
            $(
                #[xml(tag = $value)]
                $variant($variant),
            )*
        }

        #[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
        #[cfg_attr(test, derive(PartialEq))]
        #[xml(tag = $tag)]
        pub struct $name {
            $(
                #[xml(attr = $value2)]
                pub $variant2: Option<$ty2>,
            )*

            #[xml(
                $(
                    child = $value,
                )*
            )]
            pub content: Vec<$choicename>,
        }

        impl $name {
            $(
                #[inline(always)]
                pub fn $variant2<T: Into<$ty2>>(mut self, value: T) -> Self {
                    self.$variant2 = Some(value.into());
                    self
                }
            )*
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __define_struct {
    ( ($tag:expr, $name:ident, $a:lifetime) { $($value:expr, $variant:ident, $ty: ty)* }) => {
        #[derive(Debug, XmlRead, XmlWrite, Clone, Default)]
        #[cfg_attr(test, derive(PartialEq))]
        #[xml(tag = $tag)]
        pub struct $name<$a> {
            $(
                #[xml(attr = $value)]
                pub $variant: Option<$ty>,
            )*
        }

        impl<$a> $name<$a> {
            $(
                #[inline(always)]
                pub fn $variant<T: Into<$ty>>(mut self, value: T) -> Self {
                    self.$variant = Some(value.into());
                    self
                }
            )*
        }
    };

    ( ($tag:expr, $name:ident) { $($value:expr, $variant:ident, $ty: ty)* }) => {
        #[derive(Debug, XmlRead, XmlWrite, Clone, Default)]
        #[cfg_attr(test, derive(PartialEq))]
        #[xml(tag = $tag)]
        pub struct $name {
            $(
                #[xml(attr = $value)]
                pub $variant: Option<$ty>,
            )*
        }

        impl $name {
            $(
                #[inline(always)]
                pub fn $variant<T: Into<$ty>>(mut self, value: T) -> Self {
                    self.$variant = Some(value.into());
                    self
                }
            )*
        }
    };

    ( ($tag:expr, $name:ident) { $($value:expr, $variant:ident, $ty: ty)* } { $($value2:expr, $variant2:ident, $ty2: ty)* }) => {
        #[derive(Debug, XmlRead, XmlWrite, Clone, Default)]
        #[cfg_attr(test, derive(PartialEq))]
        #[xml(tag = $tag)]
        pub struct $name {
            $(
                #[xml(attr = $value)]
                pub $variant: Option<$ty>,
            )*
            $(
                #[xml(child = $value2)]
                pub $variant2: Option<$ty2>,
            )*
        }

        impl $name {
            $(
                #[inline(always)]
                pub fn $variant<T: Into<$ty>>(mut self, value: T) -> Self {
                    self.$variant = Some(value.into());
                    self
                }
            )*

            $(
                #[inline(always)]
                pub fn $variant2<T: Into<$ty2>>(mut self, value: T) -> Self {
                    self.$variant2 = Some(value.into());
                    self
                }
            )*
        }
    };



    ( ($tag:expr, $name:ident, $a:lifetime) { $($value:expr, $variant:ident, $ty: ty)* } { $($value2:expr, $variant2:ident, $ty2: ty)* }) => {
        #[derive(Debug, XmlRead, XmlWrite, Clone, Default)]
        #[cfg_attr(test, derive(PartialEq))]
        #[xml(tag = $tag)]
        pub struct $name<$a> {
            $(
                #[xml(attr = $value)]
                pub $variant: Option<$ty>,
            )*
            $(
                #[xml(child = $value2)]
                pub $variant2: Option<$ty2>,
            )*
        }

        impl <$a> $name<$a> {
            $(
                #[inline(always)]
                pub fn $variant<T: Into<$ty>>(mut self, value: T) -> Self {
                    self.$variant = Some(value.into());
                    self
                }
            )*

            $(
                #[inline(always)]
                pub fn $variant2<T: Into<$ty2>>(mut self, value: T) -> Self {
                    self.$variant2 = Some(value.into());
                    self
                }
            )*
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __define_enum {
    ($name:ident { $($variant:ident = $value:expr, )* }) => {
        #[derive(Debug, Clone)]
        #[cfg_attr(test, derive(PartialEq))]
        pub enum $name {
            $( $variant, )*
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match *self {
                    $( $name::$variant => write!(f, $value), )*
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($value => Ok($name::$variant),)*
                    s => Err(format!(
                        "Unkown Value. Found `{}`, Expected `{}`",
                        s,
                        stringify!($($value,)*)
                    ))
                }
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __string_enum {
    ($name:ident { $($variant:ident = $value:expr, )* }) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match *self {
                    $( $name::$variant => write!(f, $value), )*
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($value => Ok($name::$variant),)*
                    s => Err(format!(
                        "Unkown Value. Found `{}`, Expected `{}`",
                        s,
                        stringify!($($value,)*)
                    ))
                }
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __setter {
    ($field:ident: Option<$ty:ty>) => {
        #[inline(always)]
        pub fn $field<T: Into<$ty>>(mut self, value: T) -> Self {
            self.$field = Some(value.into());
            self
        }
    };
    ($field:ident: $ty:ty) => {
        #[inline(always)]
        pub fn $field<T: Into<$ty>>(mut self, value: T) -> Self {
            self.$field = value.into();
            self
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __xml_test_suites {
    ($type:tt, $($struct:expr, $string:expr,)*) => {
        #[test]
        fn xml_test_suites() -> hard_xml::XmlResult<()> {
            let _ = env_logger::builder()
                .is_test(true)
                .format_timestamp(None)
                .try_init();

            $(
                println!("{}, {:?}", $string, ($struct).to_string()?);
                assert_eq!($string, ($struct).to_string()?);
                println!("{:?}", $type::from_str($string)?);
                assert_eq!($struct, $type::from_str($string)?);
            )*

            Ok(())
        }
    };
}

#[macro_export]
macro_rules! read {
    ($name:expr) => {{
        let mut file = archive.by_name($name)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        buffer
    }};
}

#[macro_export]
macro_rules! option_read {
    ($name:expr) => {
        match archive.by_name($name) {
            Err(ZipError::FileNotFound) => None,
            Err(e) => return Err(e.into()),
            Ok(mut file) => {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)?;
                Some(buffer)
            }
        }
    };
}

#[macro_export]
macro_rules! option_read_multiple {
    ($name:expr) => {{
        let names: Vec<_> = archive.file_names().map(|x| x.to_string()).collect();
        let name_and_value: Vec<_> = names
            .iter()
            .filter(|n| n.contains($name))
            .filter_map(|f| {
                archive.by_name(f).ok().and_then(|mut file| {
                    let mut buffer = String::new();
                    file.read_to_string(&mut buffer).ok()?;
                    Some((f.to_string(), buffer))
                })
        })
        .collect();
        name_and_value
    }};
}

#[macro_export]
macro_rules! option_read_multiple_files {
    ($name:expr) => {{
        let names: Vec<_> = archive.file_names().map(|x| x.to_string()).collect();
        let name_and_value: Vec<_> = names
            .iter()
            .filter(|n| n.contains($name))
            .filter_map(|f| {
                archive.by_name(f).ok().and_then(|mut file| {
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer).ok()?;
                    Some((f.to_string(), buffer))
                })
        })
        .collect();
        name_and_value
    }};
}