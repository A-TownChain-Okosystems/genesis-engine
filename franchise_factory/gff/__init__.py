# ruff: noqa: F401
"""Genesis Franchise Factory (GFF) — Core-Paket.

Content-Pipeline-Orchestrator des Genesis-Oekosystems.
Kanonische Spec-Quelle: specs/*.atc (AD-20..AD-43, unveraendert aus dem
Org-Archiv a-townchain-os-docs/docs/archive/monorepo-full uebernommen).

Trennungsregel (verbindlich): GFF sitzt UEBER der Genesis Engine und ist
strikte Plattform — keinerlei Abhaengigkeit zu Genesis Chronicles.
"""

__version__ = "0.1.0"

from gff.artifacts import ArtifactContractError, ArtifactEnvelope, ArtifactKind, ArtifactRef  # noqa: F401
from gff.core import Franchise, FranchiseBlueprint, FranchiseStatus, GFFCore, PipelineStage  # noqa: F401
from gff.dao import FranchiseFactory as DAOFranchiseFactory  # noqa: F401
from gff.game_factory import GAME_FACTORY_GRAPH, GameFactoryGraphError, GameFactoryNode, topological_order  # noqa: F401
from gff.lifecycle import LifecycleManager, LPhase  # noqa: F401
from gff.workflows import (  # noqa: F401
    DEFAULT_WORKFLOWS,
    GAME_SUBSYSTEMS,
    FactoryId,
    WorkflowContext,
    WorkflowDefinition,
    WorkflowEngine,
    WorkflowError,
    WorkflowRegistry,
    WorkflowStage,
)

__all__ = [name for name in globals() if not name.startswith('_')]
