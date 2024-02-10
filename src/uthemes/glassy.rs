use crate::uthemes::utheme::{UTheme, UThmTrait};
use skia_safe::{Canvas, Color, Font, FontStyle, Paint, PaintStyle, Typeface, Rect, Path, Point, TileMode, Shader, RRect, MaskFilter, BlurStyle, ClipOp};

pub struct ThmGlassy {
    pub theme: UTheme,
    }

impl ThmGlassy{
    pub fn new()-> Self{

        // Use the default Skia Typeface (which is a sans-serif font)
        let typeface = Typeface::default();

        // Create a Skia Font with the default typeface
        let font = Font::from_typeface(typeface, 24.0);


        return ThmGlassy{
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


impl UThmTrait for ThmGlassy{
    fn draw_btn(&mut self, visible_buffer: &Canvas, screenoff_buffer: &Canvas, screenoff_color: &Color, xpos: f32, ypos: f32, w: f32, h: f32 ) {
        let mut thmpaint = Paint::default();
        let mut offthmpaint = Paint::default();
        let rect = Rect::new(xpos, ypos, xpos+w, ypos+h);
        let inner_rect = Rect::new(xpos,ypos-50.0,xpos+w,ypos+h-50.0);
        let inner_rect_2 = Rect::new(xpos,ypos-70.0,xpos+w,ypos+h-70.0);
        let clip_rect = RRect::new_rect_xy(rect, 100.0, 100.0);
        let inner_blur = MaskFilter::blur(BlurStyle::Normal, 10.0, false);
        
        thmpaint.set_color(Color::from_rgb(255, 0, 0));
        thmpaint.set_style(PaintStyle::Fill);
        thmpaint.set_anti_alias(true);

        visible_buffer.draw_round_rect(rect, 100.0, 100.0, &thmpaint);

        visible_buffer.clip_rrect(clip_rect, ClipOp::Intersect, true);
        thmpaint.set_color(Color::from_argb(255, 255, 255, 255));
        visible_buffer.draw_round_rect(inner_rect, 100.0, 100.0, &thmpaint);

        thmpaint.set_color(Color::from_argb(255,255, 0, 0));
        thmpaint.set_anti_alias(true);
        thmpaint.set_style(PaintStyle::Fill);
        thmpaint.set_mask_filter(inner_blur);
        visible_buffer.draw_round_rect(inner_rect_2, 100.0, 100.0, &thmpaint);
        
        offthmpaint.set_color(*screenoff_color);
        offthmpaint.set_anti_alias(false);
        screenoff_buffer.draw_round_rect(rect, 100.0, 100.0, &offthmpaint);
        
    }
    fn draw_on_hover_btn(&mut self, visible_buffer: &Canvas, screenoff_buffer: &Canvas, screenoff_color: &Color, xpos: f32, ypos: f32, w: f32, h: f32 ) {
        let mut thmpaint = Paint::default();
        let mut offthmpaint = Paint::default();
        let rect = Rect::new(xpos, ypos, xpos+w, ypos+h);
        let inner_rect = Rect::new(xpos,ypos-50.0,xpos+w,ypos+h-50.0);
        let inner_rect_2 = Rect::new(xpos,ypos-70.0,xpos+w,ypos+h-70.0);
        let clip_rect = RRect::new_rect_xy(rect, 100.0, 100.0);
        let inner_blur = MaskFilter::blur(BlurStyle::Normal, 10.0, false);
        
        thmpaint.set_color(Color::from_rgb(0, 0, 255));
        thmpaint.set_style(PaintStyle::Fill);
        thmpaint.set_anti_alias(true);

        visible_buffer.draw_round_rect(rect, 100.0, 100.0, &thmpaint);

        visible_buffer.clip_rrect(clip_rect, ClipOp::Intersect, true);
        thmpaint.set_color(Color::from_argb(255, 255, 255, 255));
        visible_buffer.draw_round_rect(inner_rect, 100.0, 100.0, &thmpaint);

        thmpaint.set_color(Color::from_argb(255,0, 0, 255));
        thmpaint.set_anti_alias(true);
        thmpaint.set_style(PaintStyle::Fill);
        thmpaint.set_mask_filter(inner_blur);
        visible_buffer.draw_round_rect(inner_rect_2, 100.0, 100.0, &thmpaint);
        
        offthmpaint.set_color(*screenoff_color);
        offthmpaint.set_anti_alias(false);
        screenoff_buffer.draw_round_rect(rect, 100.0, 100.0, &offthmpaint);
        
    }
}