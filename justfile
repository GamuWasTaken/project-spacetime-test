
init:
    nix-shell -p rocmPackages.llvm.lld binaryen openssl_oqs pkg-config nodejs_22 pnpm svelte-language-server typescript-language-server

latest-spacetime:
    nix-shell -I nixpkgs=https://github.com/NixOS/nixpkgs/archive/master.tar.gz -p spacetimedb

gen:
    spacetime generate --lang typescript --out-dir client/src/lib/module_bindings --module-path spacetimedb/

publish:
    spacetime publish -s local maindb --delete-data

[working-directory: 'client']
run-client:
    pnpm run dev

run-db:
    spacetime start

follow-logs:
    spacetime logs --follow maindb -s local 

split:
    alacritty & disown
    alacritty & disown
    alacritty & disown
    alacritty & disown
