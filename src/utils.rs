use std::fs;

pub fn crear_carpeta(ruta: &str, nombre: &str) {
    let path = format!("{}/{}", ruta, nombre);
    fs::create_dir_all(&path).expect("No se pudo crear la carpeta");
}

pub fn crear_archivo(ruta: &str, nombre: &str, contenido: &str) {
    let path = format!("{}/{}", ruta, nombre);
    fs::write(&path, contenido).expect("No se pudo crear el archivo");
}

pub fn ejecutar_comando(comando: &str, args: &[&str], directorio: Option<&str>) -> bool {
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

    status.success()
}

pub fn inicializar_git(directorio: &str) -> bool {
    ejecutar_comando("git", &["init", directorio, "-q"], None)
}

// npm exige nombres en minusculas y sin caracteres especiales
pub fn nombre_paquete(nombre: &str) -> String {
    let paquete: String = nombre
        .to_lowercase()
        .chars()
        .map(|c| { if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '-' } })
        .collect();
    let paquete = paquete.trim_matches(|c| c == '-' || c == '_' || c == '.');

    if paquete.is_empty() {
        "app".to_string()
    } else {
        paquete.to_string()
    }
}

// Revisa que el nombre sea un identificador válido de Python
pub fn nombre_python_valido(nombre: &str) -> bool {
    let mut caracteres = nombre.chars();
    let identificador = matches!(caracteres.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && caracteres.all(|caracter| caracter.is_ascii_alphanumeric() || caracter == '_');
    identificador && !palabra_reservada_python(nombre)
}

// Las palabras reservadas de Python
fn palabra_reservada_python(nombre: &str) -> bool {
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
