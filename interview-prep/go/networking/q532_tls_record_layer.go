// Question #532: TLS Record Layer
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: TLS record, framing, sequence, AEAD
// Description: Explain TLS record framing, sequence numbers, and AEAD protection.
package networking

import (
        "net"
        "sync"
        "time"
)

// TLS Record Layer
// Implements a networking concept for question #532.
type TlsRecordLayer struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewTlsRecordLayer creates a new network handler.
func NewTlsRecordLayer(timeout time.Duration) *TlsRecordLayer {
        return &TlsRecordLayer{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *TlsRecordLayer) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *TlsRecordLayer) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *TlsRecordLayer) Send(id string, data []byte) error {
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
