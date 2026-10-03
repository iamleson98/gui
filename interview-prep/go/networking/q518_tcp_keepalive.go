// Question #518: TCP Keepalive
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: keepalive, dead peer, idle, probes
// Description: Configure keepalives to detect dead peers without application-level heartbeats.
package networking

import (
        "net"
        "sync"
        "time"
)

// TCP Keepalive
// Implements a networking concept for question #518.
type Q518_TcpKeepalive struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ518_TcpKeepalive creates a new network handler.
func NewQ518_TcpKeepalive(timeout time.Duration) *Q518_TcpKeepalive {
        return &Q518_TcpKeepalive{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q518_TcpKeepalive) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q518_TcpKeepalive) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q518_TcpKeepalive) Send(id string, data []byte) error {
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
