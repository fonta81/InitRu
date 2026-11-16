use std::fs;

pub fn crear_carpeta(ruta: &str, nombre: &str) {
    let path = format!("{}/{}", ruta, nombre);
    fs::create_dir_all(&path).expect("No se pudo crear la carpeta");
}

pub fn crear_archivo(ruta: &str, nombre: &str, contenido: &str) {
    let path = format!("{}/{}", ruta, nombre);
    fs::write(&path, contenido).expect("No se pudo crear el archivo");
}

pub fn ejecutar_comando(comando: &str, args: &[&str]) {
    let status = std::process::Command::new(comando)
        .args(args)
        .status()
        .expect("No se pudo ejecutar el comando");

    if !status.success() {
        eprintln!("El comando terminó con código: {:?}", status.code());
    }
}
