use crate::uwidgets::uwidgetype::WidgetType;
use crate::uwidgets::button::Button;

pub struct UWidget{
    pub wdgt: WidgetType,
    // canvas parameters
    //==================================
    // pub modulate: Color,
    // pub self_modulate: Color,
    // pub show_behind_parent: bool,
    // pub show_on_top: bool,
    // pub use_parent_paint: bool,
    pub visible: bool,

    // control parameters
    //==================================
    // pub anchor_bottom: f32,
    // pub anchor_left: f32,
    // pub anchor_right: f32,
    // pub anchor_top: f32,
    // pub focus_mode: i32,
    // pub focus_neighbour_bottom: String,
    // pub focus_neighbour_left: String,
    // pub focus_neighbour_right: String,
    // pub focus_neighbour_top: String,
    // pub focus_next: String,
    // pub focus_previous: String,
    // pub grow_horizontal: i32,
    // pub grow_vertical: i32,
    // pub hint_tooltip: String,
    // pub input_pass_on_modal_close_click: bool,
    // pub margin_bottom :f32,
    // pub margin_left :f32,
    // pub margin_right :f32,
    // pub margin_top :f32,
    // pub mouse_default_cursor_shape: i32,
    // pub mouse_filter: i32,
    // pub rect_clip_content: bool,
    pub rect_global_position: (f32,f32),
    // pub rect_min_size: (f32,f32),
    // pub rect_pivot_offset: i32,
    pub rect_position: (f32,f32),
    pub rect_rotation: f32,
    pub rect_scale: (f32,f32),
    pub rect_size: (f32,f32),
    // pub size_flags_horizontal: i32,
    // pub size_flags_stretch_ratio: f32,
    // pub size_flags_vertical: i32,
    // pub theme: i32,
    // pub theme_type_variation: String

    // container parameters
    //=========================================
}

impl UWidget{
    pub fn new()->UWidget{
        return UWidget{
            wdgt:WidgetType::Button(Button::new()),
            visible: true,
            rect_global_position: (0.0,0.0),
            rect_position: (0.0,0.0),
            rect_scale: (1.1,1.1),
            rect_rotation: 0.0,
            rect_size: (0.0,0.0)
        }
    }
}