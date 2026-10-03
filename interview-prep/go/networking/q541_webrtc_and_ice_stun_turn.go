// Question #541: WebRTC and ICE/STUN/TURN
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: WebRTC, ICE, STUN, TURN
// Description: Establish peer-to-peer media with ICE candidate gathering and TURN fallback.
package networking

import (
        "net"
        "sync"
        "time"
)

// WebRTC and ICE/STUN/TURN
// Implements a networking concept for question #541.
type WebrtcAndIceStunTurn struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewWebrtcAndIceStunTurn creates a new network handler.
func NewWebrtcAndIceStunTurn(timeout time.Duration) *WebrtcAndIceStunTurn {
        return &WebrtcAndIceStunTurn{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *WebrtcAndIceStunTurn) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *WebrtcAndIceStunTurn) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *WebrtcAndIceStunTurn) Send(id string, data []byte) error {
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
