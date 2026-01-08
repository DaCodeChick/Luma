//! Integration tests for value converters in XAML parsing.

use luma_xaml::{XamlParser, ParserSettings, XamlValue};
use luma_xaml::dialects::winui3;
use luma_xaml::converters::{parse_thickness, parse_brush, parse_grid_length, parse_corner_radius, GridLength};

#[test]
fn test_parse_xaml_with_thickness_uniform() {
    let xaml = r#"<Button xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" Margin="10" />"#;
    
    let registry = winui3::create_type_registry();
    let parser = XamlParser::new(registry).with_settings(ParserSettings::default());
    let doc = parser.parse_string(xaml).unwrap();
    
    let margin = doc.root.attributes.get("Margin").unwrap();
    match margin {
        XamlValue::String(val) => {
            let thickness = parse_thickness(val).unwrap();
            assert_eq!(thickness.left, 10.0);
            assert_eq!(thickness.top, 10.0);
            assert_eq!(thickness.right, 10.0);
            assert_eq!(thickness.bottom, 10.0);
        }
        XamlValue::Integer(i) => {
            // Parser may parse simple numbers as integers
            let thickness = parse_thickness(&i.to_string()).unwrap();
            assert_eq!(thickness.left, 10.0);
        }
        _ => panic!("Expected string or integer value for Margin"),
    }
}

#[test]
fn test_parse_xaml_with_thickness_symmetric() {
    let xaml = r#"<Button xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" Padding="10,5" />"#;
    
    let registry = winui3::create_type_registry();
    let parser = XamlParser::new(registry).with_settings(ParserSettings::default());
    let doc = parser.parse_string(xaml).unwrap();
    
    let padding = doc.root.attributes.get("Padding").unwrap();
    if let XamlValue::String(val) = padding {
        let thickness = parse_thickness(val).unwrap();
        assert_eq!(thickness.left, 10.0);
        assert_eq!(thickness.top, 5.0);
        assert_eq!(thickness.right, 10.0);
        assert_eq!(thickness.bottom, 5.0);
    } else {
        panic!("Expected string value for Padding");
    }
}

#[test]
fn test_parse_xaml_with_thickness_four_values() {
    let xaml = r#"<Button xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" Margin="1,2,3,4" />"#;
    
    let registry = winui3::create_type_registry();
    let parser = XamlParser::new(registry).with_settings(ParserSettings::default());
    let doc = parser.parse_string(xaml).unwrap();
    
    let margin = doc.root.attributes.get("Margin").unwrap();
    if let XamlValue::String(val) = margin {
        let thickness = parse_thickness(val).unwrap();
        assert_eq!(thickness.left, 1.0);
        assert_eq!(thickness.top, 2.0);
        assert_eq!(thickness.right, 3.0);
        assert_eq!(thickness.bottom, 4.0);
    } else {
        panic!("Expected string value for Margin");
    }
}

#[test]
fn test_parse_xaml_with_brush_hex() {
    let xaml = r#"<Button xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" Background='#FF0000' />"#;
    
    let registry = winui3::create_type_registry();
    let parser = XamlParser::new(registry).with_settings(ParserSettings::default());
    let doc = parser.parse_string(xaml).unwrap();
    
    let background = doc.root.attributes.get("Background").unwrap();
    if let XamlValue::String(val) = background {
        let brush = parse_brush(val).unwrap();
        assert_eq!(brush, "#FF0000");
    } else {
        panic!("Expected string value for Background");
    }
}

#[test]
fn test_parse_xaml_with_brush_named() {
    let xaml = r#"<TextBlock xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" Foreground="Red" />"#;
    
    let registry = winui3::create_type_registry();
    let parser = XamlParser::new(registry).with_settings(ParserSettings::default());
    let doc = parser.parse_string(xaml).unwrap();
    
    let foreground = doc.root.attributes.get("Foreground").unwrap();
    if let XamlValue::String(val) = foreground {
        let brush = parse_brush(val).unwrap();
        assert_eq!(brush, "Red");
    } else {
        panic!("Expected string value for Foreground");
    }
}

#[test]
fn test_parse_xaml_with_corner_radius_uniform() {
    let xaml = r#"<Border xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" CornerRadius="5" />"#;
    
    let registry = winui3::create_type_registry();
    let parser = XamlParser::new(registry).with_settings(ParserSettings::default());
    let doc = parser.parse_string(xaml).unwrap();
    
    let corner_radius = doc.root.attributes.get("CornerRadius").unwrap();
    match corner_radius {
        XamlValue::String(val) => {
            let cr = parse_corner_radius(val).unwrap();
            assert_eq!(cr.top_left, 5.0);
            assert_eq!(cr.top_right, 5.0);
            assert_eq!(cr.bottom_right, 5.0);
            assert_eq!(cr.bottom_left, 5.0);
        }
        XamlValue::Integer(i) => {
            let cr = parse_corner_radius(&i.to_string()).unwrap();
            assert_eq!(cr.top_left, 5.0);
        }
        _ => panic!("Expected string or integer value for CornerRadius"),
    }
}

#[test]
fn test_parse_xaml_with_corner_radius_four_values() {
    let xaml = r#"<Border xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" CornerRadius="1,2,3,4" />"#;
    
    let registry = winui3::create_type_registry();
    let parser = XamlParser::new(registry).with_settings(ParserSettings::default());
    let doc = parser.parse_string(xaml).unwrap();
    
    let corner_radius = doc.root.attributes.get("CornerRadius").unwrap();
    if let XamlValue::String(val) = corner_radius {
        let cr = parse_corner_radius(val).unwrap();
        assert_eq!(cr.top_left, 1.0);
        assert_eq!(cr.top_right, 2.0);
        assert_eq!(cr.bottom_right, 3.0);
        assert_eq!(cr.bottom_left, 4.0);
    } else {
        panic!("Expected string value for CornerRadius");
    }
}

#[test]
fn test_parse_xaml_with_grid_length_auto() {
    let xaml = r#"<RowDefinition xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" Height="Auto"/>"#;
    
    let registry = winui3::create_type_registry();
    let parser = XamlParser::new(registry).with_settings(ParserSettings::default());
    let doc = parser.parse_string(xaml).unwrap();
    
    if let Some(XamlValue::String(val)) = doc.root.attributes.get("Height") {
        let grid_length = parse_grid_length(val).unwrap();
        assert_eq!(grid_length, GridLength::Auto);
    } else {
        panic!("Expected string value for Height");
    }
}

#[test]
fn test_parse_xaml_with_grid_length_star() {
    let xaml = r#"<ColumnDefinition xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" Width="*"/>"#;
    
    let registry = winui3::create_type_registry();
    let parser = XamlParser::new(registry).with_settings(ParserSettings::default());
    let doc = parser.parse_string(xaml).unwrap();
    
    if let Some(XamlValue::String(val)) = doc.root.attributes.get("Width") {
        let grid_length = parse_grid_length(val).unwrap();
        assert_eq!(grid_length, GridLength::Star(1.0));
    } else {
        panic!("Expected string value for Width");
    }
}

#[test]
fn test_parse_xaml_with_grid_length_star_multiplier() {
    let xaml = r#"<RowDefinition xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" Height="2*"/>"#;
    
    let registry = winui3::create_type_registry();
    let parser = XamlParser::new(registry).with_settings(ParserSettings::default());
    let doc = parser.parse_string(xaml).unwrap();
    
    if let Some(XamlValue::String(val)) = doc.root.attributes.get("Height") {
        let grid_length = parse_grid_length(val).unwrap();
        assert_eq!(grid_length, GridLength::Star(2.0));
    } else {
        panic!("Expected string value for Height");
    }
}

#[test]
fn test_parse_xaml_with_grid_length_absolute() {
    let xaml = r#"<ColumnDefinition xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" Width="100"/>"#;
    
    let registry = winui3::create_type_registry();
    let parser = XamlParser::new(registry).with_settings(ParserSettings::default());
    let doc = parser.parse_string(xaml).unwrap();
    
    let width = doc.root.attributes.get("Width").unwrap();
    match width {
        XamlValue::String(val) => {
            let grid_length = parse_grid_length(val).unwrap();
            assert_eq!(grid_length, GridLength::Absolute(100.0));
        }
        XamlValue::Integer(i) => {
            let grid_length = parse_grid_length(&i.to_string()).unwrap();
            assert_eq!(grid_length, GridLength::Absolute(100.0));
        }
        _ => panic!("Expected string or integer value for Width"),
    }
}

#[test]
fn test_parse_complex_xaml_with_multiple_converters() {
    let xaml = r#"<Border xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" Background='#FF0000' CornerRadius="5" Padding="10,5" />"#;
    
    let registry = winui3::create_type_registry();
    let parser = XamlParser::new(registry).with_settings(ParserSettings::default());
    let doc = parser.parse_string(xaml).unwrap();
    
    // Verify parsing succeeded
    assert_eq!(doc.root.type_name.name, "Border");
    
    // Verify Background
    if let XamlValue::String(val) = doc.root.attributes.get("Background").unwrap() {
        let brush = parse_brush(val).unwrap();
        assert_eq!(brush, "#FF0000");
    }
    
    // Verify CornerRadius
    if let XamlValue::String(val) = doc.root.attributes.get("CornerRadius").unwrap() {
        let cr = parse_corner_radius(val).unwrap();
        assert_eq!(cr.top_left, 5.0);
    }
    
    // Verify Padding
    if let XamlValue::String(val) = doc.root.attributes.get("Padding").unwrap() {
        let thickness = parse_thickness(val).unwrap();
        assert_eq!(thickness.left, 10.0);
        assert_eq!(thickness.top, 5.0);
    }
}
