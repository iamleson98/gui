// Question #525: HTTP/2 HPACK Header Compression
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: HPACK, header compression, Huffman, dynamic table
// Description: Compress HTTP/2 headers with HPACK static and dynamic tables plus Huffman coding.
package networking

import (
        "net"
        "sync"
        "time"
)

// HTTP/2 HPACK Header Compression
// Implements a networking concept for question #525.
type Q525_Http2HpackHeaderCompression struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewQ525_Http2HpackHeaderCompression creates a new network handler.
func NewQ525_Http2HpackHeaderCompression(timeout time.Duration) *Q525_Http2HpackHeaderCompression {
        return &Q525_Http2HpackHeaderCompression{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Q525_Http2HpackHeaderCompression) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Q525_Http2HpackHeaderCompression) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Q525_Http2HpackHeaderCompression) Send(id string, data []byte) error {
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
