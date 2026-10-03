// Question #521: Sliding Window Flow Control
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: sliding window, flow control, window, backpressure
// Description: Implement sliding-window flow control between sender and receiver.
package networking

import (
        "net"
        "sync"
        "time"
)

// Sliding Window Flow Control
// Implements a networking concept for question #521.
type Q521_SlidingWindowFlowControl struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ521_SlidingWindowFlowControl creates a new network handler.
func NewQ521_SlidingWindowFlowControl(timeout time.Duration) *Q521_SlidingWindowFlowControl {
        return &Q521_SlidingWindowFlowControl{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q521_SlidingWindowFlowControl) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q521_SlidingWindowFlowControl) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q521_SlidingWindowFlowControl) Send(id string, data []byte) error {
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
