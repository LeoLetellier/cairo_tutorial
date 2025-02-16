use cairo::{Context, Format, ImageSurface};
use rand::prelude::*;
use std::{f64, fs::File};

/// Draw all examples
fn main() {
    principle();
    paint();
    rand_lines();
    basics();
    mask();
    source1();
    source2();
    curves();
    pattern();
    scale();
}

/// Basic principles for using cairo
///
/// The surface is the object we are drawing into.
///
/// The Context is the drawing element, the pen, which is used through action verbs.
///
/// The surface can be saved to a file.
fn principle() {
    // Create the surface, the element on which we will be drawing
    let surface =
        ImageSurface::create(Format::ARgb32, 200, 200).expect("Failed to create image surface!");

    // Create the context, through which we will be drawing onto the surface
    // One can see it as "the pen" we are using
    let context = Context::new(&surface).expect("Failed to create context!");

    // We can change the properties of the context (the pen) we are drawing with
    // i.e the color, initial position, ...
    context.set_source_rgb(0.8, 0.8, 0.8);

    // We can define a path that the context will follow, as a line, rectangle, ...
    context.rectangle(50.0, 50.0, 100.0, 100.0);

    // The context can draw the boundaries of the path, or fill it with a color
    context.fill().expect("Failed to fill rectangle");

    // The surface can be saved into a file (here png with the png feature)
    let mut file = File::create("example_output/principle.png").expect("Failed to create file!");
    surface
        .write_to_png(&mut file)
        .expect("Failed to write to PNG");
}

/// Use the paint method to set the background color
pub fn paint() {
    let surface = ImageSurface::create(Format::ARgb32, 200, 200).expect("Failed to create surface");
    let context = Context::new(&surface).expect("Failed to create context!");

    // Update the color used by the context
    context.set_source_rgb(1.0, 0.0, 0.0);
    // Draw on the whole surface, i.e with an infinite mask
    context.paint().expect("Failed to paint!");

    let mut file = File::create("example_output/paint.png").expect("Failed to create file!");
    surface
        .write_to_png(&mut file)
        .expect("Failed to write png!");
}

/// Draw multiple linked lined with random orientation
///
/// Adapted from [Keith Peters](https://medium.com/@bit101/intro-to-cairo-graphics-in-rust-35470a6aed86)
pub fn rand_lines() {
    let surface = ImageSurface::create(Format::ARgb32, 200, 200).expect("Failed to create surface");
    let context = Context::new(&surface).expect("Failed to create context!");
    let mut rng = rand::rng();

    // Create white canvas
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint().expect("Failed to paint!");

    // Switch color to black
    context.set_source_rgb(0.0, 0.0, 0.0);
    // Sketch random linked lines
    for _ in 0..80 {
        let x = rng.random::<f64>() * 200.0;
        let y = rng.random::<f64>() * 200.0;
        // Define a straight line from the current position to the target
        context.line_to(x, y);
    }
    // Draw the sketched lines
    context.stroke().expect("Failed to stroke lines");

    let mut file = File::create("example_output/rand_lines.png").expect("Failed to create file!");
    surface
        .write_to_png(&mut file)
        .expect("Failed to write png!");
}

/// Basic usage of cairo capabilities
///
/// Adapted from [Cairo Tutorials](https://www.cairographics.org/tutorial/)
pub fn basics() {
    let surface = ImageSurface::create(Format::ARgb32, 200, 200).expect("Failed to create surface");
    let context = Context::new(&surface).expect("Failed to create context!");

    // Black background
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.paint().expect("Failed to paint!");

    // Frame as rectangle stroke
    context.set_line_width(8.0);
    context.set_source_rgb(26.0 / 255.0, 188.0 / 255.0, 156.0 / 255.0);
    context.rectangle(0.0, 0.0, 200.0, 200.0);
    context.stroke().expect("Failed to stroke lines");

    // Text in the middle
    context.set_source_rgb(204.0 / 255.0, 174.0 / 255.0, 249.0 / 255.0);
    context.select_font_face("Georgia", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
    context.set_font_size(40.0);
    let text = "cairo";
    let te = context.text_extents(text).expect("Failed to create text!");
    context.move_to(
        100.0 - te.width() / 2.0 - te.x_bearing(),
        100.0 - te.height() / 2.0 - te.y_bearing(),
    );
    context.show_text(text).expect("Failed to show text");

    let mut file = File::create("example_output/basics.png").expect("Failed to create file!");
    surface
        .write_to_png(&mut file)
        .expect("Failed to write png!");
}

/// Use gradients (linear and radial) as base for coloring or masking
///
/// Adapted from [Cairo Tutorials](https://www.cairographics.org/tutorial/)
fn mask() {
    let surface = ImageSurface::create(Format::ARgb32, 200, 200).expect("Failed to create surface");
    let context = Context::new(&surface).expect("Failed to create context!");

    // Create a linear gradient between a begin point and an end point
    let gradient_lin = cairo::LinearGradient::new(0.0, 0.0, 200.0, 200.0);
    // Associate colors at different position onto the linear distance of the gradient
    // offset 0 corresponds to (x0, y0)
    // offset 1 corresponds to (x1, y1)
    cairo::Gradient::add_color_stop_rgb(&gradient_lin, 0.0, 0.0, 0.3, 0.8);
    cairo::Gradient::add_color_stop_rgb(&gradient_lin, 1.0, 0.0, 0.8, 0.3);

    // Create a radial gradient between a begin circle (center and radius) and end circle (center and radius)
    let gradient_rad = cairo::RadialGradient::new(100.0, 100.0, 10.0, 100.0, 100.0, 140.0);
    // Associate colors at different position onto the linear distance of the gradient
    // offset 0 corresponds to (x0, y0)
    // offset 1 corresponds to (x1, y1)
    cairo::Gradient::add_color_stop_rgba(&gradient_rad, 0.0, 0.0, 0.0, 0.0, 1.0);
    cairo::Gradient::add_color_stop_rgba(&gradient_rad, 0.5, 0.0, 0.0, 0.0, 0.0);

    // Apply the color from the linear gradient
    context
        .set_source(gradient_lin)
        .expect("Failed to set source!");
    // Use the radial gradient as mask
    context.mask(gradient_rad).expect("Failed to set mask!");

    let mut file = File::create("example_output/mask.png").expect("Failed to create file!");
    surface
        .write_to_png(&mut file)
        .expect("Failed to write png!");
}

/// Example of handling multiple element with different sources for coloring
///
/// Adapted from [Cairo Tutorials](https://www.cairographics.org/tutorial/)
fn source1() {
    let surface = ImageSurface::create(Format::ARgb32, 200, 200).expect("Failed to create surface");
    let context = Context::new(&surface).expect("Failed to create context!");

    // Draw a cross on entire image
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.move_to(0.0, 0.0);
    context.line_to(200.0, 200.0);
    context.move_to(200.0, 0.0);
    context.line_to(0.0, 200.0);
    context.set_line_width(25.0);
    context.stroke().expect("Failed to stroke!");

    // Draw red rectangle in the up left corner
    context.rectangle(0.0, 0.0, 100.0, 100.0);
    context.set_source_rgba(225.0 / 255.0, 96.0 / 255.0, 78.0 / 255.0, 0.8);
    context.fill().expect("Failed to fill!");

    // Draw green rectangle in the low left corner
    context.rectangle(0.0, 100.0, 100.0, 100.0);
    context.set_source_rgba(56.0 / 255.0, 215.0 / 255.0, 28.0 / 255.0, 0.4);
    context.fill().expect("Failed to fill!");

    // Draw blue rectangle in the up right corner
    context.rectangle(100.0, 0.0, 100.0, 100.0);
    context.set_source_rgba(46.0 / 255.0, 147.0 / 255.0, 213.0 / 255.0, 0.6);
    context.fill().expect("Failed to fill!");

    let mut file = File::create("example_output/source1.png").expect("Failed to create file!");
    surface
        .write_to_png(&mut file)
        .expect("Failed to write png!");
}

/// Example of handling multiple element with different gradients
///
/// Adapted from [Cairo Tutorials](https://www.cairographics.org/tutorial/)
fn source2() {
    let surface = ImageSurface::create(Format::ARgb32, 200, 200).expect("Failed to create surface");
    let context = Context::new(&surface).expect("Failed to create context!");

    // Create radial gradient
    let gradient_rad = cairo::RadialGradient::new(
        200.0 * 0.25,
        200.0 * 0.25,
        200.0 * 0.1,
        200.0 * 0.5,
        200.0 * 0.5,
        200.0 * 0.5,
    );
    gradient_rad.add_color_stop_rgb(0.0, 1.0, 0.8, 0.8);
    gradient_rad.add_color_stop_rgb(1.0, 0.9, 0.0, 0.0);

    // Draw a grid of 9 x 9 squares with spacing
    (1..10).for_each(|i| {
        (1..10).for_each(|j| {
            context.rectangle(
                200.0 * (i as f64 / 10.0 - 0.04),
                200.0 * (j as f64 / 10.0 - 0.04),
                200.0 * 0.08,
                200.0 * 0.08,
            );
        })
    });
    context
        .set_source(gradient_rad)
        .expect("Failed to set source!");
    context.fill().expect("Failed to fill!");

    // Create linear gradient
    let gradient_lin =
        cairo::LinearGradient::new(200.0 * 0.25, 200.0 * 0.35, 200.0 * 0.75, 200.0 * 0.65);
    gradient_lin.add_color_stop_rgba(0.0, 1.0, 1.0, 1.0, 0.0);
    gradient_lin.add_color_stop_rgba(0.25, 0.0, 1.0, 0.0, 0.5);
    gradient_lin.add_color_stop_rgba(0.5, 1.0, 1.0, 1.0, 0.0);
    gradient_lin.add_color_stop_rgba(0.75, 0.0, 0.0, 1.0, 0.5);
    gradient_lin.add_color_stop_rgba(1.0, 1.0, 1.0, 1.0, 0.0);

    context.rectangle(0.0, 0.0, 200.0 * 1.0, 200.0 * 1.0);
    context
        .set_source(gradient_lin)
        .expect("Failed to set source!");
    context.fill().expect("Failed to fill!");

    let mut file = File::create("example_output/source2.png").expect("Failed to create file!");
    surface
        .write_to_png(&mut file)
        .expect("Failed to write png!");
}

/// Usage of curves, arcs, segments, and path closing
///
/// Adapted from [Cairo Tutorials](https://www.cairographics.org/tutorial/)
fn curves() {
    let surface = ImageSurface::create(Format::ARgb32, 200, 200).expect("Failed to create surface");
    let context = Context::new(&surface).expect("Failed to create context!");

    // Set starting point
    context.move_to(50.0, 50.0);

    // Draw linked lines
    context.line_to(100.0, 75.0);
    context.rel_line_to(50.0, -25.0);

    // Draw arc
    context.arc(
        100.0,
        100.0,
        50.0 * 2.0_f64.sqrt(),
        -0.25 * f64::consts::PI,
        0.25 * f64::consts::PI,
    );

    // Link with curve
    context.rel_curve_to(-50.0, -25.0, -50.0, 25.0, -100.0, 0.0);

    // Close the figure using a straight line
    context.close_path();

    context.set_source_rgb(6.0 / 255.0, 117.0 / 255.0, 114.0 / 255.0);
    context.stroke().expect("Failed to stroke!");

    let mut file = File::create("example_output/curves.png").expect("Failed to create file!");
    surface
        .write_to_png(&mut file)
        .expect("Failed to write png!");
}

/// Working with patterns and masks to create complex images
fn pattern() {
    let surface = ImageSurface::create(Format::ARgb32, 200, 200).expect("Failed to create surface");
    let context = Context::new(&surface).expect("Failed to create context!");
    context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
    context.paint().expect("Failed to paint!");

    // Draw a red line on a second surface
    let pattern_surface = surface.clone();
    let pattern_context = Context::new(&pattern_surface).expect("Failed to create context!");
    pattern_context.set_source_rgb(1.0, 0.0, 0.0);
    pattern_context.set_line_width(8.0);
    pattern_context.move_to(100.0, 0.0);
    pattern_context.line_to(100.0, 200.0);
    pattern_context.stroke().expect("Failed to stroke!");

    // Create a pattern using the second surface
    let pattern = cairo::SurfacePattern::create(pattern_surface);

    // Make the pattern repeating
    pattern.set_extend(cairo::Extend::Repeat);
    // Define a valid matrix
    // x_new = xx * x + xy * y + x0
    // y_new = yx * x + yy * y + y0
    // see [cairo_matrix_t](https://www.cairographics.org/manual/cairo-cairo-matrix-t.html#cairo-matrix-t)
    // Seems that xx and yy can't be null
    let matrix = cairo::Matrix::new(60.0, 0.0, 0.0, 1.0, 10.0, 0.0);
    pattern.set_matrix(matrix);

    // Set the operation between masks
    context.set_operator(cairo::Operator::Atop);
    // Apply the pattern onto the main surface
    context.set_source(pattern).expect("Failed to set source!");
    context.paint().expect("Failed to paint!");

    let mut file = File::create("example_output/pattern.png").expect("Failed to create file!");
    surface
        .write_to_png(&mut file)
        .expect("Failed to write png!");
}

/// Setup a local scale to abstract over the real image pixel resolution
fn scale() {
    let surface = ImageSurface::create(Format::ARgb32, 200, 200).expect("Failed to create surface");
    let context = Context::new(&surface).expect("Failed to create context!");

    // Define the working frame to be of 150 pixels wide and high
    // Deplace the working frame of 25 pixels on x and y to center it
    // The coordinates inside the working frame range from 0 to 1
    context.scale(150.0, 150.0);
    context.translate(1.0 / 6.0, 1.0 / 6.0);
    // Could also translate from 25 pixels (25/150=1/6), and then scale to 150

    // Fill a rectangle in the whole working area
    context.rectangle(0.0, 0.0, 1.0, 1.0);
    context.set_source_rgb(1.0, 1.0, 0.0);
    context.fill().expect("Failed to fill!");

    let mut file = File::create("example_output/scale.png").expect("Failed to create file!");
    surface
        .write_to_png(&mut file)
        .expect("Failed to write png!");
}
