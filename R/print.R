#' Clear the terminal screen
#'
#' @description
#' Sends ANSI escape sequences to clear the visible screen and, when supported,
#' the scrollback buffer of the current terminal. This is primarily intended for
#' use in interactive sessions to reset the display before printing images or
#' other rich output.
#'
#' Note that support for clearing the scrollback buffer depends on the terminal
#' emulator. In some environments, only the visible screen will be cleared.
#'
#' This function has no effect in non-interactive sessions.
#'
#' @returns Invisibly returns `NULL`.
#' @export
clear <- function() {
  if (!interactive()) {
    return(invisible())
  }
  cat("\033[3J\033[H\033[2J")
  flush.console()
  cat("\033[3J\033[H\033[2J")
  flush.console()
  invisible()
}

#' Print a native raster image to the terminal
#'
#' @description
#' Displays a `nativeRaster` object directly in the terminal using
#' terminal graphics protocols (e.g., Sixel, Kitty, or iTerm2),
#' depending on what is supported by the current environment.
#'
#' Internally, the raster is converted to an RGBA image
#' and rendered via a Rust backend.
#'
#' In non-interactive sessions, the input is returned unchanged and
#' no output is produced.
#'
#' @param x A `nativeRaster` object.
#' @param clear_first Logical. If `TRUE`, the terminal is cleared using
#'  `clear()` before printing the image. Defaults to `TRUE`.
#'
#' @returns Invisibly returns `x`.
#' @export
print_nr <- function(x, clear_first = TRUE) {
  if (!interactive()) {
    return(invisible(x))
  }
  if (!inherits(x, "nativeRaster") || length(x) != prod(dim(x))) {
    stop("`x` must be a nativeRaster object.")
  }
  if (clear_first) {
    clear()
  }
  print_with_viuer(
    as.integer(x),
    nrow(x),
    ncol(x),
    as.integer(getOption("width", default = 80L))
  )
  cat("\n")
  flush.console()
  invisible(x)
}
