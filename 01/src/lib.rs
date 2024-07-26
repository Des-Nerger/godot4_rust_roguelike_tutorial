#![warn(clippy::pedantic, elided_lifetimes_in_paths, explicit_outlives_requirements)]
#![allow(
	confusable_idents,
	mixed_script_confusables,
	non_camel_case_types,
	non_snake_case,
	uncommon_codepoints
)]

use godot::prelude::*;

struct Godot4_RustRoguelikeTutorial;

#[gdextension]
unsafe impl ExtensionLibrary for Godot4_RustRoguelikeTutorial {}

mod entities;
mod game;
mod utils;
