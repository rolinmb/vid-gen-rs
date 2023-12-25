package parse

import (
  "fmt"
  "math"
  "go/token"
  "go/ast"
  "reflect"
  "strconv"
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