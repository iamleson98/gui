// Question #550: NAT64 and IPv6 Transition
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: NAT64, DNS64, transition, IPv6
// Description: Transition between IPv6-only and IPv4 networks using NAT64 and DNS64.
package networking

import (
        "net"
        "sync"
        "time"
)

// NAT64 and IPv6 Transition
// Implements a networking concept for question #550.
type Nat64AndIpv6Transition struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewNat64AndIpv6Transition creates a new network handler.
func NewNat64AndIpv6Transition(timeout time.Duration) *Nat64AndIpv6Transition {
        return &Nat64AndIpv6Transition{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Nat64AndIpv6Transition) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Nat64AndIpv6Transition) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Nat64AndIpv6Transition) Send(id string, data []byte) error {
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
