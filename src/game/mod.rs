pub mod event_handler;

use {
	crate::{
		entities::{
			entity::{Mode, *},
			entity_definition::*,
		},
		game::event_handler::EventHandler,
	},
	chrono::Utc,
	godot::{
		classes::{Camera2D, INode2D, Shader, ShaderMaterial},
		prelude::*,
	},
};

#[rustfmt::skip]
#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Game {
	#[init(val = load("res://assets/definitions/entities/actors/entity_definition_player.tres"))]
		playerDefinition: Gd<EntityDefinition>,
	#[init(val = OnReady::manual())] player: OnReady<Gd<Entity>>,
	#[init(node = "eventHandler")] eventHandler: OnReady<Gd<EventHandler>>,
	#[init(node = "%entities")] entities: OnReady<Gd<Node2D>>,
	base: Base<<Self as GodotClass>::Base>,
}

#[godot_api]
impl INode2D for Game {
	fn ready(o /*object */: &mut Self) {
		o.base_mut().get_window().unwrap().set_title(
			format!("{} v{} | {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), Utc::now().format("%v"))
				.into(),
		);
		let mut npc = {
			let playerStartPos = Vector2i::new(9, 2);
			o.player.init(Entity::nеw(playerStartPos, o.playerDefinition.clone()));
			o.player.add_child(Camera2D::new_alloc());
			o.entities.add_child(o.player.clone());
			Entity::nеw(playerStartPos + Vector2i::new(1, -1), o.playerDefinition.clone())
		};
		{
			let mut shaderMaterial = ShaderMaterial::new_gd();
			{
				let mut shader = Shader::new_gd();
				shader.set_code(
					r#"
						shader_type canvas_item;
						void fragment() { COLOR.rgb = COLOR.grb; }
					"#
					.into(),
				);
				shaderMaterial.set_shader(shader);
			}
			npc.set_material(shaderMaterial);
		}
		o.entities.add_child(npc);
	}

	fn process(o /*bject */: &mut Self, delta: f64) {
		let mode = o.player.bind().mode;
		match mode {
			Mode::ListeningToInput => o.player.bind_mut().process_action(o.eventHandler.bind().get_action()),
			Mode::PerformingRequestedTurn { .. } => o.player.bind_mut().perform_turn(delta),
		}
	}
}
