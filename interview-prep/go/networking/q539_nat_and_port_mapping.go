// Question #539: NAT and Port Mapping
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: NAT, PAT, port mapping, end-to-end
// Description: Reason about NAT, port translation, and the end-to-end concerns they raise.
package networking

import (
        "net"
        "sync"
        "time"
)

// NAT and Port Mapping
// Implements a networking concept for question #539.
type NatAndPortMapping struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewNatAndPortMapping creates a new network handler.
func NewNatAndPortMapping(timeout time.Duration) *NatAndPortMapping {
        return &NatAndPortMapping{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *NatAndPortMapping) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *NatAndPortMapping) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *NatAndPortMapping) Send(id string, data []byte) error {
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
