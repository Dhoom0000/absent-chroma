# Absent Chroma

Asymmetric multiplayer RPG built in Rust with Bevy.
One player sees but cannot hear. The other hears but cannot see.
Cooperation is mandatory.

---

## Overview

**Genre:** Asymmetric Multiplayer RPG
**Platform:** Windows (initial target) 
**Engine:** Bevy 
**Language:** Rust 
**Networking Model:** Authoritative server (Renet) 

Absent Chroma is a two-player cooperative experience where sensory perception is partitioned:

* **Blind Player (Robot Assistance Tool)**

  * Hears but cannot see 
  * Constrained 3D flight
  * Ranged + melee combat
  * Echolocation and spatial audio navigation

* **Deaf Player (Human)**

  * Sees but cannot hear 
  * 2D ground movement
  * Hand-to-hand combat
  * Healing and visual scouting

The shared objective is exploration toward a dynamically shifting **Beacon of Light**, culminating in a boss encounter .

---

## Core Design Pillars

* **Strict Sensory Separation**
  Each client receives only the data their character is allowed to perceive.

* **Procedural World**
  Open-world terrain generated at runtime .

* **Authoritative Multiplayer**
  Server validates state and synchronizes clients .

* **Accessibility-First**
  Colorblind considerations and perception-aware UX .

---

## Gameplay Loop

1. Spawn in procedurally generated world.
2. Explore using asymmetric perception.
3. Detect threats using complementary senses.
4. Coordinate combat and healing.
5. Advance toward the Beacon.
6. Survive boss encounter.

The Beacon may dynamically reposition to preserve directional pressure .

---

## Technical Stack

### Engine & Core

* Bevy 0.16 
* Rapier (physics) 
* bevy_renet (multiplayer) 

### Audio

* audionimbus (HRTF / spatial audio) 
* rodio 

### Procedural Generation

* bevy_generative (noise-based systems) 

### Cryptography

* FIPS203 key agreement 
* ChaCha20-Poly1305 encryption 
* HKDF / SHA-2 key derivation 

### Serialization

* bincode v2 

---

## Project Structure

```
absent-chroma/
 ├─ src/
 │   ├─ client_m.rs
 │   └─ server_m.rs
 ├─ Cargo.toml
```

Two binaries are defined:

* `client` → `src/client_m.rs` 
* `server` → `src/server_m.rs` 

---

## Build & Run

### Requirements

* Rust (Edition 2024) 
* Windows (initial target) 

### Build

```
cargo build
```

### Run Server

```
cargo run --bin server
```

### Run Client

```
cargo run --bin client
```

---

## Development Roadmap

* Planning & Design
* Prototype
* Demo
* Polish
* Release 

---

## Out of Scope (Initial Version)

* MMO scale
* Multiplayer lobbies
* Voice/video chat
* Modding/scripting
* AI bots 

---

## Art & Audio Direction

* Pixel art textures on 3D models 
* 64×64 sprite standard (scaled) 
* 24 FPS animation standard 
* FamiStudio chiptune soundtrack 

---

## Design Intent

Absent Chroma explores:

* Cooperation under asymmetric constraints
* Sensory deprivation as a gameplay mechanic
* Secure, encrypted multiplayer in Rust
* ECS-driven separation of perception

The project serves as both a portfolio-quality game and a technical exploration of asymmetric systems, procedural generation, and secure networking.

---

**Status:** Halted development. Reached MVP goal.

---
