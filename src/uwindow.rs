use crate::uwidgets::uwidget::UWidget;
use crate::uwidgets::uwidgetype::WidgetType;
use crate::uwidgets::button::Button;
use crate::uthemes::utheme::{UTheme, UThmTrait};
use crate::uthemes::default::ThmDefault;
use std::num::NonZeroU32;
use softbuffer::{Context, Surface};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Window};
use skia_safe::{surfaces,Color, Canvas, ImageInfo, ColorType, AlphaType, ColorSpace, SurfaceProps, Paint, PaintStyle};
use rand::{self, Rng};

enum RedrawRequestReason {
    DueUControl,
    DueUWindow
}


pub struct UWindow{
    pub window : WindowBuilder,
    pub theme : Vec<Box<dyn UThmTrait>>,
    children: Vec<UWidget>,
    children_screenoff_color: Vec<Color>,
    event_handler: Option<Box<dyn FnMut(Event<()>, &mut Self)>>,
    redraw_request_reason: RedrawRequestReason,
}


impl UWindow{

    pub fn new(event_handler: impl FnMut(Event<()>, &mut Self) + 'static)->UWindow{

        let window = WindowBuilder::new();

        return UWindow {
            window: window,
            theme: vec![Box::new(ThmDefault::new()) as Box<dyn UThmTrait>],
            children: Vec::new(),
            children_screenoff_color: Vec::new(),
            event_handler: Some(Box::new(event_handler)),
            redraw_request_reason: RedrawRequestReason::DueUWindow,
        }
    }


    pub fn run(mut self, an_event_loop: EventLoop<()>, mut event_handler: impl FnMut(Event<()>, &mut Self)){

        let w =self.window.build(&an_event_loop).unwrap();
        let context = unsafe { Context::new(&w) }.unwrap();
        let mut surface = unsafe { Surface::new(&context, &w) }.unwrap();
        let mut last_pixel = Color::from_argb(0 , 0 , 0 , 0);
        let mut _current_pixel  = Color::from_argb(0 , 0 , 0 , 0);
        
        let (width, height) = {
            let size = w.inner_size();
            (size.width, size.height)
        };
            
        let mut ii = ImageInfo::new((width as i32,height as i32), ColorType::RGBA8888, AlphaType::Premul, ColorSpace::new_srgb());
        let mut visible_buffer = surfaces::raster(&ii,(0) as usize, Some(&SurfaceProps::default())).expect("surface");
        let mut screenoff_buffer = surfaces::raster(&ii,(0) as usize, Some(&SurfaceProps::default())).expect("surface");


        an_event_loop.run(move |event, _, control_flow| {

            *control_flow = ControlFlow::Wait;
        
            match event {

                Event::RedrawRequested(window_id) if window_id == w.id() => {
                    let (width, height) = {
                        let size =w.inner_size();
                        (size.width, size.height)
                    };
                    surface
                        .resize(
                            NonZeroU32::new(width).unwrap(),
                            NonZeroU32::new(height).unwrap(),
                        )
                        .unwrap();


                    let iii = ImageInfo::new((width as i32,height as i32), ColorType::RGBA8888, AlphaType::Premul, ColorSpace::new_srgb());
                    visible_buffer = surfaces::raster(&iii,(0) as usize, Some(&SurfaceProps::default())).expect("surface");
                    screenoff_buffer = surfaces::raster(&iii, 0, Some(&SurfaceProps::default())).expect("surface");
                    visible_buffer.canvas().clear(Color::WHITE);
                    screenoff_buffer.canvas().clear(Color::WHITE);
                                        
                    for i in 0..self.children.len() {
                        if let Some(theme) = self.theme.get_mut(0){
                            match &self.children[i].wdgt {
                                WidgetType::Button(button) =>{
                                self.children[i].rect_position = ((width / 2) as f32, (height / 2) as f32);
                                theme.draw_btn(visible_buffer.canvas(), screenoff_buffer.canvas(), &self.children_screenoff_color[i], self.children[i].rect_position.0, self.children[i].rect_position.1, 10.0, 10.0);
                            }
                        }
                    }
                }

                
                    let pixmap = visible_buffer.peek_pixels().unwrap().bytes().unwrap();
                    let mut buffer = surface.buffer_mut().unwrap();

                    for index in 0..(width * height) as usize {
                        buffer[index] = pixmap[index * 4 + 2] as u32
                            | (pixmap[index * 4 + 1] as u32) << 8
                            | (pixmap[index * 4] as u32) << 16;
                    }
                    buffer.present().unwrap();
                }
    
            Event::WindowEvent { event, .. } => match event {
                // Close the software when window closed
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::CursorMoved { position,device_id , modifiers } 
                    => {
                        let x = position.x as i32;
                        let y = position.y as i32;
                        
                        if (x <=  screenoff_buffer.image_info().bounds().width()) &&(y <= screenoff_buffer.image_info().bounds().height()) && (x>0) && (y>0){
                            _current_pixel = screenoff_buffer.peek_pixels().unwrap().get_color((x,y));
                        }
                        if last_pixel != _current_pixel {
                            last_pixel = _current_pixel;
                            println!("mouse pixel: {last_pixel:?}");
                            if self.children_screenoff_color.contains(&_current_pixel){
                                let cindex = self.children_screenoff_color.iter().position(|&c| c == _current_pixel).unwrap();
                                    if let Some(theme) = self.theme.get_mut(0){
                                        match &self.children[cindex].wdgt {
                                            WidgetType::Button(button) => {
                                                theme.draw_btn(visible_buffer.canvas(), screenoff_buffer.canvas(), &self.children_screenoff_color[cindex], self.children[cindex].rect_position.0, self.children[cindex].rect_position.1, 10.0, 10.0);
                                                // Run on_hover function
                                                (button.on_hover)(&self.children[cindex]);

                                            }
                                            // Add other branches for different widget types if needed
                                        }
                                    }
                            }                            
                        };
                    }
            _ => {}
        }_ => {}
    
        }; 
        });


    }

    pub fn add_child(&mut self, child: UWidget) {
        self.children.push(child);

        // Generate a random unique color
        let mut rng = rand::thread_rng();
        let mut new_color;
        loop {
            new_color = Color::from_rgb(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>());
            if !self.children_screenoff_color.contains(&new_color) {
                break;
            }
        }

        // Add the unique color to UWindow.children_screenoff_color
        self.children_screenoff_color.push(new_color);
    }
}

