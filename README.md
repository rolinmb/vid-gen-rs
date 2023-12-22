Generating .mp4 videos using rust image and ffmpeg, similar to my vid-gen-go and vid-gen repos

main.rs:
    * Main generation driver
    Example: (from root) go build -C src -o main && cargo build && cargo run

main.go:
    * Used to create an executable that can parse an expression string into an executable function (via AST tokenizing) and then evaluates with parameters x & y and outputs result to stdout to collect back in main.rs
    * To use the golang output binary, replace "(" w/ [" and ")" w/ "]" and also wrap expression with an extra set of "[]" as such (3 total arguments):
      > go build -C src -o main && ./main [[[pow[y,2+x]]/[1+x]]*sin[y]] 2 10 
      > (output) -1813.40370

TODO: finish implementing custom string defined function parsing in rust by using output of calling main.exe (result of building main.go)
