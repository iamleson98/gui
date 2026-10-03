// Question #513: TCP Congestion Control (CUBIC)
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: CUBIC, cubic, window, congestion
// Description: Explain the CUBIC cubic window growth function used as default Linux congestion control.
package networking

import (
        "net"
        "sync"
        "time"
)

// TCP Congestion Control (CUBIC)
// Implements a networking concept for question #513.
type Q513_TcpCongestionControlCubic struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ513_TcpCongestionControlCubic creates a new network handler.
func NewQ513_TcpCongestionControlCubic(timeout time.Duration) *Q513_TcpCongestionControlCubic {
        return &Q513_TcpCongestionControlCubic{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q513_TcpCongestionControlCubic) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q513_TcpCongestionControlCubic) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q513_TcpCongestionControlCubic) Send(id string, data []byte) error {
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
