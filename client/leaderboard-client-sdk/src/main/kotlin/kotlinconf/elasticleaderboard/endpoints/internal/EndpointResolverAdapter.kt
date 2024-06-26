// Code generated by smithy-kotlin-codegen. DO NOT EDIT!

package kotlinconf.elasticleaderboard.endpoints.internal

import aws.smithy.kotlin.runtime.client.SdkClientOption
import aws.smithy.kotlin.runtime.client.endpoints.Endpoint
import aws.smithy.kotlin.runtime.collections.get
import aws.smithy.kotlin.runtime.http.operation.EndpointResolver
import aws.smithy.kotlin.runtime.http.operation.ResolveEndpointRequest
import kotlinconf.elasticleaderboard.ElasticLeaderboardClient
import kotlinconf.elasticleaderboard.endpoints.ElasticLeaderboardEndpointParameters

internal class EndpointResolverAdapter(
    private val config: ElasticLeaderboardClient.Config
): EndpointResolver {

    override suspend fun resolve(request: ResolveEndpointRequest): Endpoint {
        val params = resolveEndpointParams(config, request)
        val endpoint = config.endpointProvider.resolveEndpoint(params)
        return endpoint
    }
}

internal fun resolveEndpointParams(config: ElasticLeaderboardClient.Config, request: ResolveEndpointRequest): ElasticLeaderboardEndpointParameters {
    return ElasticLeaderboardEndpointParameters {
        val opName = request.context[SdkClientOption.OperationName]
        opContextBindings[opName]?.invoke(this, request)
    }
}
private typealias BindOperationContextParamsFn = (ElasticLeaderboardEndpointParameters.Builder, ResolveEndpointRequest) -> Unit

private val opContextBindings = mapOf<String, BindOperationContextParamsFn> (
)
