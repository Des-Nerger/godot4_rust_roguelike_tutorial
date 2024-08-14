mod event_handler;

use {
	crate::{entities::actors::actions::Action, game::event_handler::EventHandler, utils::grid::Vector2i_Ext},
	godot::{
		classes::{AnimatedSprite2D, INode2D},
		prelude::*,
	},
};

#[rustfmt::skip]
#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Game {
	#[init(val = Vector2i::ZERO)] playerGridPos: Vector2i,
	#[init(node = "player")] player: OnReady<Gd<AnimatedSprite2D>>,
	#[init(node = "eventHandler")] eventHandler: OnReady<Gd<EventHandler>>,
	#[init(val = Mode::ListeningToInput)] mode: Mode,
	base: Base<<Self as GodotClass>::Base>,
}

enum Mode {
	ListeningToInput,
	_PerformingRequestedTurn,
}

#[godot_api]
impl INode2D for Game {
	fn process(&mut self, _ /*delta */: f64) {
		let o /*bject */ = self;
		match o.mode {
			Mode::ListeningToInput => {
				match o.eventHandler.bind().get_action() {
					Some(Action::Movement { offset }) => {
						o.playerGridPos += offset;
						o.player.set_position(o.playerGridPos.grid_to_world().cast_float());
						o.player.set_animation(["walk_", offset.to_direction_id()].concat().into());
						// mode = Mode::PerformingRequestedTurn;
					}
					Some(Action::Escape) => o.base().get_tree().unwrap().quit(),
					None => {}
				}
			}
			Mode::_PerformingRequestedTurn => {}
		}
	}
}
