# AUS (Artificial Universe Simulation)  

![Status](https://img.shields.io/badge/status-alpha-orange)
![Rust](https://img.shields.io/badge/rust-2024_edition-blue)
![License](https://img.shields.io/badge/license-GPLv3-green)

**AUS** is an engine and simulation written in Rust. It models Space, Time, Geometric calculations, and Physical laws, along with objects, and in the future, it will provide a space for the emergence of spontaneous reactions and events, such as chemistry and biology.

> Status: 0.4.0-alpha. Under active development. Architecture and APIs are subject to change.

## Philosophy

The engine provides you only with basic geometry and body physics, while you build your own simulation through Objects. The engine is meant solely for *creating* simulations, not being one itself. You can create your unique objects via the `Body` trait, set up their physics via `MoveableBody` and `CollidableBody`, place them in space, and write custom methods for them. This is the **Engine-Body-Object** architecture, where you build your objects using the layers below.  
## What's already here

- Geometry, basic shapes, and collisions  
- Physics, object collision, and movement with velocity
- Body traits and subtraits for physics implementation
- All of this is described in [DOCS.md](DOCS.md).

## Roadmap
- More physical variables and laws: mass, gravity, friction for applicable objects.
- More traits and body types.
- Geometry improvements and adding a large number of new shapes.
- Chemistry and biology simulation.
- 3D visualization instead of console output, and much more.

## Quick Start
```bash
git clone https://github.com/nonelop/AUS.git  
cd AUS  
cargo run  
```
*(Currently outputs the movement and collision of an atom. Check [main.rs](https://github.com/nonelop/AUS/blob/main/src/main.rs) for modifications).*

### Made entirely by humans, with zero AI coding.
The Telegram channel is currently inactive, but it exists: [Telegram Channel.](https://t.me/AUS_DevLog)
Contact the developer: [Telegram](https://t.me/Nonelop).