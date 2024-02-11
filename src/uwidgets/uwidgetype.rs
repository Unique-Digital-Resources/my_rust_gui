// NOTES:
//==============================================================================
// written by Abdo Mahmoud (Unique-Digital-Resources): https://github.com/Unique-Digital-Resources
// Copyright 2024 Abdo Mahmoud
// Apache 2.0
//==============================================================================

use crate::uwidgets::button::Button;
use std::any::Any;

pub enum WidgetType{
    Button(Button<dyn Any>),
    // Add more variants as needed for other widget types
}
