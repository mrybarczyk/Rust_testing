#[derive(Debug, Copy, Clone)]
pub struct Zespolona {
    i: f64,
    r: f64
}

impl Zespolona {
    pub fn new(i: f64, r: f64) -> Self {
        Self {
            i,
            r
        }
    }
}

impl std::fmt::Display for Zespolona {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.r < 0.0 {
            write!(f, "{}i - {}", self.i, self.r.abs())
        } else if self.r == 0.0 {
            write!(f, "{}i", self.i)
        } else {
            write!(f, "{}i + {}", self.i, self.r)
        }
    }
}

impl std::ops::Add<f64> for Zespolona {
    type Output = Zespolona;
    fn add(self, _rhs: f64) -> Zespolona {
        let nowa = Zespolona {
            i: self.i,
            r: self.r + _rhs
        };
        nowa
    }
}

impl std::ops::Add<Zespolona> for f64 {
    type Output = Zespolona;
    fn add(self, _rhs: Zespolona) -> Zespolona {
        let nowa = Zespolona {
            i: _rhs.i,
            r: _rhs.r + self
        };
        nowa
    }
}

impl std::ops::Add<Zespolona> for Zespolona {
    type Output = Zespolona;
    fn add(self, _rhs: Zespolona) -> Zespolona {
        let nowa = Zespolona {
            i: self.i + _rhs.i,
            r: self.r + _rhs.r
        };
        nowa
    }
}

impl std::ops::Mul<Zespolona> for Zespolona {
    type Output = Zespolona;
    fn mul(self, _rhs: Zespolona) -> Zespolona {
        // (ai + b) * (ci + d) = ac*i^2 + ad*i + bc*i + bd
        let nowa = Zespolona {
            i: self.i * _rhs.r + self.r * _rhs.i,
            r: (self.i * _rhs.i) * (-1 as f64) + (self.r * _rhs.r)
        };
        nowa
    }
}

impl std::ops::Mul<Zespolona> for f64 {
    type Output = Zespolona;
    fn mul(self, _rhs: Zespolona) -> Zespolona {
        let nowa = Zespolona {
            i: self * _rhs.i,
            r: self * _rhs.r
        };
        nowa
    }
}

impl std::ops::Mul<f64> for Zespolona {
    type Output = Zespolona;
    fn mul(self, _rhs: f64) -> Zespolona {
        let nowa = Zespolona {
            i: self.i * _rhs,
            r: self.r * _rhs
        };
        nowa
    }
}

impl std::ops::Sub<Zespolona> for f64 {
    type Output = Zespolona;
    fn sub(self, _rhs: Zespolona) -> Zespolona {
        let nowa = Zespolona {
            i: _rhs.i,
            r: _rhs.r - self
        };
        nowa
    }
}

impl std::ops::Sub<Zespolona> for Zespolona {
    type Output = Zespolona;
    fn sub(self, _rhs: Zespolona) -> Zespolona {
        let nowa = Zespolona {
            i: self.i - _rhs.i,
            r: self.r - _rhs.r
        };
        nowa
    }
}

impl std::ops::Sub<f64> for Zespolona {
    type Output = Zespolona;
    fn sub(self, _rhs: f64) -> Zespolona {
        let nowa = Zespolona {
            i: self.i,
            r: self.r - _rhs
        };
        nowa
    }
}

impl std::ops::Div<f64> for Zespolona {
    type Output = Zespolona;
    fn div(self, _rhs: f64) -> Zespolona {
        let nowa = Zespolona {
            i: self.i / _rhs,
            r: self.r / _rhs
        };
        nowa
    }
}