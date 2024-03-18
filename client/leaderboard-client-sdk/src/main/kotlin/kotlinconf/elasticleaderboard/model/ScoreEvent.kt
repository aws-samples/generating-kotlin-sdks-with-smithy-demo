// Code generated by smithy-kotlin-codegen. DO NOT EDIT!

package kotlinconf.elasticleaderboard.model

import aws.smithy.kotlin.runtime.SdkDsl

public class ScoreEvent private constructor(builder: Builder) {
    public val score: kotlin.Int? = builder.score
    public val user: kotlin.String? = builder.user

    public companion object {
        public operator fun invoke(block: Builder.() -> kotlin.Unit): kotlinconf.elasticleaderboard.model.ScoreEvent = Builder().apply(block).build()
    }

    override fun toString(): kotlin.String = buildString {
        append("ScoreEvent(")
        append("score=$score,")
        append("user=$user")
        append(")")
    }

    override fun hashCode(): kotlin.Int {
        var result = score ?: 0
        result = 31 * result + (user?.hashCode() ?: 0)
        return result
    }

    override fun equals(other: kotlin.Any?): kotlin.Boolean {
        if (this === other) return true
        if (other == null || this::class != other::class) return false

        other as ScoreEvent

        if (score != other.score) return false
        if (user != other.user) return false

        return true
    }

    public inline fun copy(block: Builder.() -> kotlin.Unit = {}): kotlinconf.elasticleaderboard.model.ScoreEvent = Builder(this).apply(block).build()

    @SdkDsl
    public class Builder {
        public var score: kotlin.Int? = null
        public var user: kotlin.String? = null

        @PublishedApi
        internal constructor()
        @PublishedApi
        internal constructor(x: kotlinconf.elasticleaderboard.model.ScoreEvent) : this() {
            this.score = x.score
            this.user = x.user
        }

        @PublishedApi
        internal fun build(): kotlinconf.elasticleaderboard.model.ScoreEvent = ScoreEvent(this)

        internal fun correctErrors(): Builder {
            return this
        }
    }
}