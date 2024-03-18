// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_validation_exception_error(value: &crate::error::ValidationException) -> Result<String, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_validation_exception::ser_validation_exception(&mut object, value)?;
    object.finish();
    Ok(out)
}

pub fn ser_validation_exception(
                         object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
                         input: &crate::error::ValidationException,
                    ) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.field_list {
        let mut array_2 = object.key("fieldList").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_validation_exception_field::ser_validation_exception_field(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
     {
        object.key("message").string(input.message.as_str());
    }
    Ok(())
}
