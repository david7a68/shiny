# Shiny

Shiny aims to be a fast and small 2D vector graphics library with selectable
(and replaceable) backends in the Rust programming language.

## References

- Dokter, M., Hladky, J., Parger, M., Schmalstieg, D., Seidel, H.-P. and Steinberger, M. (2019), Hierarchical Rasterization of Curved Primitives for Vector Graphics Rendering on the GPU. Computer Graphics Forum, 38: 93-103. <https://doi.org/10.1111/cgf.13622>

## Pipeline

- Initialize Shiny with `Shiny::new()`.
- Create an `Image` with `shiny.new_image(width, height)`.
- Create a `Canvas` object that around the `Image` with `Canvas::new(image)`.
- Create a `PathBuilder` and build a path with lines and curves.
- Construct an optimized `Path` with `path_builder.build()`.
  - This is a rather expensive operation, so an async implementation may be desired.
- Add the `Path` to `canvas`.
- Call `canvas.draw()` to submit work to the backend.
- Retrieve the returned image for use.

2 kinds of canvas:

- implicit stack: `Canvas`
- explicit stack: `ScopedCanvas`
