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

// ---------------------------------------------------------------------------
// Platform
// ---------------------------------------------------------------------------

data class Platform(val cargoTarget: String, val nativeFolder: String, val libPattern: String)

fun detectPlatform(): Platform {
    val os = System.getProperty("os.name").lowercase()
    val arch = System.getProperty("os.arch").lowercase()
    val isArm = arch.contains("aarch64") || arch.contains("arm64")
    return when {
        os.contains("linux")   -> Platform("x86_64-unknown-linux-gnu",  "linux",   "*.so")
        os.contains("win")     -> Platform("x86_64-pc-windows-msvc",    "windows", "*.dll")
        os.contains("freebsd") -> Platform("x86_64-unknown-freebsd",    "freebsd", "*.so")
        os.contains("mac") && isArm -> Platform("aarch64-apple-darwin", "mac_arm", "*.dylib")
        os.contains("mac")     -> Platform("x86_64-apple-darwin",       "mac_x86", "*.dylib")
        else -> throw GradleException("Unsupported OS: $os")
    }
}

// ---------------------------------------------------------------------------
// Tasks
// ---------------------------------------------------------------------------

val isCiBuild = System.getenv("GITHUB_ACTIONS") == "true"

val compileRustLocal by tasks.registering(Exec::class) {
    description = "Compile Rust for the host platform"
    workingDir = file("src-rust")
    val platform = detectPlatform()
    commandLine("cargo", "build", "--target", platform.cargoTarget, "--release")
}

tasks.processResources {
    if (isCiBuild) return@processResources

    val platform = detectPlatform()
    dependsOn(compileRustLocal)
    val rustOutputDir = "${project.projectDir}/src-rust/target/${platform.cargoTarget}/release/"
    from(rustOutputDir) {
        include(platform.libPattern)
        into("native/${platform.nativeFolder}")
    }
}
