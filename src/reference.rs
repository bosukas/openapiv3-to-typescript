use crate::error::ReferenceError;

pub fn extract_reference_type(reference: &str) -> Result<String, ReferenceError> {
    let reference_type = reference
        .split("/")
        .last()
        .ok_or_else(|| ReferenceError::TypeExtractionFailed)?;
    Ok(reference_type.to_string())
}
