// Question #512: TCP Congestion Control (BBR)
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: BBR, congestion, bandwidth, RTT
// Description: Explain BBR's model-based congestion control versus loss-based schemes.
package networking

import (
        "net"
        "sync"
        "time"
)

// TCP Congestion Control (BBR)
// Implements a networking concept for question #512.
type Q512_TcpCongestionControlBbr struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ512_TcpCongestionControlBbr creates a new network handler.
func NewQ512_TcpCongestionControlBbr(timeout time.Duration) *Q512_TcpCongestionControlBbr {
        return &Q512_TcpCongestionControlBbr{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q512_TcpCongestionControlBbr) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q512_TcpCongestionControlBbr) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q512_TcpCongestionControlBbr) Send(id string, data []byte) error {
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
