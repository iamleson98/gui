// Question #545: Thrift and Avro
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: Thrift, Avro, serialization, RPC
// Description: Contrast Thrift and Avro serialization and RPC frameworks.
package networking

import (
        "net"
        "sync"
        "time"
)

// Thrift and Avro
// Implements a networking concept for question #545.
type Q545_ThriftAndAvro struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ545_ThriftAndAvro creates a new network handler.
func NewQ545_ThriftAndAvro(timeout time.Duration) *Q545_ThriftAndAvro {
        return &Q545_ThriftAndAvro{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q545_ThriftAndAvro) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q545_ThriftAndAvro) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q545_ThriftAndAvro) Send(id string, data []byte) error {
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
