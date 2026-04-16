# How to use this piece of crap

# Template manual
## Let's say that you need to configure Waybar. In this case, you need to create the following files in the ~/.config/waybar folder.
1. style.css - main style file
2. style-temp.css - your template file

## Example:
```css
* {
    font-family: "JetBrains Mono";
    font-weight: bold;
    font-size: 14px;
    color: #{{base05}};
}
```

# How theme files are encoded
## Every theme file is encoded in all your "base16" are belong to us.

# How to make a config

## Structure of configuration file:
1. data - Your source of colors and shit (Which is optional if used in graphical shell mode)
2. data_dir - The directory where all of your themes are stored. Default: ~/.config/muscat/themes.
3. targets - Software, whose color pallete you want to change.
4. restarts - Programs that will be restarted after applying themes (Optional).

## How to edit configuration file:

1. Open ~/.config/muscat/config.jsonc in your preffered text editor.
2. Input the arguments accordingly. For reference:

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

# How to use in GUI MODE
## To enable graphical shell, you need to run this program with "--gui" argument.

This is how it looks like:
![screenshot](assets/preview.png)

# Notice about CLI MODE
## If used in terminal, "restarts" field is optional, but data field is a must.

# ЛИИИНУУУКС КАКАШКООО, ШИИНДОООУС 10 ЛУУУЧШЕЕЕЕЕЕЕЮФЩВЫТАШГФКИШЩНЫИКГЩРИЫКВ
