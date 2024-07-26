use godot::builtin::Vector2i;

pub enum Action {
	Escape,
	Movement { offset: Vector2i },
}
