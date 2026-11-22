use std::fs;

pub fn crear_carpeta(ruta: &str, nombre: &str) {
    let path = format!("{}/{}", ruta, nombre);
    fs::create_dir_all(&path).expect("No se pudo crear la carpeta");
}

pub fn crear_archivo(ruta: &str, nombre: &str, contenido: &str) {
    let path = format!("{}/{}", ruta, nombre);
    fs::write(&path, contenido).expect("No se pudo crear el archivo");
}

pub fn ejecutar_comando(comando: &str, args: &[&str], directorio: Option<&str>) {
    let mut cmd = std::process::Command::new(comando);
    if let Some(dir) = directorio {
        cmd.current_dir(dir);
    }
    let status = cmd
        .args(args)
        .status()
        .expect("No se pudo ejecutar el comando");

    if !status.success() {
        eprintln!("El comando terminó con código: {:?}", status.code());
    }
}

pub fn inicializar_git(directorio: &str) {
    ejecutar_comando("git", &["init", directorio], None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejecuta_en_el_directorio_indicado() {
        let dir = std::env::temp_dir().join(format!("initru-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();

        ejecutar_comando("sh", &["-c", "pwd > salida.txt"], Some(dir.to_str().unwrap()));

        let salida = fs::read_to_string(dir.join("salida.txt")).unwrap();
        assert_eq!(
            fs::canonicalize(salida.trim()).unwrap(),
            fs::canonicalize(&dir).unwrap()
        );

        fs::remove_dir_all(&dir).unwrap();
    }
}
