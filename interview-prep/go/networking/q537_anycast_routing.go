// Question #537: Anycast Routing
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: anycast, BGP, POP, latency
// Description: Route users to the nearest POP using BGP anycast for low-latency edge services.
package networking

import (
        "net"
        "sync"
        "time"
)

// Anycast Routing
// Implements a networking concept for question #537.
type AnycastRouting struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewAnycastRouting creates a new network handler.
func NewAnycastRouting(timeout time.Duration) *AnycastRouting {
        return &AnycastRouting{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *AnycastRouting) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *AnycastRouting) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *AnycastRouting) Send(id string, data []byte) error {
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
