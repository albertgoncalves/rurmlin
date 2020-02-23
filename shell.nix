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
            sxiv
        ] ++ shared;
        shellHook = hook;
    };
}
