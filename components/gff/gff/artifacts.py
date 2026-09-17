"""Typed artifact contracts for deterministic Game Factory orchestration."""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import StrEnum
from typing import Any, Mapping


class ArtifactKind(StrEnum):
    GAME_BIBLE = "game-bible"
    WORLD_BIBLE = "world-bible"
    LORE = "lore"
    CHARACTER = "character"
    CREATURE = "creature"
    QUEST = "quest"
    LEVEL = "level"
    ITEM = "item"
    WEAPON = "weapon"
    ANIMATION = "animation"
    AUDIO = "audio"
    VFX = "vfx"
    COMBAT = "combat"
    NPC_AI = "npc-ai"
    ECONOMY = "economy"
    MULTIPLAYER = "multiplayer"
    BUILD = "build"
    QA_REPORT = "qa-report"
    LIVEOPS_PLAN = "liveops-plan"


@dataclass(frozen=True, slots=True)
class ArtifactRef:
    """Stable reference to an artifact produced by a factory stage."""

    id: str
    kind: ArtifactKind
    version: str = "1.0.0"
    producer: str = ""
    content_hash: str | None = None


@dataclass(slots=True)
class ArtifactEnvelope:
    """Artifact plus provenance/evidence metadata."""

    ref: ArtifactRef
    payload: Mapping[str, Any] = field(default_factory=dict)
    dependencies: tuple[ArtifactRef, ...] = ()
    evidence: list[dict[str, Any]] = field(default_factory=list)

    def add_evidence(self, kind: str, value: Any) -> None:
        self.evidence.append({"kind": kind, "value": value})


class ArtifactContractError(ValueError):
    """Raised when an artifact contract is incomplete or inconsistent."""


def validate_artifact(artifact: ArtifactEnvelope) -> None:
    """Fail closed on missing identity, producer, or dependency identifiers."""
    if not artifact.ref.id.strip():
        raise ArtifactContractError("artifact.id must not be empty")
    if not artifact.ref.version.strip():
        raise ArtifactContractError("artifact.version must not be empty")
    if not artifact.ref.producer.strip():
        raise ArtifactContractError("artifact.producer must not be empty")
    for dependency in artifact.dependencies:
        if not dependency.id.strip():
            raise ArtifactContractError("artifact dependency id must not be empty")
