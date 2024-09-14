example-e2ee-simple:
  cargo run -p e2ee --example e2ee_simple
example-e2ee-key-generation:
  cargo run -p e2ee --example e2ee_key_generation
example-e2ee-server-encrypt message size:
  cargo run -p e2ee --example e2ee_server_encrypt -- -m "{{message}}" -s {{size}}
example-e2ee-client-encrypt message:
  cargo run -p e2ee --example e2ee_client_encrypt -- -m "{{message}}"
example-e2ee-server-decrypt:
  cargo run -p e2ee --example e2ee_server_decrypt
test-e2ee-lib:
  cargo test -p e2ee --tests 
test-e2ee-doc:
  cargo test -p e2ee --doc

