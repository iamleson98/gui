// Question #523: QUIC Protocol
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: QUIC, UDP, 0-RTT, streams
// Description: Explain QUIC's UDP-based streams, 0-RTT, and transport-level encryption.
package networking

import (
        "net"
        "sync"
        "time"
)

// QUIC Protocol
// Implements a networking concept for question #523.
type QuicProtocol struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQuicProtocol creates a new network handler.
func NewQuicProtocol(timeout time.Duration) *QuicProtocol {
        return &QuicProtocol{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *QuicProtocol) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *QuicProtocol) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *QuicProtocol) Send(id string, data []byte) error {
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
