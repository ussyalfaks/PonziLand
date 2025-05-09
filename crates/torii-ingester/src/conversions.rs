pub use dojo_types::primitive::Primitive;
use starknet::core::types::Felt;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Invalid value: Expected {expected}, got {actual}")]
    InvalidValue {
        expected: &'static str,
        actual: Primitive,
    },
}

pub trait FromPrimitive: Sized {
    fn from_primitive(primitive: &Primitive) -> Result<Self, Error>;
}

pub trait PrimitiveInto<T: Sized> {
    fn to_value(&self) -> Result<T, Error>;
}

impl<T: FromPrimitive> PrimitiveInto<T> for Primitive {
    fn to_value(&self) -> Result<T, Error> {
        T::from_primitive(&self)
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
        match primitive.clone() {
            Primitive::Felt252(Some(e)) => Ok(e),
            Primitive::ContractAddress(Some(e)) => Ok(e),
            Primitive::ClassHash(Some(e)) => Ok(e),
            Primitive::EthAddress(Some(e)) => Ok(e),

            actual => Err(Error::InvalidValue {
                expected: "Felt-like",
                actual,
            }),
        }
    }
}
