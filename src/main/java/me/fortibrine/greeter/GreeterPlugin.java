package me.fortibrine.greeter;

import org.bukkit.plugin.java.JavaPlugin;

public class GreeterPlugin extends JavaPlugin {

    @Override
    public void onEnable() {
        getLogger().info(RustBridge.onEnable(getDataFolder().getAbsolutePath()));
    }

}
