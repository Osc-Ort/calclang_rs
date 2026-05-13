# calclang_rs
Lenguaje calculadora de juguete con números enteros de tamaño variable, hecho en Rust.

Los números se meten a la pila, los operadores operan con el tope de la pila en modo LIFO
+,-,/,*,POW operadores validos
PRINT imprime el tope
BEFORE contiene el tope de la operación anterior

Ejecución sin argumentos para modo interactivo, argumentos para ejecución secuencial y concatenada de archivos (comparte pila).

Ej:
```
10 10 +
PRINT
"Imprimira 20"

0 10 /
PRINT
"Imprimira 10**100000 (número muy alto para representar infinito con enteros)"

```
