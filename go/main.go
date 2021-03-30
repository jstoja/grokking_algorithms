package main

func person_is_seller(name string) bool {
	return name[len(name)-1] == 'm'
}

func search(graph map[string][]string, name string) bool {
	search_queue := []string{}
	for _, friend := range graph[name] {
		search_queue = append(search_queue, friend)
	}

	for len(search_queue) > 0 {
		person := search_queue[0]
		search_queue = search_queue[1:]
		if person_is_seller(person) {
			println("Found a seller", person)
			return true
		} else {
			for _, friend := range graph[person] {
				search_queue = append(search_queue, friend)
			}
		}
	}
	return false
}
