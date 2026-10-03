// Question #535: DNS over HTTPS/TLS
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: DoH, DoT, encryption, privacy
// Description: Encrypt DNS queries with DoH/DoT and reason about privacy and policy implications.
package networking

import (
        "net"
        "sync"
        "time"
)

// DNS over HTTPS/TLS
// Implements a networking concept for question #535.
type Q535_DnsOverHttpsTls struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ535_DnsOverHttpsTls creates a new network handler.
func NewQ535_DnsOverHttpsTls(timeout time.Duration) *Q535_DnsOverHttpsTls {
        return &Q535_DnsOverHttpsTls{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q535_DnsOverHttpsTls) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q535_DnsOverHttpsTls) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q535_DnsOverHttpsTls) Send(id string, data []byte) error {
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
