// Question #303: Design a Web Crawler
// Category: System Design | Difficulty: Hard
// Concepts: crawler, frontier, politeness, dedup
// Description: Design a distributed web crawler with URL frontier scheduling, politeness, and dedup.
package systemdesign

import "sync"

// Design a Web Crawler
// Implements a system design component for question #303.
type DesignAWebCrawler struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignAWebCrawler creates a new system component.
func NewDesignAWebCrawler() *DesignAWebCrawler {
        return &DesignAWebCrawler{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignAWebCrawler) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignAWebCrawler) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignAWebCrawler) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
