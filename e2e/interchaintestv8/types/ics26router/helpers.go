package ics26router

import "encoding/base64"

func ToBinary(bz []byte) Binary {
	b64 := base64.StdEncoding.EncodeToString(bz)
	return Binary(b64)
}
