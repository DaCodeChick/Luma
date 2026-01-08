use luma_gui::prelude::*;

fn main() -> Result<()> {
    let mut app = Application::new()?;
    let mut window = Window::builder()
        .title("Button Callback Test")
        .size(300, 200)
        .build()?;
    
    // Create a button with a callback
    let _button = Button::builder()
        .label("Click Me!")
        .position(100, 80)
        .size(100, 40)
        .on_click(|| {
            println!("Button was clicked!");
        })
        .build(&window)?;
    
    window.show()?;
    app.run()
}
