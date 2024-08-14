#![warn(clippy::pedantic, elided_lifetimes_in_paths, explicit_outlives_requirements)]
#![allow(non_camel_case_types, non_snake_case, unstable_name_collisions)]

use godot::prelude::*;

struct RoguelikeTutorial;

#[gdextension]
unsafe impl ExtensionLibrary for RoguelikeTutorial {}

mod entities;
mod game;
mod utils;
