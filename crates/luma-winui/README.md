# luma-winui

WinUI 3 backend for Luma GUI framework.

## Status

**⚠️ Proof-of-Concept**: This crate is in the very early proof-of-concept phase. It is highly experimental and not recommended for production use. Use `luma-windows` (Win32 backend) for stable applications.

## Warning

Based on extensive research (see `RESEARCH_RUST_XAML.md` in the repository root), this backend faces significant challenges:

- WinUI 3 has undocumented behavior and assumptions about host processes
- Microsoft explicitly warned that XAML support outside C# is "unsustainable"
- You would be the first Rust user, meaning all bugs are yours to solve
- Breaking changes occur between WinUI 3 releases

**Exit Criteria**: If the proof-of-concept phase reveals critical instability or unsolvable issues, this backend will be abandoned in favor of Win32 + custom rendering.

## Requirements

- Windows 10 version 1809 or later (Windows 11 recommended)
- Windows App SDK runtime installed
- Visual C++ Redistributables

## Features

- Modern Windows 11 Fluent Design UI
- Native dark mode support
- WinUI 3 controls (Button, TextBlock, NavigationView, etc.)
- Optional XAML loading (via `luma-xaml`)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
luma-winui = "0.0.1"
```

For XAML support:

```toml
[dependencies]
luma-winui = { version = "0.0.1", features = ["xaml-support"] }
```

## Usage

**Note**: Most functionality is not yet implemented.

```rust
use luma_winui::prelude::*;

fn main() -> Result<()> {
    // Initialize WinUI runtime
    let _runtime = WinUIRuntime::initialize()?;
    
    // Create a window (not yet implemented)
    // let window = WinUIWindow::new("Hello WinUI", 800, 600)?;
    // window.show()?;
    
    Ok(())
}
```

## Roadmap

### Phase 1: Proof-of-Concept (Month 7-9)
- [ ] Windows App SDK initialization
- [ ] Basic window creation
- [ ] Button widget
- [ ] TextBlock widget
- [ ] StackPanel layout
- [ ] 100+ hours stability testing
- [ ] **GO/NO-GO DECISION POINT**

If POC succeeds:

### Phase 2: Widget Set (Month 10-11)
- [ ] Complete basic widgets
- [ ] Layout integration with Luma
- [ ] Event handling

### Phase 3: XAML Integration (Month 12+)
- [ ] XAML loading via `luma-xaml`
- [ ] Resource dictionaries
- [ ] Styles and templates

## Architecture

`luma-winui` implements Luma's backend traits using WinUI 3 controls from the Windows App SDK. It can optionally consume `luma-xaml` for declarative UI support.

## Contributing

This is currently a personal passion project. The API will remain unstable until the proof-of-concept phase completes.

## License

Licensed under either of:

- MIT license
- Apache License, Version 2.0

at your option.
