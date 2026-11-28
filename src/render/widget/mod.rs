//! # widget
//! This module holds the logic for widgets and their layouts.

use super::Style;

/// # Widget
/// One UI component. Holds a layout and a style. Widgets can be nested.
/// See `Layout` for rendering.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Widget {
    styling: Style,
    layout: Layout,
}

impl Widget {
    /// Create a new Widget with the given Layout
    pub fn new(layout: Layout) -> Self {
        Self {
            styling: Default::default(),
            layout,
        }
    }

    /// Get a mutable reference to the widget's style object
    pub fn style(&mut self) -> &mut Style {
        &mut self.styling
    }
}

/// # Layout
/// Controls how a Widget renders, and contains the 3 main types of Widgets.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Layout {
    /// The simplest layout. Just a piece of text, with styling provided by the Widget object.
    Label(String),

    /// A compositing layout that holds an unknown number of children and renders into a column.
    Column(Vec<Widget>),

    /// A compositing layout that holds an unknown number of children and renders into a row.
    Row(Vec<Widget>),
}

impl Layout {
    /// Create a new Column Layout
    pub fn new_column() -> Self {
        Self::Column(Vec::new())
    }

    /// Create a new Row Layout
    pub fn new_row() -> Self {
        Self::Row(Vec::new())
    }
}
