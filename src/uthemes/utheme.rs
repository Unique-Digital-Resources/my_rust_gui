// NOTES:
//==============================================================================
// written by Abdo Mahmoud (Unique-Digital-Resources): https://github.com/Unique-Digital-Resources
// Copyright 2024 Abdo Mahmoud
// Apache 2.0
//==============================================================================
// This script uses skia_safe crate/library : https://github.com/rust-skia/rust-skia
// Go to the links to find out the original authors of each library, the license and more examples if availabe.
//==============================================================================

use skia_safe::{Paint, Canvas, PaintStyle, Color, Typeface, Font, FontStyle};

pub struct UTheme{
    // btn_icon:
    // btn_icon_align:
    pub btn_color: Vec<Color>,
    pub btn_color_blend: bool,
    pub btn_font:Font,
    pub btn_font_outline_size: f32,
    pub btn_font_outline_color:Vec<Color>,
    pub btn_font_size: f32,
    // pub btn_font_align:
    pub btn_font_color: Vec<Color>,
    pub btn_font_icon_switch: bool,
    pub btn_rcorners: (f32,f32,f32,f32),
    pub btn_outline_size: f32,
    // pub btn_outline_blend:
    pub btn_outline_color: Vec<Color>,
    pub btn_text_icon_separation: f32, 

    // btn_prsd_color: Vec<Color>,
    // btn_prsd_color_blend: bool,
    // btn_prsd_font:Font,
    // btn_prsd_font_outline_size: f32,
    // btn_prsd_font_outline_color:Vec<Color>,
    // btn_prsd_font_size: f32,
    // // btn_prsd_font_align:
    // btn_prsd_font_color: Vec<Color>,
    // btn_prsd_font_icon_switch: bool,
    // btn_prsd_rcorners: (f32,f32,f32,f32),
    // btn_prsd_outline_size: f32,
    // // btn_prsd_outline_blend:
    // btn_prsd_outline_color: Vec<Color>,
    // btn_prsd_text_icon_separation: f32, 

    // btn_dis_color: Vec<Color>,
    // btn_dis_color_blend: bool,
    // btn_dis_font:Font,
    // btn_dis_font_outline_size: f32,
    // btn_dis_font_outline_color:Vec<Color>,
    // btn_dis_font_size: f32,
    // // btn_dis_font_align:
    // btn_dis_font_color: Vec<Color>,
    // btn_dis_font_icon_switch: bool,
    // btn_dis_rcorners: (f32,f32,f32,f32),
    // btn_dis_outline_size: f32,
    // // btn_dis_outline_blend:
    // btn_dis_outline_color: Vec<Color>,
    // btn_dis_text_icon_separation: f32, 

    // pub btn_hvr_color: Vec<Color>,
    // pub btn_hvr_color_blend: bool,
    // pub btn_hvr_font:Font,
    // pub btn_hvr_font_outline_size: f32,
    // pub btn_hvr_font_outline_color:Vec<Color>,
    // pub btn_hvr_font_size: f32,
    // pub btn_hvr_font_align:
    // pub btn_hvr_font_color: Vec<Color>,
    // pub btn_hvr_font_icon_switch: bool,
    // pub btn_hvr_rcorners: (f32,f32,f32,f32),
    // pub btn_hvr_outline_size: f32,
    // pub btn_hvr_outline_blend:
    // pub btn_hvr_outline_color: Vec<Color>,
    // pub btn_hvr_text_icon_separation: f32, 

    // btn_focus_color: Vec<Color>,
    // btn_focus_color_blend: bool,
    // btn_focus_font:Font,
    // btn_focus_font_outline_size: f32,
    // btn_focus_font_outline_color:Vec<Color>,
    // btn_focus_font_size: f32,
    // // btn_focus_font_align:
    // btn_focus_font_color: Vec<Color>,
    // btn_focus_font_icon_switch: bool,
    // btn_focus_rcorners: (f32,f32,f32,f32),
    // btn_focus_outline_size: f32,
    // // btn_focus_outline_blend:
    // btn_focus_outline_color: Vec<Color>,
    // btn_focus_text_icon_separation: f32, 
}


pub trait UThmTrait{
    // fn new(&self)-> UTheme{}
    fn draw_btn(&mut self, visible_buffer: &Canvas, screenoff_buffer: &Canvas, screenoff_color: &Color, xpos: f32, ypos: f32, w: f32, h: f32 );
    fn draw_on_hover_btn(&mut self, visible_buffer: &Canvas, screenoff_buffer: &Canvas, screenoff_color: &Color, xpos: f32, ypos: f32, w: f32, h: f32 );
}

// impl UThmTrait for UTheme {
//     fn draw_btn(&mut self, visible_buffer: &Canvas, screenoff_buffer: &Canvas, screenoff_color: &Color, xpos: f32, ypos: f32, w: f32, h: f32 ) {
//         let mut thmpaint = Paint::default();
//         thmpaint.set_color(Color::from_rgb(255, 0, 150));
//         thmpaint.set_style(PaintStyle::Fill);
//         thmpaint.set_anti_alias(true);
//         visible_buffer.draw_circle((xpos, ypos), 100.0, &thmpaint);
//         thmpaint.set_color(*screenoff_color);
//         thmpaint.set_anti_alias(false);
//         screenoff_buffer.draw_circle((xpos, ypos), 100.0, &thmpaint);
//     }
// }
