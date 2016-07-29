// TODO:
// * Make this usize instead
// * Keep track of the first zero index
pub struct Bitmap<'a> { buffer: &'a mut [u8] }

impl<'a> Bitmap<'a> {
    pub fn new(buffer: &mut [u8]) -> Bitmap {
        Bitmap { buffer: buffer }
    }

    pub fn len(&self) -> usize {
        8 * self.buffer.len()
    }

    pub fn get(&self, index: usize) -> bool {
        let (byte, bit) = calculate_indices(index);
        self.buffer[byte] & (1 << bit) != 0
    }

    pub fn find_zero(&self) -> Option<usize> {
        for index in 0 .. self.len() {
            if !self.get(index) {
                return Some(index);
            }
        }
        None
    }

    pub fn set(&mut self, index: usize, value: bool) {
        let (byte, bit) = calculate_indices(index);
        if value {
            self.buffer[byte] |= 1 << bit;
        } else {
            self.buffer[byte] &= !(1 << bit);
        }
    }
}

fn calculate_indices(index: usize) -> (usize, usize) {
    (index / 8, index % 8)
}
