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
- **Música de fondo.** Instrumental sintetizado por código, en bucle durante el juego.
- **Efectos de sonido.** Un golpe seco al chocar contra una pared y un arpegio de victoria al alcanzar la meta.
- **Pantalla de bienvenida con selección de niveles.** Pantalla de título y, tras `Enter`, un menú para elegir entre los dos sectores.
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

Ambos superan el mínimo exigido de 12 x 9 y son distintos del `maze.txt` de referencia.

Caracteres usados:

| Carácter | Significado |
|---|---|
| `+` | Pared de piedra antigua, gris y agrietada |
| `#` | Pared de metal oxidado, con costuras y remaches |
| `%` | Pared de piedra cubierta de enredadera |
| espacio | Piso transitable |
| `p` | Posición inicial del jugador |
| `g` | Meta |

Los laberintos se generaron con retroceso recursivo, que garantiza por construcción que todas las celdas del piso queden conectadas, y después se abrieron celdas adicionales para dar amplitud a los corredores. La existencia de un camino de `p` a `g` se verificó ejecutando una búsqueda en anchura sobre cada mapa, no razonando el recorrido a mano; ambos reportaron `REACHED GOAL`. También se verificó que todas las filas de cada archivo tengan exactamente la misma longitud, para que el análisis fila por fila del archivo no se rompa.

## Texturas

Las texturas de pared se generan por código en `src/texture.rs`, en mapas de 64 x 64 píxeles, usando una función de ruido determinista: la piedra se compone de bloques con junta de mortero, el metal de placas con costuras, remaches y manchas de óxido, y la enredadera de una base de piedra musgosa con tallos ondulados y hojas. No se descargaron imágenes de internet, así que ninguna textura del proyecto proviene de un archivo externo.

Si se quiere usar texturas reales en su lugar, basta colocar archivos llamados `stone.png`, `metal.png` y `vine.png` en `assets/textures/`. El juego intenta cargarlos primero con la crate `image` y solo si falla recurre a las texturas generadas por código.

## Sobre la música

La pista instrumental de fondo es original y está sintetizada por código en `src/audio.rs`: una progresión de acordes con bajo y una melodía de onda cuadrada, en bucle. Es un instrumental original inspirado en estilo pop, ya que no se puede incluir audio con derechos de autor. No se descargó ni se incluyó ningún archivo de audio de una canción real.

Los efectos de sonido también se sintetizan por código: el golpe contra la pared es una onda baja con decaimiento y un poco de ruido, y el sonido de victoria es un arpegio ascendente.

## Nota sobre el texto en pantalla

El texto de las pantallas se dibuja con una fuente de mapa de bits de 5x7 escrita a mano en `src/text.rs`, que solo cubre letras sin tilde, dígitos y algunos signos. Por eso los rótulos del juego aparecen en mayúsculas y sin acentos; es una limitación de la fuente, no del idioma del proyecto.

## Estructura

```
.
├── Cargo.toml
├── README.md
├── assets/
│   └── textures/
├── mazes/
│   ├── glade.txt
│   └── sector_oeste.txt
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
