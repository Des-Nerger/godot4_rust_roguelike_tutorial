#![warn(clippy::pedantic, elided_lifetimes_in_paths, explicit_outlives_requirements)]
#![allow(
	confusable_idents,
	mixed_script_confusables,
	non_camel_case_types,
	non_snake_case,
	uncommon_codepoints,
	unstable_name_collisions
)]

use godot::prelude::*;

struct RoguelikeTutorial;

#[gdextension]
unsafe impl ExtensionLibrary for RoguelikeTutorial {}

mod entities;
pub mod game;
mod utils;
