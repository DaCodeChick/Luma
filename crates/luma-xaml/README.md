# luma-xaml

Pure Rust XAML parser supporting WinUI 3, WPF, and generic XAML.

## Status

**⚠️ Early Development**: This crate is in the initial development phase. The API is unstable and many features are not yet implemented.

## Features

- Pure Rust implementation (no C++ dependencies)
- Support for multiple XAML dialects:
  - WinUI 3 (Microsoft.UI.Xaml)
  - WPF (System.Windows)
  - Generic XAML
- Type system with metadata
- Markup extensions ({Binding}, {StaticResource}, etc.)
- Resource dictionaries
- Namespace handling

## Architecture

`luma-xaml` is designed to be a pure parsing library. It converts XAML text into an in-memory object model but does not render or instantiate actual UI controls. This design allows it to be used by different backends (like `luma-winui`) or for XAML analysis tools.

## Usage

**Note**: Most functionality is not yet implemented. This is a placeholder for the planned API.

```rust
use luma_xaml::{XamlParser, dialects::winui3};

// Create a parser with WinUI 3 type registry
let registry = winui3::create_type_registry();
let parser = XamlParser::new(registry);

// Parse a XAML file (not yet implemented)
// let document = parser.parse_file("MainWindow.xaml")?;
```

## Roadmap

### Phase 1: Foundation (Current)
- [x] Project structure
- [x] Error types
- [x] Core data structures (XamlElement, XamlValue, etc.)
- [ ] XML parsing foundation
- [ ] Basic element parsing

### Phase 2: Type System
- [ ] Type registry implementation
- [ ] WinUI 3 type definitions
- [ ] Property metadata
- [ ] Type validation

### Phase 3: Markup Extensions
- [ ] Markup extension parsing
- [ ] StaticResource
- [ ] Binding
- [ ] Other common extensions

### Phase 4: Advanced Features
- [ ] Resource dictionaries
- [ ] Attached properties
- [ ] Content properties
- [ ] Collections

## Design Philosophy

This crate prioritizes:

1. **Quality over speed**: Thorough implementation with comprehensive error handling
2. **Flexibility**: Support multiple XAML dialects
3. **Safety**: Pure Rust with no unsafe code
4. **Documentation**: Well-documented public API
5. **Testing**: Comprehensive test coverage

## Contributing

This is currently a personal passion project. Contributions will be welcome once the core architecture stabilizes.

## License

Licensed under either of:

- MIT license
- Apache License, Version 2.0

at your option.
