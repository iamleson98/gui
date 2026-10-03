// Question #531: Cookies vs Tokens over HTTP
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: cookies, tokens, CORS, CSRF
// Description: Contrast cookie-based and token-based authentication over HTTP and their tradeoffs.
package networking

import (
        "net"
        "sync"
        "time"
)

// Cookies vs Tokens over HTTP
// Implements a networking concept for question #531.
type Q531_CookiesVsTokensOverHttp struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ531_CookiesVsTokensOverHttp creates a new network handler.
func NewQ531_CookiesVsTokensOverHttp(timeout time.Duration) *Q531_CookiesVsTokensOverHttp {
        return &Q531_CookiesVsTokensOverHttp{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q531_CookiesVsTokensOverHttp) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q531_CookiesVsTokensOverHttp) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q531_CookiesVsTokensOverHttp) Send(id string, data []byte) error {
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
