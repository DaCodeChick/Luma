use luma_gui::prelude::*;

fn main() -> Result<()> {
    let mut app = Application::new()?;
    
    let mut window = Window::builder()
        .title("Layout + Callback Test")
        .size(300, 250)
        .build()?;
    
    // Create a vertical layout with 10px gap
    let mut layout = BoxLayout::vertical().with_gap(10);
    
    // Create labels and buttons with callbacks
    let label1 = Label::builder()
        .text("Click buttons to test callbacks:")
        .build(&window)?;
    layout.add(
        Box::new(label1),
        LayoutConstraints::default()
            .preferred_height(20)
            .padding(Padding::new(10, 10, 0, 10))
    );
    
    let button1 = Button::builder()
        .label("Button 1")
        .on_click(|| {
            println!("Button 1 clicked!");
        })
        .build(&window)?;
    layout.add(
        Box::new(button1),
        LayoutConstraints::default()
            .preferred_height(40)
            .padding(Padding::symmetric(10, 10))
            .expand_horizontal(true)
    );
    
    let button2 = Button::builder()
        .label("Button 2")
        .on_click(|| {
            println!("Button 2 clicked!");
        })
        .build(&window)?;
    layout.add(
        Box::new(button2),
        LayoutConstraints::default()
            .preferred_height(40)
            .padding(Padding::symmetric(10, 10))
            .expand_horizontal(true)
    );
    
    let button3 = Button::builder()
        .label("Exit")
        .on_click(|| {
            println!("Exit button clicked - closing app!");
            std::process::exit(0);
        })
        .build(&window)?;
    layout.add(
        Box::new(button3),
        LayoutConstraints::default()
            .preferred_height(40)
            .padding(Padding::symmetric(10, 10))
            .expand_horizontal(true)
    );
    
    // Apply the layout to the window
    window.set_layout(layout)?;
    
    window.show()?;
    
    app.run()
}
