use godot::{
   classes::{Material, SpriteFrames},
   prelude::*,
};

#[rustfmt::skip]
#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct EntityDefinition {
   #[export] pub offset: Vector2,
   #[export] pub sprite_frames: Option<Gd<SpriteFrames>>,
   #[export] pub autoplay: GString,
   #[export] pub material: Option<Gd<Material>>,
   base: Base<<Self as GodotClass>::Base>,
}
