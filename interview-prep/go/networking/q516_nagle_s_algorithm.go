// Question #516: Nagle's Algorithm
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: Nagle, delayed ACK, coalescing, latency
// Description: Explain Nagle's algorithm and its interaction with delayed ACK and latency.
package networking

import (
        "net"
        "sync"
        "time"
)

// Nagle's Algorithm
// Implements a networking concept for question #516.
type NagleSAlgorithm struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewNagleSAlgorithm creates a new network handler.
func NewNagleSAlgorithm(timeout time.Duration) *NagleSAlgorithm {
        return &NagleSAlgorithm{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *NagleSAlgorithm) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *NagleSAlgorithm) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *NagleSAlgorithm) Send(id string, data []byte) error {
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
