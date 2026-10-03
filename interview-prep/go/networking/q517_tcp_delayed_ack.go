// Question #517: TCP Delayed ACK
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: delayed ACK, Nagle, latency, writes
// Description: Explain delayed ACK and the latency it can introduce with small writes.
package networking

import (
        "net"
        "sync"
        "time"
)

// TCP Delayed ACK
// Implements a networking concept for question #517.
type Q517_TcpDelayedAck struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ517_TcpDelayedAck creates a new network handler.
func NewQ517_TcpDelayedAck(timeout time.Duration) *Q517_TcpDelayedAck {
        return &Q517_TcpDelayedAck{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q517_TcpDelayedAck) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q517_TcpDelayedAck) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q517_TcpDelayedAck) Send(id string, data []byte) error {
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
