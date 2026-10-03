// Question #548: ICMP and Ping/Traceroute
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: ICMP, ping, traceroute, TTL
// Description: Use ICMP echo and time-exceeded messages to implement ping and traceroute.
package networking

import (
        "net"
        "sync"
        "time"
)

// ICMP and Ping/Traceroute
// Implements a networking concept for question #548.
type IcmpAndPingTraceroute struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewIcmpAndPingTraceroute creates a new network handler.
func NewIcmpAndPingTraceroute(timeout time.Duration) *IcmpAndPingTraceroute {
        return &IcmpAndPingTraceroute{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *IcmpAndPingTraceroute) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *IcmpAndPingTraceroute) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *IcmpAndPingTraceroute) Send(id string, data []byte) error {
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
