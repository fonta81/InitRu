/// Revisa que el nombre sea un identificador válido de Python
pub fn nombre_python_valido(nombre: &str) -> bool {
    let mut caracteres = nombre.chars();
    let identificador = matches!(caracteres.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && caracteres.all(|caracter| caracter.is_ascii_alphanumeric() || caracter == '_');
    identificador && !palabra_reservada_python(nombre)
}

/// Las palabras reservadas de Python
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
