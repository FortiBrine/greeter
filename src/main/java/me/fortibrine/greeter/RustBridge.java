package me.fortibrine.greeter;

import java.io.File;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.StandardCopyOption;

public class RustBridge {
    static {
        try {
            String os = System.getProperty("os.name").toLowerCase();
            String arch = System.getProperty("os.arch").toLowerCase();
            String subFolder;
            String extension;
            String prefix = "lib";

            if (os.contains("win")) {
                subFolder = "windows";
                extension = ".dll";
                prefix = "";
            } else if (os.contains("linux")) {
                subFolder = "linux";
                extension = ".so";
            } else if (os.contains("freebsd")) {
                subFolder = "freebsd";
                extension = ".so";
            } else if (os.contains("mac")) {
                extension = ".dylib";
                subFolder = (arch.contains("aarch64") || arch.contains("arm64")) ? "mac_arm" : "mac_x86";
            } else {
                throw new RuntimeException("Unknown OS: " + os);
            }

            String libName = prefix + "greeter_jni" + extension;
            String resourcePath = "/native/" + subFolder + "/" + libName;

            File tempLib = File.createTempFile("greeter_jni_", extension);
            tempLib.deleteOnExit();

            try (InputStream in = RustBridge.class.getResourceAsStream(resourcePath)) {
                if (in == null) {
                    throw new IllegalArgumentException("Not found binary in JAR: " + resourcePath);
                }
                Files.copy(in, tempLib.toPath(), StandardCopyOption.REPLACE_EXISTING);
            }

            System.load(tempLib.getAbsolutePath());
        } catch (Exception e) {
            throw new RuntimeException("Critical error RustBridge: ", e);
        }
    }

    public static native String onEnable(String dataFolder);

    public static native String onPlayerJoin(String uuid);

    public static native String onGreetCommand(String uuid, String message);
}
