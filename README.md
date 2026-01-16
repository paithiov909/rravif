# rravif


<!-- README.md is generated from README.qmd. Please edit that file -->

<!-- badges: start -->

[![Lifecycle:
experimental](https://img.shields.io/badge/lifecycle-experimental-orange.svg)](https://lifecycle.r-lib.org/articles/stages.html#experimental)
<!-- badges: end -->

rravif is a package for encoding images in AVIF format with
[ravif](https://github.com/kornelski/cavif-rs) Rust crate.

## Usage

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

## License

BSD 3-clause License.
