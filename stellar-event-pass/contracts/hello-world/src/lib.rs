#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    symbol_short,
    Address,
    Env,
};


// ============================================================
// CONTRATO EVENT PASS
// ============================================================
//
// Este contrato permite:
//
// 1. Obtener un pase.
// 2. Comprobar si una dirección tiene pase.
// 3. Utilizar el pase una sola vez.
// 4. Comprobar si el pase ya fue utilizado.
//
// ============================================================

#[contract]
pub struct EventPass;


#[contractimpl]
impl EventPass {

    // ========================================================
    // FUNCIÓN: buy_pass
    // ========================================================
    //
    // La persona obtiene un pase.
    //
    // Después de ejecutar esta función:
    //
    // has_pass = true
    // used     = false
    //
    // ========================================================

    pub fn buy_pass(
        env: Env,
        user: Address,
    ) {

        // La persona debe autorizar la operación.
        user.require_auth();


        // Guardamos que esta dirección tiene un pase.
        env.storage()
            .persistent()
            .set(
                &(symbol_short!("pass"), user.clone()),
                &true
            );


        // El pase todavía no ha sido utilizado.
        env.storage()
            .persistent()
            .set(
                &(symbol_short!("used"), user),
                &false
            );
    }


    // ========================================================
    // FUNCIÓN: use_pass
    // ========================================================
    //
    // Utiliza el pase.
    //
    // Solo puede utilizarse una vez.
    //
    // ========================================================

    pub fn use_pass(
        env: Env,
        user: Address,
    ) {

        // La persona debe autorizar el uso del pase.
        user.require_auth();


        // ----------------------------------------------------
        // COMPROBAR SI TIENE PASE
        // ----------------------------------------------------

        let has_pass: bool = env
            .storage()
            .persistent()
            .get(
                &(symbol_short!("pass"), user.clone())
            )
            .unwrap_or(false);


        // Si no tiene pase, rechazamos la operación.
        if !has_pass {
            panic!("User does not have a pass");
        }


        // ----------------------------------------------------
        // COMPROBAR SI YA UTILIZÓ EL PASE
        // ----------------------------------------------------

        let used: bool = env
            .storage()
            .persistent()
            .get(
                &(symbol_short!("used"), user.clone())
            )
            .unwrap_or(false);


        // Si ya fue utilizado, rechazamos la operación.
        if used {
            panic!("Pass already used");
        }


        // ----------------------------------------------------
        // MARCAR EL PASE COMO UTILIZADO
        // ----------------------------------------------------

        env.storage()
            .persistent()
            .set(
                &(symbol_short!("used"), user),
                &true
            );
    }


    // ========================================================
    // FUNCIÓN: has_pass
    // ========================================================
    //
    // Comprueba si una dirección tiene un pase.
    //
    // Devuelve:
    //
    // true  → tiene pase
    // false → no tiene pase
    //
    // ========================================================

    pub fn has_pass(
        env: Env,
        user: Address,
    ) -> bool {

        env.storage()
            .persistent()
            .get(
                &(symbol_short!("pass"), user)
            )
            .unwrap_or(false)
    }


    // ========================================================
    // FUNCIÓN: is_used
    // ========================================================
    //
    // Comprueba si el pase ya fue utilizado.
    //
    // Devuelve:
    //
    // true  → ya fue utilizado
    // false → todavía no
    //
    // ========================================================

    pub fn is_used(
        env: Env,
        user: Address,
    ) -> bool {

        env.storage()
            .persistent()
            .get(
                &(symbol_short!("used"), user)
            )
            .unwrap_or(false)
    }
}


// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod test;