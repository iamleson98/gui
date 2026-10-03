// Question #538: BGP Basics
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: BGP, AS path, interdomain, routing
// Description: Explain BGP path selection and how AS-path attributes drive interdomain routing.
package networking

import (
        "net"
        "sync"
        "time"
)

// BGP Basics
// Implements a networking concept for question #538.
type BgpBasics struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewBgpBasics creates a new network handler.
func NewBgpBasics(timeout time.Duration) *BgpBasics {
        return &BgpBasics{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *BgpBasics) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *BgpBasics) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *BgpBasics) Send(id string, data []byte) error {
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
