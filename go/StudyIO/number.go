package StudyIO

import (
	"fmt"
)

func ParsePositiveInt(str string) (int, error) {
	var val int
	var suffix rune
	n, err := fmt.Sscanf(str, "%d%c", &val, &suffix)
	if n == 1 {
		return val, nil // ignore EOF error
	}
	if err == nil {
		if suffix == 'k' {
			return val * 1e3, nil
		}
		if suffix == 'M' {
			return val * 1e6, nil
		}
		err = fmt.Errorf("Unknown suffix \"%c\"", suffix)
	}
	return val, err
}
