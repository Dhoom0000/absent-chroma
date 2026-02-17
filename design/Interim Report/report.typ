
#set page(
  paper: "a4",
  header: align(right+horizon)[MOD002691 - Interim Report], 
  numbering: "1",
)

#set par(justify: true,first-line-indent:(all: true,amount: 1.5em))
#set text(
  font: "Times New Roman",
  size: 12pt,
)
#set heading(
  numbering: "1.",
)


#set document(
  title: "Interim Report",
  author: "Dhrumil Shah",
)

#align(center,text(18pt)[
  *Interim Report*
])

#grid(
  columns: (1fr,1fr),
  align(center)[
    MOD002691 Final Project
  ],
  align(center)[
    #datetime.today().display("[day]/[month]/[year]") \
  ]
)

\

#align(center)[
  #set par(justify: false)
  *Summary* \
  This report identifies personal goals and plans for my Final Project and further future after graduation, aiming at summarizing important discussion points for mentor feedback and guidance. \
]

\


= Introduction to the Project
== Aim
To design and develop Absent Chroma, a multiplayer game that promotes inclusivity by simulating sensory limitations through gameplay. The project aims to deliver strong entertainment value while integrating accessibility features such as HRTF-based spatial audio and chromesthesia-inspired visual aids. It also demonstrates secure, future-proof multiplayer infrastructure using post-quantum cryptographic systems for communication integrity.
== Objectives
- Develop a functional multiplayer prototype of *Absent Chroma* demonstrating cooperative gameplay between two asymmetrically abled characters.  
- Implement procedural terrain generation to enable dynamic and varied playthroughs.  
- Integrate HRTF-based directional audio and chromesthesia-inspired visual cues to simulate sensory limitations.  
- Establish secure, server-authoritative multiplayer communication using post-quantum encryption standards (FIPS-203, ChaCha20Poly1305).  
- Build the game using Rust and Bevy ECS to explore low-level systems in rendering, networking, and real-time audio.  
- Design and integrate minimal pixel-art assets and NES-style chiptune music to achieve a cohesive aesthetic.  
- Deploy the networked server architecture on AWS to test scalability and infrastructure robustness.
== Further Information
I am building a multiplayer video-game focused on game design, entertainment value, and underlying infrastructure. The game is titled "Absent Chroma" signifying the game mechanic where the two playable main characters are blind and deaf, respectively.

The aim of this project is to enhance inclusivity in gaming by designing an experience that allows players to understand sensory limitations while maintaining strong gameplay value.

The game also aims at accessible gaming, as well as simulating the lack of senses for sighted and hearing people. Additionally, I will be implementing HRTF sound system for spatial audio, and also Chromesthesia-like systems for player aid.

Beyond these, this game showcases secure and future-proof multiplayer infrastrucure by employing post-quantum cryptography and encryption algorithms to secure messaging and communication between the server and the clients.

The game will be a very minimal combat-RPG with procedurally generated maps/terrains to showcase variance in the system.
- Story:
  - The characters 'Gray' and 'Note' are human and drone, respectively. 'Gray' is deaf and 'Note' is blind. They are lost in the wilderness between an onslaught of enemies. 
  - The players are strategically assigned random roles and have to collaborate and communicate to survive as long as they can.
  - 'Gray' can only do melee attacks and walk on the ground. 'Note' can only do ranged attacks and can move in 3-dimensions. However, if they are too far apart, they will not be able to communicate.
  - Their goal is to either survive for 15 nights, or reach the stronghold shining a beacon of light in the sky.

- Scope:
  - Portfolio project at minimum. A MVP at most.
  - Platform: Windows, Linux (optionally, Web)
  - Controls: Keyboard and Mouse
  - Proximity Voice Chat/Text
  - 2 players maximum
  - Directional Audio
- Nice-to-haves:
  - AI enemy behaviour
  - Multiplayer lobbies
- Tech:
  - Rust programming Language
  - Bevy ECS Engine
- Art:
  - 2D Pixel Art models imported to a 3D space
  - _Standards_: 64x64 canvas, export at 10x scale. Animation: 24fps
- Music: 
  - FamiStudio NES-style chiptunes
- Networking:
  - Server-authoritative star-mesh network
  - Host server application on AWS
  - Post-quantum cryptographic methods
- Encryption:
  - FIPS-203 based symmetric key generation.
  - ChaCha20Poly1305 encryption algorithm to encode and decode messages.

By developing this project, my goal is to learn low-level concepts involved with rendering, audio, networking, cloud architecture, dynamic programming concepts, encryption, and user experience.


= Plan of work
- *Gantt Chart (_next page_)*
- *Phase 1* Research and Design:
  - Research market requirements, accessibility, game engines and programming languages.
  - Finalize on game mechanics and features.
  - Plan network architecture and encryption.
- *Phase 2* Prototyping:
  - Create minimal flowcharts, network diagrams and wireframes.
  - Install necessary dependencies, learn basics of RUst and Bevy Engine.
  - Implement initial networking and multiplayer framework.
- *Phase 3* Core Implementations:
  - Add post-quantum cryptography key generation.
  - Add logic for symmetric-key encryption algorithms.
  - Implement procedurally generated terrain.
  - Implement enemy damage and health logics.
  - Add HRTF spatial audio capabilities.
  - Add Chromesthesia-like effects.
- *Phase 4* Testing and Optimization:
  - Test gameplay, networking and accessibility.
  - Conduct latency tests locally and in cloud-deployed server.
  - Collect player feedback from friends and volunteers.
- *phase 5* Report Writing and DOcumentation:
  - Generate documentation from code comments.
  - Do final checks for project requirements and deliverables.
  - Complete Final Report and Poster.

#pagebreak()

#page(flipped: true, paper: "a4")[
#align(horizon)[
  #figure(
    image("gantt-chart.svg", fit: "contain", width: 110%),
    caption: [Gantt Chart]
  )]]
#pagebreak()

= Description of 1-2-1 employability engagement
I had a very productive one-to-one meeting with advisor _Judy Edwards_ from the Employability and Careers Advice team on _Wednesday, 24 September 2025_.

I had my CV and LinkedIn profile reviewed with care. She suggested many minor as well as major changes that I needed to make to make my CV noticable to the employer and also ATS-friendly. I have already made those changes, and will submit for another CV review soon.

We also discussed what platforms I should use to search and apply for jobs, what career choices I have, and what mindset I should have during interviews. Overall, it was a fruitful experience with satisfactory meeting results.

= Exit Plan
// next steps, deliverables, risks, mitigations (1 mark)

== Project Risks
- High technical complexity (post-quantum cryptography, HRTF audio effects, procedural generation, multiplayer networking)
- Performance or compatibility issues across platforms.
- AWS deployment and network latency challenges.
- Limited accessibility/multiplayer testing feedback.
=== Mitigations
The project involves high technical complexity with cryptography, HRTF audio, procedural generation, and networking. This will be managed by breaking tasks into smaller modules, using existing Rust/Bevy libraries, and testing prototypes frequently. Performance and compatibility issues across platforms will be mitigated through profiling, optimization, and cross-platform testing. AWS deployment and network latency will be handled by local simulations, incremental scaling, and server-authoritative design. Limited accessibility and multiplayer testing will be addressed by early feedback from diverse testers and debug tools for sensory systems.

== Next steps
_In short term_, I plan on pursuing further studies immediately after graduation. I have already secured a place in M.Sc. Cyber Security at ARU starting January 2026. I made this reckless but calculated decision due to several reasons. Foremost, the job market is really tough as of now with a lot of competition even for entry level positions. Moreover, if I complete my postgraduate before my loan repayment period starts, it will help stabilize my finances in the long term. I also hope to learn a lot and enter the computing field confident in my work and project experiences.

I also plan on completing my final project, polishing it and making it available open-source on my GitHub. I am really inspired by the open-source community and hope to contribute something of value as well. Additionally, I would like to start pursuing my hobbies again after such a long break because of my studies.

A great risk would be having no real work experience when I complete my Masters degree and enter the job market. However, I have come across a lot of internship opportunities specifically targeted towards postgraduate students, and I am also very confident in my project portfolio. I believe that this risk is very little compared to the benefits.

_In long term_, I plan on finding a stable and yet opportunistic field to work in to grow my knowledge and lifestyle. I would also look into getting a teaching license and training and also a PhD while I'm at it.

== Career Risks
Completing postgraduate studies without substantial work experience may limit immediate job opportunities. Shifting focus to a specialized M.Sc. in Cyber Security could create a temporary skill gap in general software development. Balancing open-source projects and hobbies alongside studies risks delaying career-oriented progress.

=== Mitigations
To mitigate these risks, the game project will be open-sourced with full documentation to strengthen the portfolio. Short-term internships or work placements will provide practical experience in security and development. Careful time management will ensure academic milestones remain the priority while still allowing for personal projects and hobbies.

= C.V.

_C.V. attached in the following pages_



// = Submission checklist
// - Summary of project
// - Plan of work
// - CV
// - 1-2-1 employability engagement description
// - Exit plan

