use godot::prelude::*;

pub enum Action {
   Escape,
   Movement { direction: Vector2i },
   Strike { direction: Vector2i },
}
