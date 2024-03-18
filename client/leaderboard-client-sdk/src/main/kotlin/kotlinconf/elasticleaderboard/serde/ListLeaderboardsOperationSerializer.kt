// Code generated by smithy-kotlin-codegen. DO NOT EDIT!

package kotlinconf.elasticleaderboard.serde

import aws.smithy.kotlin.runtime.http.HttpBody
import aws.smithy.kotlin.runtime.http.HttpMethod
import aws.smithy.kotlin.runtime.http.operation.HttpSerializer
import aws.smithy.kotlin.runtime.http.request.HttpRequestBuilder
import aws.smithy.kotlin.runtime.http.request.url
import aws.smithy.kotlin.runtime.operation.ExecutionContext
import aws.smithy.kotlin.runtime.text.encoding.PercentEncoding
import kotlinconf.elasticleaderboard.model.ListLeaderboardsRequest


internal class ListLeaderboardsOperationSerializer: HttpSerializer.NonStreaming<ListLeaderboardsRequest> {
    override fun serialize(context: ExecutionContext, input: ListLeaderboardsRequest): HttpRequestBuilder {
        val builder = HttpRequestBuilder()
        builder.method = HttpMethod.GET

        builder.url {
            path.encoded = "/leaderboards"
            parameters.decodedParameters(PercentEncoding.SmithyLabel) {
                if (input.maxResults != null) add("maxResults", "${input.maxResults}")
                if (input.nextToken != null) add("nextToken", input.nextToken)
            }
        }

        return builder
    }
}