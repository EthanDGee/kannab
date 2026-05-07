//! The messaging and update layer of the application.
//!
//! This module implements the "Update" part of the Elm architecture (Model-View-Update).
//! It defines the set of all possible `Action`s and provides handlers for updating
//! the `AppState` in response to those actions.

pub mod action;
mod board_actions;
mod column_actions;
mod help_actions;
mod io_actions;
mod modal_actions;
mod navigation_actions;
mod picker_actions;
mod task_actions;
pub mod update;
