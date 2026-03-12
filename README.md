# How to use this program

# Template manual

# Config manual

## Structure of config
1. Data - your source of colors and etc. Optional in GUI MODE
2. Targets - your configs for applying theme
3. Restarts - programms to restart after applying theme. Fully optional

## First you need to set up your config

1. Open ~/.config/muscat/config.jsonc
2. Fill out the config, example:

```json
{
  "data": "~/dotfiles/.config/muscat/themes/catppuccin.json",
    
  "targets": [
    "~/dotfiles/.config/waybar/config.jsonc",
    "~/dotfiles/.config/waybar/style.css",
    "~/dotfiles/.config/swaync/style.css",
    "~/dotfiles/.config/cava/config",
    "~/dotfiles/.config/starship.toml",
    "~/dotfiles/.config/kitty/kitty.conf",
    "~/dotfiles/.config/gtk-3.0/gtk.css",
    "~/dotfiles/.config/gtk-4.0/gtk.css",
    "~/dotfiles/.config/vesktop/themes/theme.css",
    "~/dotfiles/.config/zed/settings.json",
    "~/dotfiles/.config/zed/themes/base16.json",
    "~/dotfiles/.config/hypr/hyprland.conf",
    "~/dotfiles/.config/alacritty/alacritty.toml",
    "~/dotfiles/.config/rofi/theme.rasi"
  ],
  
  "restarts": [
    "waybar",
    "zed",
    "swaync"
  ]
}
```

# GUI MODE
## You need to run this programm with --gui argument and select your theme

![screenshot](assets/preview.png)

In GUI MODE programm searches theme in ~/.config/muscat/themes

# CLI MODE
## "restarts" field is still optional but you need to fill "data" field