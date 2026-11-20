# AUS
**AUS** is a game and simulation engine written in Rust. Unlike many others, it uses ECS instead of OOP; there are many tools for convenient and easy development, a rich standard set of components and systems, and much more.

![Version](https://img.shields.io/badge/version-0.6.0-green)
![GitHub License](https://img.shields.io/github/license/nonelop/AUS)
![GitHub language count](https://img.shields.io/github/languages/top/nonelop/AUS)

> Status - 0.6.0, new features are in active development, and the API will change.

## Goal and architecture
Initially, the engine was conceived for a narrow task - simulating the universe and physics in it, and later life. But now the goal is different: the engine is being made with an emphasis on relative universality, while the original goal has not been replaced, only supplemented.

Architecturally, the engine is not a single component but a collection of other engines. An engine for ECS, a physics engine, our own algebra crate, and in the future our own crate for 3D rendering. Communication between them is carried out via bridges; none of them knows about the others.

A unique (as far as I know) feature is the EQL language - a query language for ECS capable of running at runtime. Example query: ```(SELECT WITH Position3D)``` - will give you all entities with the Position3D component. Just three words, and you can write them anywhere, including a terminal during execution. No compilation needed.

## What already exists
- ECS and minimal EQL
- Minimal physics engine
- Algebra crate
- Simple 3D visualization on kiss3d

## What is in development
- Full-fledged EQL language
- Physics support for all main 3D shapes / polygons
- Expansion of the standard ECS set - more components and systems out of the box
- Our own OpenGL visualization
- GUI engine and possibly our own scripting language

## Quick start
```
git clone https://github.com/nonelop/AUS
cd AUS/crates/demo
cargo run
```

Outputs the result of several systems in 3D via kiss3d.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
 * MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.


#### To learn how to contribute to the project, read CONTRIBUTING.md.

##### Project development channel: https://t.me/AUS_DevLog  
##### My Telegram: https://t.me/Nonelop

Ask me questions personally or via issues.