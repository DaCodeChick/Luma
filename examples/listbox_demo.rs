// ListBox Demo - demonstrates single-select and multi-select listboxes with dynamic layout
use luma_gui::prelude::*;

fn main() -> Result<()> {
    let mut app = Application::new()?;
    
    let mut window = Window::builder()
        .title("ListBox Demo - Luma (Resizable)")
        .size(600, 500)
        .build()?;
    
    // Create main vertical layout
    let mut layout = BoxLayout::vertical().with_gap(10);
    
    // Title section
    let title_label = Label::builder()
        .text("ListBox Demonstration - Resize the window!")
        .build(&window)?;
    layout.add(
        Box::new(title_label),
        LayoutConstraints::default()
            .preferred_height(25)
            .padding(Padding::new(10, 10, 5, 10))
    );
    
    // Single-select section
    let label_single = Label::builder()
        .text("Single-Select ListBox (sorted):")
        .build(&window)?;
    layout.add(
        Box::new(label_single),
        LayoutConstraints::default()
            .preferred_height(20)
            .padding(Padding::symmetric(0, 10))
    );
    
    let listbox_single = ListBox::builder()
        .items(vec![
            "Apple",
            "Banana",
            "Cherry",
            "Date",
            "Elderberry",
            "Fig",
            "Grape",
            "Honeydew",
            "Kiwi",
            "Lemon",
        ])
        .sorted(true)
        .on_select_single(|index| {
            match index {
                Some(idx) => println!("Single-select: Selected item {}", idx),
                None => println!("Single-select: No selection"),
            }
        })
        .build(&window)?;
    layout.add(
        Box::new(listbox_single),
        LayoutConstraints::default()
            .preferred_height(120)
            .padding(Padding::symmetric(0, 10))
            .expand_horizontal(true)
    );
    
    let hint1 = Label::builder()
        .text("↑ Click to select an item")
        .build(&window)?;
    layout.add(
        Box::new(hint1),
        LayoutConstraints::default()
            .preferred_height(18)
            .padding(Padding::new(2, 10, 8, 10))
    );
    
    // Multi-select section
    let label_multi = Label::builder()
        .text("Multi-Select ListBox (sorted):")
        .build(&window)?;
    layout.add(
        Box::new(label_multi),
        LayoutConstraints::default()
            .preferred_height(20)
            .padding(Padding::symmetric(0, 10))
    );
    
    let listbox_multi = ListBox::builder()
        .items(vec![
            "C",
            "C++",
            "C#",
            "Java",
            "JavaScript",
            "Python",
            "Rust",
            "Go",
            "Swift",
            "Kotlin",
            "TypeScript",
            "Ruby",
        ])
        .multi_select(true)
        .sorted(true)
        .on_select_multi(|indices| {
            println!("Multi-select: Selected {} items: {:?}", indices.len(), indices);
        })
        .build(&window)?;
    layout.add(
        Box::new(listbox_multi),
        LayoutConstraints::default()
            .preferred_height(120)
            .padding(Padding::symmetric(0, 10))
            .expand_horizontal(true)
    );
    
    let hint2 = Label::builder()
        .text("↑ Ctrl+Click to select multiple items")
        .build(&window)?;
    layout.add(
        Box::new(hint2),
        LayoutConstraints::default()
            .preferred_height(18)
            .padding(Padding::new(2, 10, 8, 10))
    );
    
    // Unsorted section
    let label_unsorted = Label::builder()
        .text("Unsorted ListBox (insertion order):")
        .build(&window)?;
    layout.add(
        Box::new(label_unsorted),
        LayoutConstraints::default()
            .preferred_height(20)
            .padding(Padding::symmetric(0, 10))
    );
    
    let listbox_unsorted = ListBox::builder()
        .items(vec![
            "First",
            "Second",
            "Third",
            "Fourth",
            "Fifth",
            "Sixth",
        ])
        .build(&window)?;
    layout.add(
        Box::new(listbox_unsorted),
        LayoutConstraints::default()
            .preferred_height(80)
            .padding(Padding::symmetric(0, 10))
            .expand_horizontal(true)
    );
    
    let hint3 = Label::builder()
        .text("↑ Items appear in insertion order (not sorted)")
        .build(&window)?;
    layout.add(
        Box::new(hint3),
        LayoutConstraints::default()
            .preferred_height(18)
            .padding(Padding::new(2, 10, 5, 10))
    );
    
    // Info footer
    let info_label = Label::builder()
        .text("✓ All listboxes expand horizontally when window is resized")
        .build(&window)?;
    layout.add(
        Box::new(info_label),
        LayoutConstraints::default()
            .preferred_height(20)
            .padding(Padding::new(5, 10, 10, 10))
    );
    
    // Apply layout
    window.set_layout(layout)?;
    
    window.show()?;
    
    app.run()
}
