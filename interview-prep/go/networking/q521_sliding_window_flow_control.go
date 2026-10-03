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
type SlidingWindowFlowControl struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewSlidingWindowFlowControl creates a new network handler.
func NewSlidingWindowFlowControl(timeout time.Duration) *SlidingWindowFlowControl {
        return &SlidingWindowFlowControl{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *SlidingWindowFlowControl) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *SlidingWindowFlowControl) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *SlidingWindowFlowControl) Send(id string, data []byte) error {
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
