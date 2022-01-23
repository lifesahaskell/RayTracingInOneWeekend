use std::ops{Add, AddAssign};



#[derive(Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub impl Vec3 {
    fn x(&self) -> f64 {
        return self.x;
    }

    fn y(&self) -> f64 {
        return self.z;
    }

    fn z(&self) -> f64 {
        return self.z;
    }

    fn length(&self) -> f64 {
        return (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
    }

}

impl Add for Vec3 {
    type Output = Self;

    fn add(&self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    } 
}




impl AddAssign for Vec3 {
    type Output = Self;

    fn add_assign(_: &mut &self, _rhs: Self) -> Self {
        *self = Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        };
    } 
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self {
        Self {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self {
        Self {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    } 
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, _lhs: f64) -> Self {
        _lhs * self;
    } 
}

impl ops::MulAssign<f64> for Vec3 {
    type Output = Self;

    fn mul_assign(&mut self, _rhs: f64){
        self = self * _rhs;
    }
}

