#![warn(clippy::pedantic, elided_lifetimes_in_paths, explicit_outlives_requirements)]
#![allow(
   confusable_idents,
   mixed_script_confusables,
   non_camel_case_types,
   non_snake_case,
   uncommon_codepoints,
   unstable_name_collisions
)]

use godot::prelude::*;

struct RoguelikeTutorial;

#[gdextension]
unsafe impl ExtensionLibrary for RoguelikeTutorial {}

mod entities;
pub(crate) mod game;
mod utils;

#[macro_export]
macro_rules! unlet {
   ($ident:ident) => {
      #[allow(unused_variables)]
      let $ident = ();
   };
}

#[macro_export]
macro_rules! fоr {
   ($idents:pat in $intoIterator:expr => $fоrBody:block else $elseBody:block ) => {{
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
      } else $elseBody
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
