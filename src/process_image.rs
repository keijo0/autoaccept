pub struct RawImage {
    pub width:  u32,
    pub height: u32,
    pub data:   Vec<u8>,
}

impl RawImage {
    #[inline]
    fn pixel(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let off = ((y * self.width + x) * 4) as usize;
        (self.data[off], self.data[off + 1], self.data[off + 2])
    }
}

pub fn process_image(img: &RawImage) -> Option<(u32, u32)> {
    let mut matches: u32 = 0;

    for x in 0..img.width {
        for y in 0..img.height {
            let (red, green, blue) = img.pixel(x, y);

            if (50..=60).contains(&red)
                && (178..=187).contains(&green)
                && (77..=87).contains(&blue)
            {
                matches += 1;
            }

            if matches >= 9000 {
                return Some((x, y));
            }
        }
    }

    None
}
