package Assert

import "fmt"

func IsFalse(e bool) {
	if e {
		panic("assertion failed: IsFalse")
	}
}

func IsTrue(e bool) {
	if !e {
		panic("assertion failed: IsTrue")
	}
}

func AreEqual(a interface{}, b interface{}) {
	if a != b {
		panic(fmt.Sprintf("assertion failed: these values are different\n\tA.>%[1]T %[1]v<\n\tB.>%[2]T %[2]v<\n", a, b))
	}
}

func AreNotEqual(a interface{}, b interface{}) {
	if a == b {
		panic(fmt.Sprintf("assertion failed: these values are equal\n\tA.>%v<\n\tB.>%v<\n", a, b))
	}
}

func IsNotNull(p interface{}) {
	if p == nil {
		panic("assertion failed: IsNotNull")
	}
}

func IsNull(p interface{}) {
	if p != nil {
		panic("assertion failed: IsNull")
	}
}

func Fail() {
	panic("shouldn't get here")
}
