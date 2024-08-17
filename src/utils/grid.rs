use godot::prelude::*;

const TILE_SIZE: Vector2i = Vector2i::new(32 * 3, 16 * 3);

pub trait Vector2i_Ext {
	fn grid_to_world(self) -> Self;
	// fn world_to_grid(self) -> Self;
	fn to_direction_id(self) -> &'static str;
}

impl Vector2i_Ext for Vector2i {
	fn grid_to_world(self) -> Self {
		Self::new((self.x - self.y) * TILE_SIZE.x / 2, (self.x + self.y) * TILE_SIZE.y / 2)
	}
	/*
	fn world_to_grid(self) -> Self {
		self / TILE_SIZE
	}
	*/
	#[rustfmt::skip]
	fn to_direction_id(self) -> &'static str {
		match self.to_array() {
			[ 0, -1] => "00",
			[ 1,  0] => "01",
			[ 0,  1] => "02",
			[-1,  0] => "03",
			[-1, -1] => "04",
			[ 1, -1] => "05",
			[ 1,  1] => "06",
			[-1,  1] => "07",
			_ => unreachable!(),
		}
	}
}
