// Question #530: Conditional Requests (If-None-Match)
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: conditional request, If-None-Match, If-Modified-Since, 304
// Description: Use If-None-Match and If-Modified-Since to validate cached responses.
package networking

import (
        "net"
        "sync"
        "time"
)

// Conditional Requests (If-None-Match)
// Implements a networking concept for question #530.
type ConditionalRequestsIfNoneMatch struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewConditionalRequestsIfNoneMatch creates a new network handler.
func NewConditionalRequestsIfNoneMatch(timeout time.Duration) *ConditionalRequestsIfNoneMatch {
        return &ConditionalRequestsIfNoneMatch{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *ConditionalRequestsIfNoneMatch) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *ConditionalRequestsIfNoneMatch) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *ConditionalRequestsIfNoneMatch) Send(id string, data []byte) error {
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
