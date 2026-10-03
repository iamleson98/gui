// Question #514: Slow Start and Congestion Avoidance
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: slow start, congestion avoidance, ssthresh, cwnd
// Description: Reason about slow-start, congestion-avoidance, and the ssthresh transition.
package networking

import (
        "net"
        "sync"
        "time"
)

// Slow Start and Congestion Avoidance
// Implements a networking concept for question #514.
type SlowStartAndCongestionAvoidance struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewSlowStartAndCongestionAvoidance creates a new network handler.
func NewSlowStartAndCongestionAvoidance(timeout time.Duration) *SlowStartAndCongestionAvoidance {
        return &SlowStartAndCongestionAvoidance{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *SlowStartAndCongestionAvoidance) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *SlowStartAndCongestionAvoidance) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *SlowStartAndCongestionAvoidance) Send(id string, data []byte) error {
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
