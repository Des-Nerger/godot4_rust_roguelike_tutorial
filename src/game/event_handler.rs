use {
	crate::entities::actors::actions::Action,
	godot::{
		classes::{INode, Input},
		global::Key,
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
	pub fn get_action(o /*bject */: &Self) -> Option<Action> {
		if o.input.is_action_just_pressed(c"ui_cancel".into()) {
			return Some(Action::Escape);
		}
		let direction = Vector2i::from_array(
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
				_ => unreachable!(),
			},
		);
		Some(if o.input.is_physical_key_pressed(Key::SHIFT) {
			Action::Strike { direction }
		} else {
			Action::Movement { direction }
		})
	}
}
