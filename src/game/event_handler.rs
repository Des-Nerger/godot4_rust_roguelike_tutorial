use {
	crate::entities::actors::actions::Action,
	godot::{
		classes::{INode, Input},
		prelude::*,
	},
};

#[rustfmt::skip]
#[derive(GodotClass)]
#[class(base=Node)]
pub struct EventHandler {
	input: Gd<Input>,
	base: Base<<Self as GodotClass>::Base>,
}

#[godot_api]
impl INode for EventHandler {
	fn init(base: Base<Self::Base>) -> Self {
		Self { input: Input::singleton(), base }
	}
}

#[godot_api]
impl EventHandler {
	pub fn get_action(&self) -> Option<Action> {
		let o /*bject */ = self;
		if o.input.is_action_just_pressed(c"ui_cancel".into()) {
			return Some(Action::Escape);
		}
		Some(Action::Movement {
			offset: Vector2i::from_array(
				match [
					if o.input.is_action_just_pressed(c"ui_left".into()) {
						-1
					} else if o.input.is_action_just_pressed(c"ui_right".into()) {
						1
					} else {
						0
					},
					if o.input.is_action_just_pressed(c"ui_up".into()) {
						-1
					} else if o.input.is_action_just_pressed(c"ui_down".into()) {
						1
					} else {
						0
					},
				] {
					[0, 0] => return None,
					[1, 0] => [1, -1],
					[-1, 0] => [-1, 1],
					[0, 1] => [1, 1],
					[0, -1] => [-1, -1],

					[1, 1] => [1, 0],
					[1, -1] => [0, -1],
					[-1, 1] => [0, 1],
					[-1, -1] => [-1, 0],
					others => others,
				},
			),
		})
	}
}
