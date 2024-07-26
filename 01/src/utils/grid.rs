use godot::builtin::Vector2i;

const TILE_SIZE: Vector2i = Vector2i::splat(16);

pub trait ConvBetween_Grid_World {
	fn grid_to_world(self) -> Self;
	// fn world_to_grid(self) -> Self;
}

impl ConvBetween_Grid_World for Vector2i {
	fn grid_to_world(self) -> Self {
		self * TILE_SIZE
	}
	/*
	fn world_to_grid(self) -> Self {
		self / TILE_SIZE
	}
	*/
}
