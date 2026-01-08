//! Tests for WinUI 3 type system.

#[cfg(feature = "winui3")]
use luma_xaml::dialects::winui3::create_type_registry;
#[cfg(feature = "winui3")]
use luma_xaml::types::XamlTypeName;
#[cfg(feature = "winui3")]
use luma_xaml::parser::XamlParser;

#[cfg(feature = "winui3")]
#[test]
fn test_registry_creation() {
    let registry = create_type_registry();
    
    // Verify namespaces are registered
    assert_eq!(
        registry.resolve_namespace(""),
        Some("http://schemas.microsoft.com/winfx/2006/xaml/presentation")
    );
    assert_eq!(
        registry.resolve_namespace("x"),
        Some("http://schemas.microsoft.com/winfx/2006/xaml")
    );
}

#[cfg(feature = "winui3")]
#[test]
fn test_base_types_registered() {
    let registry = create_type_registry();
    
    // Check base types
    let ui_element = XamlTypeName::new(
        "http://schemas.microsoft.com/winfx/2006/xaml/presentation",
        "UIElement"
    );
    assert!(registry.lookup_type(&ui_element).is_some());
    
    let framework_element = XamlTypeName::new(
        "http://schemas.microsoft.com/winfx/2006/xaml/presentation",
        "FrameworkElement"
    );
    assert!(registry.lookup_type(&framework_element).is_some());
    
    let control = XamlTypeName::new(
        "http://schemas.microsoft.com/winfx/2006/xaml/presentation",
        "Control"
    );
    assert!(registry.lookup_type(&control).is_some());
}

#[cfg(feature = "winui3")]
#[test]
fn test_control_types_registered() {
    let registry = create_type_registry();
    let ns = "http://schemas.microsoft.com/winfx/2006/xaml/presentation";
    
    // Check common controls
    assert!(registry.lookup_type(&XamlTypeName::new(ns, "Button")).is_some());
    assert!(registry.lookup_type(&XamlTypeName::new(ns, "TextBlock")).is_some());
    assert!(registry.lookup_type(&XamlTypeName::new(ns, "TextBox")).is_some());
    assert!(registry.lookup_type(&XamlTypeName::new(ns, "CheckBox")).is_some());
    assert!(registry.lookup_type(&XamlTypeName::new(ns, "RadioButton")).is_some());
}

#[cfg(feature = "winui3")]
#[test]
fn test_panel_types_registered() {
    let registry = create_type_registry();
    let ns = "http://schemas.microsoft.com/winfx/2006/xaml/presentation";
    
    // Check panels
    assert!(registry.lookup_type(&XamlTypeName::new(ns, "StackPanel")).is_some());
    assert!(registry.lookup_type(&XamlTypeName::new(ns, "Grid")).is_some());
    assert!(registry.lookup_type(&XamlTypeName::new(ns, "Canvas")).is_some());
    assert!(registry.lookup_type(&XamlTypeName::new(ns, "ScrollViewer")).is_some());
}

#[cfg(feature = "winui3")]
#[test]
fn test_button_properties() {
    let registry = create_type_registry();
    let button_type_name = XamlTypeName::new(
        "http://schemas.microsoft.com/winfx/2006/xaml/presentation",
        "Button"
    );
    
    // Get all properties including inherited ones
    let all_properties = registry.get_all_properties(&button_type_name);
    
    // Check Button has Content property (from ContentControl)
    let has_content = all_properties.iter().any(|p| p.name == "Content");
    assert!(has_content, "Button should have Content property (inherited from ContentControl)");
    
    // Check Button has Command property (defined on Button)
    let has_command = all_properties.iter().any(|p| p.name == "Command");
    assert!(has_command, "Button should have Command property");
    
    // Check Button has Background property (from Control)
    let has_background = all_properties.iter().any(|p| p.name == "Background");
    assert!(has_background, "Button should have Background property (inherited from Control)");
}

#[cfg(feature = "winui3")]
#[test]
fn test_text_block_properties() {
    let registry = create_type_registry();
    let text_block_type = registry.lookup_type(&XamlTypeName::new(
        "http://schemas.microsoft.com/winfx/2006/xaml/presentation",
        "TextBlock"
    )).expect("TextBlock type should exist");
    
    let properties = text_block_type.properties();
    
    // Check TextBlock has Text property
    let text_prop = properties.iter().find(|p| p.name == "Text");
    assert!(text_prop.is_some(), "TextBlock should have Text property");
    
    // Verify it's the content property
    assert_eq!(text_block_type.content_property(), Some("Text"));
}

#[cfg(feature = "winui3")]
#[test]
fn test_grid_attached_properties() {
    let registry = create_type_registry();
    let grid_type = registry.lookup_type(&XamlTypeName::new(
        "http://schemas.microsoft.com/winfx/2006/xaml/presentation",
        "Grid"
    )).expect("Grid type should exist");
    
    let properties = grid_type.properties();
    
    // Check Grid.Row attached property
    let row_prop = properties.iter().find(|p| p.name == "Row");
    assert!(row_prop.is_some(), "Grid should have Row attached property");
    assert!(row_prop.unwrap().is_attached(), "Row should be an attached property");
    
    // Check Grid.Column attached property
    let col_prop = properties.iter().find(|p| p.name == "Column");
    assert!(col_prop.is_some(), "Grid should have Column attached property");
    assert!(col_prop.unwrap().is_attached(), "Column should be an attached property");
}

#[cfg(feature = "winui3")]
#[test]
fn test_panel_children_property() {
    let registry = create_type_registry();
    let stack_panel_type_name = XamlTypeName::new(
        "http://schemas.microsoft.com/winfx/2006/xaml/presentation",
        "StackPanel"
    );
    
    // Get all properties including inherited ones
    let all_properties = registry.get_all_properties(&stack_panel_type_name);
    
    // Check Children property (from Panel base class)
    let children_prop = all_properties.iter().find(|p| p.name == "Children");
    assert!(children_prop.is_some(), "StackPanel should have Children property (inherited from Panel)");
    
    let children = children_prop.unwrap();
    assert!(children.is_collection(), "Children should be a collection property");
    assert!(children.is_readonly(), "Children should be readonly");
    assert!(children.is_content_property(), "Children should be the content property");
}

#[cfg(feature = "winui3")]
#[test]
fn test_parse_button_with_registry() {
    let xaml = r#"<Button xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" Content="Click Me"/>"#;
    
    let registry = create_type_registry();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Should parse Button");
    
    assert_eq!(doc.root.type_name.name, "Button");
    assert_eq!(
        doc.root.get_attribute("Content").and_then(|v| v.as_string()),
        Some("Click Me")
    );
}

#[cfg(feature = "winui3")]
#[test]
fn test_parse_stack_panel_with_buttons() {
    let xaml = r#"
        <StackPanel xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation">
            <Button Content="Button 1"/>
            <Button Content="Button 2"/>
        </StackPanel>
    "#;
    
    let registry = create_type_registry();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Should parse StackPanel");
    
    assert_eq!(doc.root.type_name.name, "StackPanel");
    assert_eq!(doc.root.child_elements().count(), 2);
}

#[cfg(feature = "winui3")]
#[test]
fn test_parse_grid_with_definitions() {
    let xaml = r#"
        <Grid xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation">
            <Grid.RowDefinitions>
                <RowDefinition Height="Auto"/>
                <RowDefinition Height="*"/>
            </Grid.RowDefinitions>
            <TextBlock Grid.Row="0" Text="Header"/>
            <TextBlock Grid.Row="1" Text="Body"/>
        </Grid>
    "#;
    
    let registry = create_type_registry();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Should parse Grid");
    
    assert_eq!(doc.root.type_name.name, "Grid");
    
    // Check RowDefinitions property element
    assert!(doc.root.get_property("RowDefinitions").is_some());
}

#[cfg(feature = "winui3")]
#[test]
fn test_parse_window() {
    let xaml = r#"
        <Window
            xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
            xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
            x:Name="MainWindow"
            Title="My Window">
            <StackPanel>
                <TextBlock Text="Hello World"/>
            </StackPanel>
        </Window>
    "#;
    
    let registry = create_type_registry();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Should parse Window");
    
    assert_eq!(doc.root.type_name.name, "Window");
    assert_eq!(doc.root.name, Some("MainWindow".to_string()));
    assert_eq!(
        doc.root.get_attribute("Title").and_then(|v| v.as_string()),
        Some("My Window")
    );
}

#[cfg(feature = "winui3")]
#[test]
fn test_type_inheritance() {
    let registry = create_type_registry();
    
    // Button inherits from ContentControl
    let button_type = registry.lookup_type(&XamlTypeName::new(
        "http://schemas.microsoft.com/winfx/2006/xaml/presentation",
        "Button"
    )).unwrap();
    
    assert_eq!(
        button_type.base_type().map(|t| t.name.as_str()),
        Some("ContentControl")
    );
    
    // ContentControl inherits from Control
    let content_control = registry.lookup_type(&XamlTypeName::new(
        "http://schemas.microsoft.com/winfx/2006/xaml/presentation",
        "ContentControl"
    )).unwrap();
    
    assert_eq!(
        content_control.base_type().map(|t| t.name.as_str()),
        Some("Control")
    );
}

#[cfg(feature = "winui3")]
#[test]
fn test_abstract_types() {
    let registry = create_type_registry();
    
    // UIElement should be abstract
    let ui_element = registry.lookup_type(&XamlTypeName::new(
        "http://schemas.microsoft.com/winfx/2006/xaml/presentation",
        "UIElement"
    )).unwrap();
    assert!(ui_element.is_abstract());
    
    // Button should not be abstract
    let button = registry.lookup_type(&XamlTypeName::new(
        "http://schemas.microsoft.com/winfx/2006/xaml/presentation",
        "Button"
    )).unwrap();
    assert!(!button.is_abstract());
}

#[cfg(not(feature = "winui3"))]
#[test]
fn test_placeholder() {
    // Placeholder test when winui3 feature is not enabled
}
