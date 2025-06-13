# Overview
- A multiplayer game, where one player can only see but can't hear, and the other player can hear but can't see.
- RPG, open-world, proc gen world
- Blind player is a 'Robot Assitance Tool' capable of flight - constrained 3d movement, Ranged attacks, melee attacks
- Deaf player is a 'Human' capable of walking - 2d movement, hand-to-hand combat, Healing
- Goal is Exploration, and going towards the 'Beacon of light' i.e. boss battle, can possibly be converted into mini games
- Future scope may include PvP/Team battles
- If players move in any other directions, the 'Beacon' changes positions, so it always seems like they are moving towards it? sounds dumb, but we will see.

# Implementations
- Sound ray-tracing using kira audio and rapier?
- Chromesthesia ?
- Audio direction compass ?
- First do blind, then implement deaf with graphics

# Echo & Sight: Asymmetric Multiplayer RPG

---

## Overview

- A multiplayer game where one player can **hear but can't see** (Blind Player), and the other player can **see but can't hear** (Deaf Player).
- Open-world RPG with procedurally generated terrain, cities, and dungeons.
- **Blind Player:** "Robot Assistance Tool"  
  - Relies on sound (no or very limited vision).  
  - Capable of constrained 3D flight movement.  
  - Equipped with ranged attacks and melee combat.
- **Deaf Player:** "Human"  
  - Relies on vision (no or heavily muted audio).  
  - Limited to 2D walking movement.  
  - Equipped with hand-to-hand combat and healing abilities.
- Shared goal: Exploration and progression toward the **“Beacon of Light”** — a boss battle or major event.
- The beacon dynamically shifts position to encourage forward movement.
- Potential future expansions include PvP and team battles.
- Gameplay can evolve into mini-games based on asymmetrical sensory roles.

---

## Core Gameplay Loop

1. **Spawn & Start:**  
   - Blind player navigates by sound and echolocation, flying through the environment.  
   - Deaf player scouts visually on foot.
2. **Movement & Combat:**  
   - Blind player uses aerial maneuvers and attacks relying on audio cues.  
   - Deaf player moves cautiously, using vision for navigation and healing/combat.
3. **Communication:**  
   - Voice chat or in-game signals (visual flashes for deaf player, audio pings for blind player).
4. **Beacon Pursuit:**  
   - Beacon changes position to maintain player engagement and guide exploration.
5. **Boss Fight:**  
   - Players combine sensory strengths to defeat the final boss.

---

## Implementation Details

### 1. Sound Ray-Tracing for Blind Player

- Use `bevy_rapier3d` for physics and raycasting.  
- Emit sound rays (echolocation pings) that reflect in the environment.  
- Use `bevy_kira_audio` for 3D spatialized audio feedback simulating echoes, occlusions, and reverbs.

### 2. Chromesthesia Visualization for Deaf Player

- Convert important sounds and blind player’s audio signals into **visual effects, colored pulses, or HUD elements**.  
- Enables deaf player to “see” sounds as light patterns or flashes.

### 3. Audio Direction Compass

- Provide blind player with a HUD compass indicating directions of key sounds: beacon, teammate, enemies.

### 4. Procedural World

- Generate terrain, cities, and dungeons procedurally, with unique acoustic and visual properties.

### 5. Character Abilities

| Player        | Movement             | Combat                     | Special Ability                        |
|---------------|----------------------|----------------------------|--------------------------------------|
| Blind Robot   | Constrained 3D flight| Ranged energy blasts, melee| Echolocation pings revealing sounds  |
| Deaf Human    | 2D walking           | Hand-to-hand combat, healing| Visual signals to assist teammate    |

---

## Future Scope Ideas

- **PvP/Team Battles:** Asymmetric teams leveraging sensory differences.  
- **Mini-Games:** Sound-only or sight-only challenges.  
- **Dynamic Beacon:** Intelligent repositioning to maintain exploration tension.

---

## Development Roadmap

### Phase 1: Blind Player Mechanics  
- Procedural world generation.  
- Audio ray tracing and echolocation system.

### Phase 2: Deaf Player Mechanics  
- Visual-only gameplay with chromesthesia effects.  
- Movement, combat, and healing abilities.

### Phase 3: Multiplayer & Communication  
- Networking with sensory data partitioning.  
- Communication systems: voice chat, pings.

### Phase 4: Polish & Extensions  
- Boss encounters, mini-games, PvP modes.

---

## Technical Notes

- Use **Bevy** and **Rapier** for game engine, physics, and raycasting.  
- Use **Kira Audio** for spatial sound.  
- ECS for sensory data separation per player.  
- Functional programming principles for deterministic audio/visual logic.

---

