# err_mac

A simple no dependency macro for creating error enums with automatic `From` implementations.

## Features

- Automatically implements `From` traits for wrapped error types
- Supports both unit variants and struct variants
- Implements `std::fmt::Display` using the `Debug` implementation
- Works seamlessly with the `?` operator for error propagation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
err_mac = "0.1.0"
```

## Usage

The macro `create_err_with_impls!` takes:

1. Optional attributes for the enum (e.g., `#[derive(Debug)]`)
2. Visibility modifier and enum name
3. A list of unit variants, optionally with wrapped types
4. A semicolon (;)
5. A list of struct variants with named fields

### Basic Example

```rust
use err_mac::create_err_with_impls;

// Define errors that our application might wrap
#[derive(Debug)]
struct DatabaseError;

#[derive(Debug)]
struct ValidationError;

// Create our error enum
create_err_with_impls!(
    #[derive(Debug)]
    pub AppError,
    // Unit variants without wrapped types
    InvalidInput,
    NotFound,
    // Variants with wrapped error types
    Database(DatabaseError),
    Validation(ValidationError),
    Io(std::io::Error),
    // Struct variants after the semicolon
    ;
    InvalidRange {
        min: i32,
        max: i32
    }
);

// The macro enables ergonomic error handling with '?'
fn do_something() -> Result<(), AppError> {
    // These will automatically convert using the generated From impls
    save_to_database()?; // Returns Result<(), DatabaseError>
    validate_input()?;   // Returns Result<(), ValidationError>

    // You can also construct errors directly
    if something_wrong {
        return Err(AppError::InvalidRange { min: 0, max: 100 });
    }
    Ok(())
}
```

### Generated Code

For the example above, the macro generates:

```rust
#[derive(Debug)]
pub enum AppError {
    InvalidInput,
    NotFound,
    Database(DatabaseError),
    Validation(ValidationError),
    Io(std::io::Error),
    InvalidRange { min: i32, max: i32 },
}

// Automatic From implementations for wrapped types
impl From<DatabaseError> for AppError {
    fn from(v: DatabaseError) -> Self {
        Self::Database(v)
    }
}

impl From<ValidationError> for AppError {
    fn from(v: ValidationError) -> Self {
        Self::Validation(v)
    }
}

impl From<std::io::Error> for AppError {
    fn from(v: std::io::Error) -> Self {
        Self::Io(v)
    }
}

// Display implementation
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
```
