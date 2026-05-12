//! # Just Nvim
//! 
//! ## Example 
//!
//! ```rust, ignore
//! use just_nvim::nvim_set_hl;
//! use nvim_oxi::api::Error;
//! use nvim_oxi::api::opts::SetHighlightOpts;
//! 
//! #[test]
//! #[ignore = "test won't work as this is a `cdylib`"]
//! fn it_works() -> Result<(), Error> {
//!     // without macro
//!     let opts = SetHighlightOpts::builder()
//!         .bold(true)
//!         .foreground("#FFFFFF")
//!         .build();
//!     nvim_oxi::api::set_hl(0, "Comment", &opts)?;
//! 
//!     // with macro
//!     // full example
//!     nvim_set_hl!(0, "Comment", {
//!         bold: true,
//!         foreground: "FFFFFF",
//!         undercurl: true,
//!         altfont: true,
//!         background: "000000",
//!         bg_indexed: true,
//!         blend: 5,
//!         cterm: "some str",
//!         ctermbg: "some str",
//!         ctermfg: "some str",
//!         fallback: true,
//!         fg_indexed: true,
//!         force: true,
//!         italic: true,
//!         link: "any type which satisfy HlGroup Trait",
//!         nocombine: true,
//!         reverse: true,
//!         special: "some str",
//!         standout: true,
//!         strikethrough: true,
//!         undercurl: true,
//!         underdashed: true,
//!         underdotted: true,
//!         underdouble: true,
//!         underline: true,
//!     })?;
//! 
//!     Ok(())
//! }
//! ```
//!

#[macro_export]
#[doc(hidden)]
#[rustfmt::skip]
macro_rules! __hl_opt {
    // bool
    ($opts:expr, altfont = $val:tt) => { $opts.altfont($val); }; // 1
    ($opts:expr, bg_indexed = $val:tt) => { $opts.bg_indexed($val); }; // 2
    ($opts:expr, bold = $val:tt) => { $opts.bold($val); }; // 3
    ($opts:expr, builder = $val:tt) => { $opts.builder($val); }; // 4
    ($opts:expr, fallback = $val:tt) => { $opts.fallback($val); }; // 5
    ($opts:expr, fg_indexed = $val:tt) => { $opts.fg_indexed($val); }; // 6
    ($opts:expr, force = $val:tt) => { $opts.force($val); }; // 7
    ($opts:expr, italic = $val:tt) => { $opts.italic($val); }; // 8
    ($opts:expr, nocombine = $val:tt) => { $opts.nocombine($val); }; // 9
    ($opts:expr, reverse = $val:tt) => { $opts.reverse($val); }; // 10
    ($opts:expr, standout = $val:tt) => { $opts.standout($val); }; // 11
    ($opts:expr, strikethrough = $val:tt) => { $opts.strikethrough($val); }; // 12
    ($opts:expr, undercurl = $val:tt) => { $opts.undercurl($val); }; // 13
    ($opts:expr, underdashed = $val:tt) => { $opts.underdashed($val); }; // 14
    ($opts:expr, underdotted = $val:tt) => { $opts.underdotted($val); }; // 15
    ($opts:expr, underdouble = $val:tt) => { $opts.underdouble($val); }; // 16
    ($opts:expr, underline = $val:tt) => { $opts.underline($val); }; // 17

    // &str
    ($opts:expr, blend = $val:tt) => { $opts.blend($val); }; // 18
    ($opts:expr, special = $val:tt) => { $opts.special($val); }; // 19
    ($opts:expr, background = $val:tt) => { $opts.background($val); }; // 20
    ($opts:expr, foreground = $val:tt) => { $opts.foreground($val); }; // 21
    ($opts:expr, cterm = $val:tt) => { $opts.cterm($val); }; // 22
    ($opts:expr, ctermbg = $val:tt) => { $opts.ctermbg($val); }; // 23
    ($opts:expr, ctermfg = $val:tt) => { $opts.ctermfg($val); }; // 24

    // Hl
    ($opts:expr, link = $val:tt) => { $opts.link($val); }; // 25
}

// nvim_set_hl({ns_id}, {name}, {val})
#[macro_export]
macro_rules! nvim_set_hl {
    ( $ns_id:expr, $name:literal, { $( $key:ident : $val:expr ),* $(,)? } ) => {{
        let mut opts = nvim_oxi::api::opts::SetHighlightOpts::builder();
        $(
            $crate::__hl_opt!(opts, $key = $val);
        )*
            nvim_oxi::api::set_hl($ne_id, $name, &opts.build())
    }};
}
