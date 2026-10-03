// Question #528: HTTP Status Codes and Semantics
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: HTTP status, safe, idempotent, cacheable
// Description: Choose correct HTTP status codes reflecting safe, idempotent, and cacheable semantics.
package networking

import (
        "net"
        "sync"
        "time"
)

// HTTP Status Codes and Semantics
// Implements a networking concept for question #528.
type HttpStatusCodesAndSemantics struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewHttpStatusCodesAndSemantics creates a new network handler.
func NewHttpStatusCodesAndSemantics(timeout time.Duration) *HttpStatusCodesAndSemantics {
        return &HttpStatusCodesAndSemantics{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *HttpStatusCodesAndSemantics) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *HttpStatusCodesAndSemantics) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *HttpStatusCodesAndSemantics) Send(id string, data []byte) error {
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
