// Code generated by smithy-kotlin-codegen. DO NOT EDIT!

package kotlinconf.elasticleaderboard.auth

import aws.smithy.kotlin.runtime.auth.AuthSchemeId
import aws.smithy.kotlin.runtime.http.auth.AnonymousIdentityProvider
import aws.smithy.kotlin.runtime.identity.IdentityProvider
import aws.smithy.kotlin.runtime.identity.IdentityProviderConfig
import kotlinconf.elasticleaderboard.ElasticLeaderboardClient

internal class ElasticLeaderboardIdentityProviderConfigAdapter (private val config: ElasticLeaderboardClient.Config): IdentityProviderConfig {

    override fun identityProviderForScheme(schemeId: AuthSchemeId): IdentityProvider = when(schemeId.id) {
        "smithy.api#noAuth" -> AnonymousIdentityProvider
        else -> error("auth scheme $schemeId not configured for client")
    }

}
