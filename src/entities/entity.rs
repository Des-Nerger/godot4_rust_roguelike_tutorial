use {
	super::{actors::actions::Action, entity_definition::EntityDefinition},
	crate::{
		nameof,
		utils::grid::{Vector2i_Ext, EPS_SQUARED},
	},
	godot::{
		classes::{AudioStream, Engine, Timer},
		prelude::*,
	},
};

#[rustfmt::skip]
#[derive(Clone, Copy)]
pub enum Mode {
	ListeningToInput,
	PerformingRequestedTurn { dest /*ination */: Vector2i, isActualPosChange: bool },
}

impl Drop for Entity {
	fn drop(&mut self) {
		if Engine::singleton().is_editor_hint() {
			self.soundDelay.queue_free();
			self.dirtStepSound1.queue_free();
			self.dirtStepSound2.queue_free();
			self.meleeSound.queue_free();
		}
	}
}

#[rustfmt::skip]
#[derive(GodotClass)]
#[class(init, base=AnimatedSprite2D)]
pub struct Entity {
	gridPos: Vector2i,
	tmp_worldPos: Vector2,
	direction: Vector2i,
	#[init(val = Mode::ListeningToInput)] pub mode: Mode,
	#[init(val = Timer::new_alloc())] soundDelay: Gd<Timer>,
	#[init(val = AudioStreamPlayer::new_alloc())] dirtStepSound1: Gd<AudioStreamPlayer>,
	#[init(val = AudioStreamPlayer::new_alloc())] dirtStepSound2: Gd<AudioStreamPlayer>,
	#[init(val = AudioStreamPlayer::new_alloc())] meleeSound: Gd<AudioStreamPlayer>,
	base: Base<<Self as GodotClass>::Base>,
}

#[godot_api]
impl Entity {
	pub fn nеw(startPos: Vector2i, def: Gd<EntityDefinition>) -> Gd<Self> {
		let mut o /*bject */ = Self::new_alloc();
		o.set_centered(false);
		{
			let (soundDelay, audioStreamPlayers) = {
				let on_soundDelay_timeout = o.callable(nameof!(Self::on_soundDelay_timeout));
				let mut o /*bject */ = o.bind_mut();
				o.soundDelay.set_one_shot(true);
				o.soundDelay.connect(c"timeout".into(), on_soundDelay_timeout);
				o.dirtStepSound1
					.set_stream(load::<AudioStream>("res://assets/audio/voxelibre/default_dirt_footstep.1.ogg"));
				o.dirtStepSound2
					.set_stream(load::<AudioStream>("res://assets/audio/voxelibre/default_dirt_footstep.2.ogg"));
				{
					const STEP_VOLUME_DB: f32 = -15.;
					o.dirtStepSound1.set_volume_db(STEP_VOLUME_DB);
					o.dirtStepSound2.set_volume_db(STEP_VOLUME_DB);
				}
				o.meleeSound.set_stream(load::<AudioStream>("res://assets/audio/flare/melee_attack.ogg"));
				o.set_gridPos(startPos);
				(o.soundDelay.clone(), [o.dirtStepSound1.clone(), o.dirtStepSound2.clone(), o.meleeSound.clone()])
			};
			o.add_child(soundDelay);
			for audioStreamPlayer in audioStreamPlayers {
				o.add_child(audioStreamPlayer);
			}
		}
		o.set_position(startPos.grid_to_world().cast_float());
		{
			let def = def.bind();
			o.set_offset(def.offset);
			o.set_sprite_frames(def.sprite_frames.clone());
			o.set_autoplay(def.autoplay.clone());
			o.set_material(def.material.clone());
		}
		o
	}

	#[func]
	fn set_gridPos(o /*bject */: &mut Self, gridPos: Vector2i) {
		o.base_mut().set_position(gridPos.grid_to_world().cast_float());
		o.gridPos = gridPos;
	}

	// fn mоvе(o /*bject */: &mut Self, offset: Vector2i) {
	// 	o.set_gridPos(o.gridPos + offset);
	// }

	#[func]
	fn on_soundDelay_timeout(o /*bject */: &mut Self) {
		match o.mode {
			Mode::PerformingRequestedTurn { isActualPosChange, .. } => {
				if isActualPosChange {
					o.dirtStepSound2.play()
				} else {
					o.meleeSound.play()
				}
			}
			_ => {}
		}
	}

	pub fn process_action(o /*bject */: &mut Self, action: Option<Action>) {
		use Action::*;
		match action {
			Some(ref action @ (Movement { direction } | Action::Strike { direction })) => {
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
				o.base_mut().set_animation([animPrefix, direction.to_direction_id()].concat().into());
				o.mode = Mode::PerformingRequestedTurn { dest: o.gridPos + 1 * direction, isActualPosChange };
				o.tmp_worldPos = o.gridPos.grid_to_world().cast_float();
				o.direction = direction;
			}
			Some(Action::Escape) => o.base().get_tree().unwrap().quit(),
			None => {}
		}
	}

	pub fn perform_turn(o /*bject */: &mut Self, delta: f64) {
		let Mode::PerformingRequestedTurn { dest /*ination */, isActualPosChange } = o.mode else {
			unreachable!();
		};
		const SPEED: real = 1.1189;
		o.tmp_worldPos += o.direction.grid_to_world().cast_float() * SPEED * (delta as real);
		let worldDest = dest.grid_to_world().cast_float();
		if o.tmp_worldPos.distance_squared_to(worldDest) < EPS_SQUARED {
			if isActualPosChange {
				o.gridPos = dest;
			}
			o.tmp_worldPos = worldDest;
			{
				let animName = StringName::from(["stand_", o.direction.to_direction_id()].concat());
				o.base_mut().set_animation(animName);
			}
			o.direction = Vector2i::ZERO;
			o.mode = Mode::ListeningToInput;
		}
		if isActualPosChange {
			let tmp_worldPos = o.tmp_worldPos;
			o.base_mut().set_position(tmp_worldPos);
		}
	}
}
