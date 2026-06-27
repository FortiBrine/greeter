package me.fortibrine.greeter;

import org.bukkit.command.Command;
import org.bukkit.command.CommandExecutor;
import org.bukkit.command.CommandSender;
import org.bukkit.entity.Player;

public class GreetCommand implements CommandExecutor {

    @Override
    public boolean onCommand(CommandSender sender, Command command, String label, String[] args) {
        if (!(sender instanceof Player)) {
            sender.sendMessage("Only players can use this command.");
            return true;
        }
        if (args.length == 0) {
            sender.sendMessage("Usage: /greet <message>");
            return true;
        }

        Player player = (Player) sender;
        String message = String.join(" ", args);
        player.sendMessage(RustBridge.onGreetCommand(player.getUniqueId().toString(), message));
        return true;
    }
}
