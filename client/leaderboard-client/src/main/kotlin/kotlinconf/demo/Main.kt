package kotlinconf.demo

import aws.smithy.kotlin.runtime.client.endpoints.Endpoint
import kotlinconf.elasticleaderboard.ElasticLeaderboardClient
import kotlinconf.elasticleaderboard.createLeaderboard
import kotlinconf.elasticleaderboard.endpoints.ElasticLeaderboardEndpointProvider
import kotlinconf.elasticleaderboard.getLeaderboard
import kotlinconf.elasticleaderboard.submitScoreEvent

suspend fun main() {
    ElasticLeaderboardClient {
        endpointProvider = ElasticLeaderboardEndpointProvider { Endpoint("http://localhost:13734") }
    }.use { client ->
        val leaderboardId = client.createLeaderboard {
            name = "kotlinconf2024"
            maxEntries = 20
        }.id

        println("created leaderboard id: $leaderboardId")

        println("Found leaderboards:")
        client.listLeaderboards().leaderboards?.forEach(::println)

        client.submitScoreEvent {
            id = leaderboardId
            scoreEvent {
                user = "Ian"
                score = 25
            }
        }

        client.submitScoreEvent {
            id = leaderboardId
            scoreEvent {
                user = "Aaron"
                score = 35
            }
        }

        val resp = client.getLeaderboard { id = leaderboardId }
        println("Leaderboard ${resp.name} scores:")
        resp.highScores?.forEach { event ->
            println("User ${event.user} scored ${event.score}")
        }
    }
}
