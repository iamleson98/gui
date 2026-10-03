// Question #533: 0-RTT TLS
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: 0-RTT, resumption, replay, TLS 1.3
// Description: Achieve 0-RTT resumption and reason about its replay risk for non-idempotent requests.
package networking

import (
        "net"
        "sync"
        "time"
)

// 0-RTT TLS
// Implements a networking concept for question #533.
type Q533_0RttTls struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ533_0RttTls creates a new network handler.
func NewQ533_0RttTls(timeout time.Duration) *Q533_0RttTls {
        return &Q533_0RttTls{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q533_0RttTls) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q533_0RttTls) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q533_0RttTls) Send(id string, data []byte) error {
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
