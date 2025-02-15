use rand;
use cairo::{Context, Format, ImageSurface};
use std::fs::File;

/// Draw all examples
fn main() {
    principle();
    paint();
    rand_lines();
    basics();
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
    let surface = ImageSurface::create(Format::ARgb32, 256, 256)
        .expect("Failed to create image surface!");

    // Create the context, through which we will be drawing onto the surface
    // One can see it as "the pen" we are using
    let context = Context::new(&surface).expect("Failed to create context!");

    // We can change the properties of the context (the pen) we are drawing with
    // i.e the color, initial position, ...
    context.set_source_rgb(0.8, 0.8, 0.8);

    // We can define a path that the context will follow, as a line, rectangle, ...
    context.rectangle(50.0, 50.0, 150.0, 150.0);

    // The context can draw the boundaries of the path, or fill it with a color
    context.fill().expect("Failed to fill rectangle");

    // The surface can be saved into a file (here png with the png feature)
    let mut file = File::create("example_output/principle.png").expect("Failed to create file!");
    surface
        .write_to_png(&mut file)
        .expect("Failed to write to PNG");
}

pub fn paint() {
    let surface = ImageSurface::create(Format::ARgb32, 256, 256).expect("Failed to create surface");
    let context = Context::new(&surface).expect("Failed to create context!");

    // Update the color used by the context
    context.set_source_rgb(1.0, 0.0, 0.0);
    // Draw on the whole surface, i.e with an infinite mask
    context.paint().expect("Failed to paint!");

    let mut file = File::create("example_output/paint.png").expect("Failed to create file!");
    surface.write_to_png(&mut file).expect("Failed to write png!");
}

pub fn rand_lines() {
    let surface = ImageSurface::create(Format::ARgb32, 600, 600).expect("Failed to create surface");
    let context = Context::new(&surface).expect("Failed to create context!");
    
    // Create white canvas
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint().expect("Failed to paint!");

    // Switch color to black
    context.set_source_rgb(0.0, 0.0, 0.0);
    // Sketch random linked lines
    for _ in 0..100 {
        let x = rand::random::<f64>() * 600.0;
        let y = rand::random::<f64>() * 600.0;
        // Define a straight line from the current position to the target
        context.line_to(x, y);
    }
    // Draw the sketched lines
    context.stroke().expect("Failed to stroke lines");

    let mut file = File::create("example_output/rand_lines.png").expect("Failed to create file!");
    surface.write_to_png(&mut file).expect("Failed to write png!");
}

pub fn basics() {
    let surface = ImageSurface::create(Format::ARgb32, 600, 600).expect("Failed to create surface");
    let context = Context::new(&surface).expect("Failed to create context!");

    // Black background
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.paint().expect("Failed to paint!");
    
    // Frame as rectangle stroke
    context.set_line_width(8.0);
    context.set_source_rgb(26.0 / 255.0, 188.0 / 255.0, 156.0 / 255.0);
    context.rectangle(0.0, 0.0, 600.0, 600.0);
    context.stroke().expect("Failed to stroke lines");

    // Text in the middle
    context.set_source_rgb(204.0 / 255.0, 174.0 / 255.0, 249.0 / 255.0);
    context.select_font_face("Georgia", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
    context.set_font_size(120.0);
    let text = "cairo";
    let te = context.text_extents(text).expect("Failed to create text!");
    context.move_to(300.0 - te.width() / 2.0 - te.x_bearing(), 300.0 - te.height() / 2.0 - te.y_bearing());
    context.show_text(text).expect("Failed to show text");

    let mut file = File::create("example_output/basics.png").expect("Failed to create file!");
    surface.write_to_png(&mut file).expect("Failed to write png!");
}