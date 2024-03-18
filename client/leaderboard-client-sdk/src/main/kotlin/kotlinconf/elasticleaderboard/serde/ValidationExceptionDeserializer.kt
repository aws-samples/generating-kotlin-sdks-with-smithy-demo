// Code generated by smithy-kotlin-codegen. DO NOT EDIT!

package kotlinconf.elasticleaderboard.serde

import aws.smithy.kotlin.runtime.http.HttpCall
import aws.smithy.kotlin.runtime.http.operation.HttpDeserializer
import aws.smithy.kotlin.runtime.http.response.HttpResponse
import aws.smithy.kotlin.runtime.operation.ExecutionContext
import aws.smithy.kotlin.runtime.serde.SdkFieldDescriptor
import aws.smithy.kotlin.runtime.serde.SdkObjectDescriptor
import aws.smithy.kotlin.runtime.serde.SerialKind
import aws.smithy.kotlin.runtime.serde.asSdkSerializable
import aws.smithy.kotlin.runtime.serde.deserializeList
import aws.smithy.kotlin.runtime.serde.deserializeMap
import aws.smithy.kotlin.runtime.serde.deserializeStruct
import aws.smithy.kotlin.runtime.serde.field
import aws.smithy.kotlin.runtime.serde.json.JsonDeserializer
import aws.smithy.kotlin.runtime.serde.json.JsonSerialName
import aws.smithy.kotlin.runtime.serde.serializeList
import aws.smithy.kotlin.runtime.serde.serializeMap
import aws.smithy.kotlin.runtime.serde.serializeStruct
import kotlin.collections.mutableListOf
import kotlinconf.elasticleaderboard.model.ValidationException
import kotlinconf.elasticleaderboard.model.ValidationExceptionField


internal class ValidationExceptionDeserializer: HttpDeserializer.NonStreaming<ValidationException> {

    override fun deserialize(context: ExecutionContext, call: HttpCall, payload: ByteArray?): ValidationException {
        val response = call.response
        val builder = ValidationException.Builder()

        if (payload != null) {
            deserializeValidationExceptionError(builder, payload)
        }
        builder.correctErrors()
        return builder.build()
    }
}

private fun deserializeValidationExceptionError(builder: ValidationException.Builder, payload: ByteArray) {
    val deserializer = JsonDeserializer(payload)
    val FIELDLIST_DESCRIPTOR = SdkFieldDescriptor(SerialKind.List, JsonSerialName("fieldList"))
    val MESSAGE_DESCRIPTOR = SdkFieldDescriptor(SerialKind.String, JsonSerialName("message"))
    val OBJ_DESCRIPTOR = SdkObjectDescriptor.build {
        field(FIELDLIST_DESCRIPTOR)
        field(MESSAGE_DESCRIPTOR)
    }

    deserializer.deserializeStruct(OBJ_DESCRIPTOR) {
        loop@while (true) {
            when (findNextFieldIndex()) {
                FIELDLIST_DESCRIPTOR.index -> builder.fieldList =
                    deserializer.deserializeList(FIELDLIST_DESCRIPTOR) {
                        val col0 = mutableListOf<ValidationExceptionField>()
                        while (hasNextElement()) {
                            val el0 = if (nextHasValue()) { deserializeValidationExceptionFieldDocument(deserializer) } else { deserializeNull(); continue }
                            col0.add(el0)
                        }
                        col0
                    }
                MESSAGE_DESCRIPTOR.index -> builder.message = deserializeString()
                null -> break@loop
                else -> skipValue()
            }
        }
    }
}
