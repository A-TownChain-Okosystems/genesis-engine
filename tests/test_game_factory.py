import pytest

from gff.artifacts import ArtifactEnvelope, ArtifactKind, ArtifactRef, ArtifactContractError, validate_artifact
from gff.game_factory import (
    GAME_FACTORY_GRAPH,
    GameFactoryGraphError,
    dependency_refs,
    topological_order,
    validate_game_factory_graph,
)


def test_game_factory_graph_is_valid_and_deterministic() -> None:
    validate_game_factory_graph()
    order = topological_order()
    assert order[0] == "concept"
    assert order.index("economy") < order.index("testing")
    assert order.index("build") < order.index("testing")
    assert order.index("testing") < order.index("liveops")
    assert len(order) == len(GAME_FACTORY_GRAPH)


def test_graph_rejects_cycle() -> None:
    cyclic = (
        (GAME_FACTORY_GRAPH[0]),
        type(GAME_FACTORY_GRAPH[0])("cycle-a", ArtifactKind.ECONOMY, (ArtifactKind.ITEM,)),
        type(GAME_FACTORY_GRAPH[0])("cycle-b", ArtifactKind.ITEM, (ArtifactKind.ECONOMY,)),
    )
    with pytest.raises(GameFactoryGraphError, match="cycle"):
        topological_order(cyclic)


def test_dependency_refs_fail_closed_when_input_missing() -> None:
    node = next(node for node in GAME_FACTORY_GRAPH if node.id == "economy")
    with pytest.raises(GameFactoryGraphError, match="Missing dependencies"):
        dependency_refs(node, ())


def test_artifact_contract_requires_producer() -> None:
    artifact = ArtifactEnvelope(
        ref=ArtifactRef(id="game-1", kind=ArtifactKind.GAME_BIBLE, producer="")
    )
    with pytest.raises(ArtifactContractError, match="producer"):
        validate_artifact(artifact)
