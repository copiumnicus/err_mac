/// Creates an error enum with automatic implementations of `From` and `Display`.
///
/// This macro simplifies the creation of error types by automatically implementing:
/// - `From` traits for wrapped error types, enabling use with the `?` operator
/// - `Display` trait using the `Debug` implementation
///
/// # Arguments
///
/// The macro accepts:
/// - Optional attributes for the enum (e.g., `#[derive(Debug)]`)
/// - Visibility modifier and enum name
/// - List of unit variants, optionally with wrapped types
/// - Semicolon (;)
/// - List of struct variants with named fields
///
/// # Example
///
/// ```rust
/// use err_mac::create_err_with_impls;
///
/// #[derive(Debug)]
/// struct DatabaseError;
///
/// create_err_with_impls!(
///     #[derive(Debug)]
///     pub AppError,
///     NotFound,
///     Database(DatabaseError),
///     Io(std::io::Error),
///     ;
///     InvalidRange { min: i32, max: i32 }
/// );
///
/// // The ? operator now works with wrapped error types
/// fn example() -> Result<(), AppError> {
///     let db_result: Result<(), DatabaseError> = Ok(());
///     db_result?; // Automatically converts DatabaseError to AppError
///     Ok(())
/// }
/// ```
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
