macro_rules! cast_enum_variant {
    ($e:expr, $p:path) => {
        match $e {
            $p(value) => Some(value),
            _ => None,
        }
    };
}

pub(crate) use cast_enum_variant;
