use super::projection::projection;
use std::fs::File;
use std::io::{ Write};

/// DOWNLOAD MATRIX DISPLAY in root folder
/// 
/// then load this fn and put the new file proj in folder
/// cat file then ./display
pub fn ex14() -> std::io::Result<()> {
    // 60 deg to radian = 1.0472
    let mut file = File::create("proj")?;
    write!(file, "{}", projection(1.0472, 10./8., 1., 2. ).str_projection())?;
    Ok(())
}