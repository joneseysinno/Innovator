#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EdgeKindGpu {
    Signal = 0,
    Stream = 1,
    Wave = 2,
    Binding = 3,
}
