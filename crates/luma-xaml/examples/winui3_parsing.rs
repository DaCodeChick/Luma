//! Example showing WinUI 3 type registry usage

#[cfg(feature = "winui3")]
use luma_xaml::dialects::winui3::create_type_registry;
#[cfg(feature = "winui3")]
use luma_xaml::parser::XamlParser;

#[cfg(feature = "winui3")]
fn main() {
    // Real WinUI 3 XAML
    let xaml = r"
        <Window
            xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation'
            xmlns:x='http://schemas.microsoft.com/winfx/2006/xaml'
            x:Name='MainWindow'
            Title='Luma GUI Framework'
            Width='800'
            Height='600'>
            
            <Grid>
                <Grid.RowDefinitions>
                    <RowDefinition Height='Auto'/>
                    <RowDefinition Height='*'/>
                    <RowDefinition Height='Auto'/>
                </Grid.RowDefinitions>
                
                <Border Grid.Row='0' Background='CornflowerBlue' Padding='16'>
                    <TextBlock Text='Welcome to Luma' FontSize='24' Foreground='White'/>
                </Border>
                
                <StackPanel Grid.Row='1' Margin='16' Spacing='8'>
                    <TextBlock Text='Enter your name:' FontWeight='Bold'/>
                    <TextBox PlaceholderText='Your name here...' Width='300'/>
                    
                    <TextBlock Text='Choose an option:' FontWeight='Bold' Margin='0,16,0,0'/>
                    <CheckBox Content='Enable notifications'/>
                    <CheckBox Content='Remember me'/>
                    
                    <TextBlock Text='Select a theme:' FontWeight='Bold' Margin='0,16,0,0'/>
                    <RadioButton Content='Light' GroupName='Theme' IsChecked='True'/>
                    <RadioButton Content='Dark' GroupName='Theme'/>
                    
                    <Button Content='Submit' Width='150' Margin='0,16,0,0'/>
                </StackPanel>
                
                <Border Grid.Row='2' Background='LightGray' Padding='8'>
                    <TextBlock Text='Status: Ready' FontSize='12'/>
                </Border>
            </Grid>
        </Window>
    ";

    // Create WinUI 3 type registry with all controls
    let registry = create_type_registry();
    let parser = XamlParser::new(registry);

    // Parse the XAML
    match parser.parse_string(xaml) {
        Ok(document) => {
            println!("✅ Successfully parsed WinUI 3 XAML!");
            println!();
            
            // Window information
            println!("Window Details:");
            println!("  - Name: {}", document.root.name.as_ref().unwrap_or(&"<unnamed>".to_string()));
            println!("  - Title: {}", document.root.get_attribute("Title")
                .and_then(|v| v.as_string()).unwrap_or("<no title>"));
            println!("  - Type: {}", document.root.type_name.name);
            println!();
            
            // Analyze structure
            println!("Content Structure:");
            analyze_element(&document.root, 0);
            
            println!();
            println!("Statistics:");
            let stats = count_elements(&document.root);
            println!("  - Total elements: {}", stats.total);
            println!("  - Controls: {}", stats.controls);
            println!("  - Panels: {}", stats.panels);
            println!("  - Text elements: {}", stats.text);
        }
        Err(e) => {
            eprintln!("❌ Failed to parse XAML: {}", e);
            if let Some(line) = e.line() {
                eprintln!("   Error at line {}", line);
            }
        }
    }
}

#[cfg(feature = "winui3")]
fn analyze_element(element: &luma_xaml::model::XamlElement, depth: usize) {
    let indent = "  ".repeat(depth);
    let type_name = &element.type_name.name;
    
    // Print element info
    print!("{}- {} ", indent, type_name);
    
    // Print key attributes
    if let Some(name) = &element.name {
        print!("(x:Name=\"{}\") ", name);
    }
    if let Some(content) = element.get_attribute("Content").and_then(|v| v.as_string()) {
        print!("[Content=\"{}\"] ", content);
    }
    if let Some(text) = element.get_attribute("Text").and_then(|v| v.as_string()) {
        print!("[Text=\"{}\"] ", text);
    }
    
    println!();
    
    // Recursively analyze children
    for child in element.child_elements() {
        analyze_element(child, depth + 1);
    }
}

#[cfg(feature = "winui3")]
struct ElementStats {
    total: usize,
    controls: usize,
    panels: usize,
    text: usize,
}

#[cfg(feature = "winui3")]
fn count_elements(element: &luma_xaml::model::XamlElement) -> ElementStats {
    let mut stats = ElementStats {
        total: 1,
        controls: 0,
        panels: 0,
        text: 0,
    };
    
    // Categorize this element
    match element.type_name.name.as_str() {
        "Button" | "CheckBox" | "RadioButton" | "TextBox" | "Slider" => {
            stats.controls += 1;
        }
        "StackPanel" | "Grid" | "Canvas" | "ScrollViewer" => {
            stats.panels += 1;
        }
        "TextBlock" => {
            stats.text += 1;
        }
        _ => {}
    }
    
    // Count children
    for child in element.child_elements() {
        let child_stats = count_elements(child);
        stats.total += child_stats.total;
        stats.controls += child_stats.controls;
        stats.panels += child_stats.panels;
        stats.text += child_stats.text;
    }
    
    stats
}

#[cfg(not(feature = "winui3"))]
fn main() {
    println!("This example requires the 'winui3' feature.");
    println!("Run with: cargo run --example winui3_parsing --features winui3");
}
