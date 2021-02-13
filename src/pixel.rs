pub(crate) const SET_PIXEL: Pixel = Pixel {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

pub(crate) struct Pixel {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8,
}

impl Pixel {
    pub(crate) fn to_slice(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Clone for Pixel {
    fn clone(&self) -> Self {
        Pixel {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}
