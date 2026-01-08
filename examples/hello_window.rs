// Hello Window Example
// A minimal example showing a basic empty window

use luma_gui::prelude::*;

fn main() -> Result<()> {
    // Create the application
    let mut app = Application::new()?;
    
    // Create a window
    let mut window = Window::builder()
        .title("Hello, Luma!")
        .size(400, 300)
        .build()?;
    
    // Show the window
    window.show()?;
    
    // Run the application event loop
    app.run()
}
