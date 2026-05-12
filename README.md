# Nvim-Macros

## Example 

```rust 
use just_nvim::nvim_set_hl;
use nvim_oxi::api::Error;
use nvim_oxi::api::opts::SetHighlightOpts;

#[test]
#[ignore = "test won't work as this is a `cdylib`"]
fn it_works() -> Result<(), Error> {
    // without macro
    let opts = SetHighlightOpts::builder()
        .bold(true)
        .foreground("#FFFFFF")
        .build();
    nvim_oxi::api::set_hl(0, "Comment", &opts)?;

    // with macro
    // full example
    nvim_set_hl!(0, "Comment", {
        bold: true,
        foreground: "FFFFFF",
        undercurl: true,
        altfont: true,
        background: "000000",
        bg_indexed: true,
        blend: 5,
        cterm: "some str",
        ctermbg: "some str",
        ctermfg: "some str",
        fallback: true,
        fg_indexed: true,
        force: true,
        italic: true,
        link: "any type which satisfy HlGroup Trait",
        nocombine: true,
        reverse: true,
        special: "some str",
        standout: true,
        strikethrough: true,
        undercurl: true,
        underdashed: true,
        underdotted: true,
        underdouble: true,
        underline: true,
    })?;

    Ok(())
}
```

