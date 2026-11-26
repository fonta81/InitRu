use crate::utils::{crear_archivo, crear_carpeta, ejecutar_comando, inicializar_git};

// Crea Django
pub fn django(nombre: &str, git: bool) -> bool {
    if !nombre_valido(nombre) {
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

// Revisa que el nombre sea valido 
fn nombre_valido(nombre: &str) -> bool {
    let mut caracteres = nombre.chars();
    let identificador = matches!(caracteres.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && caracteres.all(|caracter| caracter.is_ascii_alphanumeric() || caracter == '_');
    identificador && !palabra_reservada(nombre) && nombre != "django"
}

// Las palabras reservadas de Django
fn palabra_reservada(nombre: &str) -> bool {
    matches!(
        nombre,
        "False"
            | "None"
            | "True"
            | "and"
            | "as"
            | "assert"
            | "async"
            | "await"
            | "break"
            | "class"
            | "continue"
            | "def"
            | "del"
            | "elif"
            | "else"
            | "except"
            | "finally"
            | "for"
            | "from"
            | "global"
            | "if"
            | "import"
            | "in"
            | "is"
            | "lambda"
            | "nonlocal"
            | "not"
            | "or"
            | "pass"
            | "raise"
            | "return"
            | "try"
            | "while"
            | "with"
            | "yield"
    )
}
