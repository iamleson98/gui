// Question #522: Head-of-Line Blocking
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: HoL blocking, TCP, QUIC, stream multiplexing
// Description: Explain TCP head-of-line blocking and how QUIC addresses it.
package networking

import (
        "net"
        "sync"
        "time"
)

// Head-of-Line Blocking
// Implements a networking concept for question #522.
type Q522_HeadOfLineBlocking struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ522_HeadOfLineBlocking creates a new network handler.
func NewQ522_HeadOfLineBlocking(timeout time.Duration) *Q522_HeadOfLineBlocking {
        return &Q522_HeadOfLineBlocking{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q522_HeadOfLineBlocking) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q522_HeadOfLineBlocking) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q522_HeadOfLineBlocking) Send(id string, data []byte) error {
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
