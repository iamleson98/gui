// Question #519: TIME_WAIT and 2MSL
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: TIME_WAIT, 2MSL, segments, reuse
// Description: Explain TIME_WAIT, the 2MSL duration, and its role in preventing old segments.
package networking

import (
        "net"
        "sync"
        "time"
)

// TIME_WAIT and 2MSL
// Implements a networking concept for question #519.
type Q519_TimeWaitAnd2Msl struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ519_TimeWaitAnd2Msl creates a new network handler.
func NewQ519_TimeWaitAnd2Msl(timeout time.Duration) *Q519_TimeWaitAnd2Msl {
        return &Q519_TimeWaitAnd2Msl{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q519_TimeWaitAnd2Msl) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q519_TimeWaitAnd2Msl) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q519_TimeWaitAnd2Msl) Send(id string, data []byte) error {
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
