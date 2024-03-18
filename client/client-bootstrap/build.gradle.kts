import software.amazon.smithy.gradle.tasks.SmithyBuildTask

plugins {
    kotlin("jvm")
    id("software.amazon.smithy.gradle.smithy-base").version("0.10.1")
}

val smithyBuild by configurations.getting
dependencies {
    smithyBuild(project(":leaderboard-model"))
    smithyBuild("software.amazon.smithy.kotlin:smithy-aws-kotlin-codegen:0.32.3")
}


smithy.format.set(false)

val smithyBuildTask = tasks.named<SmithyBuildTask>("smithyBuild")

smithyBuildTask.configure {
    modelDiscoveryClasspath.set(smithyBuild)
}

val bootstrap = tasks.register("bootstrap") {
    dependsOn(smithyBuildTask)

    doLast {
        copy {
            val projectionRoot = smithy.getPluginProjectionPath("leaderboard-client", "kotlin-codegen")
            val dest = project.layout.projectDirectory.asFile.parentFile.resolve("leaderboard-client-sdk")
            from(projectionRoot)
            into(dest)
        }
    }

}
tasks.build.configure {
    finalizedBy(bootstrap)
}
