package concurrency

import (
	"sync"
	"testing"
	"time"
)

func TestCondVarSignal(t *testing.T) {
	c := NewCondVar()
	ready := make(chan struct{})
	done := make(chan bool, 1)
	flag := false
	go func() {
		close(ready)
		c.Wait(func() bool { return flag })
		done <- true
	}()
	<-ready
	time.Sleep(10 * time.Millisecond)
	c.Lock()
	flag = true
	c.Unlock()
	c.Signal()
	select {
	case <-done:
	case <-time.After(time.Second):
		t.Fatal("waiter did not wake up")
	}
}

func TestCondVarBroadcast(t *testing.T) {
	c := NewCondVar()
	var wg sync.WaitGroup
	flag := false
	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			c.Wait(func() bool { return flag })
		}()
	}
	time.Sleep(20 * time.Millisecond)
	c.Lock()
	flag = true
	c.Unlock()
	c.Broadcast()
	wg.Wait()
}
