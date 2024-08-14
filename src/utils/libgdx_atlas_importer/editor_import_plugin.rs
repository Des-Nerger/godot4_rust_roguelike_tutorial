use {
	crate::{
		unlet,
		utils::{default, TryInto_Ext, UninitConst_Ext},
	},
	arrayvec::ArrayVec,
	core::{
		any::type_name,
		cmp::max,
		str::{self, FromStr},
	},
	godot::{
		classes::{
			AtlasTexture, FileAccess, IEditorImportPlugin, ResourceLoader, ResourceSaver, SpriteFrames, Texture2D,
		},
		global::Error,
		prelude::*,
	},
	strum::{EnumCount, FromRepr, IntoStaticStr},
};

#[derive(GodotClass)]
#[class(tool, init, base=EditorImportPlugin)]
pub struct GdxAtlasImporter {
	base: Base<<Self as GodotClass>::Base>,
}

const SAVE_EXTENSION: &str = "res";

#[godot_api]
impl IEditorImportPlugin for GdxAtlasImporter {
	fn get_importer_name(&self) -> GString {
		module_path!().into()
	}

	fn get_visible_name(&self) -> GString {
		type_name::<Self>().into()
	}

	fn get_recognized_extensions(&self) -> PackedStringArray {
		(&["atlas".into()]).into()
	}

	fn get_save_extension(&self) -> GString {
		SAVE_EXTENSION.into()
	}

	fn get_resource_type(&self) -> GString {
		"Resource".into()
	}

	fn get_preset_count(&self) -> i32 {
		(Preset::COUNT).strict_as()
	}

	fn get_preset_name(&self, presetIndex: i32) -> GString {
		Preset::from_repr(presetIndex).map_or("Unknown", |preset| preset.into()).into()
	}

	fn get_import_options(&self, _ /*path */: GString, _ /*index */: i32) -> Array<Dictionary> {
		(&[]).into()
	}

	fn get_option_visibility(
		&self,
		_ /*path */: GString,
		_ /*optionName */: StringName,
		_ /*options */: Dictionary,
	) -> bool {
		true
	}

	fn get_import_order(&self) -> i32 {
		200
	}

	fn get_priority(&self) -> f32 {
		1.0
	}

	fn import(
		&self,
		srcAtlasPath: GString,
		outpResStemPath: GString,
		_ /*options */: Dictionary,
		_ /*platformVariants */: Array<GString>,
		outpTresPaths: Array<GString>,
	) -> Error {
		struct Current<'a> {
			gotPngPath: bool,
			gotFrameName: bool,
			region: Rect2,
			margin: Rect2,
			atlasStemPath: &'a str,
			outpPathBuf: String,
			outpTresPaths: Array<GString>,
			resourceSaver: Gd<ResourceSaver>,
			resourceLoader: Gd<ResourceLoader>,
			imageTexture: Gd<Texture2D>,
			atlasTexture_type: GString,
			spriteFrames: Gd<SpriteFrames>,
			animName: StringName,
			spriteFrameIdx: i32,
		}
		const GEN_DIR_SUFFIX: &str = ".atlas_textures/";
		impl<'a> Current<'a> {
			fn new_frame(&mut self, frameName: &'a str) {
				let c /*urrent */ = self;
				c.outpPathBuf.truncate(c.atlasStemPath.len() + GEN_DIR_SUFFIX.len());
				c.outpPathBuf.extend([frameName, ".tres"]);
				c.gotFrameName = true;
				{
					let (animName, spriteFrameIdx) = frameName.rsplit_once("_").unwrap();
					c.spriteFrameIdx = spriteFrameIdx.parse().unwrap();
					c.animName = animName.into();
				}
				if !c.spriteFrames.has_animation(c.animName.clone()) {
					c.spriteFrames.add_animation(c.animName.clone());
					c.spriteFrames.set_animation_loop(c.animName.clone(), true);
					c.spriteFrames.set_animation_speed(c.animName.clone(), 25.);
				}
				[c.region, c.margin] = [Rect2::UNINIT; 2];
			}

			fn finish_frame(&mut self) {
				let c /*urrent */ = self;
				let tresPath = GString::from(&c.outpPathBuf);
				let mut atlTex =
					if c.resourceLoader.exists_ex(tresPath.clone()).type_hint(c.atlasTexture_type.clone()).done() {
						c.resourceLoader.load_ex(tresPath).type_hint(c.atlasTexture_type.clone()).done().unwrap().cast()
					} else {
						AtlasTexture::new_gd()
					};
				atlTex.set_atlas(c.imageTexture.clone());
				atlTex.set_region({
					assert_ne!(c.region, Rect2::UNINIT);
					c.region
				});
				atlTex.set_margin({
					assert_ne!(c.margin, Rect2::UNINIT);
					c.margin
				});
				/*
				c.outpTresPaths.push(tresPath.clone());
				assert_eq!(c.resourceSaver.save_ex(atlTex.clone()).path(tresPath.clone()).done(), Error::OK);
				atlTex.take_over_path(tresPath);
				*/
				assert_ne!(c.animName, default());
				for _ in 0..max(0, c.spriteFrameIdx + 1 - c.spriteFrames.get_frame_count(c.animName.clone())) {
					c.spriteFrames.add_frame(c.animName.clone(), Texture2D::new_gd());
				}
				c.spriteFrames.set_frame(c.animName.clone(), c.spriteFrameIdx, atlTex);
				c.gotFrameName = false;
			}
		}

		let srcAtlasContents = FileAccess::get_file_as_bytes(srcAtlasPath.clone());
		let srcAtlasContents = str::from_utf8(srcAtlasContents.as_slice()).unwrap();
		let srcAtlasPath = String::from(srcAtlasPath);
		const FILENAME_MAX: usize = 255;

		let srcDir = &srcAtlasPath[..=srcAtlasPath.rfind('/').unwrap()];
		let pngPathBuf = &mut String::with_capacity(srcDir.len() + FILENAME_MAX);
		pngPathBuf.push_str(srcDir);

		let mut resourceLoader = ResourceLoader::singleton();
		let atlasStemPath = srcAtlasPath.strip_suffix(".atlas").unwrap();
		let mut outpPathBuf = String::with_capacity(atlasStemPath.len() + GEN_DIR_SUFFIX.len() + FILENAME_MAX);
		outpPathBuf.extend([atlasStemPath, ".sprite_frames.tres"]);
		let spriteFramesTresPath = GString::from(&outpPathBuf);
		let spriteFrames = {
			let spriteFrames_type = GString::from("SpriteFrames");
			if resourceLoader.exists_ex(spriteFramesTresPath.clone()).type_hint(spriteFrames_type.clone()).done() {
				resourceLoader
					.load_ex(spriteFramesTresPath.clone())
					.type_hint(spriteFrames_type)
					.done()
					.unwrap()
					.cast()
			} else {
				SpriteFrames::new_gd()
			}
		};
		outpPathBuf.truncate(atlasStemPath.len());
		outpPathBuf.push_str(GEN_DIR_SUFFIX);
		/*
		{
			let genDir = GString::from(&outpPathBuf);
			if !DirAccess::dir_exists_absolute(genDir.clone()) {
				assert_eq!(DirAccess::make_dir_absolute(genDir), Error::OK);
			}
		}
		*/
		let mut c /*urrent */ = Current {
			gotPngPath: false,
			gotFrameName: false,
			region: Rect2::UNINIT,
			margin: Rect2::UNINIT,
			atlasStemPath,
			outpPathBuf,
			outpTresPaths,
			resourceSaver: ResourceSaver::singleton(),
			resourceLoader,
			imageTexture: Texture2D::new_gd(),
			atlasTexture_type: GString::from("AtlasTexture"),
			spriteFrames,
			animName: default(),
			spriteFrameIdx: i32::MIN,
		};
		unlet!(atlasStemPath);

		let imageTexture_type = GString::from("ImageTexture");
		for line in srcAtlasContents.lines() {
			if !c.gotPngPath {
				pngPathBuf.truncate(srcDir.len());
				pngPathBuf.push_str(line);
				c.gotPngPath = true;
				c.imageTexture = c
					.resourceLoader
					.load_ex(pngPathBuf[..].into())
					.type_hint(imageTexture_type.clone())
					.done()
					.unwrap()
					.cast::<Texture2D>();
				continue;
			}
			if !c.gotFrameName {
				if &line[..1] != "\t" {
					c.new_frame(line);
				}
				continue;
			}
			if line == "" {
				c.finish_frame();
				c.gotPngPath = false;
				continue;
			}
			if &line[..1] == "\t" {
				let (key, value) = line[1..].split_once(": ").unwrap();
				let [x, y, w /*idth */, h /*eight */] = value
					.split(", ")
					.map(|s /*tr */| f32::from_str(s).unwrap())
					.collect::<ArrayVec<_, 4>>()
					.into_inner()
					.unwrap();
				match key {
					"bounds" => {
						c.region = Rect2::from_components(x, y, w, h);
					}
					"offsets" => {
						c.margin =
							Rect2::from_components(x, h - y - c.region.size.y, w - c.region.size.x, h - c.region.size.y);
					}
					_ => unreachable!(),
				}
				continue;
			}
			c.finish_frame();
			c.new_frame(line);
		}
		assert_eq!(c.gotFrameName, true);
		c.finish_frame();

		assert_eq!(c.resourceSaver.save_ex(c.spriteFrames).path(spriteFramesTresPath.clone()).done(), Error::OK);
		godot_print!("{spriteFramesTresPath:?} was updated.");
		c.outpTresPaths.push(spriteFramesTresPath);

		c.outpPathBuf.clear();
		c.outpPathBuf.extend([&String::from(outpResStemPath), ".", SAVE_EXTENSION]);
		assert_eq!(c.resourceSaver.save_ex(Resource::new_gd()).path(c.outpPathBuf.into()).done(), Error::OK);
		Error::OK
	}
}

#[derive(EnumCount, FromRepr, IntoStaticStr)]
#[repr(i32)]
enum Preset {
	Default,
}
