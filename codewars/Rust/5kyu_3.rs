#[derive(Copy, Clone)]
struct Vector {
    i: f64,
    j: f64,
    k: f64,
}

impl Vector {
    fn new(i: f64, j: f64, k: f64) -> Vector {
        Vector {
            i: i,
            j: j,
            k: k,
        }
    }
    
    fn get_magnitude(&self) -> f64 {
        (self.i * self.i + self.j * self.j + self.k * self.k).sqrt()
    }

    fn get_i() -> Vector {
        Vector {
            i: 1.0,
            j: 0.0,
            k: 0.0,
        }
    }

    fn get_j() -> Vector {
        Vector {
            i: 0.0,
            j: 1.0,
            k: 0.0,
        }
    }

    fn get_k() -> Vector {
        Vector {
            i: 0.0,
            j: 0.0,
            k: 1.0,
        }
    }

    fn add(&self, v: Vector) -> Vector {
        Vector {
            i: self.i + v.i,
            j: self.j + v.j,
            k: self.k + v.k,
        }
    }

    fn multiply_by_scalar(&self, s: f64) -> Vector {
        Vector {
            i: self.i * s,
            j: self.j * s,
            k: self.k * s,            
        }
    }

    fn dot(&self, v: Vector) -> f64 {
        self.i * v.i + self.j * v.j + self.k * v.k
    }

    fn cross(&self, v: Vector) -> Vector {
        Vector {
            i: self.j * v.k - self.k * v.j,
            j: self.k * v.i - self.i * v.k,
            k: self.i * v.j - self.j * v.i,           
        }
    }

    fn is_parallel_to(&self, v: Vector) -> bool {
        if self.is_zero() || v.is_zero() {
            return false;
        } else {
            let i0 = self.cross(v).i.abs();
            let j0 = self.cross(v).j.abs();
            let k0 = self.cross(v).k.abs();
            return (i0.floor() == 0.0 && i0 - i0.floor() <= 0.001) &&
                    (j0.floor() == 0.0 && j0 - j0.floor() <= 0.001) &&
                    (k0.floor() == 0.0 && k0 - k0.floor() <= 0.001)
        }
    }

    fn is_zero(&self) -> bool {
        self.i == 0.0 && self.j == 0.0 && self.k == 0.0
    }

    fn is_perpendicular_to(&self, v: Vector) -> bool {
        if self.is_zero() || v.is_zero() {
            return false;
        } else {
            let res = self.dot(v).abs();
            return res.floor() == 0.0 && res - res.floor() <= 0.001;
        }
    }

    fn normalize(&self) -> Vector {
        let mag = self.get_magnitude();
        Vector {
            i: self.i / mag,
            j: self.j / mag,
            k: self.k / mag,
        }
    }

    fn is_normalized(&self) -> bool {
        (self.get_magnitude() - 1.0).abs() <= 0.001
    }
}