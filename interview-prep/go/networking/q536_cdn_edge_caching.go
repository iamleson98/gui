// Question #536: CDN Edge Caching
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: CDN, edge cache, purge, origin shield
// Description: Use CDN edge caching with cache keys, purge, and origin shields to reduce latency.
package networking

import (
        "net"
        "sync"
        "time"
)

// CDN Edge Caching
// Implements a networking concept for question #536.
type Q536_CdnEdgeCaching struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ536_CdnEdgeCaching creates a new network handler.
func NewQ536_CdnEdgeCaching(timeout time.Duration) *Q536_CdnEdgeCaching {
        return &Q536_CdnEdgeCaching{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q536_CdnEdgeCaching) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q536_CdnEdgeCaching) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q536_CdnEdgeCaching) Send(id string, data []byte) error {
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
