package main

import (
  "os"
  //"os/exec"
  "fmt"
  "log"
  "strconv"
  "strings"
  //"io/ioutil"
  "go/parser"
)

/*const (
    RNAME = "src/red.go"
    GNAME = "src/green.go"
    BNAME = "src/blue.go"
)

func trimExprStr(exprStr string) string {
    result := ""
    if len(exprStr) >= 2 {
      result = exprStr[1 : len(exprStr) - 1]    
    }
    if len(result) == 0 || result == "" || result == "[]" {
        return "0"
    }
    return result 
}

func replaceBrackets(exprStr string) string {
    result := strings.Replace(exprStr, "[", "(", -1)
    return strings.Replace(result, "]", ")", -1)
}

func buildGoText(fnStr,fName string) string {
    return fmt.Sprintf(
    `package main
    import (
        "os"
        "fmt"
        "log"
        "strconv"
        "go/parser"
    )
    func main() {
        if len(os.Args) != 3 {
            log.Fatal("%s(): Expected 3 total arguments (program name, x, y) but received",len(os.Args))
        }
        expr, err := parser.ParseExpr("%s")
        if err != nil {
            log.Fatal("%s(): Error parsing %s with go/parser:",err)
        }
        x, err := strconv.Atoi(os.Args[1])
        if err != nil {
            log.Fatal("%s(): Error parsing os.Args[1] to int:",err)
        }
        y, err := strconv.Atoi(os.Args[2])
        if err != nil {
            log.Fatal("%s(): Error parsing os.Args[2] to int:",err)
        }
        vars := map[string]int{"x": x, "y": y}
        result, err := evaluateASTNode(expr, vars)
        if err != nil {
            fmt.Printf("0")
            os.Exit(0)
        }
        fmt.Printf(result)
        os.Exit(0)
    }
    `, fName, fnStr, fName, fnStr, fName, fName)
}

func main() {
    if len(os.Args) != 4 {
        log.Fatalf("Expected 4 total arguments (program name, 3 expression strings), but received %d arguments", len(os.Args))
    }
    trimmed1 := trimExprStr(os.Args[1])
    trimmed2 := trimExprStr(os.Args[2])
    trimmed3 := trimExprStr(os.Args[3])
    trimmed1 = replaceBrackets(trimmed1)
    trimmed2 = replaceBrackets(trimmed2)
    trimmed3 = replaceBrackets(trimmed3)
    goText1 := buildGoText(trimmed1, RNAME)
    goText2 := buildGoText(trimmed2, GNAME)
    goText3 := buildGoText(trimmed3, BNAME)
    err := ioutil.WriteFile(RNAME, []byte(goText1), 0644)
    if err != nil {
		log.Fatalf("main.go(): Error writing '%s': %v", RNAME, err)
	}
    err = ioutil.WriteFile(GNAME, []byte(goText2), 0644)
    if err != nil {
		log.Fatalf("main.go(): Error writing '%s': %v", GNAME, err)
	}
    err = ioutil.WriteFile(BNAME, []byte(goText3), 0644)
    if err != nil {
		log.Fatalf("main.go(): Error writing '%s': %v", BNAME, err)
	}
    cmd1 := exec.Command("go", "build", "-C", "src", "-o", "red.exe", "red.go")
    _, err = cmd1.CombinedOutput()
    if err != nil {
        log.Fatalf("main.go(): Error building 'src/red.go' into an executable: %v", err)
    }
    cmd2 := exec.Command("go", "build", "-C", "src", "-o", "green.exe", "green.go")
    _, err = cmd2.CombinedOutput()
    if err != nil {
        log.Fatalf("main.go(): Error building 'src/green.go' into an executable: %v", err)
    }
    cmd3 := exec.Command("go", "build", "-C", "src", "-o", "blue.exe", "blue.go")
    _, err = cmd3.CombinedOutput()
    if err != nil {
        log.Fatalf("main.go(): Error building 'src/blue.go' into an executable: %v", err)
    }
    os.Exit(0)
}*/

func main() {
    if len(os.Args) != 4 { 
        log.Fatalf("Expected 4 total arguments (program name, expression string, x, y), but received %d arguments", len(os.Args))
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
