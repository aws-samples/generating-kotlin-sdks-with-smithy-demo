// Code generated by smithy-kotlin-codegen. DO NOT EDIT!

package kotlinconf.elasticleaderboard.model

import aws.smithy.kotlin.runtime.SdkDsl

public class SubmitScoreEventResponse private constructor(builder: Builder) {

    public companion object {
        public operator fun invoke(block: Builder.() -> kotlin.Unit): kotlinconf.elasticleaderboard.model.SubmitScoreEventResponse = Builder().apply(block).build()
    }

    override fun toString(): kotlin.String = buildString {
        append("SubmitScoreEventResponse(")
        append(")")
    }

    override fun hashCode(): kotlin.Int {
        return this::class.hashCode()
    }

    override fun equals(other: kotlin.Any?): kotlin.Boolean {
        if (this === other) return true
        if (other == null || this::class != other::class) return false

        other as SubmitScoreEventResponse

        return true
    }

    @SdkDsl
    public class Builder {

        @PublishedApi
        internal constructor()
        @PublishedApi
        internal constructor(x: kotlinconf.elasticleaderboard.model.SubmitScoreEventResponse) : this() {
        }

        @PublishedApi
        internal fun build(): kotlinconf.elasticleaderboard.model.SubmitScoreEventResponse = SubmitScoreEventResponse(this)

        internal fun correctErrors(): Builder {
            return this
        }
    }
}
