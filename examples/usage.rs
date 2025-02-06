use err_mac::create_err_with_impls;
use std::fs;

// Example domain-specific errors
#[derive(Debug)]
struct ValidationError {
    field: String,
    message: String,
}

#[derive(Debug)]
struct DatabaseError {
    code: i32,
    message: String,
}

// Create our application error type
create_err_with_impls!(
    #[derive(Debug)]
    pub ApplicationError,
    // Unit variants
    NotFound,
    Unauthorized,
    // Wrapped error types
    Validation(ValidationError),
    Database(DatabaseError),
    Io(std::io::Error),
    ;
    // Struct variants
    InvalidConfig {
        setting: String,
        allowed_values: Vec<String>
    }
);

// Example functions that demonstrate error handling
fn validate_user_input(input: &str) -> Result<(), ValidationError> {
    if input.is_empty() {
        return Err(ValidationError {
            field: "user_input".to_string(),
            message: "Input cannot be empty".to_string(),
        });
    }
    Ok(())
}

fn query_database(id: i32) -> Result<(), DatabaseError> {
    if id < 0 {
        return Err(DatabaseError {
            code: 400,
            message: "Invalid ID".to_string(),
        });
    }
    Ok(())
}

// Main function demonstrating error propagation
fn process_user_request(user_input: &str, user_id: i32) -> Result<(), ApplicationError> {
    // ValidationError automatically converts to ApplicationError
    validate_user_input(user_input)?;

    // DatabaseError automatically converts to ApplicationError
    query_database(user_id)?;

    // IoError automatically converts to ApplicationError
    fs::read_to_string("config.txt")?;

    // Direct usage of struct variant
    if user_id > 1000 {
        return Err(ApplicationError::InvalidConfig {
            setting: "user_id".to_string(),
            allowed_values: vec!["0-1000".to_string()],
        });
    }

    Ok(())
}

fn main() {
    // Example usage
    match process_user_request("", 1001) {
        Ok(()) => println!("Request processed successfully"),
        Err(e) => println!("Error processing request: {}", e),
    }
}
