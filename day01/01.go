package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
)

var args = os.Args[1:]

const (
	One   = "one"
	Two   = "two"
	Three = "three"
	Four  = "four"
	Five  = "five"
	Six   = "six"
	Seven = "seven"
	Eight = "eight"
	Nine  = "nine"
)

func main() {
	file, err := os.Open(args[0])
	if err != nil {
		log.Fatal(err)
	}
	calibration := readLine(file)
	fmt.Printf("calibration: %d\n", calibration)
}

func readLine(r io.Reader) int {
	buf := bufio.NewReader(r)
	sum := 0
	for {
		line, err := buf.ReadString('\n')
		if err != nil {
			break
		}
		i, j := searchFirstAndLast(line)
		fmt.Printf("num: %s%s -> %s", i, j, line)
		numStr := i + j
		num, _ := strconv.Atoi(numStr)
		sum += num
	}
	return sum
}

func searchFirstAndLast(s string) (string, string) {
	type digit struct {
		digit string
		pos   int
	}
	var (
		first digit
		last  digit
	)
	//fmt.Printf("s: %s\n", s)
	for i, j := 0, len(s)-1; i < len(s) || j > 0; i, j = i+1, j-1 {
		if first.digit == "" {
			si := string(s[i])
			_, err := strconv.Atoi(si)
			//fmt.Printf("si: %s, snum: %d\n", si, s)
			if err != nil {
			} else {
				first.digit = si
				first.pos = i

			}
		}
		if last.digit == "" {
			sj := string(s[j])
			_, err := strconv.Atoi(sj)
			//fmt.Printf("sj: %s, lnum: %d\n", sj, l)
			if err != nil {
			} else {
				last.digit = sj
				last.pos = j
			}
		}
	}
	if first.pos > last.pos {
		return first.digit, ""
	}
	return first.digit, last.digit
}

func parser(s string) (string, string) {
	var (
		init string
		end  string
	)
	for _, l := range s {
		_, 
	}
	return init, end
}
