//! A [`MenuBar`] widget for displaying [`MenuTree`]s
//!
//! *This API requires the following crate features to be activated: `menu`*
//!
//! # Example
//!
//! ```ignore
//! use iced::widget::button;
//! use iced_aw::menu::{MenuTree, MenuBar};
//!
//! let sub_2 = MenuTree::with_children(
//!     button("Sub Menu 2"),
//!     vec![
//!         MenuTree::new(button("item_1")),
//!         MenuTree::new(button("item_2")),
//!         MenuTree::new(button("item_3")),
//!     ]
//! );
//!
//! let sub_1 = MenuTree::with_children(
//!     button("Sub Menu 1"),
//!     vec![
//!         MenuTree::new(button("item_1")),
//!         sub_2,
//!         MenuTree::new(button("item_2")),
//!         MenuTree::new(button("item_3")),
//!     ]
//! );
//!
//!
//! let root_1 = MenuTree::with_children(
//!     button("Menu 1"),
//!     vec![
//!         MenuTree::new(button("item_1")),
//!         MenuTree::new(button("item_2")),
//!         sub_1,
//!         MenuTree::new(button("item_3")),
//!     ]
//! );
//!
//! let root_2 = MenuTree::with_children(
//!     button("Menu 2"),
//!     vec![
//!         MenuTree::new(button("item_1")),
//!         MenuTree::new(button("item_2")),
//!         MenuTree::new(button("item_3")),
//!     ]
//! );
//!
//! let menu_bar = MenuBar::new(vec![root_1, root_2]);
//!
//! ```
//!

mod flex;
mod menu_bar;
mod menu_bar_overlay;
mod menu_tree;
mod types;

pub use crate::style::menu_bar::{Appearance, StyleSheet};
pub use menu_bar::MenuBar;
pub use menu_tree::{Item, Menu};
pub use types::*;

// A `MenuBar` collects `MenuTree`s and handles
// pub type MenuBar<'a, Message, Renderer> = menu_bar::MenuBar<'a, Message, Renderer>;
// pub use types::*;
// Nested menu is essentially a tree of items, a menu is a collection of items
// pub type MenuTree<'a, Message, Renderer> = menu_tree::MenuTree<'a, Message, Renderer>;
// pub use menux::Menux;
