export RUST_LOG := "info"

run:
    cargo run

build: shaderc
    cargo build

shaderc:
    #!/bin/sh
    for shader in ./res/shaders/*.slang; do
        printf "compiling %s... " $shader
        slangc $shader -target wgsl -o $shader.wgsl
        printf "done\n"
    done
