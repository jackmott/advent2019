use std::ops;

#[derive(Hash,PartialEq,Eq,Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vector3 {

    pub fn cross(&self, v: Vector3) -> Vector3 {
        let x = self.y*v.z - v.y*self.z;
        let y = -(self.x*v.z - v.x*self.z);
        let z = self.x*v.y - v.x*self.y;
        Vector3 { x,y,z }
    }

    pub fn dot(&self, v: Vector3) -> i64 {
        self.x * v.x + self.y * v.y + self.z*v.z
    }

    pub fn dist_squared(&self, v: Vector3) -> i64 {
        (v.x - self.x) * (v.x - self.x) + (v.y - self.y) * (v.y - self.y) + (v.z-self.z) * (v.z-self.z)
    }

}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<i64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: i64) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}
impl Vector2 {
    pub fn from_index(i: usize, w: usize) -> Vector2 {
        Vector2 {
            x: (i % w) as f64,
            y: (i / w) as f64,
        }
    }

    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn normalize(&self) -> Vector2 {
        let mag = (self.x * self.x + self.y * self.y).sqrt();
        Vector2 {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    pub fn cross(&self, v: Vector2) -> f64 {
        self.x * v.y - v.x * self.y
    }

    pub fn dot(&self, v: Vector2) -> f64 {
        self.x * v.x + self.y * v.y
    }

    pub fn dist_squared(&self, v: Vector2) -> f64 {
        (v.x - self.x) * (v.x - self.x) + (v.y - self.y) * (v.y - self.y)
    }

    pub fn dist(&self, v: Vector2) -> f64 {
        self.dist_squared(v).sqrt()
    }

    pub fn angle_from_vertical(&self, p: Vector2) -> f64 {
        let v = (p - *self).normalize();
        let r = v.y.atan2(v.x) - (-1.0 as f64).atan2(0.0);
        let r = r.to_degrees();
        (r + 360.0) % 360.0
    }
}
impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f64) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
