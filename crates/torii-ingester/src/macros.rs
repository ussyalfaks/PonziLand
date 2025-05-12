#[macro_export]
macro_rules! get {
    ($entity: expr, $name: literal, $typ: ty) => {{
        use torii_ingester::conversions::FromPrimitive;
        use torii_ingester::error::ToriiConversionError;

        $entity
            .get($name)
            .ok_or_else(|| ToriiConversionError::NoSuchField($name.into()))
            .and_then(|val| {
                val.as_primitive()
                    .ok_or_else(|| ToriiConversionError::NotAPrimitive($name.into()))
            })
            .and_then(|e| {
                <$typ>::from_primitive(e)
                    .map_err(|e| ToriiConversionError::WrongType($name.into(), e))
            })
            .map(|e| e.into())
    }};

    ($entity: expr, $name: literal, $as: ident) => {{
        $entity
            .get($name)
            .expect(&*format!("Unable to get field {}", $name))
            .as_primitive()
            .expect(&*format!("Field {} is not a primitive", $name))
            .$as()
            .expect(&*format!("Wrong conversion for {}! {:#?}", $name, $entity))
            .into()
    }};
    ($entity: expr, $name: literal, enum $typ: ty) => {{
        use torii_ingester::conversions::FromEnum;
        use torii_ingester::error::ToriiConversionError;

        $entity
            .get($name)
            .ok_or_else(|| ToriiConversionError::NoSuchField($name.into()))
            .and_then(|val| {
                val.as_enum()
                    .ok_or_else(|| ToriiConversionError::NotAnEnum($name.into()))
            })
            .and_then(|e| {
                <$typ>::from_enum(e).map_err(|e| ToriiConversionError::WrongType($name.into(), e))
            })
            .map(|e| e.into())
    }};
}
