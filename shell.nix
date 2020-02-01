with import <nixpkgs> {};
let
    shared = [
        rustup
        shellcheck
    ];
    hook = ''
        . .shellhook
    '';
in
{
    darwin = mkShell {
        buildInputs = shared;
        shellHook = hook;
    };
    linux = mkShell {
        buildInputs = [
            pkg-config
        ] ++ shared;
        shellHook = hook;
    };
}
