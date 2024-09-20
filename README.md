# gobase

A base for playing Go.

## Installation
[Pre-built AppImages](https://github.com/Injng/gobase/releases/tag/v0.2.1) are offered for Linux x86_64 systems. For other operating systems, clone the repository and see either the Developing or Building
sections below.

## Usage
Gobase offers a full implementation of the game of Go. Black goes first, with white moves alternating thereafter. You can also manually set the move order
by clicking on the icon corresponding to the color in the right sidebar. To ignore move order, use the icon with a plus "+" sign in it to only place 
stones of that color. You can undo and redo moves that you have made by using the left and right arrow keys, respectively.

Gobase also supports loading in other games through SGF (Smart Game Format) files. To load in a game, click the file icon on the left sidebar and select
the appropriate file ending in `.sgf`. You can then navigate through the game using the left and right arrow keys. Note that at this time, Gobase only supports
move properites in SGF (i.e. `B` and `W`).

A number of features are included to aid in the ease of your analysis/learning. You can save the state of your board at any moment, and return to that state at any
moment thereafter. You can also save the moves you have made in SGF format, to import into other applications. Finally, you can save the entire game tree, along with any
states that you have saved, in a savefile consisting of a JSON-serialized object. This savefile can be loaded in at any time to restore the state of the game.

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

