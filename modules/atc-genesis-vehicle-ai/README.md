# ATC Genesis Vehicle AI

Deterministic vehicle intelligence and lightweight vehicle dynamics for Genesis Engine.

Provides vehicle contracts, waypoint following, speed-limit handling,
obstacle-aware braking, driving profiles, and fixed-step-friendly kinematics.

The crate has no rendering, networking, randomness, wall-clock, or ML runtime
dependency. Global collision and world authority remain in existing Genesis
Engine modules.

This is a game/simulation control component, not a certified road-vehicle
autonomy stack.
