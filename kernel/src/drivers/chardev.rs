pub trait CharDev {
    fn read(&self, size: usize, buf: &mut [u8]) -> Option<usize>;

    fn write(&mut self, size: usize, buf: &[u8]) -> Option<usize>;
}
