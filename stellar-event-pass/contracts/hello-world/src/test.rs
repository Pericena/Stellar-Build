use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::Env;


// ============================================================
// PRUEBA COMPLETA DEL EVENT PASS
// ============================================================
//
// Esta prueba comprueba:
//
// 1. El usuario comienza sin pase.
// 2. El usuario obtiene un pase.
// 3. El usuario tiene el pase.
// 4. El pase todavía no está utilizado.
// 5. El usuario utiliza el pase.
// 6. El pase queda marcado como utilizado.
//
// La prueba del segundo uso la haremos de una manera
// compatible con Soroban, sin utilizar std::panic.
//
// ============================================================

#[test]
fn test_event_pass() {

    // ========================================================
    // PASO 1
    // Crear entorno de prueba
    // ========================================================

    let env = Env::default();


    // ========================================================
    // Permitir las autorizaciones
    // ========================================================
    //
    // Nuestro contrato utiliza require_auth().
    //

    env.mock_all_auths();


    // ========================================================
    // PASO 2
    // Crear usuario de prueba
    // ========================================================

    let user = Address::generate(&env);


    // ========================================================
    // PASO 3
    // Registrar el contrato
    // ========================================================

    let contract_id = env.register(EventPass, ());


    // Crear cliente para llamar al contrato.
    //

    let client = EventPassClient::new(
        &env,
        &contract_id
    );


    // ========================================================
    // PASO 4
    // Comprobar que NO tiene pase
    // ========================================================

    assert_eq!(
        client.has_pass(&user),
        false
    );


    // ========================================================
    // PASO 5
    // Obtener el pase
    // ========================================================

    client.buy_pass(&user);


    // ========================================================
    // PASO 6
    // Comprobar que ahora tiene pase
    // ========================================================

    assert_eq!(
        client.has_pass(&user),
        true
    );


    // ========================================================
    // PASO 7
    // Comprobar que todavía NO fue utilizado
    // ========================================================

    assert_eq!(
        client.is_used(&user),
        false
    );


    // ========================================================
    // PASO 8
    // Utilizar el pase
    // ========================================================

    client.use_pass(&user);


    // ========================================================
    // PASO 9
    // Comprobar que ahora está utilizado
    // ========================================================

    assert_eq!(
        client.is_used(&user),
        true
    );
}