//! The data layer of the application.
//!
//! This module contains the state representations for the application, boards, and modals.
//! It follows a hierarchical structure: `AppState` contains the top-level view and session state,
//! while `board_state` defines the persistent Kanban data.

pub mod app_state;
pub mod board_state;
pub mod modal_state;
