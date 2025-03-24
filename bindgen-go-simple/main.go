// go_winrm.go
package main

import "C"
import "fmt"

//export RunCommand
func RunCommand(cmd *C.char) *C.char {
	result := fmt.Sprintf("Executed: %s", C.GoString(cmd))
	return C.CString(result)
}

func main() {} // 必须要有 main
