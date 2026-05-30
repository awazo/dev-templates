pub trait CodeEnum: Sized + Copy {
    type Code: ?Sized;

    fn code(&self) -> &Self::Code;
    fn name(&self) -> &str;
    fn from_code(code: &Self::Code) -> Option<Self>;
    fn from_name(name: &str) -> Option<Self>;
}

#[macro_export]
macro_rules! def_code_enum {
    ($enum:ident, $repr:ident, $default:ident, { $($variant:ident = ($code:expr, $name:expr)),* $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $enum {
            $($variant),*
        }

        impl $crate::endpoint::code_enum::CodeEnum for $enum {
            type Code = $repr;
            fn code(&self) -> &Self::Code {
                match self {
                    $(Self::$variant => &$code,)*
                }
            }
            fn name(&self) -> &str {
                match self {
                    $(Self::$variant => $name,)*
                }
            }
            fn from_code(code: &Self::Code) -> Option<Self> {
                match code {
                    $($code => Some(Self::$variant),)*
                    _ => None,
                }
            }
            fn from_name(name: &str) -> Option<Self> {
                match name {
                    $($name => Some(Self::$variant),)*
                    _ => None,
                }
            }
        }

        impl Default for $enum {
            fn default() -> Self {
                Self::$default
            }
        }

        impl std::fmt::Display for $enum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name())
            }
        }
    };
}

#[macro_export]
macro_rules! def_code_enum_str {
    ($enum:ident, $default:ident, { $($variant:ident = ($code:expr, $name:expr)),* $(,)? }) => {
        $crate::def_code_enum!($enum, str, $default, { $($variant = ($code, $name)),* });
    }
}
#[macro_export]
macro_rules! def_code_enum_u8 {
    ($enum:ident, $default:ident, { $($variant:ident = ($code:expr, $name:expr)),* $(,)? }) => {
        $crate::def_code_enum!($enum, u8, $default, { $($variant = ($code, $name)),* });
    }
}
#[macro_export]
macro_rules! def_code_enum_u16 {
    ($enum:ident, $default:ident, { $($variant:ident = ($code:expr, $name:expr)),* $(,)? }) => {
        $crate::def_code_enum!($enum, u16, $default, { $($variant = ($code, $name)),* });
    }
}

macro_rules! def_code_enum_serde_mod {
    ($mod_name:ident, $deserializer_type:ty) => {
        pub mod $mod_name {
            use super::CodeEnum;
            use serde::{Deserialize, Deserializer, Serialize, Serializer};
            use std::borrow::Borrow;

            pub fn deserialize<'de, D, T>(d: D) -> Result<T, D::Error>
            where
                D: Deserializer<'de>,
                T: CodeEnum,
                $deserializer_type: Borrow<T::Code>,
            {
                let v = <$deserializer_type>::deserialize(d)?;
                T::from_code(v.borrow()).ok_or_else(|| serde::de::Error::custom("unknown code"))
            }

            pub fn serialize<S, T>(v: &T, s: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
                T: CodeEnum,
                T::Code: Serialize,
            {
                v.code().serialize(s)
            }
        }
    };
}

macro_rules! def_code_enum_opt_serde_mod {
    ($mod_name:ident, $deserializer_type:ty) => {
        pub mod $mod_name {
            use super::CodeEnum;
            use serde::{Deserialize, Deserializer, Serialize, Serializer};
            use std::borrow::Borrow;

            pub fn deserialize<'de, D, T>(d: D) -> Result<Option<T>, D::Error>
            where
                D: Deserializer<'de>,
                T: CodeEnum,
                $deserializer_type: Borrow<T::Code>,
            {
                match Option::<$deserializer_type>::deserialize(d)? {
                    Some(v) => T::from_code(v.borrow())
                        .map(Some)
                        .ok_or_else(|| serde::de::Error::custom("unknown code")),
                    None => Ok(None),
                }
            }

            pub fn serialize<S, T>(v: &Option<T>, s: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
                T: CodeEnum,
                T::Code: Serialize,
            {
                match v {
                    Some(v) => v.code().serialize(s),
                    None => s.serialize_none(),
                }
            }
        }
    };
}

def_code_enum_serde_mod!(code_enum_str_serde, String);
def_code_enum_serde_mod!(code_enum_u8_serde, u8);
def_code_enum_serde_mod!(code_enum_u16_serde, u16);

def_code_enum_opt_serde_mod!(code_enum_str_opt_serde, String);
def_code_enum_opt_serde_mod!(code_enum_u8_opt_serde, u8);
def_code_enum_opt_serde_mod!(code_enum_u16_opt_serde, u16);

pub mod code_enum_name_serde {
    use super::CodeEnum;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D, T>(d: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: CodeEnum,
    {
        let name = String::deserialize(d)?;
        T::from_name(&name).ok_or_else(|| serde::de::Error::custom("unknown name"))
    }

    pub fn serialize<S, T>(v: &T, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: CodeEnum,
    {
        v.name().serialize(s)
    }
}

pub mod code_enum_name_opt_serde {
    use super::CodeEnum;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D, T>(d: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: CodeEnum,
    {
        match Option::<String>::deserialize(d)? {
            Some(name) => T::from_name(&name)
                .map(Some)
                .ok_or_else(|| serde::de::Error::custom("unknown name")),
            None => Ok(None),
        }
    }

    pub fn serialize<S, T>(v: &Option<T>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: CodeEnum,
    {
        match v {
            Some(v) => v.name().serialize(s),
            None => s.serialize_none(),
        }
    }
}
