// Question #549: IPv6 Addressing
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: IPv6, subnetting, SLAAC, addressing
// Description: Explain IPv6 address structure, subnetting, and stateless autoconfiguration.
package networking

import (
        "net"
        "sync"
        "time"
)

// IPv6 Addressing
// Implements a networking concept for question #549.
type Ipv6Addressing struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewIpv6Addressing creates a new network handler.
func NewIpv6Addressing(timeout time.Duration) *Ipv6Addressing {
        return &Ipv6Addressing{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Ipv6Addressing) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Ipv6Addressing) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Ipv6Addressing) Send(id string, data []byte) error {
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
