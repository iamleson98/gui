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
type ServerSentEventsSse struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewServerSentEventsSse creates a new network handler.
func NewServerSentEventsSse(timeout time.Duration) *ServerSentEventsSse {
        return &ServerSentEventsSse{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *ServerSentEventsSse) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *ServerSentEventsSse) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *ServerSentEventsSse) Send(id string, data []byte) error {
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
