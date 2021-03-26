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
#[allow(non_snake_case)]
pub const fn Vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}
impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
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

pub type Color = Vec3;

pub struct Camera {
    position: Vec3,
    rotation: Vec3,
    width: usize,
    height: usize,
    steps: usize,
    precision: f32,
}
impl Camera {
    pub fn new(width: usize, height: usize) -> Self {
        Camera {
            position: Vec3(-5., 0., 0.),
            rotation: Vec3(0., 0., 0.),
            width,
            height,
            steps: 5,
            precision: 0.1,
        }
    }
    pub fn render_ascii(&self, shapes: &[&dyn Shape]) {
        const INTENSITY: &[u8] = b" .,-~:;=!*#$@";

        //let mut res = alloc::vec![0u8; (self.width + 2) * self.height];

        for y in 0..self.height {
            hal::eprintln!();
            //unsafe { *res.get_unchecked_mut(0 + y * self.width) = b'\n' };
            //unsafe { *res.get_unchecked_mut(1 + y * self.width) = b'\r' };
            for x in 0..self.width {
                let color = self.pixel_render(x, y, shapes);
                let r = color.x;
                let g = color.y;
                let b = color.z;
                let grey_v = (r + g + b) / 3.;

                let idx = (grey_v * (INTENSITY.len() - 1) as f32) as usize;

                let v = INTENSITY[idx];
                hal::eprint!("{}", v as char);
                //unsafe { *res.get_unchecked_mut((x + 2) + y * self.width) = v };
            }
        }
        //let s = unsafe { core::str::from_utf8_unchecked(&res) };
        //hal::eprintln!("{}", s);
        hal::eprintln!();
    }
    fn pixel_render(&self, x: usize, y: usize, shapes: &[&dyn Shape]) -> Color {
        let x = x as f32 / self.width as f32 - 0.5;
        let y = y as f32 / self.height as f32 - 0.5;
        let ray_direction = Vec3(1., x, y).normalize();

        let mut ray_position = self.position;

        for _ in 0..self.steps {
            let mut min_dist = shapes[0].dist(ray_position);
            let mut min_shape = shapes[0];

            for s in shapes[1..].iter() {
                let d = s.dist(ray_position);
                if d < min_dist {
                    min_dist = d;
                    min_shape = *s;
                }
            }

            ray_position += ray_direction * min_dist;

            if min_dist < self.precision {
                const LIGHT: Vec3 = Vec3(80., 30., 100.);

                let normal = min_shape.normal_at(ray_position);

                let dir_to_light = (ray_position - LIGHT).normalize();
                let diffuse_intensity = f32::max(0., Vec3::dot(&normal, &dir_to_light));

                return min_shape.color(ray_position) * diffuse_intensity;
            }
        }

        Vec3(0., 0., 0.)
    }
}

pub trait Shape {
    fn dist(&self, origin: Vec3) -> f32;
    fn color(&self, origin: Vec3) -> Color;
    fn normal_at(&self, point: Vec3) -> Vec3 {
        const EPSILON: f32 = 0.001;
        let eps_x = Vec3(EPSILON, 0., 0.);
        let eps_y = Vec3(0., EPSILON, 0.);
        let eps_z = Vec3(0., 0., EPSILON);

        let grad_x = self.dist(point + eps_x) - self.dist(point - eps_x);
        let grad_y = self.dist(point + eps_y) - self.dist(point - eps_y);
        let grad_z = self.dist(point + eps_z) - self.dist(point - eps_z);

        Vec3(grad_x, grad_y, grad_z).normalize()
    }
}

pub struct Union<U, V> {
    u: U,
    v: V,
}
impl<U: Shape, V: Shape> Shape for Union<U, V> {
    fn dist(&self, origin: Vec3) -> f32 {
        f32::min(self.u.dist(origin), self.v.dist(origin))
    }
    fn color(&self, origin: Vec3) -> Color {
        if self.u.dist(origin) < self.v.dist(origin) {
            self.u.color(origin)
        } else {
            self.v.color(origin)
        }
    }
}

pub struct Substraction<U, V>(pub U, pub V);
impl<U: Shape, V: Shape> Shape for Substraction<U, V> {
    fn dist(&self, origin: Vec3) -> f32 {
        f32::max(-self.0.dist(origin), self.1.dist(origin))
    }
    fn color(&self, origin: Vec3) -> Color {
        self.0.color(origin)
    }
}

pub struct Intersection<U, V>(pub U, pub V);
impl<U: Shape, V: Shape> Shape for Intersection<U, V> {
    fn dist(&self, origin: Vec3) -> f32 {
        f32::max(self.0.dist(origin), self.1.dist(origin))
    }
    fn color(&self, origin: Vec3) -> Color {
        if self.0.dist(origin) > self.1.dist(origin) {
            self.0.color(origin)
        } else {
            self.1.color(origin)
        }
    }
}

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub color: Color,
}
impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, r: f32) -> Self {
        Sphere {
            position: Vec3(x, y, z),
            radius: r,
            color: Vec3(1., 1., 1.),
        }
    }
}
impl Shape for Sphere {
    fn dist(&self, origin: Vec3) -> f32 {
        (self.position - origin).norm() - self.radius
    }
    fn color(&self, _: Vec3) -> Color {
        self.color
    }
}
