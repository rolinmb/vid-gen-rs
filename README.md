Generating .mp4 videos using rust image and ffmpeg, similar to my vid-gen-go and vid-gen repos

main.rs:
- Main generation driver to build .png image frames and call ffmpeg to glue them into a .mp4 file
    Example: (from project root) go build -C src -o main.exe && cargo build && cargo run

main.go:
- Used to create executables that can be used for .png image pixel color calculations by x,y coordinates
- To use the golang output binary, replace "(" w/ [" and ")" w/ "]" and also wrap expression with an extra set of "[]" as such (3 total arguments):
    -> (from src) go build -o main.exe && ./main.exe [[[pow[y,2+x]]/[1+x]]*sin[y]] 2 10 
    -> (output) -1813.40370

parser.go:
- evaluateASTNode uses go standard libraries to take the result of parsing a string with go/parser (interface{} type object) and two variables x,y ; then outputs the result

TODO:
- decompose .mp4 video into frames and then process with FX and rebuild like in vid-gen-go
