// Question #547: AMQP Message Protocol
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: AMQP, exchanges, bindings, queues
// Description: Explain AMQP exchanges, queues, and bindings for routed messaging.
package networking

import (
        "net"
        "sync"
        "time"
)

// AMQP Message Protocol
// Implements a networking concept for question #547.
type AmqpMessageProtocol struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewAmqpMessageProtocol creates a new network handler.
func NewAmqpMessageProtocol(timeout time.Duration) *AmqpMessageProtocol {
        return &AmqpMessageProtocol{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *AmqpMessageProtocol) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *AmqpMessageProtocol) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *AmqpMessageProtocol) Send(id string, data []byte) error {
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
