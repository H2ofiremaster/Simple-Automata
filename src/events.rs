use vizia::input::MouseButton;

use crate::{
    condition::{ConditionIndex, ConditionVariant, Direction},
    display::EditorTab,
    material::MaterialId,
    ruleset::RuleIndex,
};

type Index = usize;
type HexColor = String;

pub enum UpdateEvent {
    WindowSizeChanged,
    CellHovered {
        x: usize,
        y: usize,
    },
    CellUnhovered,
    CellClicked {
        x: usize,
        y: usize,
        button: MouseButton,
    },
    MaterialSelected(MaterialId),
}

pub enum RulesetEvent {
    Selected(Index),
    Saved,
    Created,
    Renamed(String),
    Reloaded,
}

pub enum MaterialEvent {
    Created,
    Renamed(Index, String),
    Recolored(Index, HexColor),
    Deleted(MaterialId),
}

pub enum GroupEvent {
    Created,
    Deleted(Index),
    Edited {
        group_index: Index,
        entry_index: Index,
        new_material_index: Index,
    },
    Renamed(Index, String),
    EntryDeleted {
        group_index: Index,
        entry_index: Index,
    },
    EntryAdded(Index),
}

pub enum RuleEvent {
    Created,
    Deleted(RuleIndex),
    OutputSet(RuleIndex, Index),
    InputSet(RuleIndex, Index),
}
pub enum ConditionEvent {
    Created(RuleIndex),
    PatternSet(ConditionIndex, Index),
    DirectionToggled(ConditionIndex, Direction),
    CountUpdated(ConditionIndex, String),
    VariantChanged(ConditionIndex, ConditionVariant),
    OperatorChanged(ConditionIndex),
    Inverted(ConditionIndex),
}
pub enum GridEvent {
    Stepped,
    Toggled,
    SpeedSet(f32),
}

pub enum EditorEvent {
    Enabled,
    Disabled,
    TabSwitched(EditorTab),
}
