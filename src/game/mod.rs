pub mod event_handler;

use {
	crate::{
		entities::actors::actions::Action,
		game::event_handler::EventHandler,
		unlet,
		utils::grid::{Vector2_Ext as _, Vector2i_Ext as _, EPS_SQUARED},
	},
	chrono::Utc,
	godot::{
		classes::{AnimatedSprite2D, INode2D, Timer},
		prelude::*,
	},
};

#[rustfmt::skip]
#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Game {
	#[init(val = Vector2i::ZERO)] playerGridPos: Vector2i,
	#[init(val = Default::default())] playerWorldPos: Vector2,
	#[init(val = Vector2i::ZERO)] direction: Vector2i,
	#[init(node = "wallTiles/player")] player: OnReady<Gd<AnimatedSprite2D>>,
	#[init(node = "eventHandler")] eventHandler: OnReady<Gd<EventHandler>>,
	#[init(node = "dirtStepSound1")] dirtStepSound1: OnReady<Gd<AudioStreamPlayer>>,
	#[init(node = "dirtStepSound2")] dirtStepSound2: OnReady<Gd<AudioStreamPlayer>>,
	#[init(node = "meleeSound")] meleeSound: OnReady<Gd<AudioStreamPlayer>>,
	#[init(node = "soundDelay")] soundDelay: OnReady<Gd<Timer>>,
	#[init(val = Mode::ListeningToInput)] mode: Mode,
	base: Base<<Self as GodotClass>::Base>,
}

enum Mode {
	ListeningToInput,
	PerformingRequestedTurn { dest: Vector2i, isActualPosChange: bool },
}

#[godot_api]
impl INode2D for Game {
	fn ready(o /*object */: &mut Self) {
		o.base_mut().get_window().unwrap().set_title(
			format!("{} v{} | {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), Utc::now().format("%v"),)
				.into(),
		);
		o.playerGridPos = o.player.get_position().world_to_grid().round().cast_int();
		o.player.set_position(o.playerGridPos.grid_to_world().cast_float());
	}

	fn process(o /*bject */: &mut Self, delta: f64) {
		use Action::*;
		match o.mode {
			Mode::ListeningToInput => match o.eventHandler.bind().get_action() {
				Some(ref action @ (Movement { direction } | Action::Strike { direction })) => {
					o.direction = direction;
					unlet!(direction);
					let (animPrefix, isActualPosChange) = match action {
						Movement { .. } => {
							o.dirtStepSound1.play();
							o.soundDelay.set_wait_time(0.50);
							o.soundDelay.start();
							("walk_", true)
						}
						Strike { .. } => {
							o.soundDelay.set_wait_time(0.33);
							o.soundDelay.start();
							("strike_", false)
						}
						_ => unreachable!(),
					};
					o.player.set_animation([animPrefix, o.direction.to_direction_id()].concat().into());
					o.mode =
						Mode::PerformingRequestedTurn { dest: o.playerGridPos + 1 * o.direction, isActualPosChange };
					o.playerWorldPos = o.playerGridPos.grid_to_world().cast_float();
				}
				Some(Action::Escape) => o.base().get_tree().unwrap().quit(),
				None => {}
			},
			Mode::PerformingRequestedTurn { dest, isActualPosChange } => {
				const SPEED: real = 1.1189;
				o.playerWorldPos += o.direction.grid_to_world().cast_float() * SPEED * (delta as real);
				let worldDest = dest.grid_to_world().cast_float();
				if o.playerWorldPos.distance_squared_to(worldDest) < EPS_SQUARED {
					if isActualPosChange {
						o.playerGridPos = dest;
					}
					o.playerWorldPos = worldDest;
					o.player.set_animation(["stand_", o.direction.to_direction_id()].concat().into());
					o.direction = Vector2i::ZERO;
					o.mode = Mode::ListeningToInput;
				}
				if isActualPosChange {
					o.player.set_position(o.playerWorldPos);
				}
			}
		}
	}
}

#[godot_api]
impl Game {
	#[func]
	fn on_soundDelay_timeout(o /*bject */: &mut Self) {
		match o.mode {
			Mode::PerformingRequestedTurn { isActualPosChange, .. } => {
				if isActualPosChange {
					o.dirtStepSound2.play();
				} else {
					o.meleeSound.play();
				}
			}
			_ => {}
		}
	}
}
