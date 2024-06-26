// Code generated by smithy-kotlin-codegen. DO NOT EDIT!

package kotlinconf.elasticleaderboard.auth

import aws.smithy.kotlin.runtime.auth.AuthOption
import aws.smithy.kotlin.runtime.client.SdkClientOption
import aws.smithy.kotlin.runtime.collections.get
import aws.smithy.kotlin.runtime.http.operation.AuthSchemeResolver
import aws.smithy.kotlin.runtime.http.operation.SdkHttpRequest
import kotlinconf.elasticleaderboard.ElasticLeaderboardClient

/**
 * Adapts the service specific auth scheme resolver to the agnostic runtime interface and binds the auth parameters
 */
internal class ElasticLeaderboardAuthSchemeProviderAdapter(private val config: ElasticLeaderboardClient.Config): AuthSchemeResolver {
    override suspend fun resolve(request: SdkHttpRequest): List<AuthOption> {
        val params = ElasticLeaderboardAuthSchemeParameters {
            operationName = request.context[SdkClientOption.OperationName]
        }
        return config.authSchemeProvider.resolveAuthScheme(params)
    }
}
