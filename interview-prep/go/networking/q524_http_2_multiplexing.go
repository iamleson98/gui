// Question #524: HTTP/2 Multiplexing
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: HTTP/2, multiplexing, streams, framing
// Description: Multiplex many streams over a single HTTP/2 connection to avoid connection sprawl.
package networking

import (
        "net"
        "sync"
        "time"
)

// HTTP/2 Multiplexing
// Implements a networking concept for question #524.
type Http2Multiplexing struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewHttp2Multiplexing creates a new network handler.
func NewHttp2Multiplexing(timeout time.Duration) *Http2Multiplexing {
        return &Http2Multiplexing{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Http2Multiplexing) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Http2Multiplexing) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Http2Multiplexing) Send(id string, data []byte) error {
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
