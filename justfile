export RUST_LOG := "info"

run: shaderc
    cargo run

run-release: shaderc
    cargo run --release

shaderc:
    #!/bin/sh
    for shader in ./res/shaders/*.slang; do
        printf "compiling %s... " $shader
        slangc $shader -target wgsl -o $shader.wgsl
        printf "done\n"
    done
