plugins {
    kotlin("jvm")
    id("software.amazon.smithy.gradle.smithy-base").version("0.10.1")
}

repositories {
    mavenLocal()
    mavenCentral()
}

dependencies {
    implementation(project(":leaderboard-model"))
    implementation("software.amazon.smithy.rust.codegen.server.smithy:codegen-server") {
        version {
            require("release-2024-03-12")
        }
    }
}


smithy.format.set(false)

val bootstrap = tasks.register("bootstrap") {
    dependsOn(tasks.named("smithyBuild"))

    doLast {
        copy {
            val projectionRoot = smithy.getPluginProjectionPath("leaderboard-server", "rust-server-codegen")
            val dest = project.layout.projectDirectory.asFile.parentFile.resolve("leaderboard-server-sdk")
            from(projectionRoot)
            into(dest)
        }
    }

}
tasks.build.configure {
    finalizedBy(bootstrap)
}
