// Code generated by smithy-kotlin-codegen. DO NOT EDIT!

package kotlinconf.elasticleaderboard.model

import aws.smithy.kotlin.runtime.SdkDsl

public class DeleteLeaderboardResponse private constructor(builder: Builder) {

    public companion object {
        public operator fun invoke(block: Builder.() -> kotlin.Unit): kotlinconf.elasticleaderboard.model.DeleteLeaderboardResponse = Builder().apply(block).build()
    }

    override fun toString(): kotlin.String = buildString {
        append("DeleteLeaderboardResponse(")
        append(")")
    }

    override fun hashCode(): kotlin.Int {
        return this::class.hashCode()
    }

    override fun equals(other: kotlin.Any?): kotlin.Boolean {
        if (this === other) return true
        if (other == null || this::class != other::class) return false

        other as DeleteLeaderboardResponse

        return true
    }

    @SdkDsl
    public class Builder {

        @PublishedApi
        internal constructor()
        @PublishedApi
        internal constructor(x: kotlinconf.elasticleaderboard.model.DeleteLeaderboardResponse) : this() {
        }

        @PublishedApi
        internal fun build(): kotlinconf.elasticleaderboard.model.DeleteLeaderboardResponse = DeleteLeaderboardResponse(this)

        internal fun correctErrors(): Builder {
            return this
        }
    }
}