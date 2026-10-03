// Question #543: Server-Sent Events (SSE)
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: SSE, EventSource, streaming, reconnect
// Description: Stream server-to-client events over a long-lived HTTP connection with EventSource.
package networking

import (
        "net"
        "sync"
        "time"
)

// Server-Sent Events (SSE)
// Implements a networking concept for question #543.
type Q543_ServerSentEventsSse struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ543_ServerSentEventsSse creates a new network handler.
func NewQ543_ServerSentEventsSse(timeout time.Duration) *Q543_ServerSentEventsSse {
        return &Q543_ServerSentEventsSse{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q543_ServerSentEventsSse) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q543_ServerSentEventsSse) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q543_ServerSentEventsSse) Send(id string, data []byte) error {
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
