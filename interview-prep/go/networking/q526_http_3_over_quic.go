// Question #526: HTTP/3 over QUIC
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: HTTP/3, QUIC, streams, frames
// Description: Map HTTP/3 semantics onto QUIC streams and frames.
package networking

import (
        "net"
        "sync"
        "time"
)

// HTTP/3 over QUIC
// Implements a networking concept for question #526.
type Http3OverQuic struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewHttp3OverQuic creates a new network handler.
func NewHttp3OverQuic(timeout time.Duration) *Http3OverQuic {
        return &Http3OverQuic{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Http3OverQuic) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Http3OverQuic) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Http3OverQuic) Send(id string, data []byte) error {
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
