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
			[ 1, -1] => "00",
			[ 1,  1] => "01",
			[-1,  1] => "02",
			[-1, -1] => "03",
			[ 0, -1] => "04",
			[ 1,  0] => "05",
			[ 0,  1] => "06",
			[-1,  0] => "07",
			_ => unreachable!(),
		}
	}
}
