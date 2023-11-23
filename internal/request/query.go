package request

import "strconv"

type Pagination struct {
	Page   int
	Size   int
	Offset int
}

func GetPagination(param map[string]string) Pagination {
	page, _ := strconv.Atoi(param["page"])
	if page < 1 {
		page = 1
	}

	size, _ := strconv.Atoi(param["size"])
	if size < 1 {
		size = 20
	}

	offset := (page - 1) * size

	return Pagination{page, size, offset}
}
