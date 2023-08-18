use gpui::Action;

use crate::{ActivateRegexMode, ActivateTextMode};
// TODO: Update the default search mode to get from config
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum SearchMode {
    #[default]
    Text,
    Regex,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Side {
    Left,
    Right,
}

impl SearchMode {
    pub(crate) fn label(&self) -> &'static str {
        match self {
            SearchMode::Text => "Text",
            SearchMode::Regex => "Regex",
        }
    }

    pub(crate) fn region_id(&self) -> usize {
        match self {
            SearchMode::Text => 3,
            SearchMode::Regex => 5,
        }
    }

    pub(crate) fn tooltip_text(&self) -> &'static str {
        match self {
            SearchMode::Text => "Activate Text Search",
            SearchMode::Regex => "Activate Regex Search",
        }
    }

    pub(crate) fn activate_action(&self) -> Box<dyn Action> {
        match self {
            SearchMode::Text => Box::new(ActivateTextMode),
            SearchMode::Regex => Box::new(ActivateRegexMode),
        }
    }

    pub(crate) fn border_right(&self) -> bool {
        match self {
            SearchMode::Regex => true,
            SearchMode::Text => true,
        }
    }

    pub(crate) fn border_left(&self) -> bool {
        match self {
            SearchMode::Text => true,
            _ => false,
        }
    }

    pub(crate) fn button_side(&self) -> Option<Side> {
        match self {
            SearchMode::Text => Some(Side::Left),
            SearchMode::Regex => Some(Side::Right),
        }
    }
}

pub(crate) fn next_mode(mode: &SearchMode) -> SearchMode {
    match mode {
        SearchMode::Text => SearchMode::Regex,
        SearchMode::Regex => SearchMode::Text,
    }
}