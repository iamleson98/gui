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
type Q532_TlsRecordLayer struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ532_TlsRecordLayer creates a new network handler.
func NewQ532_TlsRecordLayer(timeout time.Duration) *Q532_TlsRecordLayer {
        return &Q532_TlsRecordLayer{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q532_TlsRecordLayer) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q532_TlsRecordLayer) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q532_TlsRecordLayer) Send(id string, data []byte) error {
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
