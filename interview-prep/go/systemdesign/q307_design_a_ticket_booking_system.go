// Question #307: Design a Ticket Booking System
// Category: System Design | Difficulty: Hard
// Concepts: booking, seat hold, idempotency, overbooking
// Description: Design a seat-booking service with hold locks, idempotency, and overbooking prevention.
package systemdesign

import "sync"

// Design a Ticket Booking System
// Implements a system design component for question #307.
type Q307_DesignATicketBookingSystem struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ307_DesignATicketBookingSystem creates a new system component.
func NewQ307_DesignATicketBookingSystem() *Q307_DesignATicketBookingSystem {
        return &Q307_DesignATicketBookingSystem{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q307_DesignATicketBookingSystem) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q307_DesignATicketBookingSystem) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q307_DesignATicketBookingSystem) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
