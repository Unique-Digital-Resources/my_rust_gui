// NOTES:
//==============================================================================
// written by Abdo Mahmoud (Unique-Digital-Resources): https://github.com/Unique-Digital-Resources
// Copyright 2024 Abdo Mahmoud
// Apache 2.0
//==============================================================================
// This script uses winit crate/library : https://github.com/rust-windowing/winit
// Go to the links to find out the original authors of each library, the license and more examples if availabe.
//==============================================================================

extern crate my_gui_v0_0_1;
use my_gui_v0_0_1::uwindow::UWindow;
use my_gui_v0_0_1::uwidgets::uwidget::UWidget;
use my_gui_v0_0_1::uwidgets::button::Button;
use my_gui_v0_0_1::uwidgets::uwidgetype::WidgetType;
use my_gui_v0_0_1::uthemes::glassy::ThmGlassy;
use winit::event_loop::EventLoop;


fn main() {

    // Create an EventLoop
    let event_loop = EventLoop::new();

    // Create new button widget and add function for mouse hover event to it
    let mut btn = UWidget::new();
    let mut button_instance = Button::new();
    button_instance.on_hover = Box::new(|_t| test_fn());
    btn.wdgt = WidgetType::Button(button_instance);

    // Create new window ,change its theme and add the created button widget to its children
    let mut uw = UWindow::new(|event, uwindow|{});
    uw.set_theme(Box::new(ThmGlassy::new()));
    uw.add_child(btn);

    uw.run(event_loop, |event, uwindow| {
        // Handle events based on the event and uwindow reference
        
        // match event {
        //     Event::WindowEvent { event, .. } => {
        //         // Handle window events
        //     }
        //     // Handle other events as needed
        //     _ => {}
        // }
    });
}

fn test_fn(){
    println!("{}", "It's woooorking!!!, Yaaaaaaaaay")
}
