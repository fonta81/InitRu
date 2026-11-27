use std::fmt;
use std::fs;
use std::io;
use std::process::{Command, ExitStatus};

#[derive(Debug)]
pub enum InitError {
    Validation(String),
    CommandSpawn {
        program: String,
        args: Vec<String>,
        source: io::Error,
    },
    CommandFailed {
        program: String,
        args: Vec<String>,
        status: ExitStatus,
    },
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(message) => write!(f, "{message}"),
            Self::CommandSpawn {
                program,
                args,
                source,
            } => {
                write!(f, "No se pudo iniciar el comando {program}")?;
                for arg in args {
                    write!(f, " {arg:?}")?;
                }
                write!(f, ": {source}")
            }
            Self::CommandFailed {
                program,
                args,
                status,
            } => {
                write!(f, "El comando {program}")?;
                for arg in args {
                    write!(f, " {arg:?}")?;
                }
                if let Some(code) = status.code() {
                    write!(f, " terminó con código {code}.")
                } else {
                    write!(f, " terminó sin un código de salida.")
                }
            }
        }
    }
}

impl std::error::Error for InitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CommandSpawn { source, .. } => Some(source),
            Self::Validation(_) | Self::CommandFailed { .. } => None,
        }
    }
}

/// Crea una carpeta en la ruta y con el nombre que se le indique
pub fn crear_carpeta(ruta: &str, nombre: &str) {
    let path = format!("{}/{}", ruta, nombre);
    fs::create_dir_all(&path).expect("No se pudo crear la carpeta");
}

/// Crea un archivo en la ruta,nombre y con el contenido que se le indique
pub fn crear_archivo(ruta: &str, nombre: &str, contenido: &str) {
    let path = format!("{}/{}", ruta, nombre);
    fs::write(&path, contenido).expect("No se pudo crear el archivo");
}

/// Ejecuta un comando enm el directorio que se le indique
pub fn ejecutar_comando(
    comando: &str,
    args: &[&str],
    directorio: Option<&str>,
) -> Result<(), InitError> {
    let args: Vec<String> = args.iter().map(|arg| (*arg).to_string()).collect();
    let mut cmd = Command::new(comando);
    if let Some(dir) = directorio {
        cmd.current_dir(dir);
    }
    let status = cmd
        .args(&args)
        .status()
        .map_err(|source| InitError::CommandSpawn {
            program: comando.to_string(),
            args: args.clone(),
            source,
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(InitError::CommandFailed {
            program: comando.to_string(),
            args,
            status,
        })
    }
}

/// inicializa un repositorio git con -q
pub fn inicializar_git(directorio: &str) -> Result<(), InitError> {
    ejecutar_comando("git", &["init", directorio, "-q"], None)
}

/// npm exige nombres en minusculas y sin caracteres especiales
pub fn nombre_paquete(nombre: &str) -> String {
    let paquete: String = nombre
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect();
    let paquete = paquete.trim_matches(|c| c == '-' || c == '_' || c == '.');

    if paquete.is_empty() {
        "app".to_string()
    } else {
        paquete.to_string()
    }
}
