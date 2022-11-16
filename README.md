# Backlight Popup

A popup to display backlight status on Linux (uses xbacklight) written in GTK3.

## Motivation

Windows has a popup which is shown when backlight brightness level is changed.
I wanted to have this on Linux with [LeftWM](https://leftwm.org), so I decided to create this project.

## State

This project doesn't work like the Windows popup out-of-the-box yet.

### TODO
- [ ] add more configuration options
- [ ] add a convenient way to show and hide the popup after a certain amount of time

## Configuration

The config file `config.ron` is loaded from `$XDG_CONFIG_HOME/backlight-popup` directory

### Example (Default)
```ron
(
    accent_color: (0, 255, 255), // RGB color code
    refresh_interval: 100, // miliseconds
    window_opacity: 1.0 // opacity of the popup window, must be in the range 0..=1
)
```
