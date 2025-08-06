# ( R&D Stage )
## Chimera Terminal User Interface
### anything can be changed and updated based on research during development

**ChimeraTui** is a lightweight, plugin-driven TUI overlay for your terminal, built in Rust with Ratatui. It’s for power users who want to make their terminal (like Alacritty on NixOS with Niri) less *plain fucking boring* without the bloat of multiplexers like tmux or zellij. Drop plugins into `~/.config/hydraPanel`, set some keybindings, and turn your terminal into a multi-headed beast that does exactly what you want.

## Philosophy

- **Overlay, Not Takeover**: Enhances your existing terminal (e.g., Alacritty) with features like a reactive file tree, not replacing it with a full multiplexer.
- **Plugin Freedom**: Add custom TUIs (Rust crates) to `~/.config/chimeratui/plugins/`—build a tree navigator, status bar, or anything you dream up.
- **Max Customization**: Tweak keybindings and styles (eventually CSS-like) to make it *yours*, no 3rd-party drama.
- **Lightweight**: Fast and lean, keeping the terminal’s text-based, keyboard-driven soul intact.

## Features

### Default Plugin: Tree Navigator
- A sidebar showing the current directory’s files/folders (`📁` for dirs, `📜` for files).
- **Reactive**: Updates to match the active terminal’s working directory.
- **Interactive**: Use arrow keys to navigate, `Enter` to `cd`, or toggle with `Ctrl+H`.
- **Customizable**: Enable/disable via `~/.config/chimeratui/config.toml`.

### Optional Multiplexing
- Spawn up to four terminal panels with `Ctrl+T`, split horizontally or in a 2x2 grid.
- Switch panels with `Ctrl+1` to `Ctrl+4`; the tree navigator syncs to the active panel’s directory.
- A lightweight alternative to tmux/zellij, designed to play nice with Niri’s Wayland tiling.
- Enable/disable in the config—only use it if it’s your vibe.

### Future Plans
- CSS-like styling for vibrant, user-defined themes.
- More plugins (e.g., quick command palette, Git status widget).
- Subtle animations (e.g., sliding sidebars) for extra flair.

## Why ChimeraTui?
tmux and zellij are great, but they’re multiplexers that take over your terminal. ChimeraTui is a lightweight overlay that enhances your terminal with reactive, customizable features like a file tree or optional tiling. Built in Rust with Ratatui, it’s fast, safe, and ready for your wild ideas. No bloat, no drama—just a terminal that works *your* way.

## License
MIT License. Do whatever you want, just keep the terminal’s soul alive.
