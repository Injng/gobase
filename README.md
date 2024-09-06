# gobase

A base for playing Go.

## Installation
[Pre-built AppImages](https://github.com/Injng/gobase/releases/tag/v0.1.0) are offered for Linux x86_64 systems. For other operating systems, clone the repository and see either the Developing or Building
sections below.

## Usage
Gobase offers a full implementation of the game of Go. Black goes first, with white moves alternating thereafter. You can also manually set the move order
by clicking on the icon corresponding to the color in the right sidebar. To ignore move order, use the icon with a plus "+" sign in it to only place 
stones of that color. You can undo and redo moves that you have made by using the left and right arrow keys, respectively.

Gobase also supports loading in other games through SGF (Smart Game Format) files. To load in a game, click the file icon on the left sidebar and select
the appropriate file ending in `.sgf`. You can then navigate through the game using the left and right arrow keys. Note that at this time, Gobase only supports
move properites in SGF (i.e. `B` and `W`).

## Developing

This project requires Tauri and Rust to be installed. You can find instructions for installing Tauri [here](https://tauri.app/v1/guides/getting-started/prerequisites).
Once you've installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run tauri dev
```

## Building

To create a production version:

```bash
npm run tauri build
```

