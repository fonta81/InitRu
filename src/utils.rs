use std::fs;
use std::path::Path;

pub fn crear_carpeta(ruta: &str, nombre: &str) -> std::io::Result<()> {
    let path = Path::new(ruta).join(nombre);
    fs::create_dir_all(&path)?;
    Ok(())
}

pub fn crear_archivo(ruta: &str, nombre: &str, contenido: Option<&str>) -> std::io::Result<()> {
    let path = Path::new(ruta).join(nombre);
    if let Some(contenido) = contenido {
        fs::write(&path, contenido)?;
    } else {
        fs::File::create(&path)?;
    }
    Ok(())
}