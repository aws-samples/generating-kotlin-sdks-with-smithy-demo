// Code generated by smithy-kotlin-codegen. DO NOT EDIT!

package kotlinconf.elasticleaderboard.serde

import aws.smithy.kotlin.runtime.http.HttpBody
import aws.smithy.kotlin.runtime.http.HttpMethod
import aws.smithy.kotlin.runtime.http.operation.HttpSerializer
import aws.smithy.kotlin.runtime.http.request.HttpRequestBuilder
import aws.smithy.kotlin.runtime.http.request.url
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
import aws.smithy.kotlin.runtime.serde.json.JsonSerializer
import aws.smithy.kotlin.runtime.serde.serializeList
import aws.smithy.kotlin.runtime.serde.serializeMap
import aws.smithy.kotlin.runtime.serde.serializeStruct
import aws.smithy.kotlin.runtime.text.encoding.PercentEncoding
import kotlinconf.elasticleaderboard.model.UpdateLeaderboardRequest


internal class UpdateLeaderboardOperationSerializer: HttpSerializer.NonStreaming<UpdateLeaderboardRequest> {
    override fun serialize(context: ExecutionContext, input: UpdateLeaderboardRequest): HttpRequestBuilder {
        val builder = HttpRequestBuilder()
        builder.method = HttpMethod.PUT

        builder.url {
            requireNotNull(input.id) { "id is bound to the URI and must not be null" }
            path.encodedSegments {
                add(PercentEncoding.Path.encode("leaderboards"))
                add(PercentEncoding.SmithyLabel.encode("${input.id}"))
            }
        }

        val payload = serializeUpdateLeaderboardOperationBody(context, input)
        builder.body = HttpBody.fromBytes(payload)
        if (builder.body !is HttpBody.Empty) {
            builder.headers.setMissing("Content-Type", "application/json")
        }
        return builder
    }
}

private fun serializeUpdateLeaderboardOperationBody(context: ExecutionContext, input: UpdateLeaderboardRequest): ByteArray {
    val serializer = JsonSerializer()
    val NAME_DESCRIPTOR = SdkFieldDescriptor(SerialKind.String, JsonSerialName("name"))
    val OBJ_DESCRIPTOR = SdkObjectDescriptor.build {
        field(NAME_DESCRIPTOR)
    }

    serializer.serializeStruct(OBJ_DESCRIPTOR) {
        input.name?.let { field(NAME_DESCRIPTOR, it) }
    }
    return serializer.toByteArray()
}
