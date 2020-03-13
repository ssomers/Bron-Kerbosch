package Assert

import (
	"testing"
)

func TestAssert(t *testing.T) {
	IsTrue(true)
	IsFalse(false)
	AreEqual(1, 1)
	AreNotEqual(0, 1)
	IsNotNull(0)
	IsNull(nil)
}
