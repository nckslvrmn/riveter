# 🔩 riversnap

A minimal layout generator for the [River](https://codeberg.org/river/river) Wayland compositor that lets you snap windows to screen halves or maximize them with keyboard shortcuts.

## Why

River's built-in `rivertile` has no support for runtime layout commands. riversnap replaces it with a layout generator that responds to `send-layout-cmd`, giving you Windows-style window snapping within a proper tiling compositor.

## Installation

```sh
cargo install riversnap
```

Or build from source:

```sh
git clone https://github.com/yourname/riversnap
cd riversnap
cargo build --release
```

## Setup

In your `~/.config/river/init`:

```sh
# Start riversnap
riversnap &
riverctl default-layout riversnap

# Keybindings
riverctl map normal Super Left  send-layout-cmd riversnap "left"
riverctl map normal Super Right send-layout-cmd riversnap "right"
riverctl map normal Super Up    send-layout-cmd riversnap "full"
```

## Commands

| Command | Effect |
|---|---|
| `left` | Focused window takes left half, others stack on right |
| `right` | Focused window takes right half, others stack on left |
| `full` | Focused window maximizes, others sit behind it |
| `tiled` | Return to default tiling layout |
| `padding <n>` | Set padding in pixels (default: 10) |

Padding can also be changed at runtime:

```sh
riverctl send-layout-cmd riversnap "padding 20"
```

## License

MIT
