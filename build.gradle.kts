plugins {
    `java-library`
}

group = "me.fortibrine"
version = "1.0.0"

repositories {
    mavenCentral()
    maven("https://repo.papermc.io/repository/maven-public/")
}

dependencies {
    compileOnly(libs.paperApi)
}

val isCiBuild = System.getenv("GITHUB_ACTIONS") == "true"

val currentOs = System.getProperty("os.name").lowercase()
val currentArch = System.getProperty("os.arch").lowercase()

val hostTarget = when {
    currentOs.contains("linux") -> "x86_64-unknown-linux-gnu"
    currentOs.contains("win") -> "x86_64-pc-windows-msvc"
    currentOs.contains("freebsd") -> "x86_64-unknown-freebsd"
    currentOs.contains("mac") -> if (currentArch.contains("aarch64") || currentArch.contains("arm64")) {
        "aarch64-apple-darwin"
    } else {
        "x86_64-apple-darwin"
    }
    else -> throw GradleException("Unknown OS: $currentOs")
}

val compileRustLocal = tasks.register<Exec>("compileRustLocal") {
    description = "Compile rust local"
    workingDir = file("src-rust")

    commandLine("cargo", "build", "--target", hostTarget, "--release")
}

tasks.processResources {
    if (!isCiBuild) {
        // local build
        dependsOn(compileRustLocal)

        val rustOutputDir = "${project.projectDir}/src-rust/target/$hostTarget/release/"
        if (currentOs.contains("linux")) {
            from(rustOutputDir) { include("*.so"); into("native/linux") }
        } else if (currentOs.contains("win")) {
            from(rustOutputDir) { include("*.dll"); into("native/windows") }
        } else if (currentOs.contains("freebsd")) {
            from(rustOutputDir) { include("*.so"); into("native/freebsd") }
        } else if (currentOs.contains("mac")) {
            val folder = if (currentArch.contains("aarch64") || currentArch.contains("arm64")) "mac_arm" else "mac_x86"
            from(rustOutputDir) { include("*.dylib"); into("native/$folder") }
        }
    } else {
        println("🚀 Github CI build")
    }
}