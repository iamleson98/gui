// Question #527: HTTP/1.1 Keep-Alive and Pipelining
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: HTTP/1.1, keep-alive, pipelining, HoL
// Description: Use keep-alive connections and reason about pipelining's HOL limitations.
package networking

import (
        "net"
        "sync"
        "time"
)

// HTTP/1.1 Keep-Alive and Pipelining
// Implements a networking concept for question #527.
type Q527_Http11KeepAliveAndPipelining struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ527_Http11KeepAliveAndPipelining creates a new network handler.
func NewQ527_Http11KeepAliveAndPipelining(timeout time.Duration) *Q527_Http11KeepAliveAndPipelining {
        return &Q527_Http11KeepAliveAndPipelining{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q527_Http11KeepAliveAndPipelining) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q527_Http11KeepAliveAndPipelining) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q527_Http11KeepAliveAndPipelining) Send(id string, data []byte) error {
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
