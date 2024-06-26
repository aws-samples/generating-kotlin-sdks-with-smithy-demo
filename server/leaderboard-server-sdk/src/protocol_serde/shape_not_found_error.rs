// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_not_found_error_error(value: &crate::error::NotFoundError) -> Result<String, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_not_found_error::ser_not_found_error(&mut object, value)?;
    object.finish();
    Ok(out)
}

pub fn ser_not_found_error(
                         object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
                         input: &crate::error::NotFoundError,
                    ) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.message {
        object.key("message").string(var_1.as_str());
    }
    Ok(())
}

