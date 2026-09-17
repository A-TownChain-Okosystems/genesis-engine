from gff.workflows import (
    DEFAULT_WORKFLOWS,
    GAME_SUBSYSTEMS,
    FactoryId,
    WorkflowContext,
    WorkflowEngine,
    WorkflowError,
    WorkflowRegistry,
    WorkflowStage,
)


def test_catalogue_contains_all_requested_factory_workflows() -> None:
    ids = {definition.id for definition in DEFAULT_WORKFLOWS}
    assert ids == {factory.value for factory in FactoryId}
    assert len(ids) == 17


def test_every_workflow_has_quality_gate_and_starts_with_input() -> None:
    registry = WorkflowRegistry()
    for definition in registry.all():
        assert definition.stages[0] is WorkflowStage.INPUT
        assert WorkflowStage.QUALITY in definition.stages


def test_game_catalogue_contains_detailed_production_domains() -> None:
    required = {
        "concept", "world", "lore", "character", "creature", "combat",
        "quest", "level", "item", "weapon", "animation", "ai-npc",
        "audio", "vfx", "economy", "multiplayer", "testing", "liveops",
    }
    assert required <= GAME_SUBSYSTEMS.keys()
    assert "core-loop" in GAME_SUBSYSTEMS["concept"]
    assert "ai-playtests" in GAME_SUBSYSTEMS["testing"]


def test_engine_executes_stages_in_declared_order() -> None:
    seen: list[WorkflowStage] = []

    def executor(stage: WorkflowStage, context: WorkflowContext) -> WorkflowContext:
        seen.append(stage)
        context.artifacts[stage.value] = True
        return context

    result = WorkflowEngine().run(FactoryId.GAME.value, {"title": "Example"}, executor)

    assert seen == list(WorkflowEngine().registry.get(FactoryId.GAME.value).stages)
    assert result.payload == {"title": "Example"}
    assert result.artifacts["quality"] is True
    assert result.evidence[-1] == {"kind": "workflow.completed", "value": FactoryId.GAME.value}


def test_unknown_workflow_fails_closed() -> None:
    try:
        WorkflowEngine().run("does-not-exist", {}, lambda stage, context: context)
    except WorkflowError as exc:
        assert "Unknown workflow" in str(exc)
    else:
        raise AssertionError("Unknown workflows must fail closed")
