package me.fortibrine.greeter;

import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.player.PlayerJoinEvent;

public class PlayerJoinListener implements Listener {

    @EventHandler
    public void onPlayerJoin(PlayerJoinEvent event) {
        String uuid = event.getPlayer().getUniqueId().toString();
        String greet = RustBridge.onPlayerJoin(uuid);
        if (!greet.isEmpty()) {
            event.setJoinMessage(greet);
        }
    }
}
