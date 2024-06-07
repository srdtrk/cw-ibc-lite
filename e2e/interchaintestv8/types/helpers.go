package types

import (
	"bytes"
	"compress/gzip"
	"io"
	"os"
)

// CompressFile compresses the file in the given path using gzip
func CompressFile(path string) ([]byte, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}

	content, err := io.ReadAll(file)
	if err != nil {
		return nil, err
	}

	// compress the wasm file since it is too large to submit as a proposal
	var b bytes.Buffer
	gz := gzip.NewWriter(&b)
	_, err = gz.Write(content)
	if err != nil {
		return nil, err
	}

	gz.Close()

	return b.Bytes(), nil
}
