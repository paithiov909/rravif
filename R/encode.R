#' Encode an AVIF image
#'
#' Encodes an AVIF image from a nativeRaster object.
#'
#' @param nr A nativeRaster object.
#' @param quality The quality of the image.
#' @param speed The processing speed.
#' @returns A raw vector containing the encoded image data.
#' @export
encode <- function(nr, quality = 80, speed = 10) {
  if (!inherits(nr, "nativeRaster")) {
    stop("`nr` must be a nativeRaster")
  }
  if (!is.finite(quality) || quality < 0 || quality > 100) {
    stop("`quality` must be between 0 and 100")
  }
  if (!is.finite(speed) || speed < 0 || speed > 10) {
    stop("`speed` must be between 0 and 10")
  }
  encode_avif(nr, nrow(nr), ncol(nr), as.integer(quality), as.integer(speed))
}

#' Write an AVIF image to a file
#'
#' @inheritParams encode
#' @param path The path to write the image.
#' @returns
#' If `path` is `NULL`, returns a raw vector containing the encoded image data.
#' Otherwise, `path` is invisibly returned.
#' @export
write_avif <- function(nr, path = NULL, quality = 80, speed = 10) {
  raw_bytes <- encode(nr, quality, speed)
  if (is.null(path)) {
    return(raw_bytes)
  }
  writeBin(raw_bytes, path)
  invisible(path)
}
