#[macro_export] macro_rules! hex {
    ($color: expr) => {
        Color::hex($color)
    };
}
#[macro_export] macro_rules! rgb {
    ($r: expr, $g: expr, $b: expr) => {
        Color::rgb($r, $g, $b)
    };
}
#[macro_export] macro_rules! color {
    ($color: expr) => {
        hex!($color)
    };
    ($r: expr, $g: expr, $b: expr) => {
        rgb!($r, $g, $b)
    };
}
