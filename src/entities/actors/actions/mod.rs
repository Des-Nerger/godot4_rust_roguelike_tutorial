use godot::prelude::*;

pub enum Action {
	Escape,
	Movement { offset: Vector2i },
}
