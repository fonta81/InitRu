# InitRu

Una herramienta CLI rapida y sencilla para inicializar proyectos, escrita en Rust.

[![Licencia: MIT](https://img.shields.io/badge/Licencia-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-Edición_2024-orange.svg)](https://www.rust-lang.org/)

**Idiomas:** [English](README.md) | **Español**

---

## Descripcion General

InitRu es una herramienta de linea de comandos ligera desarrollada en Rust para automatizar la creacion de la estructura base (scaffolding) de proyectos de software. Permite crear proyectos estructurados con sus herramientas base y configuracion opcional de Git con un solo comando.

---

## Caracteristicas

- **Inicializacion Rapida:** Genera la base de nuevos proyectos al instante.
- **Soporte para Python:**
  - Estructura estandar de directorios y archivos de inicio.
  - Creacion automatica de entorno virtual (`python3 -m venv .venv`).
- **Integracion con Git opcional (`-g`):**
  - Inicializa el repositorio con `git init`.
  - Genera automaticamente `.gitignore` adaptado al lenguaje.


---

## Modo de Uso

```text
Uso: InitRu [OPCIONES] [COMANDO]

Comandos:
  python  Crea un nuevo proyecto en Python
  help    Muestra este mensaje o la ayuda del subcomando

Opciones:
  -g          Inicializa un repositorio git
  -h, --help  Muestra la ayuda
```

### Ejemplos

1. **Crear un proyecto Python estandar:**
   ```bash
   InitRu python mi_proyecto
   ```

2. **Crear un proyecto Python con Git inicializado:**
   ```bash
   InitRu -g python mi_proyecto
   ```

3. **Consultar la ayuda:**
   ```bash
   InitRu -h
   InitRu python -h
   ```

---

## Licencia

Este proyecto esta bajo la **Licencia MIT**. Consulta el archivo [LICENSE](LICENSE) para mas detalles.

Copyright (c) 2026 fonta81.
