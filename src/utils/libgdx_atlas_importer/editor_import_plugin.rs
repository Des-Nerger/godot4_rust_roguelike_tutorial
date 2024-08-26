use {
	crate::{
		fоr, unlet,
		utils::{default, TryInto_Ext as _, UninitConst_Ext as _},
	},
	arrayvec::ArrayVec,
	core::{
		any::type_name,
		cmp::max,
		str::{self, FromStr as _},
	},
	godot::{
		classes::{
			AtlasTexture, FileAccess, IEditorImportPlugin, ResourceLoader, ResourceSaver, SpriteFrames, Texture2D,
		},
		global::Error,
		prelude::*,
	},
	std::{collections::HashMap, fmt::Write as _},
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
		destResStemPath: GString,
		_ /*options */: Dictionary,
		_ /*platformVariants */: Array<GString>,
		destTresPaths: Array<GString>,
	) -> Error {
		struct Current {
			gotPngPath: bool,
			gotFrameName: bool,
			region: Rect2,
			margin: Rect2,
			pathBuf: String,
			destTresPaths: Array<GString>,
			resourceSaver: Gd<ResourceSaver>,
			resourceLoader: Gd<ResourceLoader>,
			imageTexture: Gd<Texture2D>,
			spriteFrames: Gd<SpriteFrames>,
			animName: StringName,
			spriteFrameIdx: i32,
			maxFrameIndices: HashMap<StringName, i32>,
		}
		impl Current {
			fn new_frame(&mut self, frameName: &str) {
				let c /*urrent */ = self;
				c.gotFrameName = true;
				{
					let (animName, spriteFrameIdx) = frameName.rsplit_once("_").unwrap();
					c.animName = animName.into();
					c.spriteFrameIdx = spriteFrameIdx.parse().unwrap();
					_ = c
						.maxFrameIndices
						.entry(c.animName.clone())
						.and_modify(|maxFrameIdx| *maxFrameIdx = max(*maxFrameIdx, c.spriteFrameIdx))
						.or_insert(c.spriteFrameIdx);
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
				assert_ne!(c.animName, default());
				for _ in 0..max(0, c.spriteFrameIdx + 1 - c.spriteFrames.get_frame_count(c.animName.clone())) {
					c.spriteFrames.add_frame(c.animName.clone(), AtlasTexture::new_gd());
				}
				let mut atlTex = c
					.spriteFrames
					.get_frame_texture(c.animName.clone(), c.spriteFrameIdx)
					.unwrap()
					.cast::<AtlasTexture>();
				atlTex.set_atlas(c.imageTexture.clone());
				atlTex.set_region({
					assert_ne!(c.region, Rect2::UNINIT);
					c.region
				});
				atlTex.set_margin({
					assert_ne!(c.margin, Rect2::UNINIT);
					c.margin
				});
				c.spriteFrames.set_frame(c.animName.clone(), c.spriteFrameIdx, atlTex);
				c.gotFrameName = false;
			}
		}

		let srcAtlasContents = FileAccess::get_file_as_bytes(srcAtlasPath.clone());
		let srcAtlasContents = str::from_utf8(srcAtlasContents.as_slice()).unwrap();
		let mut pathBuf = String::with_capacity(srcAtlasPath.len() + FILENAME_MAX);
		write!(&mut pathBuf, "{srcAtlasPath}").unwrap();
		unlet!(srcAtlasPath);
		let [atlasStemPath_len, srcDir_len] = {
			let atlasStemPath = pathBuf.strip_suffix(".atlas").unwrap();
			[atlasStemPath.len(), atlasStemPath.rfind('/').unwrap()]
		};
		const FILENAME_MAX: usize = 255;

		let mut resourceLoader = ResourceLoader::singleton();
		pathBuf.truncate(atlasStemPath_len);
		pathBuf.push_str(".sprite_frames.tres");
		let spriteFramesTresPath = GString::from(&pathBuf);
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
		let mut c /*urrent */ = Current {
			gotPngPath: false,
			gotFrameName: false,
			region: Rect2::UNINIT,
			margin: Rect2::UNINIT,
			pathBuf,
			destTresPaths,
			resourceSaver: ResourceSaver::singleton(),
			resourceLoader,
			imageTexture: Texture2D::new_gd(),
			spriteFrames,
			animName: default(),
			spriteFrameIdx: i32::MIN,
			maxFrameIndices: HashMap::new(),
		};

		let imageTexture_type = GString::from("ImageTexture");
		for line in srcAtlasContents.lines() {
			if !c.gotPngPath {
				c.pathBuf.truncate(srcDir_len + "/".len());
				c.pathBuf.push_str(line);
				c.gotPngPath = true;
				c.imageTexture = c
					.resourceLoader
					.load_ex(c.pathBuf[..].into())
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

		for animName in
			c.spriteFrames.get_animation_names().as_slice().into_iter().map(|gstring| StringName::from(gstring))
		{
			if !c.maxFrameIndices.contains_key(&animName) {
				c.spriteFrames.remove_animation(animName.clone());
				godot_print!("removed unused animation \"{animName}\"");
			}
		}
		'outer: for (animName, maxFrameIdx) in c.maxFrameIndices.into_iter() {
			let frameRange = (maxFrameIdx + 1)..c.spriteFrames.get_frame_count(animName.clone());
			fоr!(frameIdx in frameRange.clone().rev() => {
				c.spriteFrames.remove_frame(animName.clone(), frameIdx);
			} еlsе {
				continue 'outer;
			});
			godot_print!("in animation \"{animName}\": removed unused trailing frames {frameRange:?}");
		}

		assert_eq!(c.resourceSaver.save_ex(c.spriteFrames).path(spriteFramesTresPath.clone()).done(), Error::OK);
		godot_print!("{spriteFramesTresPath:?} was updated.");
		c.destTresPaths.push(spriteFramesTresPath);

		c.pathBuf.clear();
		c.pathBuf.extend([&String::from(destResStemPath), ".", SAVE_EXTENSION]);
		assert_eq!(c.resourceSaver.save_ex(Resource::new_gd()).path(c.pathBuf.into()).done(), Error::OK);
		Error::OK
	}
}

#[derive(EnumCount, FromRepr, IntoStaticStr)]
#[repr(i32)]
enum Preset {
	Default,
}
