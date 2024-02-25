package main

import (
	"context"
	"crypto/tls"

	"github.com/quic-go/quic-go"
	"github.com/quic-go/quic-go/qlog"
	"go.uber.org/zap"
)

func main() {
	logger, _ := zap.NewProduction()
	ctx := context.Background()
	defer logger.Sync()
	log := logger.Sugar()

	tlsConfig := &tls.Config{
		InsecureSkipVerify: true,
	}
	quicConfig := &quic.Config{
		Tracer:          qlog.DefaultTracer,
		EnableDatagrams: true,
	}
	conn, err := quic.DialAddr(ctx, "127.0.0.1:4443", tlsConfig, quicConfig)
	if err != nil {
		panic(err)
	}
	log.Infof("State connection %v ", conn.ConnectionState())

	stream, err := conn.OpenStreamSync(ctx)
	if err != nil {
		panic(err)
	}

	_, err = stream.Write([]byte("h"))
	if err != nil {
		panic(err)
	}
}
