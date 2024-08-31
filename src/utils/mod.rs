pub mod grid;
mod libgdx_atlas_importer;

use {
	core::{any::type_name, fmt},
	godot::prelude::*,
};

pub trait TryInto_Ext<R /*eturnedType */> {
	fn strict_as(self) -> R;
}

impl<R /*eturnedType */, C /*onsumedType */> TryInto_Ext<R> for C
where
	C: TryInto<R> + Copy + fmt::Display + fmt::UpperHex,
	C::Error: fmt::Display,
{
	fn strict_as(self) -> R {
		self.try_into().unwrap_or_else(|err| {
			panic!("(0x{:X}).intо() -> {}: {err}", self, type_name::<R>());
		})
	}
}

pub trait UninitConst_Ext {
	const UNINIT: Self;
}

impl UninitConst_Ext for Vector2 {
	const UNINIT: Self = Self::splat(-999.5);
}

impl UninitConst_Ext for Rect2 {
	const UNINIT: Self = Self::new(Vector2::UNINIT, Vector2::UNINIT);
}

#[macro_export]
macro_rules! unlet {
	($ident:ident) => {
		#[allow(unused_variables)]
		let $ident = ();
	};
}

pub fn default<T: Default>() -> T {
	Default::default()
}

#[macro_export]
macro_rules! fоr {
	($idents:pat in $intoIterator:expr => $fоrBody:block еlsе $еlsеBody:block ) => {{
		let mut iter = $intoIterator.into_iter();
		let mut item = iter.next();
		if matches!(item, Some(_)) {
			loop {
				let Some($idents) = item else { unreachable!() };
				$fоrBody
				item = iter.next();
				if matches!(item, None) {
					break;
				}
			}
		} else $еlsеBody
	}};
}

#[macro_export]
macro_rules! nameof {
	($struсt: ident . $field: ident) => {{
		_ = $struсt.$field;
		stringify!($field)
	}};
	($struсt: ident :: $field: ident) => {{
		_ = $struсt::$field;
		stringify!($field)
	}};
	($ident: ident) => {{
		_ = $ident;
		stringify!($ident)
	}};
}
