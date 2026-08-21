# Maze Runner — Ray Caster

Proyecto 1, tercera parte del curso de Gráficas por Computadora (cc2018, UVG). Es un ray caster jugable escrito en Rust, con temática de la película *Maze Runner*. El renderizado 3D se hace lanzando rayos manualmente y dibujando columnas de píxeles directamente al framebuffer, sobre la base del ejemplo de clase `rc_03_maze_movement`.

## Cómo correr el proyecto

```bash
cargo run --release
```

Se recomienda `--release` porque el ray casting recorre los rayos píxel por píxel y en modo `debug` el rendimiento baja bastante.

Si Windows App Control bloquea la compilación dentro de `Documentos` (error `os error 4551`), se puede redirigir el directorio de compilación a una ruta permitida:

```bash
set CARGO_TARGET_DIR=C:\Users\%USERNAME%\AppData\Local\mr-target && cargo run --release
```

## Controles

| Tecla / acción | Efecto |
|---|---|
| `W` / flecha arriba | Avanzar en la dirección de vista |
| `S` / flecha abajo | Retroceder |
| `A` / `D` | Rotar la cámara (respaldo del mouse) |
| Mouse horizontal | Rotar la cámara de forma proporcional al movimiento |
| Flechas arriba/abajo | Navegar entre niveles en el menú de selección |
| `Enter` | Confirmar en las pantallas de menú y de éxito |
| `Esc` | Volver atrás o salir |

## Objetivos de la rúbrica implementados

- **Estética del nivel.** Paleta de piedra, metal oxidado y enredadera. El fondo no es un color plano: hay un degradado de cielo (gris a tono de atardecer) y un degradado de piso de tierra oscura, separados por el horizonte. Las paredes se oscurecen con la distancia.
- **FPS estables alrededor de 15, mostrados en pantalla.** El ciclo principal usa un retardo de fotograma de 66 ms, ajustado por el tiempo que realmente tomó el fotograma. El contador de FPS se dibuja siempre en la esquina superior izquierda con una fuente de mapa de bits de 5x7 implementada en `src/text.rs`.
- **Cámara con movimiento y rotación.** `W`/`S` para avanzar y retroceder respetando colisiones, `A`/`D` para rotar.
- **Rotación horizontal con el mouse.** Se lee la posición del cursor con `get_mouse_pos` y se aplica el desplazamiento horizontal entre fotogramas al ángulo de vista.
- **Minimapa en una esquina.** Superpuesto sobre la vista 3D en la esquina superior derecha, a escala reducida del laberinto completo. Muestra la posición del jugador con un punto rojo y su orientación con una línea amarilla. Los tipos de pared se distinguen por color y la meta aparece en verde.
- **Música de fondo.** Archivos de audio propios en bucle: una pista para el menú y una distinta por nivel, con respaldo sintetizado por código si algún archivo falta.
- **Efectos de sonido.** Un golpe seco al chocar contra una pared, sintetizado por código, y un archivo de audio propio al alcanzar la meta.
- **Pantalla de bienvenida con selección de niveles.** Pantalla de título y, tras `Enter`, un menú para elegir entre los tres sectores.
- **Pantalla de éxito.** Al llegar a la celda `g` se muestra una pantalla dentro del framebuffer con el sector superado y el tiempo empleado, en lugar de imprimir en consola y cerrar.

## Requisitos funcionales

- **El jugador no atraviesa paredes.** La colisión se verifica por separado en `X` y en `Y`, con un margen de cuerpo en la dirección del movimiento, lo que permite deslizarse a lo largo de una pared en vez de quedarse trabado en las esquinas.
- **El juego no debe caerse.** Todos los accesos al arreglo del laberinto pasan por `get` con verificación de límites, y las coordenadas negativas se tratan como pared. Si el archivo del laberinto no se puede abrir se usa un laberinto de respaldo; si una textura no carga se genera por código; si el dispositivo de audio no está disponible el juego continúa en silencio; si la ventana no se puede crear el programa termina con un mensaje en vez de entrar en pánico.
- **Cada tipo de pared se ve distinto.** Los tres caracteres de pared tienen textura propia, y además un color sólido de respaldo por si el texturizado de esa pared específica falla.

## Los laberintos

Están en `mazes/`, uno por nivel:

| Archivo | Nivel | Dimensiones |
|---|---|---|
| `glade.txt` | El Glade | 21 x 15 |
| `sector_oeste.txt` | Sector Oeste | 25 x 17 |
| `laberinto_central.txt` | El Laberinto Central | 29 x 19 |

Los tres superan el mínimo exigido de 12 x 9 y son distintos del `maze.txt` de referencia.

Caracteres usados:

| Carácter | Significado |
|---|---|
| `+` | Pared de piedra antigua, gris y agrietada |
| `#` | Pared de metal oxidado, con costuras y remaches |
| `%` | Pared de piedra cubierta de enredadera |
| espacio | Piso transitable |
| `p` | Posición inicial del jugador |
| `g` | Meta |

El Glade y Sector Oeste se generaron con retroceso recursivo, que garantiza por construcción que todas las celdas del piso queden conectadas, y después se abrieron celdas adicionales para dar amplitud a los corredores. El Laberinto Central sigue un diseño distinto a propósito: nueve cámaras dispuestas en cuadrícula, unidas por corredores estrechos y con pilares interiores diferentes en cada cámara, con la meta en la sala central. Eso le da una lectura visual muy distinta a la de los otros dos, que son laberintos de pasillos.

La existencia de un camino de `p` a `g` se verificó en los tres mapas ejecutando una búsqueda en anchura sobre el archivo real, no razonando el recorrido a mano; los tres reportaron `REACHED GOAL`. En El Laberinto Central se comprobó además que las 252 celdas de piso son alcanzables desde el inicio, es decir que no queda ninguna cámara aislada. También se verificó que todas las filas de cada archivo tengan exactamente la misma longitud, para que el análisis fila por fila del archivo no se rompa.

## Texturas

Las texturas de pared se generan por código en `src/texture.rs`, en mapas de 64 x 64 píxeles, usando una función de ruido determinista: la piedra se compone de bloques con junta de mortero, el metal de placas con costuras, remaches y manchas de óxido, y la enredadera de una base de piedra musgosa con tallos ondulados y hojas. No se descargaron imágenes de internet, así que ninguna textura del proyecto proviene de un archivo externo.

Si se quiere usar texturas reales en su lugar, basta colocar archivos llamados `stone.png`, `metal.png` y `vine.png` en `assets/textures/`. El juego intenta cargarlos primero con la crate `image` y solo si falla recurre a las texturas generadas por código.

## Sobre el audio

La música y el sonido de victoria son **archivos de audio propios, aportados por el autor del proyecto**. No se generaron con ninguna herramienta ni se descargaron de terceros, y no contienen audio con derechos de autor de canciones existentes.

Los archivos se leen desde `assets/audio/` con estos nombres exactos:

| Archivo | Cuándo suena |
|---|---|
| `menu.mp3` | En bucle en la pantalla de bienvenida y en la de selección de nivel |
| `glade.mp3` | En bucle durante el gameplay de El Glade |
| `sector_oeste.mp3` | En bucle durante el gameplay de Sector Oeste |
| `laberinto_central.mp3` | En bucle durante el gameplay de El Laberinto Central |
| `victoria.wav` | Una vez, al alcanzar la meta |

La decodificación la hace `rodio`, que trae MP3, WAV, FLAC y Vorbis habilitados entre sus características por defecto, así que no hace falta ninguna dependencia adicional. Importante: `rodio` identifica el formato por el contenido del archivo, no por la extensión, así que renombrar un M4A/AAC a `.mp3` no funciona. Los cuatro archivos de música son MP3 y el de victoria es WAV.

En las pistas en bucle conviene tener presente que el MP3 agrega relleno de silencio al inicio y al final, lo que produce un salto audible en cada repetición; WAV y OGG no tienen ese problema.

Si alguno de estos archivos falta o no se puede decodificar, el juego **no se cae**: imprime en consola qué archivo concreto falló y recurre a una pista o efecto sintetizado por código como respaldo. Esa síntesis de respaldo también está en `src/audio.rs`, y consiste en una progresión de acordes con bajo y melodía de onda cuadrada para la música, y un arpegio ascendente para la victoria.

El golpe contra la pared no usa archivo: se sintetiza siempre por código, como una onda baja con decaimiento y un poco de ruido.

El cambio de pista está atado a las transiciones de pantalla, así que la música del menú se detiene al entrar a un nivel y la del nivel se detiene al volver al menú o al alcanzar la meta. Nunca suenan dos pistas mezcladas.

## Nota sobre el texto en pantalla

El texto de las pantallas se dibuja con una fuente de mapa de bits de 5x7 escrita a mano en `src/text.rs`, que solo cubre letras sin tilde, dígitos y algunos signos. Por eso los rótulos del juego aparecen en mayúsculas y sin acentos; es una limitación de la fuente, no del idioma del proyecto.

## Estructura

```
.
├── Cargo.toml
├── README.md
├── assets/
│   ├── audio/
│   └── textures/
├── mazes/
│   ├── glade.txt
│   ├── sector_oeste.txt
│   └── laberinto_central.txt
└── src/
    ├── main.rs
    ├── framebuffer.rs
    ├── maze.rs
    ├── player.rs
    ├── caster.rs
    ├── texture.rs
    ├── minimap.rs
    ├── audio.rs
    ├── screens.rs
    └── text.rs
```

## No implementado a propósito

No hay soporte para hardware alternativo ni para gamepad: la entrada es solo teclado y mouse. Tampoco hay animación de sprites.
