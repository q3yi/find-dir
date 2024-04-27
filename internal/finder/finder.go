package finder

import (
	"io/fs"
	"path/filepath"
	"sync"

	"github.com/charlievieth/fastwalk"
)

type Option func(*Finder)
type Filter func(string, fs.DirEntry) bool
type Handler func(string, fs.DirEntry)

// Finder search folder with given options
type Finder struct {
	recursive  bool
	followLink bool
	parallel   int
	roots      []string
	filter     Filter
	handler    Handler
}

// Walk through all subfolder in given roots
func (f *Finder) Walk() error {

	walkFn := func(path string, d fs.DirEntry, err error) error {
		if err == filepath.SkipDir {
			return nil
		}

		if err != nil {
			// TODO log error
			return nil
		}

		if !d.IsDir() {
			return nil
		}

		if f.filter(path, d) {
			f.handler(path, d)

			if !f.recursive {
				return filepath.SkipDir
			}
		}

		return nil
	}

	walkOpts := fastwalk.Config{Follow: f.followLink, NumWorkers: f.parallel}

	wg := &sync.WaitGroup{}

	for _, root := range f.roots {
		wg.Add(1)
		go func(path string) {
			fastwalk.Walk(&walkOpts, path, walkFn)
			wg.Done()
		}(root)
	}

	wg.Wait()

	return nil
}

func WithOptions(opts ...Option) (f *Finder) {
	f = new(Finder)
	for _, opt := range opts {
		opt(f)
	}
	return f
}

// Recursive search all founded folder
func Recursive(flag bool) Option {
	return func(f *Finder) {
		f.recursive = flag
	}
}

// SearchPaths set search roots for finder
func SearchPaths(paths []string) Option {
	return func(f *Finder) {
		f.roots = paths
	}
}

// FollowLink whether follow symbol link when walk through folder
func FollowLink(flag bool) Option {
	return func(f *Finder) {
		f.followLink = flag
	}
}

// ParallelWorkerNum number of parallel worker to use
func ParallelWorkerNum(workerNum int) Option {
	return func(f *Finder) {
		f.parallel = workerNum
	}
}

// WithFilter add filter to finder
func WithFilter(filter Filter) Option {
	return func(f *Finder) {
		f.filter = filter
	}
}

// WithHandler add handler to handle founded path
func WithHandler(fn Handler) Option {
	return func(f *Finder) {
		f.handler = fn
	}
}
