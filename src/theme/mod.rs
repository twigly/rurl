pub(crate) mod default;
pub(crate) mod style;

use style::Style;

pub trait Theme {
    fn request(&self) -> Box<dyn RequestTheme>;
    fn response(&self) -> Box<dyn ResponseTheme>;
}

pub trait HeaderTheme {
    fn header_name(&self, standard: bool) -> Style;
    fn header_value(&self, standard: bool) -> Style;
}

pub trait RequestTheme: HeaderTheme {
    fn as_header(&self) -> &dyn HeaderTheme;
    fn primary(&self) -> Style;
    fn secondary(&self) -> Style;
    fn url(&self) -> Style;
    fn direction(&self, standard: bool) -> Style;
}

pub trait ResponseTheme: HeaderTheme {
    fn as_header(&self) -> &dyn HeaderTheme;
    fn primary(&self) -> Style;
    fn secondary(&self) -> Style;
    fn version(&self) -> Style;
    fn status(&self) -> Style;
    fn direction(&self, standard: bool) -> Style;
}

#[cfg(test)]
impl core::fmt::Debug for dyn Theme {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Theme")
    }
}
