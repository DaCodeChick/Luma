// Form Demo with BoxLayout - demonstrates automatic widget positioning
use luma_gui::prelude::*;

fn main() -> Result<()> {
    let mut app = Application::new()?;
    
    let mut window = Window::builder()
        .title("Contact Form - BoxLayout Demo")
        .size(420, 650)  // Increased from 400x500 to fit all content
        .build()?;
    
    // Create a vertical layout with 5px gap between widgets
    let mut layout = BoxLayout::vertical().with_gap(5);
    
    // Name field (label + input)
    let label_name = Label::builder()
        .text("Name:")
        .build(&window)?;
    layout.add(
        Box::new(label_name),
        LayoutConstraints::default()
            .preferred_height(20)
            .padding(Padding::new(5, 10, 0, 10))
    );
    
    let input_name = TextInput::builder()
        .build(&window)?;
    layout.add(
        Box::new(input_name),
        LayoutConstraints::default()
            .preferred_height(24)
            .padding(Padding::new(5, 10, 5, 10))
            .expand_horizontal(true)
    );
    
    // Email field
    let label_email = Label::builder()
        .text("Email:")
        .build(&window)?;
    layout.add(
        Box::new(label_email),
        LayoutConstraints::default()
            .preferred_height(20)
            .padding(Padding::symmetric(0, 10))
    );
    
    let input_email = TextInput::builder()
        .build(&window)?;
    layout.add(
        Box::new(input_email),
        LayoutConstraints::default()
            .preferred_height(24)
            .padding(Padding::new(5, 10, 5, 10))
            .expand_horizontal(true)
    );
    
    // Phone field
    let label_phone = Label::builder()
        .text("Phone:")
        .build(&window)?;
    layout.add(
        Box::new(label_phone),
        LayoutConstraints::default()
            .preferred_height(20)
            .padding(Padding::symmetric(0, 10))
    );
    
    let input_phone = TextInput::builder()
        .build(&window)?;
    layout.add(
        Box::new(input_phone),
        LayoutConstraints::default()
            .preferred_height(24)
            .padding(Padding::new(5, 10, 5, 10))
            .expand_horizontal(true)
    );
    
    // Newsletter checkbox
    let checkbox_newsletter = CheckBox::builder()
        .label("Subscribe to newsletter")
        .checked(true)
        .build(&window)?;
    layout.add(
        Box::new(checkbox_newsletter),
        LayoutConstraints::default()
            .preferred_height(20)
            .padding(Padding::new(5, 10, 5, 10))
    );
    
    // Terms checkbox
    let checkbox_terms = CheckBox::builder()
        .label("I agree to the terms and conditions")
        .build(&window)?;
    layout.add(
        Box::new(checkbox_terms),
        LayoutConstraints::default()
            .preferred_height(20)
            .padding(Padding::new(0, 10, 5, 10))
    );
    
    // Country field
    let label_country = Label::builder()
        .text("Country:")
        .build(&window)?;
    layout.add(
        Box::new(label_country),
        LayoutConstraints::default()
            .preferred_height(20)
            .padding(Padding::new(5, 10, 3, 10))
    );
    
    let listbox_country = ListBox::builder()
        .items(vec![
            "United States",
            "United Kingdom",
            "Canada",
            "Australia",
            "Germany",
            "France",
            "Japan",
            "Brazil",
            "India",
            "China",
        ])
        .sorted(true)
        .build(&window)?;
    layout.add(
        Box::new(listbox_country),
        LayoutConstraints::default()
            .preferred_height(100)
            .padding(Padding::symmetric(0, 10))
            .expand_horizontal(true)
    );
    
    // Submit button
    let button_submit = Button::builder()
        .label("Submit")
        .build(&window)?;
    layout.add(
        Box::new(button_submit),
        LayoutConstraints::default()
            .preferred_height(30)
            .preferred_width(100)
            .padding(Padding::new(5, 10, 0, 10))
    );
    
    // Cancel button
    let button_cancel = Button::builder()
        .label("Cancel")
        .build(&window)?;
    layout.add(
        Box::new(button_cancel),
        LayoutConstraints::default()
            .preferred_height(30)
            .preferred_width(100)
            .padding(Padding::new(5, 10, 0, 10))
    );
    
    // Info label
    let label_info = Label::builder()
        .text("* All fields are required")
        .build(&window)?;
    layout.add(
        Box::new(label_info),
        LayoutConstraints::default()
            .preferred_height(20)
            .padding(Padding::new(5, 10, 5, 10))
    );
    
    // Apply the layout to the window
    window.set_layout(layout)?;
    
    window.show()?;
    
    app.run()
}
