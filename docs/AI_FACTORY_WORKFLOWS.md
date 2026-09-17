# AI Factory Workflow Catalogue

The Franchise Factory is a production and automation platform, not a collection of isolated generators. Each workflow follows a controlled lifecycle:

`INPUT → ANALYZE → PLAN → PRODUCE → QUALITY → INTEGRATE → PUBLISH → MONITOR → OPTIMIZE`

The runtime engine is provider-neutral. AI/model providers are injected as executors; credentials and external side effects remain outside the workflow core.

## Factory catalogue

1. Text / Content Factory — writing, books, scripts, social content, SEO, translation and localization.
2. Software Factory — specification, architecture, frontend/backend/API, databases, tests, documentation and deployment.
3. Game Factory — game design, world, lore, characters, creatures, quests, gameplay, assets, AI NPCs, testing and LiveOps.
4. Marketing Factory — campaigns, audiences, ads, landing pages, experiments and analytics.
5. Business Factory — business models, market analysis, financial models, pricing and KPIs.
6. Document Factory — contracts, offers, invoices, presentations, reports, SOPs and manuals.
7. Research Factory — research, source analysis, monitoring, synthesis, knowledge updates and reports.
8. E-Commerce Factory — catalog, product analysis, descriptions, pricing, campaigns and sales analysis.
9. Customer-Service Factory — support, FAQ, triage, responses, escalation and CRM integration.
10. Automation Factory — triggers, agents, APIs, data processing, notifications and recurring workflows.
11. Knowledge Factory — ingestion, RAG, semantic search, knowledge graph and memory.
12. Agent Factory — identity, role, tools, memory, permissions, workflows and agent-to-agent communication.
13. Franchise Factory — business idea → analysis → brand → product → content → software → marketing → sales → automation → franchise package.
14. Startup Factory — validation, MVP, branding, product, launch and KPI monitoring.
15. Education Factory — curriculum, learning material, exercises, assessment, platform and AI tutoring.
16. Book Factory — research, outline, manuscript, editing, layout, translation and publication.
17. Virtual World Factory — geography, cities, buildings, NPCs, economy, factions, lore and simulation.

## Game Factory production system

The Game Factory exposes a dedicated subsystem catalogue in `gff.workflows.GAME_SUBSYSTEMS`:

- **Concept:** genre, target, platforms, USP, core gameplay loop, modes, monetization and technical requirements.
- **World:** continents, regions, biomes, cities, dungeons, buildings, climate, day/night, weather and portals.
- **Lore:** origin, peoples, factions, wars, timeline, secrets, canon and artifacts.
- **Character:** player characters, NPCs, classes, attributes, skills, progression and relationships.
- **Creature:** anatomy, abilities, weaknesses, attacks, movement, loot, variants and boss mechanics.
- **Combat:** melee, ranged, magic, combos, dodge, parry, status effects, boss phases and PvP balance.
- **Quest:** main, side, faction, events, puzzles, boss, hidden and dynamic quests.
- **Level:** terrain, rooms, paths, encounters, loot, checkpoints, puzzles, secrets and scaling.
- **Item:** weapons, armor, accessories, consumables, resources, relics, artifacts, skins and crafting.
- **Weapon:** concept, design, 3D, animation, VFX, sound and gameplay integration.
- **Animation:** locomotion, attacks, combos, hit/death reactions, emotes and interactions.
- **AI NPC:** identity, memory, personality, goals, knowledge, behavior, routines and relationships.
- **Audio:** SFX, voice, ambience, music, dynamic music and spatial audio.
- **VFX:** fire, water, explosions, magic, energy, portals, weather, abilities and boss effects.
- **Economy:** loot, resources, crafting, pricing, demand, inflation, progression and rewards.
- **Multiplayer:** matchmaking, lobby, party, guilds, PvP/PvE, raids, leaderboards, seasons and server logic.
- **Testing:** bugs, exploits, levels, combat, quests, economy, performance, network, UI, progression and AI playtests.
- **LiveOps:** telemetry, retention, funnels, balance, bugs, seasons, events and new content.

### Game-in-a-Box target

A future production profile can describe a game once and materialize a controlled artifact graph:

`Game Idea → Game Bible → World → Lore → Characters → Creatures → Combat → Quests → Levels → Items → Economy → Assets → Animation → Audio → Code → QA → AI Playtest → Build → Release → LiveOps`

This repository currently implements the orchestration contract and subsystem catalogue. Provider-specific generation, Genesis Engine integration, GCL bus integration, ATC-VM binding and publishing/storage adapters remain separate integration work and must not be represented as implemented until evidence exists.
