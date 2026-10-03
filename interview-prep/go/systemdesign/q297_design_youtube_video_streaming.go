// Question #297: Design YouTube / Video Streaming
// Category: System Design | Difficulty: Hard
// Concepts: video streaming, adaptive bitrate, transcoding, CDN
// Description: Design a video platform with adaptive bitrate, transcoding pipelines, and CDN.
package systemdesign

import "sync"

// Design YouTube / Video Streaming
// Implements a system design component for question #297.
type DesignYoutubeVideoStreaming struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignYoutubeVideoStreaming creates a new system component.
func NewDesignYoutubeVideoStreaming() *DesignYoutubeVideoStreaming {
        return &DesignYoutubeVideoStreaming{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignYoutubeVideoStreaming) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignYoutubeVideoStreaming) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignYoutubeVideoStreaming) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
