use core::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Functions {
    Close,
    Minimize,
    Maximize,
    Tray,
}

impl fmt::Display for Functions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
