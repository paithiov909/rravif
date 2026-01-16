#' Encode an AVIF image
#'
#' Encodes an AVIF image from a nativeRaster object.
#'
#' @param nr A nativeRaster object
#' @param quality The quality of the image
#' @param speed The speed of processing
#' @returns A raw vector containing the encoded image data
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
