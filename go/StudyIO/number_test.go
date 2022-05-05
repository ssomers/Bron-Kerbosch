package StudyIO

import (
	"BronKerboschStudy/Assert"
	"testing"
)

func Test1(t *testing.T) {
	i, err := ParsePositiveInt("1")
	Assert.AreEqual(i, 1)
	Assert.AreEqual(err, nil)
}

func Test2k(t *testing.T) {
	i, err := ParsePositiveInt("2k")
	Assert.AreEqual(i, 2_000)
	Assert.AreEqual(err, nil)
}

func Test2K(t *testing.T) {
	_, err := ParsePositiveInt("2K")
	Assert.AreNotEqual(err, nil)
}

func Test2_k(t *testing.T) {
	_, err := ParsePositiveInt("2 k")
	Assert.AreNotEqual(err, nil)
}
