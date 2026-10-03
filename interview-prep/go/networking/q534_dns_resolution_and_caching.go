// Question #534: DNS Resolution and Caching
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: DNS, recursive, iterative, cache
// Description: Explain iterative and recursive DNS resolution and the role of resolver caches.
package networking

import (
        "net"
        "sync"
        "time"
)

// DNS Resolution and Caching
// Implements a networking concept for question #534.
type DnsResolutionAndCaching struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewDnsResolutionAndCaching creates a new network handler.
func NewDnsResolutionAndCaching(timeout time.Duration) *DnsResolutionAndCaching {
        return &DnsResolutionAndCaching{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *DnsResolutionAndCaching) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *DnsResolutionAndCaching) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *DnsResolutionAndCaching) Send(id string, data []byte) error {
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
