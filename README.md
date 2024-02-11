## my experimental rust gui framework
It uses both: Skia_safe as a graphics engine, Winit as a window manager, and Softbuffer to connect them.
Features:
- Ability to add custom user interface components
- Using themes and adding other themes
- and more coming soon.

### Note:
It is still in the early stages of development


## How it works?

When a new window is created using `UWindow.new()`, `UWindow.run()` after the button widget has been added to `UWindow.children`, the event loop runs, and depending on the event, the functions associated with each specific `UWidgetTtype` are activated, for example: when the movement of the mouse cursor occurs and it is on Button element,`UWindow` activates the `Button.on_hover` function of the button.
It should be noted that the tool is detected with the mouse cursor by the color of the pixels in offscreen buffer that created in `UWindow`, instead of calculating the tool’s coordinates and size. This method is easier and more accurate.




![map](https://github.com/Unique-Digital-Resources/my_rust_gui/assets/144396669/ac6b9fcc-1652-410f-80f3-c0fc37ce2760)

## Documentation

coming soon

