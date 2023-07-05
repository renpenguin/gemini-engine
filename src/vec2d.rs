use std::{usize, ops::{Rem, RemAssign, Add, Sub}};

#[derive(Copy, Clone)]
pub struct Vec2D {
    pub x: isize,
    pub y: isize
}

impl Add<Vec2D> for Vec2D { type Output = Vec2D;
    fn add(self, rhs: Vec2D) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.x + rhs.y
        }
    }
}

impl Sub<Vec2D> for Vec2D { type Output = Vec2D;
	fn sub(self, rhs: Vec2D) -> Self::Output {
		Self {
			x: self.x - rhs.x,
            y: self.x - rhs.y
		}
	}
}

impl Rem<Vec2D> for Vec2D { type Output = Vec2D;
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y
        }
    }
}

impl RemAssign<Vec2D> for Vec2D {
    fn rem_assign(&mut self, rhs: Vec2D) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

impl From<(isize, isize)> for Vec2D {
    fn from(value: (isize, isize)) -> Self {
        Vec2D { x: value.0, y: value.1 }
    }
}

impl Vec2D {
    fn to_view_position(&self, view_width: usize) -> usize {
        let ux = usize::try_from(self.x).expect("Failed to convert Vec2D.x to usize");
        let uy = usize::try_from(self.y).expect("Failed to convert Vec2D.y to usize");
        return view_width * uy + ux;
    }
}