#[macro_export]
macro_rules! create_err_with_impls {
    ($(#[$meta:meta])* $vis:vis $enum_name:ident, $($variant:ident $(($type:ty))? ),* $(,)? ; $($struct_variant:ident { $($field:ident: $field_type:ty),* $(,)? } ),* $(,)?) => {
        $(#[$meta])*
        $vis enum $enum_name {
            $(
                $variant $(($type))?,
            )*
            $(
                $struct_variant { $($field: $field_type),* },
            )*
        }

        $(
            $(
                impl From<$type> for $enum_name {
                    fn from(v: $type) -> Self {
                        Self::$variant(v)
                    }
                }
            )?
        )*

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}
