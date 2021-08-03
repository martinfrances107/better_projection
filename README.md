# Better projection.
 As part of the redesign of rust_geo.

 This is a skeleton demonstration showing the simplest outline for a projection/projection_factory.


 Specifcally is shows how to swap out  clip_factory permitting precision() and clip_angle() in the orignal projection to swap out

 * ClipAntimeridian -> ClipCircle
 * ResampleNone -> Resample
