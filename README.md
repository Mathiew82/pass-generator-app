<div align="center"><img src="https://raw.githubusercontent.com/Mathiew82/pass-generator-app/main/screenshot.png" alt="Screenshot"></div>

[![Download Pass Generator App](https://a.fsdn.com/con/app/sf-download-button)](https://sourceforge.net/projects/pass-generator-app/files/latest/download)

# Pass Generator App

Una aplicación sencilla para generar **contraseñas seguras** para tus cuentas. **Desarrollada en Rust** con el objetivo de ser **práctica, clara y eficiente**.

## Características

- 🔐 **Generación de contraseñas personalizables**: Elige la longitud y los tipos de caracteres.
- 🛡️ **Feedback de seguridad**: Evaluación automática del nivel de seguridad de la contraseña generada.
- 🖥️ **Interfaz gráfica intuitiva**: Construida con GTK4 para una experiencia de usuario fluida.
- 🆓 **Totalmente gratuita**: Licenciada bajo MIT.

## Estructura del Proyecto

La aplicación sigue una arquitectura modular para mantener el código organizado y mantenible:

```
pass-generator-app/
├── src/
│   ├── main.rs                         # Punto de entrada de la aplicación
│   ├── ui/                             # Módulo de interfaz de usuario
│   │   ├── mod.rs                      # Declaraciones del módulo UI
│   │   ├── layout.rs                   # Construcción y disposición de la UI
│   │   ├── controller.rs               # Lógica de conexión de eventos
│   │   ├── widgets.rs                  # Widgets básicos de GTK
│   │   ├── texts.rs                    # Constantes de texto
│   │   ├── styles.rs                   # Estilos y temas
│   │   └── components/                 # Componentes reutilizables
│   │       ├── mod.rs
│   │       ├── options_panel.rs        # Panel de opciones de generación
│   │       ├── generated_password.rs   # Display de contraseña generada
│   │       └── security_feedback.rs    # Feedback de seguridad
│   └── logic/                          # Módulo de lógica de negocio
│       ├── mod.rs
│       ├── password.rs                 # Algoritmo de generación de contraseñas
│       ├── feedback.rs                 # Evaluación de seguridad
│       └── state.rs                    # Estructuras de datos para opciones
├── assets/                             # Recursos gráficos
│   ├── icon.ico
│   └── logo.png
├── Cargo.toml                          # Configuración de Rust/Cargo
└── README.md                           # Este archivo
```

## Desarrollo y Patrones Utilizados

### Arquitectura

- **Separación de responsabilidades**: La aplicación se divide claramente en módulos `ui` (interfaz) y `logic` (lógica de negocio), facilitando el mantenimiento y las pruebas.
- **Componentes modulares**: La UI está construida con componentes reutilizables en el directorio `components/`, siguiendo el patrón de composición.
- **Estructuras de datos inmutables**: Uso de structs como `PasswordOptions` para representar el estado de manera segura y eficiente.

### Patrones de Diseño

- **MVC implícito**: Separación entre vista (widgets GTK), controlador (conexiones de eventos) y modelo (lógica de generación y estado).
- **Builder pattern**: Uso del builder de GTK4 para construir widgets de manera fluida.
- **Generación segura de contraseñas**: El algoritmo asegura que al menos un carácter de cada tipo seleccionado esté presente si la longitud lo permite, mejorando la seguridad.

### Tecnologías

- **Rust**: Lenguaje de sistemas seguro y eficiente, ideal para aplicaciones críticas como generadores de contraseñas.
- **GTK4**: Framework moderno para interfaces gráficas nativas, proporcionando una experiencia consistente en múltiples plataformas.
- **Rand crate**: Para generación de números aleatorios criptográficamente seguros.
- **Modularidad con módulos**: Organización del código en módulos para evitar conflictos de nombres y mejorar la legibilidad.

### Aspectos Destacables

- **Eficiencia**: Generación rápida de contraseñas sin compromisos en la aleatoriedad.
- **Seguridad**: Uso de caracteres excluyendo confusos (como 'O' y '0') para mejorar la usabilidad.
- **Internacionalización preparada**: Constantes de texto separadas facilitan futuras traducciones.
- **Build multiplataforma**: Configurado para compilar en Windows y Linux (GTK4 es multiplataforma).

## Uso de la App

### Versiones Disponibles

- **Windows**: [download](https://sourceforge.net/projects/pass-generator-app/files/Pass_Generator_App-Windows.zip/download)
- **Linux**: [download](https://sourceforge.net/projects/pass-generator-app/files/Pass_Generator_App-x86_64-Linux.tar.xz/download)

## Licencia

Este proyecto está bajo la Licencia MIT. Consulta [LICENSE](https://github.com/Mathiew82/pass-generator-app/blob/master/LICENSE) para más detalles.
