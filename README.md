# rravif


<!-- README.md is generated from README.qmd. Please edit that file -->

<!-- badges: start -->

[![Lifecycle:
experimental](https://img.shields.io/badge/lifecycle-experimental-orange.svg)](https://lifecycle.r-lib.org/articles/stages.html#experimental)
<!-- badges: end -->

rravif is an R package for encoding images in AVIF format using the
[ravif](https://github.com/kornelski/cavif-rs) Rust crate.

Originally, this package was a small utility for writing AVIF images
from `nativeRaster` objects. While AVIF encoding can also be done via
other packages such as [magick](https://github.com/ropensci/magick),
this package provides a minimal Rust-backed implementation with few
dependencies.

In addition, it now includes a small helper for displaying
`nativeRaster` objects directly in the terminal using terminal graphics
protocols.

## Usage

### Encoding AVIF images

``` r
library(rravif)

cap <- ragg::agg_capture(width = 480, height = 360, background = "cyan")

grid::grid.newpage()
grid::grid.circle(
  x = 0.5,
  y = 0.5,
  r = 0.3,
  gp = grid::gpar(
    col = "snow",
    fill = "hotpink",
    lwd = 12
  )
)

rast <- cap(native = TRUE)
dev.off()
#> pdf
#>   2

avif_img <- encode(rast)
writeBin(avif_img, "man/figures/test.avif")
```

![test.avif](man/figures/test.avif)

### Terminal preview

rravif also provides a simple way to preview `nativeRaster` images
directly in the terminal:

``` r
if (interactive()) {
  print_nr(rast)
}
```

This renders the image inline using terminal graphics protocols (such as
[Sixel](https://vt100.net/docs/vt3xx-gp/chapter14.html),
[Kitty](https://sw.kovidgoyal.net/kitty/graphics-protocol/), or
[iTerm2](https://iterm2.com/documentation-images.html)), depending on
what your terminal supports.

Note that terminal support varies across environments. For best results,
use a terminal emulator with image protocol support (e.g., Kitty,
WezTerm, or recent versions of Windows Terminal / VS Code terminal with
images enabled).

## Related work

Similar ideas are explored in other packages:

- [terminalgraphics](https://codeberg.org/djvanderlaan/terminalgraphics)
- [rsixel](https://github.com/Fan-iX/rsixel)

Compared to these, rravif does not implement a full graphics device.
Instead, it focuses on lightweight, ad-hoc display of `nativeRaster`
objects, making it suitable as a simple preview tool.

## License

MIT License.
