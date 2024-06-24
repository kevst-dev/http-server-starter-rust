/*
    Hace falta una forma de hacer varias solicitudes al servidor
    simultaneamente

    Running tests. Logs should appear shortly...

    [tester::#EJ5] Running tests for Stage #EJ5 (Concurrent connections)
    [tester::#EJ5] $ ./your_server.sh
    [tester::#EJ5] Creating 2 parallel connections
    [your_program] warning: unused import: `std::collections::HashMap`
    [your_program] Directory: "."
    [your_program] Server is starting...
    [tester::#EJ5] client-1: $ curl -v http://localhost:4221/
    [your_program] Accepting connection from 127.0.0.1:58022
    [your_program] Accepting connection from 127.0.0.1:58028
    [tester::#EJ5] Received response with 200 status code
    [tester::#EJ5] client-2: $ curl -v http://localhost:4221/
    [tester::#EJ5] Received response with 200 status code
    [tester::#EJ5] Creating 2 parallel connections
    [tester::#EJ5] client-1: $ curl -v http://localhost:4221/
    [your_program] Accepting connection from 127.0.0.1:58030
    [your_program] Accepting connection from 127.0.0.1:58046
    [tester::#EJ5] Received response with 200 status code
    [tester::#EJ5] client-2: $ curl -v http://localhost:4221/
    [tester::#EJ5] Received response with 200 status code
    [tester::#EJ5] Test passed.

*/
