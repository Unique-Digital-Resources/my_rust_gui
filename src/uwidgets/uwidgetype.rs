use crate::uwidgets::button::Button;
use std::any::Any;

pub enum WidgetType{
    Button(Button<dyn Any>),
    // Add more variants as needed for other widget types
}