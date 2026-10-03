// Question #544: gRPC and Protocol Buffers
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: gRPC, protobuf, HTTP/2, streaming
// Description: Design gRPC services with protobuf IDL, streaming, and HTTP/2 transport.
package networking

import (
        "net"
        "sync"
        "time"
)

// gRPC and Protocol Buffers
// Implements a networking concept for question #544.
type Q544_GrpcAndProtocolBuffers struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ544_GrpcAndProtocolBuffers creates a new network handler.
func NewQ544_GrpcAndProtocolBuffers(timeout time.Duration) *Q544_GrpcAndProtocolBuffers {
        return &Q544_GrpcAndProtocolBuffers{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q544_GrpcAndProtocolBuffers) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q544_GrpcAndProtocolBuffers) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q544_GrpcAndProtocolBuffers) Send(id string, data []byte) error {
        n.mu.Lock()
        conn, ok := n.connections[id]
        n.mu.Unlock()
        if !ok {
                return nil
        }
        conn.SetWriteDeadline(time.Now().Add(n.timeout))
        _, err := conn.Write(data)
        return err
}
