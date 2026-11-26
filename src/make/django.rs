use crate::utils::{
    crear_archivo, crear_carpeta, ejecutar_comando, inicializar_git, nombre_python_valido,
};

// Crea Django
pub fn django(nombre: &str, git: bool) -> bool {
    if !nombre_python_valido(nombre) || nombre == "django" {
        eprintln!("El nombre del proyecto Django debe ser un identificador válido de Python.");
        return false;
    }
    if std::path::Path::new(nombre).exists() {
        eprintln!("El directorio del proyecto ya existe: {nombre}");
        return false;
    }

    crear_carpeta(".", nombre);
    crear_archivo(nombre, "requirements.txt", "Django\n");

    if !ejecutar_comando("python3", &["-m", "venv", ".venv"], Some(nombre)) {
        return false;
    }
    if !ejecutar_comando(
        ".venv/bin/python",
        &["-m", "pip", "install", "Django"],
        Some(nombre),
    ) {
        return false;
    }
    if !ejecutar_comando(
        ".venv/bin/python",
        &["-m", "django", "startproject", nombre, "."],
        Some(nombre),
    ) {
        return false;
    }

    if git {
        if !inicializar_git(nombre) {
            return false;
        }
        crear_archivo(
            nombre,
            ".gitignore",
            ".venv/\n__pycache__/\n*.py[cod]\ndb.sqlite3\n",
        );
    }

    true
}
