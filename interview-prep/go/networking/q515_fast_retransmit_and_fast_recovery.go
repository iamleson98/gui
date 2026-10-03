// Question #515: Fast Retransmit and Fast Recovery
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: fast retransmit, fast recovery, dup ACK, loss
// Description: Recover from packet loss without timing out using duplicate ACKs.
package networking

import (
        "net"
        "sync"
        "time"
)

// Fast Retransmit and Fast Recovery
// Implements a networking concept for question #515.
type Q515_FastRetransmitAndFastRecovery struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ515_FastRetransmitAndFastRecovery creates a new network handler.
func NewQ515_FastRetransmitAndFastRecovery(timeout time.Duration) *Q515_FastRetransmitAndFastRecovery {
        return &Q515_FastRetransmitAndFastRecovery{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q515_FastRetransmitAndFastRecovery) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q515_FastRetransmitAndFastRecovery) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q515_FastRetransmitAndFastRecovery) Send(id string, data []byte) error {
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
