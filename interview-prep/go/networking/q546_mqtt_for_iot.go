// Question #546: MQTT for IoT
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: MQTT, QoS, topics, retained
// Description: Use MQTT topics, QoS levels, and retained messages for constrained IoT devices.
package networking

import (
        "net"
        "sync"
        "time"
)

// MQTT for IoT
// Implements a networking concept for question #546.
type Q546_MqttForIot struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ546_MqttForIot creates a new network handler.
func NewQ546_MqttForIot(timeout time.Duration) *Q546_MqttForIot {
        return &Q546_MqttForIot{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q546_MqttForIot) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q546_MqttForIot) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q546_MqttForIot) Send(id string, data []byte) error {
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
