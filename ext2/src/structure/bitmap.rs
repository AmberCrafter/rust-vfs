use crate::constants::{BlockSize1K,BlockSize2K,BlockSize4K,BlockSize8K};

pub struct Bitmap1K([u8; 128]);
pub struct Bitmap2K([u8; 256]);
pub struct Bitmap4K([u8; 512]);
pub struct Bitmap8K([u8; 1024]);


