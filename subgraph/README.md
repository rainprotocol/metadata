steps
- nix-shell --run init
- nix-shell --run docker-up
- nix-shell --run local-test

rust test 
- nix-shell --run init
- nix-shell --run docker-up
- `cargo test` or `cargo test -- --nocapture` for console output