use std::{
    fmt::{self},
    marker::PhantomData,
};

pub use dojo_types::primitive::Primitive;
use dojo_types::schema::{Enum, EnumError};
use serde::{de, Deserialize, Deserializer};
use starknet::core::types::Felt;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid value: Expected {expected}, got {actual}")]
    InvalidValue {
        expected: &'static str,
        actual: Primitive,
    },
    #[error("Field {0}, expected an enum")]
    NotAnEnum(String),
    #[error("Invalid enum")]
    InvalidEnum(#[from] EnumError),
    #[error("Invalid enum variant: {0}")]
    InvalidEnumVariant(String),
}

pub trait FromPrimitive: Sized {
    /// Converts a `Primitive` into the implementing type.
    ///
    /// # Errors
    ///
    /// Returns an error if the `Primitive` cannot be converted into the implementing type.
    fn from_primitive(primitive: &Primitive) -> Result<Self, Error>;
}

pub trait FromEnum: Sized {
    /// Converts an `Enum` into the implementing type.
    ///
    /// # Errors
    ///
    /// Returns an error if the `Enum` cannot be converted into the implementing type.
    fn from_enum(variant: &Enum) -> Result<Self, Error>;
}

pub trait PrimitiveInto<T: Sized> {
    /// Converts the primitive into the implementing type.
    ///
    /// # Errors
    ///
    /// Returns an error if the `Primitive` cannot be converted into the implementing type.
    fn to_value(&self) -> Result<T, Error>;
}

impl<T: FromPrimitive> PrimitiveInto<T> for Primitive {
    fn to_value(&self) -> Result<T, Error> {
        T::from_primitive(self)
    }
}

macro_rules! impl_from_primitive {
    ($as: ident, $typ: ty) => {
        impl FromPrimitive for $typ {
            fn from_primitive(primitive: &Primitive) -> Result<Self, Error> {
                match primitive.$as() {
                    Some(u) => Ok(u),
                    None => Err(Error::InvalidValue {
                        expected: stringify!($typ),
                        actual: primitive.clone(),
                    }),
                }
            }
        }
    };
}

impl_from_primitive!(as_i8, i8);
impl_from_primitive!(as_i16, i16);
impl_from_primitive!(as_i32, i32);
impl_from_primitive!(as_i64, i64);
impl_from_primitive!(as_i128, i128);
impl_from_primitive!(as_u8, u8);
impl_from_primitive!(as_u16, u16);
impl_from_primitive!(as_u32, u32);
impl_from_primitive!(as_u64, u64);
impl_from_primitive!(as_u128, u128);
impl_from_primitive!(as_bool, bool);

// Handle the special felt case
impl FromPrimitive for Felt {
    fn from_primitive(primitive: &Primitive) -> Result<Self, Error> {
        match *primitive {
            Primitive::ContractAddress(Some(e))
            | Primitive::ClassHash(Some(e))
            | Primitive::EthAddress(Some(e))
            | Primitive::Felt252(Some(e)) => Ok(e),

            actual => Err(Error::InvalidValue {
                expected: "Felt-like",
                actual,
            }),
        }
    }
}

// Red: This only works for unit enums, but this is all the time I have to do for now.
/// Deserialize an enum from a map with a single key.
///
/// # Errors
/// Returns an error if the input is not a map with a single key.
pub fn torii_enum_deserializer<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    // Visitor for handling the Torii enum format: {"Zero":[]}
    struct EnumVisitor<T>(PhantomData<T>);

    impl<'de, T> de::Visitor<'de> for EnumVisitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an enum represented as a map with a single key")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            // Extract the variant name and empty array
            let (variant, _): (String, Vec<()>) = map
                .next_entry()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;

            // Create a string that represents what serde expects for deserializing into an enum
            // For example, if the variant is "Zero", we create a token stream that looks like
            // what would come from the JSON: "Zero"
            T::deserialize(de::value::StringDeserializer::new(variant))
        }
    }

    deserializer.deserialize_map(EnumVisitor(PhantomData))
}
