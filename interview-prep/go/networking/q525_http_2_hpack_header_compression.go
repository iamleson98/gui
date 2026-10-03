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
type Http2HpackHeaderCompression struct {
        mu        sync.Mutex
        connections map[string]net.Conn
        timeout   time.Duration
}

// NewHttp2HpackHeaderCompression creates a new network handler.
func NewHttp2HpackHeaderCompression(timeout time.Duration) *Http2HpackHeaderCompression {
        return &Http2HpackHeaderCompression{
                connections: make(map[string]net.Conn),
                timeout:     timeout,
        }
}

// AddConnection registers a connection.
func (n *Http2HpackHeaderCompression) AddConnection(id string, conn net.Conn) {
        n.mu.Lock()
        n.connections[id] = conn
        n.mu.Unlock()
}

// RemoveConnection removes a connection.
func (n *Http2HpackHeaderCompression) RemoveConnection(id string) {
        n.mu.Lock()
        delete(n.connections, id)
        n.mu.Unlock()
}

// Send writes data to a connection.
func (n *Http2HpackHeaderCompression) Send(id string, data []byte) error {
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
