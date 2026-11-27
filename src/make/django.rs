use crate::utils::django_utils::nombre_python_valido;
use crate::utils::general::{
    InitError, crear_archivo, crear_carpeta, ejecutar_comando, inicializar_git,
};

// Crea Django
pub fn django(nombre: &str, git: bool) -> Result<(), InitError> {
    if !nombre_python_valido(nombre) || nombre == "django" {
        return Err(InitError::Validation(
            "El nombre del proyecto Django debe ser un identificador válido de Python.".to_string(),
        ));
    }
    if std::path::Path::new(nombre).exists() {
        return Err(InitError::Validation(format!(
            "El directorio del proyecto ya existe: {nombre}"
        )));
    }

    crear_carpeta(".", nombre);
    crear_archivo(nombre, "requirements.txt", "Django\n");

    ejecutar_comando("python3", &["-m", "venv", ".venv"], Some(nombre))?;
    ejecutar_comando(
        ".venv/bin/python",
        &["-m", "pip", "install", "Django"],
        Some(nombre),
    )?;
    ejecutar_comando(
        ".venv/bin/python",
        &["-m", "django", "startproject", nombre, "."],
        Some(nombre),
    )?;

    if git {
        inicializar_git(nombre)?;
        crear_archivo(
            nombre,
            ".gitignore",
            ".venv/\n__pycache__/\n*.py[cod]\ndb.sqlite3\n",
        );
    }

    Ok(())
}
