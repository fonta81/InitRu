use crate::utils::general::{crear_archivo, crear_carpeta, ejecutar_comando, inicializar_git};

// Bash
pub fn bash(nombre: &str, git: bool) {
    // carpeta principal
    crear_carpeta(nombre, "src");

    // archivos
    crear_archivo(nombre, "README.md", "hello this is my proyec");
    crear_archivo(&format!("{}/src", nombre), "main.sh", "#!/bin/bash\necho 'hola mundo'\n");

    // permisos de ejecución
    ejecutar_comando("chmod", &["+x", &format!("{}/src/main.sh", nombre)], None);

    if git {
        inicializar_git(nombre);
        crear_archivo(nombre, ".gitignore", "");
    }
}
