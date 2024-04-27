package main

import (
	"flag"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"runtime"

	"github.com/q3yi/fdir/internal/finder"
)

const usageMsg = `Find all parent folder that has given folder or file.

Usage: %[1]s [OPTIONS] [PATHS]...

Arguments:
  [PATHS]...  Search roots

Options:
`

var (
	recursive  bool
	followLink bool
	parallel   int
	has        string
	shell      string
	folders    []string
)

func init() {
	flag.BoolVar(&recursive, "recursive", false, "Search folder recursively.")
	flag.BoolVar(&recursive, "R", false, "Search folder recursively (shorthand).")
	flag.BoolVar(&followLink, "L", false, "Follow symbol links when searching.")

	flag.IntVar(&parallel, "P", runtime.NumCPU(), "Number of parallel worker to use when searching.")
	flag.StringVar(&has, "has", ".git", "Subfolder of file to search.")
	// flag.StringVar(&shell, "exec", "", "Run shell command with founded paths.")

	flag.Usage = usage
}

func usage() {
	fmt.Fprintf(os.Stdout, usageMsg, filepath.Base(os.Args[0]))
	flag.PrintDefaults()
}

func buildFilter() finder.Filter {
	return func(path string, d fs.DirEntry) bool {
		dest := filepath.Join(path, has)
		if _, err := os.Stat(dest); err == nil {
			return true
		}

		return false
	}
}

func printFoundedPath(path string, d fs.DirEntry) {
	fmt.Println(path)
}

func main() {
	flag.Parse()
	roots := flag.Args()
	if len(roots) == 0 {
		usage()
		os.Exit(1)
	}

	fd := finder.WithOptions(
		finder.Recursive(recursive),
		finder.FollowLink(followLink),
		finder.ParallelWorkerNum(parallel),
		finder.WithFilter(buildFilter()),
		finder.WithHandler(printFoundedPath),
		finder.SearchPaths(roots),
	)

	fd.Walk()
}
