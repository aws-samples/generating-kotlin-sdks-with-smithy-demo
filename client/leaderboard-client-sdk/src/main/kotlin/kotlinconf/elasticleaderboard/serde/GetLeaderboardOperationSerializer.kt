// Code generated by smithy-kotlin-codegen. DO NOT EDIT!

package kotlinconf.elasticleaderboard.serde

import aws.smithy.kotlin.runtime.http.HttpBody
import aws.smithy.kotlin.runtime.http.HttpMethod
import aws.smithy.kotlin.runtime.http.operation.HttpSerializer
import aws.smithy.kotlin.runtime.http.request.HttpRequestBuilder
import aws.smithy.kotlin.runtime.http.request.url
import aws.smithy.kotlin.runtime.operation.ExecutionContext
import aws.smithy.kotlin.runtime.text.encoding.PercentEncoding
import kotlinconf.elasticleaderboard.model.GetLeaderboardRequest


internal class GetLeaderboardOperationSerializer: HttpSerializer.NonStreaming<GetLeaderboardRequest> {
    override fun serialize(context: ExecutionContext, input: GetLeaderboardRequest): HttpRequestBuilder {
        val builder = HttpRequestBuilder()
        builder.method = HttpMethod.GET

        builder.url {
            requireNotNull(input.id) { "id is bound to the URI and must not be null" }
            path.encodedSegments {
                add(PercentEncoding.Path.encode("leaderboards"))
                add(PercentEncoding.SmithyLabel.encode("${input.id}"))
            }
        }

        return builder
    }
}
