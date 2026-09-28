# Inversión mínima de 500 en el RWA Launchpad

Este documento explica, paso a paso, qué había en el proyecto, qué se cambió y cómo quedó desplegado en testnet. Sirve como bitácora del trabajo y como borrador para el blog.

## Entregables

| Qué | Dónde |
| --- | --- |
| Repositorio local | `C:\stellar\stellar-Launchpad` (dentro del repo git `C:\stellar`, remoto [Pericena/Stellar-Build](https://github.com/Pericena/Stellar-Build)) |
| Contract ID | `CB5SEQH4REXIVHLGZ5MW7O7TFRZV5JM4RWIZAOIMDLA2NHFZDOQ55FUS` |
| Contrato en Stellar Expert | [contrato en testnet](https://stellar.expert/explorer/testnet/contract/CB5SEQH4REXIVHLGZ5MW7O7TFRZV5JM4RWIZAOIMDLA2NHFZDOQ55FUS) |
| Inversión exitosa de 500 | [transacción `3b8ea15…a4cfe8`](https://stellar.expert/explorer/testnet/tx/3b8ea15f82b7065ff2358bda74bfbd24d5032703595ea837810d904d79a4cfe8) |
| Captura de la inversión exitosa | [`docs/invest-500-stellar-expert.png`](docs/invest-500-stellar-expert.png) |
| Inversión fallida de 100 | No tiene hash. La CLI la rechaza en la simulación con `Error(Contract, #7)` antes de enviarla a la red. La evidencia es la salida de la terminal. |

## Qué es esta carpeta

`stellar-Launchpad` es el contrato del día 3 del bootcamp de RWA Launchpad (Soroban). El mismo código de partida vive en `C:\stellar\rwa-launchpad-bootcamp\dia-3`. Esta carpeta es la copia sobre la que se implementó la variación, se corrieron los tests y se desplegó a testnet.

El contrato se llama `RwaLaunchpad`. Un administrador crea un activo RWA, aprueba inversionistas y, si quiere, pausa el contrato o retira los tokens de pago cobrados. El inversionista paga con un token SEP-41 y recibe a cambio unidades del activo RWA.

El precio quedó fijado en la inicialización:

- nombre: `RWAToken`
- `price_per_unit`: 100
- `total_supply` declarado: 1 000 000
- token de pago: el contrato SAC del activo clásico `PAY`

Con ese precio, pagar 500 unidades de `PAY` acuña `500 / 100 = 5` unidades RWA.

## Qué había antes de tocarlo

`check_variation_gate` era un gancho vacío. El día 2 del bootcamp deja esa función para que cada equipo ponga su regla de acceso. En el día 3, `invest` la llama antes de cobrar y de acuñar.

La versión original no recibía el monto y siempre devolvía `Ok(())`:

```rust
fn check_variation_gate(env: &Env, investor: &Address) -> Result<(), Error> {
    let _ = (env, investor);
    Ok(())
}
```

`invest` ya hacía el resto del trabajo: exigir que el contrato esté inicializado, que el inversionista firme, que no esté pausado, que el monto sea positivo, que el inversionista esté en la whitelist, cobrar el token de pago y acuñar el RWA.

Los scripts `scripts/admin-tool.sh` y `scripts/user-tool.sh` ya invocaban el contrato en testnet. El de admin inicializaba, ponía en whitelist, y tenía comandos extra de mint, withdraw, pause y unpause. El de usuario invertía 500, consultaba el balance y transfería.

## La regla nueva

Cada llamada a `invest` debe enviar al menos 500 unidades del token de pago. Si el monto es menor, la inversión aborta con el error `AmountTooLow`.

500 entra. 100 no entra. El umbral es inclusivo: “al menos 500”.

## Cambios en el contrato

### 1. Error nuevo

En `src/lib.rs`, el enum `Error` ya tenía códigos del 1 al 6. Se agregó el 7:

```rust
AmountTooLow = 7,
```

En testnet ese código aparece como `HostError: Error(Contract, #7)`. El número 7 es `AmountTooLow`. No es un fallo de red ni de firma.

### 2. La compuerta recibe el monto

La función original no veía cuánto quería invertir la persona. Se le agregó `payment_amount` y una constante:

```rust
const MIN_INVESTMENT: i128 = 500;

fn check_variation_gate(
    env: &Env,
    investor: &Address,
    payment_amount: i128,
) -> Result<(), Error> {
    let _ = (env, investor);
    if payment_amount < Self::MIN_INVESTMENT {
        return Err(Error::AmountTooLow);
    }
    Ok(())
}
```

`invest` ahora la llama así, justo después de `require_auth` y antes de la whitelist y del cobro:

```rust
if let Err(err) = Self::check_variation_gate(&env, &investor, payment_amount) {
    panic_with_error!(&env, err);
}
```

Si el monto es menor a 500, el contrato entra en pánico con el error 7. No se transfiere `PAY` y no se acuña RWA. El orden importa: un intento de 100 falla por monto bajo aunque el inversionista esté en la whitelist.

## El test

`src/test.rs` tiene `test_invest_minimum_payment_amount`. Hace las dos comprobaciones en un solo test:

1. Prepara un contrato, un token de pago y un inversionista con 1 000 unidades.
2. El admin lo mete en la whitelist.
3. `try_invest` de 100 devuelve `AmountTooLow`. El balance RWA sigue en 0 y el token de pago sigue en 1 000: no se cobró nada.
4. `invest` de 500 devuelve 5. El balance RWA queda en 5 y el token de pago baja a 500.

Los tests que ya existían (`test_invest`, `test_withdraw`, `test_invest_not_whitelisted`) siguen pasando, porque invierten 500 y por lo tanto cruzan la compuerta.

Resultado local:

```text
running 4 tests
test test::test_invest_not_whitelisted ... ok
test test::test_invest ... ok
test test::test_invest_minimum_payment_amount ... ok
test test::test_withdraw ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured
```

## Cuentas y contratos en testnet

Las llaves viven solo en la máquina, en `~/.config/stellar/identity`. No van al repositorio.

| Rol | Identidad de la CLI | Dirección |
| --- | --- | --- |
| Admin | `alice` | `GBLCANYC7VQDIVCTWJPUIPY65JNDBXCJARMK3IVLHX76NOHOMS5VUKU4` |
| Inversionista | `bob` | `GBCCUOT523IM3O3KZKA6MMMQZRUCHNMKSAFFD4DP2TJ3LQS6RCGFDV2M` |

El bootcamp espera un token de pago publicado por el instructor. Aquí no había esa dirección, así que se desplegó un activo clásico propio y su Stellar Asset Contract:

| Pieza | Valor |
| --- | --- |
| Activo clásico | `PAY:GBLCANYC7VQDIVCTWJPUIPY65JNDBXCJARMK3IVLHX76NOHOMS5VUKU4` |
| Contrato del token de pago | `CBNZONKIEZBZF4CN3TGQHYRUDEOI6MWNHCHDBX2JUVPXFBEKOLG6JUTL` |
| Launchpad | `CB5SEQH4REXIVHLGZ5MW7O7TFRZV5JM4RWIZAOIMDLA2NHFZDOQ55FUS` |
| Hash del Wasm | `7c60dc0dd81dc7563f259b397e134da2b62aca87b39720519597741adb2e9f61` |

Ambas cuentas se crearon y se fondearon con Friendbot:

```powershell
stellar keys generate alice --network testnet --fund
stellar keys generate bob --network testnet --fund
```

## Paso a paso del despliegue

### 1. Compilar

Desde `C:\stellar\stellar-Launchpad`, con el target `wasm32v1-none` ya instalado:

```powershell
stellar contract build
```

Eso produjo el Wasm optimizado de unas 7.8 KB.

### 2. Publicar el token de pago

```powershell
stellar contract asset deploy `
  --asset "PAY:GBLCANYC7VQDIVCTWJPUIPY65JNDBXCJARMK3IVLHX76NOHOMS5VUKU4" `
  --source alice `
  --network testnet
```

Contrato resultante: `CBNZONKIEZBZF4CN3TGQHYRUDEOI6MWNHCHDBX2JUVPXFBEKOLG6JUTL`.

Bob necesita una trustline para poder recibir `PAY`:

```powershell
stellar tx new change-trust `
  --source bob `
  --network testnet `
  --line "PAY:GBLCANYC7VQDIVCTWJPUIPY65JNDBXCJARMK3IVLHX76NOHOMS5VUKU4"
```

Alice, que es la emisora, acuña 10 000 unidades a Bob:

```powershell
stellar contract invoke `
  --id CBNZONKIEZBZF4CN3TGQHYRUDEOI6MWNHCHDBX2JUVPXFBEKOLG6JUTL `
  --source alice `
  --network testnet `
  -- mint `
  --to GBCCUOT523IM3O3KZKA6MMMQZRUCHNMKSAFFD4DP2TJ3LQS6RCGFDV2M `
  --amount 10000
```

Esa acuñación quedó en [esta transacción](https://stellar.expert/explorer/testnet/tx/aa47653e4c763692bba68b79355173b2ed4b675a54686703c32d5187034347db). Volver a ejecutar el mismo `mint` suma otras 10 000; la corrida hecha después desde la terminal local quedó en [esta otra](https://stellar.expert/explorer/testnet/tx/37bcc1b76e67183f854e2d2564808f84c6ea43937a25f269113dd5e44eed7581).

### 3. Desplegar el launchpad

```powershell
stellar contract deploy `
  --wasm "<ruta del wasm generado por stellar contract build>" `
  --source alice `
  --network testnet `
  --alias rwa-launchpad
```

Transacciones:

- subida del Wasm: [5ee47251…](https://stellar.expert/explorer/testnet/tx/5ee4725136da1479c8d917a32e62afc26b1c604c80ff93f53f790e9f92605fe4)
- creación de la instancia: [cca3c0e5…](https://stellar.expert/explorer/testnet/tx/cca3c0e58d509ee4912b71a8ab3a9657187b2a58da546f2472a9f8589094e511)

Contract ID: `CB5SEQH4REXIVHLGZ5MW7O7TFRZV5JM4RWIZAOIMDLA2NHFZDOQ55FUS`.

## Paso a paso del flujo admin

El script es `scripts/admin-tool.sh`. Se ejecuta con Git Bash y estas variables:

```powershell
$env:NETWORK="testnet"
$env:ADMIN_KEY="alice"
$env:CONTRACT_ID="CB5SEQH4REXIVHLGZ5MW7O7TFRZV5JM4RWIZAOIMDLA2NHFZDOQ55FUS"
$env:PAYMENT_TOKEN="CBNZONKIEZBZF4CN3TGQHYRUDEOI6MWNHCHDBX2JUVPXFBEKOLG6JUTL"
$env:INVESTOR="GBCCUOT523IM3O3KZKA6MMMQZRUCHNMKSAFFD4DP2TJ3LQS6RCGFDV2M"
& "C:\Program Files\Git\bin\bash.exe" scripts/admin-tool.sh
```

Hace dos cosas y se detiene:

1. `initialize`. Alice queda como admin. El activo se llama `RWAToken`, el precio por unidad es 100 y el token de pago es el SAC de `PAY`.
2. `set_whitelist` con `--approved true` para la dirección de Bob.

En Stellar CLI 28 los enteros `i128` del JSON tienen que ir entre comillas. El script original mandaba `1000000` y `100` como números y la CLI respondía `invalid type: number, expected string or map`. El script ahora manda `"1000000"` y `"100"`, que es el formato que muestra `initialize --help`.

Transacciones:

- initialize: [c9e2e622…](https://stellar.expert/explorer/testnet/tx/c9e2e6224168effc06a545eaf1707b6e0d6e5968f6c4c684bfb757cf1a107c2c)
- whitelist: [749fd360…](https://stellar.expert/explorer/testnet/tx/749fd36092362823e6cb8c9930dc8a953e2ad26b5340376c9d1a26cd2f37029e)

`initialize` solo puede correr una vez. Una segunda llamada falla con `AlreadyInitialized` (error 2).

Mint, withdraw, pause y unpause siguen en el script. No forman parte de este flujo. Se corren con `RUN_EXTRAS=1`. Si se lanzan antes de que el contrato haya cobrado, `withdraw` de 500 falla porque todavía no hay tokens de pago adentro.

## Paso a paso del flujo del inversionista

El script es `scripts/user-tool.sh`, con las mismas variables y `USER_KEY=bob`.

### Intento de 100: debe fallar

```powershell
stellar contract invoke `
  --id CB5SEQH4REXIVHLGZ5MW7O7TFRZV5JM4RWIZAOIMDLA2NHFZDOQ55FUS `
  --source bob `
  --network testnet `
  --send yes `
  -- invest `
  --investor GBCCUOT523IM3O3KZKA6MMMQZRUCHNMKSAFFD4DP2TJ3LQS6RCGFDV2M `
  --payment_amount 100
```

Salida real:

```text
❌ error: transaction simulation failed: HostError: Error(Contract, #7)

Event log (newest first):
   0: [Diagnostic Event] contract:CB5SEQH4REXIVHLGZ5MW7O7TFRZV5JM4RWIZAOIMDLA2NHFZDOQ55FUS, topics:[error, Error(Contract, #7)], data:"escalating error to VM trap from failed host function call: fail_with_error"
   1: [Diagnostic Event] contract:CB5SEQH4REXIVHLGZ5MW7O7TFRZV5JM4RWIZAOIMDLA2NHFZDOQ55FUS, topics:[error, Error(Contract, #7)], data:["failing with contract error", 7]
   2: [Diagnostic Event] topics:[fn_call, CB5SEQH4REXIVHLGZ5MW7O7TFRZV5JM4RWIZAOIMDLA2NHFZDOQ55FUS, invest], data:[GBCCUOT523IM3O3KZKA6MMMQZRUCHNMKSAFFD4DP2TJ3LQS6RCGFDV2M, 100]
```

Qué se lee ahí:

- La línea `fn_call` muestra que Bob llamó `invest` con monto `100`.
- El contrato respondió con el error de contrato número 7, que es `AmountTooLow`.
- `--send yes` no crea una transacción en el ledger. La CLI simula primero. Si la simulación revienta dentro del contrato, no hay transacción válida que enviar, así que Stellar Expert no tiene un hash de este fallo.
- Para el blog o el video, la captura es esta terminal. Ahí se ve el intento de 100 y el `#7`.

El script de usuario espera ese fallo: si `invest` de 100 saliera con código 0, el script se detiene. Si falla, imprime `invest 100 failed as expected` y sigue.

### Inversión de 500: funciona

La misma llamada con `--payment_amount 500` se envió y quedó confirmada.

- Hash: `3b8ea15f82b7065ff2358bda74bfbd24d5032703595ea837810d904d79a4cfe8`
- Enlace: https://stellar.expert/explorer/testnet/tx/3b8ea15f82b7065ff2358bda74bfbd24d5032703595ea837810d904d79a4cfe8
- Ledger: 4923038
- Hora: 2026-09-28 23:39:37 UTC
- Cuenta que firmó: Bob (`GBCCUO…GFDV2M`)
- Llamada que muestra Stellar Expert: `invest(GBCC…DV2M, 500) → 5`
- Evento del token `PAY`: transferencia de 500 desde Bob hacia el contrato del launchpad
- Evento `invest` del launchpad: inversionista, pago 500, RWA acuñado 5

La captura está en `docs/invest-500-stellar-expert.png`.

### Consulta de balance

```text
=== balance ===
ℹ️  Simulation identified as read-only. Send by rerunning with `--send=yes`.
"5"
```

`balance` no escribe en el ledger, por eso la CLI solo simula. El resultado `"5"` es el saldo RWA de Bob: las 5 unidades que acaba de recibir. No hace falta `--send=yes` para leerlo.

## Qué mostrar en el video o en las capturas

1. Terminal del intento de 100, con `Error(Contract, #7)` y el argumento `100` en el log. Esa pantalla ya está en la terminal local. No hay página de Stellar Expert para ese fallo.
2. La página de la transacción de 500, donde se lee `invest(..., 500) → 5`. La imagen guardada es `docs/invest-500-stellar-expert.png`.
3. Opcional: la línea `"5"` del `balance`.

## Mapa de archivos tocados

| Archivo | Qué cambió |
| --- | --- |
| `src/lib.rs` | Error `AmountTooLow = 7`. `check_variation_gate` exige `payment_amount >= 500`. `invest` le pasa el monto. |
| `src/test.rs` | Test que rechaza 100 y acepta 500. |
| `scripts/admin-tool.sh` | JSON de `initialize` con los `i128` como strings. El flujo por defecto es initialize + whitelist. |
| `scripts/user-tool.sh` | Primero intenta invertir 100 y exige que falle. Después invierte 500 y consulta el balance. |
| `docs/invest-500-stellar-expert.png` | Captura de la inversión exitosa. |
| `.gitignore` | Ignora `target`, Wasm, `Cargo.lock` y `.env`. |

## Cómo repetir solo la parte del usuario

El contrato ya está inicializado y Bob ya está en la whitelist. No vuelvas a correr `initialize`. Para repetir la demo:

```powershell
cd C:\stellar\stellar-Launchpad
$env:NETWORK="testnet"
$env:USER_KEY="bob"
$env:CONTRACT_ID="CB5SEQH4REXIVHLGZ5MW7O7TFRZV5JM4RWIZAOIMDLA2NHFZDOQ55FUS"
& "C:\Program Files\Git\bin\bash.exe" scripts/user-tool.sh
```

Bob todavía tiene `PAY` de sobra. Otra inversión de 500 vuelve a funcionar y suma otras 5 unidades RWA. Otra de 100 vuelve a fallar con el error 7.
