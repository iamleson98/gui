// Question #540: UDP Reliability at Application Layer
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: UDP, reliability, ordering, congestion
// Description: Build reliability, ordering, and congestion control on top of UDP.
package networking

import (
        "net"
        "sync"
        "time"
)

// UDP Reliability at Application Layer
// Implements a networking concept for question #540.
type UdpReliabilityAtApplicationLayer struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewUdpReliabilityAtApplicationLayer creates a new network handler.
func NewUdpReliabilityAtApplicationLayer(timeout time.Duration) *UdpReliabilityAtApplicationLayer {
        return &UdpReliabilityAtApplicationLayer{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *UdpReliabilityAtApplicationLayer) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *UdpReliabilityAtApplicationLayer) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *UdpReliabilityAtApplicationLayer) Send(id string, data []byte) error {
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
