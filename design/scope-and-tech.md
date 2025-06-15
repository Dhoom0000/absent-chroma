# Scope
## Goal - Minimal
- Portfolio Project at minimum
- Windows ~(Web, Linux, Android, MacOS, iOS)~
- Input - Keyboard/Mouse, Gamepad ~(Touch)~
- Basic UI
- Multiplayer - Local or Cloud
    - 2 to 4
    - ~MMO~
    - Authoritative network server model
    - No lobbies yet
- Chat/messaging
- Login/Accounts
    - Lightweight local login

## Milestones
- [ ] Planning and Design
- [ ] Prototype
- [ ] Demo
- [ ] Polish
- [ ] Release

## Out of Scope - Nice to haves
- AI bots
- Auth Logins, unique IDs
- Multiplayer lobbies
- Customizations
- Voice Chat/Video call
- Scripting/Modding

# Tech
## Language
- Rust

## Game Engine
- Bevy

## Art
- Krita - Pixel Art
- Blender - Import art as Plane, and make it 3D
- Plask AI - for motion capture to animation - [https://plask.ai/en-US]
- Bevy Tween - [https://crates.io/crates/bevy_tween]
- Bevy Lunex - for UI - [https://github.com/bytestring-net/bevy_lunex]
- Bevy Blur - [https://github.com/atbentley/bevy_blur_regions]
- ### Standards
    - Sprite Size - 64x64 - import to 640x640 px
    - Animation framerate - 24fps

## Music
- FamiStudio
- Bevy Kira plugin - [https://github.com/NiklasEi/bevy_kira_audio]

## Tools
- Rapier - for physics - [https://sburris.xyz/posts/bevy-gravity/]
- Bevy Noise Map - for proc gen - [https://github.com/YegorStolyarov/bevy_noise_map]
- Bevy Hanabi - for particle system - [https://crates.io/crates/bevy_hanabi]
- Bevy Atmosphere - for sky gen - [https://crates.io/crates/bevy_atmosphere]
- Bevy Mod Picking - for 3D mouse picking - [https://github.com/aevyrie/bevy_mod_picking]
- Bevy Image Export - for Screenshot/Photo - [https://crates.io/crates/bevy_image_export]
- Bevy ECS Marker - to mark entities - [https://github.com/ChoppedStudio/bevy_ecs_markers]
- Bevy Enhanced Input - to handle input - [https://crates.io/crates/bevy_enhanced_input]

## Dev
- bevy-inspector-egui
- tracing + tracing-subscriber
- Deployment options - Fly.io, AWS

## Networking
- Bevy Renet - [https://github.com/lucaspoffo/renet/tree/master/bevy_renet]

## Accessibility
- Coloring for Blindness - [https://davidmathlogic.com/colorblind/]
- Only for dev purposes - [https://crates.io/crates/bevy_color_blindness]

## Extras
- [https://thegrimsey.net/2022/10/15/Bevy-Part-Two.html]