package main

import (
  "fmt"
  "log"
  "strconv"
  "strings"
  "go/parser"
  "os"
)

func main() {
    if len(os.Args) != 4 { 
        log.Fatalf("Expected 4 total arguments (program name + 3 params), but received %d arguments", len(os.Args))
    }
    var trimmedExpr string
    if len(os.Args[1]) >= 2 {
        trimmedExpr = os.Args[1][1 : len(os.Args[1])-1]
    }
    if len(trimmedExpr) == 0 || trimmedExpr == "" || trimmedExpr == "[]" {
        fmt.Printf("0")
        os.Exit(0)
    }
    trimmedExpr = strings.Replace(trimmedExpr, "[", "(", -1)
    trimmedExpr = strings.Replace(trimmedExpr, "]", ")", -1)
    expr, err := parser.ParseExpr(trimmedExpr)
    if err != nil {
        log.Fatal(err)
    }
    x, err := strconv.Atoi(os.Args[2])
    if err != nil {
        log.Fatal(err)
    }
    y, err := strconv.Atoi(os.Args[3])
    if err != nil {
        log.Fatal(err)
    }
    vars := map[string]int{"x": x, "y": y}
    result, err := evaluateASTNode(expr, vars)
    if err != nil {
        fmt.Printf("0")
        os.Exit(0)
    }
    fmt.Printf("%.5f", result)
    os.Exit(0)
}
