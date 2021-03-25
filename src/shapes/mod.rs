pub trait Sqrt {
    fn sqrt(self) -> Self;
    fn inv_sqrt(self) -> Self;
}
impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        f32::from_bits((self.to_bits() + 0x3f80_0000) >> 1)
    }
    fn inv_sqrt(self) -> Self {
        const THREEHALFS: f32 = 1.5;
        let x2: f32 = self * 0.5;
        let mut i = self.to_bits();
        i = 0x5f375a86 - (i >> 1);
        let y = f32::from_bits(i);
        let y = y * (THREEHALFS - (x2 * y * y));
        let y = y * (THREEHALFS - (x2 * y * y));
        y
    }
}

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    //
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
    pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
    pub fn norm_squared(&self) -> f32 {
        Self::dot(self, self)
    }
    pub fn norm(&self) -> f32 {
        self.norm_squared().sqrt()
    }
    pub fn normalize(&self) -> Self {
        *self * self.norm_squared().inv_sqrt()
    }
}
impl core::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl core::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(mut self, rhs: Vec3) -> Self::Output {
        self += rhs;
        self
    }
}
impl core::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl core::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

pub struct Camera {
    position: Vec3,
    rotation: Vec3,
    width: usize,
    height: usize,
    precision: usize,
}
impl Camera {
    pub fn new(width: usize, height: usize) -> Self {
        Camera {
            position: Vec3::new(-10., 0., 0.),
            rotation: Vec3::new(0., 0., 0.),
            width,
            height,
            precision: 10,
        }
    }
    pub fn render_ascii(&self, shapes: &[&dyn Shape]) {
        const INTENSITY: &[u8] = b" .,-~:;=!*#$@";

        for x in 0..self.width {
            hal::eprintln!();
            for y in 0..self.height {
                let p = self.pixel_render(x, y, shapes);
                let r = (p >> 12) & 0b1111;
                let g = (p >> 8) & 0b1111;
                let b = (p >> 4) & 0b1111;

                let v = (r + g + b) as usize / 3;

                let s = v * (INTENSITY.len() - 1) / 0b1111;

                let v = INTENSITY[s];
                hal::eprint!("{}", v as char);
            }
        }
    }
    pub fn render(&self, shapes: &[&dyn Shape], buffer: &mut [u16]) {
        for x in 0..self.width {
            for y in 0..self.height {
                let p = self.pixel_render(x, y, shapes);
                buffer[x + y * self.width] = p;
            }
        }
    }
    fn pixel_render(&self, x: usize, y: usize, shapes: &[&dyn Shape]) -> u16 {
        let x = x as f32 / self.width as f32 - 0.5;
        let y = y as f32 / self.height as f32 - 0.5;
        let dir = Vec3::new(1., x, y).normalize();

        let mut start = self.position;

        for _ in 0..self.precision {
            let mut min_dist = shapes[0].dist(start);
            let mut min_shape = shapes[0];

            for s in shapes[1..].iter() {
                let d = s.dist(start);
                if d < min_dist {
                    min_dist = d;
                    min_shape = *s;
                }
            }

            if min_dist < 0.01 {
                return min_shape.color(start);
            }

            start += dir * min_dist;
        }

        0
    }
}

pub trait Shape {
    fn dist(&self, origin: Vec3) -> f32;
    fn color(&self, origin: Vec3) -> u16;
}

pub struct Union<U, V> {
    u: U,
    v: V,
}
impl<U: Shape, V: Shape> Shape for Union<U, V> {
    fn dist(&self, origin: Vec3) -> f32 {
        f32::min(self.u.dist(origin), self.v.dist(origin))
    }
    fn color(&self, origin: Vec3) -> u16 {
        if self.u.dist(origin) < self.v.dist(origin) {
            self.u.color(origin)
        } else {
            self.v.color(origin)
        }
    }
}

pub struct Substraction<U, V> {
    u: U,
    v: V,
}
impl<U: Shape, V: Shape> Shape for Substraction<U, V> {
    fn dist(&self, origin: Vec3) -> f32 {
        f32::max(-self.u.dist(origin), self.v.dist(origin))
    }
    fn color(&self, origin: Vec3) -> u16 {
        self.u.color(origin)
    }
}

pub struct Intersection<U, V> {
    u: U,
    v: V,
}
impl<U: Shape, V: Shape> Shape for Intersection<U, V> {
    fn dist(&self, origin: Vec3) -> f32 {
        f32::max(self.u.dist(origin), self.v.dist(origin))
    }
    fn color(&self, origin: Vec3) -> u16 {
        if self.u.dist(origin) > self.v.dist(origin) {
            self.u.color(origin)
        } else {
            self.v.color(origin)
        }
    }
}

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub color: u16,
}
impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, r: f32) -> Self {
        Sphere {
            position: Vec3::new(x, y, z),
            radius: r,
            color: 0xFFFF,
        }
    }
}
impl Shape for Sphere {
    fn dist(&self, origin: Vec3) -> f32 {
        (self.position - origin).norm() - self.radius
    }
    fn color(&self, _: Vec3) -> u16 {
        self.color
    }
}
