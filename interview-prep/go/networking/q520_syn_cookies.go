// Question #520: SYN Cookies
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: SYN cookies, SYN flood, stateless, DoS
// Description: Defend against SYN floods with stateless SYN cookies encoded in the sequence number.
package networking

import (
        "net"
        "sync"
        "time"
)

// SYN Cookies
// Implements a networking concept for question #520.
type SynCookies struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewSynCookies creates a new network handler.
func NewSynCookies(timeout time.Duration) *SynCookies {
        return &SynCookies{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *SynCookies) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *SynCookies) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *SynCookies) Send(id string, data []byte) error {
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
