use luma_gui::prelude::*;

fn main() -> Result<()> {
    let mut app = Application::new()?;
    
    let mut window = Window::builder()
        .title("Horizontal BoxLayout Test")
        .size(600, 400)
        .build()?;
    
    // Create a horizontal layout with 10px gap
    let mut layout = BoxLayout::horizontal().with_gap(10);
    
    // Create three buttons side-by-side
    let button1 = Button::builder()
        .label("Left")
        .on_click(|| {
            println!("Left button clicked!");
        })
        .build(&window)?;
    layout.add(
        Box::new(button1),
        LayoutConstraints::default()
            .preferred_width(150)
            .padding(Padding::all(10))
            .expand_vertical(true)  // Test vertical expansion
    );
    
    let button2 = Button::builder()
        .label("Center")
        .on_click(|| {
            println!("Center button clicked!");
        })
        .build(&window)?;
    layout.add(
        Box::new(button2),
        LayoutConstraints::default()
            .preferred_width(150)
            .padding(Padding::all(10))
            .expand_vertical(true)
    );
    
    let button3 = Button::builder()
        .label("Right")
        .on_click(|| {
            println!("Right button clicked!");
        })
        .build(&window)?;
    layout.add(
        Box::new(button3),
        LayoutConstraints::default()
            .preferred_width(150)
            .padding(Padding::all(10))
            .expand_vertical(true)
    );
    
    // Apply the layout to the window
    window.set_layout(layout)?;
    
    window.show()?;
    
    println!("Window shown. Try resizing the window!");
    println!("Buttons should maintain their width but expand/contract vertically.");
    
    app.run()
}
