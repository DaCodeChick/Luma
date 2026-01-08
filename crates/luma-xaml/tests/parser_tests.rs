//! Integration tests for XAML parser.

use luma_xaml::parser::{XamlParser, ParserSettings};
use luma_xaml::types::TypeRegistry;

#[test]
fn test_parse_simple_element() {
    let xaml = r#"<Button xmlns="http://test" Content="Click Me" Width="100"/>"#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Failed to parse XAML");
    
    // Check root element
    assert_eq!(doc.root.type_name.name, "Button");
    assert_eq!(doc.root.type_name.namespace, "http://test");
    
    // Check attributes
    assert_eq!(
        doc.root.get_attribute("Content")
            .and_then(|v| v.as_string()),
        Some("Click Me")
    );
    assert_eq!(
        doc.root.get_attribute("Width")
            .and_then(|v| v.as_integer()),
        Some(100)
    );
}

#[test]
fn test_parse_nested_elements() {
    let xaml = r#"
        <Window xmlns="http://test">
            <StackPanel>
                <Button Content="Button 1"/>
                <Button Content="Button 2"/>
            </StackPanel>
        </Window>
    "#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Failed to parse XAML");
    
    // Check root
    assert_eq!(doc.root.type_name.name, "Window");
    
    // Check children
    assert_eq!(doc.root.child_elements().count(), 1);
    
    let stack_panel = doc.root.child_elements().next().unwrap();
    assert_eq!(stack_panel.type_name.name, "StackPanel");
    assert_eq!(stack_panel.child_elements().count(), 2);
    
    // Check buttons
    let buttons: Vec<_> = stack_panel.child_elements().collect();
    assert_eq!(buttons[0].type_name.name, "Button");
    assert_eq!(
        buttons[0].get_attribute("Content").and_then(|v| v.as_string()),
        Some("Button 1")
    );
    assert_eq!(buttons[1].type_name.name, "Button");
    assert_eq!(
        buttons[1].get_attribute("Content").and_then(|v| v.as_string()),
        Some("Button 2")
    );
}

#[test]
fn test_parse_text_content() {
    let xaml = r#"<TextBlock xmlns="http://test">Hello World</TextBlock>"#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Failed to parse XAML");
    
    assert_eq!(doc.root.type_name.name, "TextBlock");
    assert_eq!(doc.root.text_content(), "Hello World");
}

#[test]
fn test_parse_property_element() {
    let xaml = r#"
        <Button xmlns="http://test">
            <Button.Content>
                <TextBlock>Complex Content</TextBlock>
            </Button.Content>
        </Button>
    "#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Failed to parse XAML");
    
    assert_eq!(doc.root.type_name.name, "Button");
    
    // Check property
    let content = doc.root.get_property("Content");
    assert!(content.is_some());
    
    let text_block = content.and_then(|v| v.as_element()).unwrap();
    assert_eq!(text_block.type_name.name, "TextBlock");
    assert_eq!(text_block.text_content(), "Complex Content");
}

#[test]
fn test_parse_namespaces() {
    let xaml = r#"
        <Window
            xmlns="http://default"
            xmlns:x="http://xaml"
            xmlns:local="clr-namespace:MyApp"
            x:Name="MainWindow">
            <local:CustomControl/>
        </Window>
    "#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Failed to parse XAML");
    
    // Check root namespace
    assert_eq!(doc.root.type_name.namespace, "http://default");
    assert_eq!(doc.root.type_name.name, "Window");
    
    // Check x:Name
    assert_eq!(doc.root.name, Some("MainWindow".to_string()));
    
    // Check namespace declarations
    assert_eq!(doc.root.resolve_namespace(""), Some("http://default"));
    assert_eq!(doc.root.resolve_namespace("x"), Some("http://xaml"));
    assert_eq!(doc.root.resolve_namespace("local"), Some("clr-namespace:MyApp"));
    
    // Check child with namespace prefix
    let custom_control = doc.root.child_elements().next().unwrap();
    assert_eq!(custom_control.type_name.namespace, "clr-namespace:MyApp");
    assert_eq!(custom_control.type_name.name, "CustomControl");
}

#[test]
fn test_parse_boolean_values() {
    let xaml = r#"<CheckBox xmlns="http://test" IsChecked="true" IsEnabled="False"/>"#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Failed to parse XAML");
    
    assert_eq!(
        doc.root.get_attribute("IsChecked").and_then(|v| v.as_bool()),
        Some(true)
    );
    assert_eq!(
        doc.root.get_attribute("IsEnabled").and_then(|v| v.as_bool()),
        Some(false)
    );
}

#[test]
fn test_parse_numeric_values() {
    let xaml = r#"<Rectangle xmlns="http://test" Width="100" Height="50.5"/>"#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Failed to parse XAML");
    
    assert_eq!(
        doc.root.get_attribute("Width").and_then(|v| v.as_integer()),
        Some(100)
    );
    
    // Height is parsed as float
    let height = doc.root.get_attribute("Height");
    assert!(height.is_some());
    // Note: 50.5 will be parsed as float, not integer
}

#[test]
fn test_preserve_whitespace_flag() {
    let xaml = r#"<TextBlock xmlns="http://test">  Text with spaces  </TextBlock>"#;
    
    // Test without preserve whitespace
    let registry1 = TypeRegistry::new();
    let parser1 = XamlParser::new(registry1);
    let doc1 = parser1.parse_string(xaml).expect("Failed to parse");
    let _text1 = doc1.root.text_content();
    
    // Test with preserve whitespace
    let registry2 = TypeRegistry::new();
    let settings = ParserSettings::new().preserve_whitespace();
    let parser2 = XamlParser::new(registry2).with_settings(settings);
    let doc2 = parser2.parse_string(xaml).expect("Failed to parse");
    let text2 = doc2.root.text_content();
    
    // With preserve_whitespace, we should get the exact text including spaces
    assert_eq!(text2, "  Text with spaces  ");
}

#[test]
fn test_empty_element() {
    let xaml = r#"<Button xmlns="http://test"/>"#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Failed to parse XAML");
    
    assert_eq!(doc.root.type_name.name, "Button");
    assert_eq!(doc.root.children.len(), 0);
    assert!(!doc.root.has_children());
}

#[test]
fn test_mixed_content() {
    let xaml = r#"
        <StackPanel xmlns="http://test">
            Text before
            <Button Content="Click"/>
            Text after
        </StackPanel>
    "#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Failed to parse XAML");
    
    // Should have 3 children: text, element, text
    assert_eq!(doc.root.children.len(), 3);
    assert_eq!(doc.root.child_elements().count(), 1);
    
    // First child should be text
    let first_text = doc.root.children[0].as_text();
    assert!(first_text.is_some());
    assert!(first_text.unwrap().contains("Text before"));
    
    // Second child should be Button
    let button = doc.root.children[1].as_element();
    assert!(button.is_some());
    assert_eq!(button.unwrap().type_name.name, "Button");
    
    // Third child should be text
    let second_text = doc.root.children[2].as_text();
    assert!(second_text.is_some());
    assert!(second_text.unwrap().contains("Text after"));
}

#[test]
fn test_x_key_attribute() {
    let xaml = r#"
        <ResourceDictionary xmlns="http://test" xmlns:x="http://xaml">
            <SolidColorBrush x:Key="PrimaryBrush" Color="Blue"/>
        </ResourceDictionary>
    "#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Failed to parse XAML");
    
    let brush = doc.root.child_elements().next().unwrap();
    assert_eq!(brush.key, Some("PrimaryBrush".to_string()));
    assert!(brush.has_flag(luma_xaml::flags::ElementFlags::HAS_KEY));
}

#[test]
fn test_complex_nested_structure() {
    let xaml = r#"
        <Window xmlns="http://test" xmlns:x="http://xaml" x:Name="MainWindow">
            <Window.Resources>
                <Style x:Key="ButtonStyle">
                    <Setter Property="Background" Value="Blue"/>
                </Style>
            </Window.Resources>
            <Grid>
                <Grid.RowDefinitions>
                    <RowDefinition Height="Auto"/>
                    <RowDefinition Height="*"/>
                </Grid.RowDefinitions>
                <TextBlock Grid.Row="0" Text="Header"/>
                <StackPanel Grid.Row="1">
                    <Button Content="Button 1"/>
                    <Button Content="Button 2"/>
                </StackPanel>
            </Grid>
        </Window>
    "#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Failed to parse XAML");
    
    // Check root
    assert_eq!(doc.root.type_name.name, "Window");
    assert_eq!(doc.root.name, Some("MainWindow".to_string()));
    
    // Check that Window has Resources property and Grid child
    assert!(doc.root.get_property("Resources").is_some());
    
    // Find the Grid child
    let grid = doc.root.child_elements().next().unwrap();
    assert_eq!(grid.type_name.name, "Grid");
    
    // Grid should have RowDefinitions property
    assert!(grid.get_property("RowDefinitions").is_some());
}
