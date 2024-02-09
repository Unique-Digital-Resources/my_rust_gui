pub struct Button <T: ?Sized>{
    pub disable: bool,
    pub on_hover: Box<dyn Fn(&T)>,
}

impl <T: ?Sized> Button <T>{

    pub fn new() -> Button<T> {
        Button {
            disable: true,
            on_hover: Box::new(|_t|{})
        }
    }
}