plugins {
    kotlin("jvm")
    id("software.amazon.smithy.gradle.smithy-jar").version("0.10.1")
}

repositories {
    mavenLocal()
    mavenCentral()
}

smithy {
    format.set(false)
}


dependencies {
    implementation("software.amazon.smithy:smithy-model:1.45.0")
    implementation("software.amazon.smithy:smithy-linters:1.45.0")
    implementation("software.amazon.smithy:smithy-validation-model:1.45.0")
    implementation("software.amazon.smithy:smithy-aws-traits:1.45.0")
}
