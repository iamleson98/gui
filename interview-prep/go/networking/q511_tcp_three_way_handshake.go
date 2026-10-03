// Question #511: TCP Three-Way Handshake
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: TCP, handshake, SYN, state machine
// Description: Explain SYN, SYN-ACK, ACK and the resulting connection state machine.
package networking

import (
        "net"
        "sync"
        "time"
)

// TCP Three-Way Handshake
// Implements a networking concept for question #511.
type TcpThreeWayHandshake struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewTcpThreeWayHandshake creates a new network handler.
func NewTcpThreeWayHandshake(timeout time.Duration) *TcpThreeWayHandshake {
        return &TcpThreeWayHandshake{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *TcpThreeWayHandshake) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *TcpThreeWayHandshake) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *TcpThreeWayHandshake) Send(id string, data []byte) error {
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
