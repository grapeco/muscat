# How to use this program

# Template manual
## Let's say you need to configure Waybar. In this case, you need to create the following files in the ~/.config/waybar folder.
1. style.css - main style file
2. style-temp.css - your template file

## Example of template file:
```css
* {
    font-family: "JetBrains Mono";
    font-weight: bold;
    font-size: 14px;
    color: #{{base05}};
}
```

# Theme file manual
## Every theme is coding in "base16"

# Config manual

## Structure of config
1. data - your source of colors and etc. Optional in GUI MODE
2. data_dir - your theme directory for showing list of themes in GUI MODE. Optional. By default path is ~/.config/muscat/themes
3. targets - your configs for applying theme
4. restarts - programms to restart after applying theme. Optional

## First you need to set up your config

1. Open ~/.config/muscat/config.jsonc
2. Enter your settings. For reference:

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
## To enable GUI MODE you need to run this program with "--gui" argument and select your theme

![screenshot](assets/preview.png)

# CLI MODE
## In CLI mode, "restarts" field is still optional, but you must fill "data" field

# Special thanks
## You can check this [fork](https://github.com/milestale/RGBT) with improved English README