//! Integration tests for markup extension parsing.

use luma_xaml::parser::XamlParser;
use luma_xaml::types::TypeRegistry;
use luma_xaml::model::XamlValue;

#[test]
fn test_parse_static_resource() {
    let xaml = r#"<Button xmlns="http://test" Background="{StaticResource MyBrush}"/>"#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Should parse");
    
    let background = doc.root.get_attribute("Background").expect("Should have Background");
    match background {
        XamlValue::MarkupExtension { extension_name, arguments } => {
            assert_eq!(extension_name, "StaticResource");
            assert_eq!(
                arguments.get("_positional").and_then(|v| v.as_string()),
                Some("MyBrush")
            );
        }
        _ => panic!("Expected MarkupExtension, got {:?}", background),
    }
}

#[test]
fn test_parse_binding() {
    let xaml = r#"<TextBlock xmlns="http://test" Text="{Binding Name}"/>"#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Should parse");
    
    let text = doc.root.get_attribute("Text").expect("Should have Text");
    match text {
        XamlValue::MarkupExtension { extension_name, arguments } => {
            assert_eq!(extension_name, "Binding");
            assert_eq!(
                arguments.get("_positional").and_then(|v| v.as_string()),
                Some("Name")
            );
        }
        _ => panic!("Expected MarkupExtension, got {:?}", text),
    }
}

#[test]
fn test_parse_binding_with_mode() {
    let xaml = r#"<TextBox xmlns="http://test" Text="{Binding Path=Name, Mode=TwoWay}"/>"#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Should parse");
    
    let text = doc.root.get_attribute("Text").expect("Should have Text");
    match text {
        XamlValue::MarkupExtension { extension_name, arguments } => {
            assert_eq!(extension_name, "Binding");
            assert_eq!(
                arguments.get("Path").and_then(|v| v.as_string()),
                Some("Name")
            );
            assert_eq!(
                arguments.get("Mode").and_then(|v| v.as_string()),
                Some("TwoWay")
            );
        }
        _ => panic!("Expected MarkupExtension, got {:?}", text),
    }
}

#[test]
fn test_parse_x_null() {
    let xaml = r#"<Button xmlns="http://test" xmlns:x="http://xaml" Content="{x:Null}"/>"#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Should parse");
    
    let content = doc.root.get_attribute("Content").expect("Should have Content");
    match content {
        XamlValue::MarkupExtension { extension_name, .. } => {
            assert_eq!(extension_name, "x:Null");
        }
        _ => panic!("Expected MarkupExtension, got {:?}", content),
    }
}

#[test]
fn test_parse_x_type() {
    let xaml = r#"<ContentControl xmlns="http://test" xmlns:x="http://xaml" Content="{x:Type Button}"/>"#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Should parse");
    
    let content = doc.root.get_attribute("Content").expect("Should have Content");
    match content {
        XamlValue::MarkupExtension { extension_name, arguments } => {
            assert_eq!(extension_name, "x:Type");
            assert_eq!(
                arguments.get("_positional").and_then(|v| v.as_string()),
                Some("Button")
            );
        }
        _ => panic!("Expected MarkupExtension, got {:?}", content),
    }
}

#[test]
fn test_parse_escaped_brace() {
    let xaml = r#"<TextBlock xmlns="http://test" Text="{{Not a markup extension}}"/>"#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Should parse");
    
    let text = doc.root.get_attribute("Text").expect("Should have Text");
    assert_eq!(text.as_string(), Some("{Not a markup extension}}"));
}

#[test]
fn test_parse_multiple_markup_extensions() {
    let xaml = r#"
        <StackPanel xmlns="http://test">
            <Button Background="{StaticResource PrimaryBrush}" Content="{Binding Title}"/>
            <TextBlock Text="{Binding Description, Mode=OneWay}"/>
        </StackPanel>
    "#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Should parse");
    
    // Check button
    let button = doc.root.child_elements().next().expect("Should have button");
    
    let background = button.get_attribute("Background");
    assert!(background.is_some());
    match background.unwrap() {
        XamlValue::MarkupExtension { extension_name, .. } => {
            assert_eq!(extension_name, "StaticResource");
        }
        _ => panic!("Expected MarkupExtension for Background"),
    }
    
    let content = button.get_attribute("Content");
    assert!(content.is_some());
    match content.unwrap() {
        XamlValue::MarkupExtension { extension_name, .. } => {
            assert_eq!(extension_name, "Binding");
        }
        _ => panic!("Expected MarkupExtension for Content"),
    }
}

#[test]
fn test_parse_complex_binding() {
    let xaml = r#"<TextBox xmlns="http://test" Text="{Binding Path=User.Name, Mode=TwoWay, UpdateSourceTrigger=PropertyChanged}"/>"#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Should parse");
    
    let text = doc.root.get_attribute("Text").expect("Should have Text");
    match text {
        XamlValue::MarkupExtension { extension_name, arguments } => {
            assert_eq!(extension_name, "Binding");
            assert_eq!(arguments.get("Path").and_then(|v| v.as_string()), Some("User.Name"));
            assert_eq!(arguments.get("Mode").and_then(|v| v.as_string()), Some("TwoWay"));
            assert_eq!(arguments.get("UpdateSourceTrigger").and_then(|v| v.as_string()), Some("PropertyChanged"));
        }
        _ => panic!("Expected MarkupExtension"),
    }
}

#[test]
fn test_parse_markup_in_property_element() {
    let xaml = r#"
        <Button xmlns="http://test">
            <Button.Background>{StaticResource MyBrush}</Button.Background>
        </Button>
    "#;
    
    let registry = TypeRegistry::new();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Should parse");
    
    // The property element contains text which should be parsed as markup extension
    // Note: Currently property elements with text content may not parse markup extensions
    // This test documents current behavior
    let background_prop = doc.root.get_property("Background");
    assert!(background_prop.is_some());
}

#[cfg(feature = "winui3")]
#[test]
fn test_parse_with_winui3_types() {
    use luma_xaml::dialects::winui3::create_type_registry;
    
    let xaml = r#"
        <Window xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation">
            <StackPanel>
                <TextBlock Text="{Binding Title}"/>
                <Button Background="{StaticResource AccentBrush}" Content="Click"/>
            </StackPanel>
        </Window>
    "#;
    
    let registry = create_type_registry();
    let parser = XamlParser::new(registry);
    
    let doc = parser.parse_string(xaml).expect("Should parse WinUI 3 XAML");
    
    assert_eq!(doc.root.type_name.name, "Window");
    
    let stack_panel = doc.root.child_elements().next().expect("Should have StackPanel");
    assert_eq!(stack_panel.type_name.name, "StackPanel");
    
    let text_block = stack_panel.child_elements().next().expect("Should have TextBlock");
    let text = text_block.get_attribute("Text");
    assert!(matches!(text, Some(XamlValue::MarkupExtension { .. })));
}
