plugins {
    kotlin("jvm")
}

dependencies {
    implementation(kotlin("stdlib"))
implementation("aws.smithy.kotlin:aws-json-protocols:1.2.3")
implementation("aws.smithy.kotlin:aws-protocol-core:1.2.3")
implementation("aws.smithy.kotlin:http:1.2.3")
implementation("aws.smithy.kotlin:http-auth:1.2.3")
implementation("aws.smithy.kotlin:http-client-engine-default:1.2.3")
implementation("aws.smithy.kotlin:identity-api:1.2.3")
implementation("org.jetbrains.kotlin:kotlin-stdlib:1.9.21")
implementation("aws.smithy.kotlin:serde:1.2.3")
implementation("aws.smithy.kotlin:serde-json:1.2.3")
implementation("aws.smithy.kotlin:telemetry-defaults:1.2.3")
api("aws.smithy.kotlin:http-client:1.2.3")
api("aws.smithy.kotlin:runtime-core:1.2.3")
api("aws.smithy.kotlin:smithy-client:1.2.3")
api("aws.smithy.kotlin:telemetry-api:1.2.3")
}
val optInAnnotations = listOf(
    "aws.smithy.kotlin.runtime.InternalApi"
)
kotlin {

    sourceSets.all {
        optInAnnotations.forEach { languageSettings.optIn(it) }
    }
}

tasks.test {
    useJUnitPlatform()
    testLogging {
        events("passed", "skipped", "failed")
        showStandardStreams = true
        showStackTraces = true
        showExceptions = true
        exceptionFormat = org.gradle.api.tasks.testing.logging.TestExceptionFormat.FULL
    }
}
