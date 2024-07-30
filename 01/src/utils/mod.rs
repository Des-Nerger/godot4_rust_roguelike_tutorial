pub mod grid;

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

#[macro_export]
macro_rules! init_onReadies {
	($o /*bject */: ident, $base: expr $(, $onReady: ident )+ $(,)? ) => {
		let base = $base;
		$(
			let $onReady = base.get_node_as(nameof!($o.$onReady));
		)+
		$(
			$o.$onReady.init($onReady);
		)+
	};
}
