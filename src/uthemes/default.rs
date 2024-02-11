// NOTES:
//==============================================================================
// written by Abdo Mahmoud (Unique-Digital-Resources): https://github.com/Unique-Digital-Resources
// Copyright 2024 Abdo Mahmoud
// Apache 2.0
//==============================================================================
// This script uses skia_safe crate/library : https://github.com/rust-skia/rust-skia
// Go to the links to find out the original authors of each library, the license and more examples if availabe.
//==============================================================================

use crate::uthemes::utheme::{UTheme, UThmTrait};
use skia_safe::{wrapper::ValueWrapper, Canvas, Color, Font, FontStyle, Paint, PaintStyle, Typeface, Rect};

pub struct ThmDefault {
    pub theme: UTheme,
    }

impl ThmDefault{
    pub fn new()-> Self{

        // Use the default Skia Typeface (which is a sans-serif font)
        let typeface = Typeface::default();

        // Create a Skia Font with the default typeface
        let font = Font::from_typeface(typeface, 24.0);


        return ThmDefault{
            theme: UTheme {
            btn_color: vec![Color::from_rgb(255, 100, 20), Color::from_rgb(25, 100, 200)],
            btn_color_blend: false,
            btn_font:font,
            btn_font_outline_size: 2.0,
            btn_font_outline_color:vec![Color::from_rgb(20, 10, 220), Color::from_rgb(25, 150, 30)],
            btn_font_size: 45.0,
            // btn_font_align:
            btn_font_color: vec![Color::from_rgb(0, 0, 0)],
            btn_font_icon_switch: false,
            btn_rcorners: (20.0,50.0,10.0,10.0),
            btn_outline_size: 0.0,
            // pub btn_outline_blend:
            btn_outline_color: vec![],
            btn_text_icon_separation: 0.0, 
// 
            // btn_hvr_color: Vec<Color>,
            // btn_hvr_color_blend: bool,
            // btn_hvr_font:Font,
            // btn_hvr_font_outline_size: f32,
            // btn_hvr_font_outline_color:Vec<Colo>,
            // btn_hvr_font_size: f32,
            // pub btn_hvr_font_align:
            // btn_hvr_font_color: Vec<Color>,
            // btn_hvr_font_icon_switch: bool,
            // btn_hvr_rcorners: (f32,f32,f32,f32),
            // btn_hvr_outline_size: f32,
            // pub btn_hvr_outline_blend:
            // btn_hvr_outline_color: Vec<Color>,
            // btn_hvr_text_icon_separation: f32, 
        }
    }
}
}


impl UThmTrait for ThmDefault{
    fn draw_btn(&mut self, visible_buffer: &Canvas, screenoff_buffer: &Canvas, screenoff_color: &Color, xpos: f32, ypos: f32, w: f32, h: f32 ) {
        let mut thmpaint = Paint::default();
        let rect = Rect::new(xpos, ypos, xpos+w, ypos+h);
        thmpaint.set_color(Color::from_rgb(255, 0, 0));
        thmpaint.set_style(PaintStyle::Fill);
        thmpaint.set_anti_alias(true);
        visible_buffer.draw_round_rect(rect, 10.0, 10.0, &thmpaint);
        thmpaint.set_color(*screenoff_color);
        thmpaint.set_anti_alias(false);
        screenoff_buffer.draw_round_rect(rect, 10.0, 10.0, &thmpaint);
    }
    fn draw_on_hover_btn(&mut self, visible_buffer: &Canvas, screenoff_buffer: &Canvas, screenoff_color: &Color, xpos: f32, ypos: f32, w: f32, h: f32 ) {
        let mut thmpaint = Paint::default();
        let rect = Rect::new(xpos, ypos, xpos+w, ypos+h);
        thmpaint.set_color(Color::from_rgb(0, 255, 0));
        thmpaint.set_style(PaintStyle::Fill);
        thmpaint.set_anti_alias(true);
        visible_buffer.draw_round_rect(rect, 10.0, 10.0, &thmpaint);
        thmpaint.set_color(*screenoff_color);
        thmpaint.set_anti_alias(false);
        screenoff_buffer.draw_round_rect(rect, 10.0, 10.0, &thmpaint);
    }
}
