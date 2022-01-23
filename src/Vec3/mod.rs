use std::ops;

pub mod Vec3{

    #[derive(Debug)]
    struct Vec3 {
        x: f64,
        y: f64,
        z: f64,
    }

    impl Vec3 {
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
            return (x*x + y*y + z*z).sqrt();
        }

    }

    impl ops::Add for Vec3 {
        type Output = Self;

        fn add(&self, _rhs: Self) -> Self {
            Self {
                x: x + _rhs.x,
                y: y + _rhs.y,
                z: z + _rhs.z,
            }
        } 
    }

    impl ops::AddAssign for Vec3 {
        type Output = Self;

        fn add_assign(&mut &self, _rhs: Self) -> Self {
            *self = Self {
                x: x + _rhs.x,
                y: y + _rhs.y,
                z: z + _rhs.z,
            };
        } 
    }

    impl ops::Mul for Vec3 {
        type Output = Self;

        fn mul(self, _rhs: Self) -> Self {
            Self {
                x: x * _rhs.x,
                y: y * _rhs.y,
                z: z * _rhs.z,
            }
        }
    }

    impl ops::Mul<f64> for Vec3 {
        type Output = Self;

        fn mul(self, _rhs: f64) -> Self {
            Self {
                x: x * _rhs,
                y: y * _rhs,
                z: z * _rhs,
            }
        } 
    }

    impl ops::Mul<f64> for Vec3 {
        type Output = Self;

        fn mul(_lhs: f64, self) -> Self {
            _lhs * self;
        } 
    }

    impl ops::MulAssign<f64> for Vec3 {
        type Output = Self;

        fn mul_assign(&mut self, _rhs: f64){
            self = self * _rhs;
        }
    }

}