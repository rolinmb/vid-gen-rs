package main

import (
  "fmt"
  "log"
  "math"
  "go/parser"
  "go/token"
  "go/ast"
  "reflect"
  "strconv"
  "strings"
  "os"
)

func evaluateASTNode(node interface{}, vars map[string]int) (float64, error) {
    switch n := node.(type) {
    case *ast.BasicLit:
        if n.Kind == token.INT {
            val, err := strconv.Atoi(n.Value)
            if err != nil {
                return 0, err
            }
            return float64(val), nil
        } else if n.Kind == token.FLOAT {
            val, err := strconv.ParseFloat(n.Value, 64)
            if err != nil {
                return 0, err
            }
            return val, nil
        }
    case *ast.Ident:
        varName := n.Name
        if val, ok := vars[varName]; ok {
            return float64(val), nil
        }
        return 0, fmt.Errorf("Undefined variable: %s", varName)
    case *ast.CallExpr:
        funcName := n.Fun.(*ast.Ident).Name
        args := n.Args
        if funcName == "sin" && len(args) == 1 {
            argVal, err := evaluateASTNode(args[0], vars)
            if err != nil {
                return 0, err
            }
            return math.Sin(argVal), nil
        } else if funcName == "cos" && len(args) == 1 {
            argVal, err := evaluateASTNode(args[0], vars)
            if err != nil {
                return 0, err
            }
            return math.Cos(argVal), nil
        } else if funcName == "tan" && len(args) == 1 {
            argVal, err := evaluateASTNode(args[0], vars)
            if err != nil {
                return 0, err
            }
            return math.Tan(argVal), nil
        } else if funcName == "exp" && len(args) == 1 {
            argVal, err := evaluateASTNode(args[0], vars)
            if err != nil {
                return 0, err
            }
            return math.Exp(argVal), nil
        } else if funcName == "sqrt" && len(args) == 1 {
            argVal1, err := evaluateASTNode(args[0], vars)
            if err != nil {
                return 0, err
            }
            return math.Sqrt(argVal1), nil
        } else if funcName == "abs" && len(args) == 1 {
            argVal1, err := evaluateASTNode(args[0], vars)
            if err != nil {
                return 0, err
            }
            return math.Abs(argVal1), nil
        } else if funcName == "pow" && len(args) == 2 {
            argVal1, err := evaluateASTNode(args[0], vars)
            if err != nil {
                return 0, err
            }
            argVal2, err := evaluateASTNode(args[1], vars)
            if err != nil {
                return 0, err
            }
            return math.Pow(argVal1, argVal2), nil
        }
    case *ast.BinaryExpr:
        left, err := evaluateASTNode(n.X, vars)
        if err != nil {
            return 0, err
        }
        right, err := evaluateASTNode(n.Y, vars)
        if err != nil {
        return 0, err
        }
        switch n.Op {
        case token.ADD:
            return left + right, nil
        case token.SUB:
            return left - right, nil
        case token.MUL:
            return left * right, nil
        case token.QUO:
            return left / right, nil
        }
    case *ast.ParenExpr:
        return evaluateASTNode(n.X, vars)
    case *ast.UnaryExpr:
        operand, err := evaluateASTNode(n.X, vars)
        if err != nil {
        return 0, err
        }
        switch n.Op {
        case token.ADD:
            return operand, nil
        case token.SUB:
            return -operand, nil
        }
    }
    return 0, fmt.Errorf("Unsupported expression: %s", reflect.TypeOf(node))
}

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
