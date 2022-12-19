# Tiny

A tiny implementation of classic games (Sokoban and Snake) in rust running in the terminal.
This is a project to learn rust, the games are not really meant to be played although the mechanics are implemented.
The common parts are extracted into a library **Tiny**.

## Running the examples:

```
cargo run --bin sokoban
```

or

```
cargo run --bin snake
```

## Design of the **Tiny** library

The tiny library provides a plaform abstraction to get the input and clear the terminal.
The app works by impleneting **Flow**s. **Flow** is a trait providing functions for input processing, update and rendering.
Am implementation of **Flow** can launch a new flow by returning it, this allows to transition levels or from intro screen to level, etc...