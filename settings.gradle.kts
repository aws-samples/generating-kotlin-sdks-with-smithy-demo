import java.net.URI

rootProject.name = "KotlinConf2024"

pluginManagement {
    repositories {
        mavenCentral()
        gradlePluginPortal()
    }
}

sourceControl {
    gitRepository(URI("https://github.com/smithy-lang/smithy-rs.git")) {
        producesModule("software.amazon.smithy.rust.codegen.server.smithy:codegen-server")
    }
}


include(":leaderboard-model")
include(":server-bootstrap")
project(":server-bootstrap").projectDir = file("server/server-bootstrap")

include(":client:client-bootstrap")
if (file("client/leaderboard-client-sdk").exists()) {
    include(":client:leaderboard-client-sdk")
    include(":client:leaderboard-client")
}else {
    println("client not bootstrapped yet, run './gradlew -p client/client-bootstrap build'")
}

