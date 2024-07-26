mod event_handler;

use {
	crate::{
		entities::actors::actions::Action, game::event_handler::EventHandler, init_onReadies, nameof,
		utils::grid::ConvBetween_Grid_World,
	},
	godot::{
		classes::{INode2D, Sprite2D},
		prelude::*,
	},
};

#[rustfmt::skip]
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Game {
	playerGridPos: Vector2i,
	player: OnReady<Gd<Sprite2D>>,
	eventHandler: OnReady<Gd<EventHandler>>,
	base: Base<<Self as GodotClass>::Base>,
}

#[godot_api]
impl INode2D for Game {
	fn init(base: Base<Self::Base>) -> Self {
		Self { playerGridPos: Vector2i::ZERO, player: OnReady::manual(), eventHandler: OnReady::manual(), base }
	}
	fn ready(&mut self) {
		let o /*bject */ = self;
		init_onReadies!(o, o.base(), player, eventHandler);
	}
	fn process(&mut self, _ /*delta */: f64) {
		let o /*bject */ = self;
		match o.eventHandler.bind().get_action() {
			Some(Action::Movement { offset }) => {
				o.playerGridPos += offset;
				o.player.set_position(Vector2::from_vector2i(o.playerGridPos.grid_to_world()));
			}
			Some(Action::Escape) => o.base().get_tree().unwrap().quit(),
			None => {}
		}
	}
}
