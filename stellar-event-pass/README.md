# Stellar Event Pass

Smart contract desarrollado con Soroban sobre Stellar Testnet para implementar un sistema básico de pases para eventos.

El proyecto permite registrar un pase para un usuario, consultar su existencia, verificar si ha sido utilizado y actualizar su estado cuando el pase es utilizado.

Este proyecto fue desarrollado como parte del programa **Stellar Elite Bolivia**.



## Descripción

Stellar Event Pass implementa un contrato inteligente que representa el ciclo de vida de un pase digital para un evento.

El contrato mantiene el estado de cada usuario y permite realizar las siguientes operaciones:

* Registrar un pase.
* Verificar si un usuario posee un pase.
* Consultar si un pase ha sido utilizado.
* Marcar un pase como utilizado.

El flujo principal del contrato es:

```text
buy_pass
    |
    v
has_pass
    |
    v
is_used = false
    |
    v
use_pass
    |
    v
is_used = true
```



## Tecnologías

* Rust
* Soroban
* Stellar
* Stellar CLI
* Stellar Testnet
* WebAssembly (WASM)
* Cargo



## Estructura del proyecto

```text
stellar-event-pass/
|
├── contracts/
│   └── hello-world/
│       ├── src/
│       │   ├── lib.rs
│       │   └── test.rs
│       │
│       └── Cargo.toml
│
├── Cargo.toml
├── AGENTS.md
└── README.md
```

### contracts

Contiene los contratos inteligentes del proyecto.

### contracts/hello-world

Contiene la implementación del contrato Event Pass.

### src/lib.rs

Contiene la lógica principal del contrato Soroban.

### src/test.rs

Contiene las pruebas utilizadas para validar el comportamiento del contrato.

### Cargo.toml

Contiene la configuración del proyecto y las dependencias necesarias para compilar el contrato.



## Requisitos

Antes de ejecutar el proyecto es necesario contar con:

* Rust
* Cargo
* Git
* Stellar CLI
* Target de Rust `wasm32v1-none`
* Una cuenta de Stellar Testnet con fondos suficientes para realizar transacciones

Verificar Rust:

```powershell
rustc --version
```

Verificar Cargo:

```powershell
cargo --version
```

Verificar Git:

```powershell
git --version
```

Verificar Stellar CLI:

```powershell
stellar --version
```



## Preparación del proyecto

Ubicar el proyecto en la carpeta de trabajo:

```powershell
cd C:\stellar
```

Entrar al proyecto:

```powershell
cd stellar-event-pass
```

Verificar la estructura:

```powershell
dir
```

El directorio debe contener:

```text
contracts
Cargo.toml
AGENTS.md
README.md
```



## Compilación

El contrato puede compilarse utilizando Stellar CLI:

```powershell
stellar contract build
```

El proceso genera el archivo WebAssembly utilizado para desplegar el contrato.

El archivo generado se encuentra en:

```text
target/wasm32v1-none/release/hello_world.wasm
```

Durante la compilación del proyecto se obtuvo:

```text
Wasm File:
target/wasm32v1-none/release/hello_world.wasm

Wasm Hash:
13f466c5972b343c534ff22ee34f1c83da0a196b8aed80965c591b9664b8a5ae

Wasm Size:
927 bytes optimizado
```

El contrato exporta las siguientes funciones:

```text
buy_pass
has_pass
is_used
use_pass
```



## Despliegue

El contrato fue desplegado sobre Stellar Testnet utilizando Stellar CLI.

Comando utilizado:

```powershell
stellar contract deploy --source-account eventpass --network testnet
```

El proceso realiza la compilación del contrato, publica el WASM y genera una instancia del contrato en la red.

### Contract ID

```text
CCJOMWCF3UI4Z66PP7KINJF4I7IHLHVLWUVETC4BHBLIBATLMEFLUC2S
```

### Red

```text
Stellar Testnet
```

### Contrato en Stellar Lab

https://lab.stellar.org/r/testnet/contract/CCJOMWCF3UI4Z66PP7KINJF4I7IHLHVLWUVETC4BHBLIBATLMEFLUC2S



## Funciones del contrato

### buy_pass

Registra un pase para una dirección de usuario.

```text
buy_pass(user)
```

Ejemplo:

```powershell
stellar contract invoke --id CCJOMWCF3UI4Z66PP7KINJF4I7IHLHVLWUVETC4BHBLIBATLMEFLUC2S --source-account eventpass --network testnet -- buy_pass --user GD26UBYVEYYVVOVCMOLPMIKPWQRFV34LK3I7LHBNTUGYHYIKFMEREH2A
```

Una ejecución exitosa genera una transacción en Stellar Testnet.



### has_pass

Comprueba si un usuario posee un pase.

```text
has_pass(user)
```

Ejemplo:

```powershell
stellar contract invoke --id CCJOMWCF3UI4Z66PP7KINJF4I7IHLHVLWUVETC4BHBLIBATLMEFLUC2S --source-account eventpass --network testnet -- has_pass --user GD26UBYVEYYVVOVCMOLPMIKPWQRFV34LK3I7LHBNTUGYHYIKFMEREH2A
```

Después de registrar correctamente el pase, el resultado esperado es:

```text
true
```



### is_used

Comprueba si el pase de un usuario ya fue utilizado.

```text
is_used(user)
```

Ejemplo:

```powershell
stellar contract invoke --id CCJOMWCF3UI4Z66PP7KINJF4I7IHLHVLWUVETC4BHBLIBATLMEFLUC2S --source-account eventpass --network testnet -- is_used --user GD26UBYVEYYVVOVCMOLPMIKPWQRFV34LK3I7LHBNTUGYHYIKFMEREH2A
```

Antes de utilizar el pase, el resultado esperado es:

```text
false
```



### use_pass

Marca el pase como utilizado.

```text
use_pass(user)
```

Ejemplo:

```powershell
stellar contract invoke --id CCJOMWCF3UI4Z66PP7KINJF4I7IHLHVLWUVETC4BHBLIBATLMEFLUC2S --source-account eventpass --network testnet -- use_pass --user GD26UBYVEYYVVOVCMOLPMIKPWQRFV34LK3I7LHBNTUGYHYIKFMEREH2A
```

Después de ejecutar correctamente la función, el estado del pase cambia.

Al consultar nuevamente:

```powershell
stellar contract invoke --id CCJOMWCF3UI4Z66PP7KINJF4I7IHLHVLWUVETC4BHBLIBATLMEFLUC2S --source-account eventpass --network testnet -- is_used --user GD26UBYVEYYVVOVCMOLPMIKPWQRFV34LK3I7LHBNTUGYHYIKFMEREH2A
```

el resultado esperado es:

```text
true
```



## Flujo de ejecución

El flujo completo de interacción con el contrato es:

```text
1. Desplegar el contrato
          |
          v
2. Registrar el pase
       buy_pass
          |
          v
3. Verificar el pase
       has_pass
          |
          v
4. Comprobar estado
       is_used
          |
          v
5. Utilizar el pase
       use_pass
          |
          v
6. Comprobar nuevamente
       is_used
          |
          v
       true
```

Este flujo demuestra tanto la consulta como la modificación del estado almacenado por el contrato.



## Evidencia de transacción

Durante las pruebas se realizó correctamente una invocación de `buy_pass`.

Transaction Hash:

```text
6be268a284eee59916485c32eadc2d89d092c1b14c60443969cb504996147181
```

La transacción puede verificarse en Stellar Expert:

https://stellar.expert/explorer/testnet/tx/6be268a284eee59916485c32eadc2d89d092c1b14c60443969cb504996147181

La transacción corresponde a una invocación del contrato sobre Stellar Testnet.



## Verificación

El contrato desplegado puede consultarse desde Stellar Lab:

https://lab.stellar.org/r/testnet/contract/CCJOMWCF3UI4Z66PP7KINJF4I7IHLHVLWUVETC4BHBLIBATLMEFLUC2S

Las transacciones generadas durante las pruebas pueden verificarse mediante Stellar Expert.

Esto permite comprobar que las operaciones fueron ejecutadas sobre Stellar Testnet y no únicamente de manera local.



## Resultados

| Elemento             | Resultado                                                          |
| -- |  |
| Plataforma           | Stellar                                                            |
| Entorno de contratos | Soroban                                                            |
| Red                  | Testnet                                                            |
| Lenguaje             | Rust                                                               |
| Formato              | WebAssembly                                                        |
| Contract ID          | `CCJOMWCF3UI4Z66PP7KINJF4I7IHLHVLWUVETC4BHBLIBATLMEFLUC2S`         |
| Funciones            | `buy_pass`, `has_pass`, `is_used`, `use_pass`                      |
| WASM Hash            | `13f466c5972b343c534ff22ee34f1c83da0a196b8aed80965c591b9664b8a5ae` |



## Aprendizajes

El desarrollo de este proyecto permitió trabajar de manera práctica con el ciclo completo de un contrato Soroban:

```text
Desarrollo
    |
    v
Compilación
    |
    v
WebAssembly
    |
    v
Despliegue
    |
    v
Stellar Testnet
    |
    v
Invocación
    |
    v
Transacción
    |
    v
Actualización del estado
```

Uno de los principales aprendizajes fue comprender la diferencia entre desarrollar y probar un contrato localmente y realizar una interacción real con un contrato desplegado en una red blockchain.

El proyecto también permitió comprobar cómo un contrato puede mantener información asociada a usuarios y modificar su estado mediante transacciones verificables públicamente.



## Conclusión

Stellar Event Pass demuestra una implementación funcional de un caso de uso sencillo utilizando Soroban sobre Stellar Testnet.

El proyecto cubre el desarrollo, compilación, despliegue e interacción con un contrato inteligente, además de la verificación de las transacciones generadas en la red.

El resultado es un flujo completo en el que un usuario puede registrar un pase, consultar su existencia, verificar su estado y marcarlo como utilizado mediante funciones del contrato.

Este proyecto forma parte del proceso de aprendizaje y experimentación con el ecosistema Stellar y Soroban dentro de **Stellar Elite Bolivia**.



## Recursos

* Stellar: https://stellar.org/
* Soroban: https://soroban.stellar.org/
* Stellar Lab: https://lab.stellar.org/
* Stellar Expert: https://stellar.expert/

### Contract ID

```text
CCJOMWCF3UI4Z66PP7KINJF4I7IHLHVLWUVETC4BHBLIBATLMEFLUC2S
```

### Stellar Lab

https://lab.stellar.org/r/testnet/contract/CCJOMWCF3UI4Z66PP7KINJF4I7IHLHVLWUVETC4BHBLIBATLMEFLUC2S
