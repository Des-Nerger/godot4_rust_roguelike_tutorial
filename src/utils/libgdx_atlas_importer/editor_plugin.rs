use {
	super::editor_import_plugin::GdxAtlasImporter,
	godot::{classes::IEditorPlugin, prelude::*},
};

#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base=EditorPlugin)]
struct GdxAtlasPlugin {
	importPlugin: Option<Gd<GdxAtlasImporter>>,
	base: Base<<Self as GodotClass>::Base>,
}

#[godot_api]
impl IEditorPlugin for GdxAtlasPlugin {
	fn enter_tree(&mut self) {
		let o /*bject */ = self;
		let importPlugin = GdxAtlasImporter::new_gd();
		o.base_mut().add_import_plugin(importPlugin.clone());
		o.importPlugin = Some(importPlugin);
	}

	fn exit_tree(&mut self) {
		let o = self;
		let importPlugin = o.importPlugin.take().unwrap();
		o.base_mut().remove_import_plugin(importPlugin);
	}
}
