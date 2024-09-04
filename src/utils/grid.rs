use godot::prelude::*;

pub const TILE_SIZE: Vector2i = Vector2i::new(32 * 3, 16 * 3);
const TILE_SIZE_HALF: Vector2i = Vector2i::new(TILE_SIZE.x / 2, TILE_SIZE.y / 2);

/*
pub trait Vector2_Ext {
   fn world_to_grid(self) -> Self;
}
*/

pub trait Vector2i_Ext {
   fn grid_to_world(self) -> Self;
   fn to_direction_id(self) -> &'static str;
}

// In case needed, the derivation of the world_to_grid and grid_to_world's formulas can be found there:
// https://clintbellanger.net/articles/isometric_math/

/*
impl Vector2_Ext for Vector2 {
   fn world_to_grid(self) -> Self {
      let [tileWidth, tileHeight]: [real; 2] = TILE_SIZE.to_array().map(|coord| coord as _);
      Self::new(self.x / tileWidth + self.y / tileHeight, self.y / tileHeight - self.x / tileWidth)
   }
}
*/

impl Vector2i_Ext for Vector2i {
   fn grid_to_world(self) -> Self {
      Self::new((self.x - self.y) * TILE_SIZE_HALF.x, (self.x + self.y) * TILE_SIZE_HALF.y)
   }

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

const EPS: real = 1.4142;
pub const EPS_SQUARED: real = EPS * EPS;
