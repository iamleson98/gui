// Question #542: WebSocket Protocol
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: WebSocket, upgrade, framing, ping/pong
// Description: Upgrade to WebSocket for bidirectional, low-latency messaging with framing and ping/pong.
package networking

import (
        "net"
        "sync"
        "time"
)

// WebSocket Protocol
// Implements a networking concept for question #542.
type Q542_WebsocketProtocol struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ542_WebsocketProtocol creates a new network handler.
func NewQ542_WebsocketProtocol(timeout time.Duration) *Q542_WebsocketProtocol {
        return &Q542_WebsocketProtocol{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q542_WebsocketProtocol) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q542_WebsocketProtocol) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q542_WebsocketProtocol) Send(id string, data []byte) error {
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
