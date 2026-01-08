//! Example usage of the XAML parser

use luma_xaml::parser::{XamlParser, ParserSettings};
use luma_xaml::types::TypeRegistry;

fn main() {
    // Define some XAML
    let xaml = r#"
        <Window
            xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
            xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
            x:Name="MainWindow"
            Title="Hello Luma"
            Width="800"
            Height="600">
            
            <StackPanel Orientation="Vertical">
                <TextBlock Text="Welcome to Luma!" FontSize="24"/>
                <Button Content="Click Me" Width="100"/>
            </StackPanel>
        </Window>
    "#;

    // Create a type registry (will be populated with WinUI 3 types in Phase 2)
    let registry = TypeRegistry::new();

    // Create a parser with default settings (strict mode, validate types)
    let parser = XamlParser::new(registry);

    // Or create a parser with custom settings
    let _custom_parser = XamlParser::new(TypeRegistry::new())
        .with_settings(
            ParserSettings::new()
                .lenient()                  // Allow unknown types
                .preserve_whitespace()      // Keep exact whitespace
                .validate_namespaces()      // Validate namespace prefixes
        );

    // Parse the XAML
    match parser.parse_string(xaml) {
        Ok(document) => {
            println!("✅ Successfully parsed XAML document!");
            println!("Root element: {}", document.root.type_name.name);
            
            if let Some(name) = &document.root.name {
                println!("Window name: {}", name);
            }
            
            // Access attributes
            if let Some(title) = document.root.get_attribute("Title") {
                if let Some(title_str) = title.as_string() {
                    println!("Window title: {}", title_str);
                }
            }
            
            // Access children
            println!("Number of children: {}", document.root.child_elements().count());
            
            for child in document.root.child_elements() {
                println!("  - Child: {}", child.type_name.name);
                
                // Access nested children
                for nested in child.child_elements() {
                    println!("    - Nested: {}", nested.type_name.name);
                    
                    if let Some(content) = nested.get_attribute("Content") {
                        println!("      Content: {:?}", content);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to parse XAML: {}", e);
            
            // Error includes line number information when available
            if let Some(line) = e.line() {
                eprintln!("   Error occurred at line {}", line);
            }
        }
    }
}

// Example output:
// ✅ Successfully parsed XAML document!
// Root element: Window
// Window name: MainWindow
// Window title: Hello Luma
// Number of children: 1
//   - Child: StackPanel
//     - Nested: TextBlock
//     - Nested: Button
//       Content: String("Click Me")
