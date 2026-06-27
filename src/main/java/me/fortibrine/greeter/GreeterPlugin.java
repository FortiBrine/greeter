package me.fortibrine.greeter;

import org.bukkit.plugin.java.JavaPlugin;

public class GreeterPlugin extends JavaPlugin {

    @Override
    public void onEnable() {
        getLogger().info(RustBridge.onEnable(getDataFolder().getAbsolutePath()));
        getCommand("greet").setExecutor(new GreetCommand());
        getServer().getPluginManager().registerEvents(new PlayerJoinListener(), this);
    }

}
