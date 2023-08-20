pub fn interpolate(i0: isize, d0: f64, i1: isize, d1: f64) -> Vec<isize> {
    if i0 == i1 {
        return vec![d0.round() as isize];
    }
    let mut values = vec![];

    let a = (d1 - d0) / (i1 - i0) as f64;
    let mut d = d0;
    for _i in i0..(i1 + 1) {
        values.push(d.clone().round() as isize);
        d += a;
    }
    values
}

/// Wrapping is used to determine how you want to handle out-of-bounds pixels during plotting pixels to the screen. Here's how each possible value functions:
///
/// `Wrapping::Wrap` wraps any out of bounds pixels around to the other side. This is useful if you have an object that travels the entirety of the screen and appears on the other side when it reaches the end.
///
/// `Wrapping::Ignore` simply skips all out-of-bounds pixels. This is useful if you might have an object clipping through the edge of the screen.
///
/// `Wrapping::Panic` will `panic!` if any pixels are out of bounds. You should use this if you have your own wrapping system implemented
#[derive(Copy)]
pub enum Wrapping {
    Wrap,
    Ignore,
    Panic,
}

impl Clone for Wrapping {
    fn clone(&self) -> Self {
        match self {
            Wrapping::Wrap => Wrapping::Wrap,
            Wrapping::Ignore => Wrapping::Ignore,
            Wrapping::Panic => Wrapping::Panic,
        }
    }
}
