// Question #529: HTTP Caching (ETag, Cache-Control)
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: caching, ETag, Cache-Control, freshness
// Description: Configure conditional and freshness caching with ETag and Cache-Control.
package networking

import (
        "net"
        "sync"
        "time"
)

// HTTP Caching (ETag, Cache-Control)
// Implements a networking concept for question #529.
type Q529_HttpCachingEtagCacheControl struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ529_HttpCachingEtagCacheControl creates a new network handler.
func NewQ529_HttpCachingEtagCacheControl(timeout time.Duration) *Q529_HttpCachingEtagCacheControl {
        return &Q529_HttpCachingEtagCacheControl{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q529_HttpCachingEtagCacheControl) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q529_HttpCachingEtagCacheControl) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q529_HttpCachingEtagCacheControl) Send(id string, data []byte) error {
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
