extern crate my_gui_v0_0_1;
use my_gui_v0_0_1::uwindow::UWindow;
use my_gui_v0_0_1::uwidgets::uwidget::UWidget;
use my_gui_v0_0_1::uwidgets::button::Button;
use my_gui_v0_0_1::uwidgets::uwidgetype::WidgetType;
use winit::event_loop::EventLoop;
use skia_safe::Color;

fn main() {
    // Create an EventLoop
    let event_loop = EventLoop::new();
    let mut btn = UWidget::new();
    
    let mut button_instance = Button::new();
    button_instance.on_hover = Box::new(|_t| test_fn());
    btn.wdgt = WidgetType::Button(button_instance);

    let mut uw = UWindow::new(|event, uwindow|{});
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
