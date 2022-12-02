all:
.PHONY: all

day0%/inputs/input.txt day%/inputs/input.txt: cookie.txt
	mkdir --parents $(@D)
	curl --cookie "cookie.txt" --output "$@" \
		"https://adventofcode.com/2022/day/$*/input"
