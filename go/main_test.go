package main

import "testing"

func TestSearch(t *testing.T) {
	graph := map[string][]string{
		"you":    {"claire", "bob", "alice"},
		"bob":    {"anuj", "peggy"},
		"alice":  {"peggy"},
		"claire": {"thom", "jonny"},
	}

	tests := []map[string]bool{
		{"you": true},
		{"lol": false},
	}
	for _, test := range tests {
		for k, v := range test {
			if search(graph, k) != v {
				t.Error("Should have been", v)
			}
		}
	}
}
