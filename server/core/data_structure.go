package core

// Set As Go doesn't have a built-in set data structure, this is a workaround in order to have unique data.
type Set map[string]struct{}

func (s Set) Add(data string) {
	s[data] = struct{}{}
}

func (s Set) Remove(data string) {
	delete(s, data)
}

func (s Set) Exists(data string) bool {
	_, ok := s[data]
	return ok
}

func (s Set) ToSlice() []string {
	var converted []string
	for value, _ := range s {
		converted = append(converted, value)
	}

	return converted
}
