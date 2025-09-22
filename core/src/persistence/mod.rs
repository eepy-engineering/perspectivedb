use std::ops::DerefMut;

pub trait Log {
    type Error;

    fn write(&mut self, at: usize, data: &[u8]) -> Result<(), Self::Error>;
    fn read<'a, T: DerefMut<Target = [u8]>>(
        &mut self,
        at: usize,
        fill: &'a mut T,
    ) -> Result<&'a mut T, Self::Error>;
    fn sync(&mut self) -> Result<(), Self::Error>;
    fn len(&mut self) -> Result<u64, Self::Error>;
}
