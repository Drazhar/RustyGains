#![doc = include_str!("../README.md")]

/// The main state of the app
pub mod app;

/// Basic application settings. Later this should also parse a config file.
mod settings;

/// Event generation
pub mod event;

/// Handling of the events
pub mod handler;

/// Basic setup of the tui
pub mod tui;

/// Everything related to the UI
pub mod ui;

/// All handling of the data backend
pub mod data;

/// Generates the heatmap for the overview -> Should be moved to the ui later!
pub mod heatmap;
