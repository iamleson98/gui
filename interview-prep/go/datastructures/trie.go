// Compressed Trie (Patricia Trie).
package datastructures

type trieNode struct {
        children map[byte]*trieNode
        isEnd    bool
}

type Trie struct {
        root *trieNode
}

func NewTrie() *Trie {
        return &Trie{root: &trieNode{children: make(map[byte]*trieNode)}}
}

func (t *Trie) Insert(word string) {
        node := t.root
        for i := 0; i < len(word); i++ {
                c := word[i]
                child, ok := node.children[c]
                if !ok {
                        child = &trieNode{children: make(map[byte]*trieNode)}
                        node.children[c] = child
                }
                node = child
        }
        node.isEnd = true
}

func (t *Trie) Search(word string) bool {
        node := t.root
        for i := 0; i < len(word); i++ {
                c := word[i]
                child, ok := node.children[c]
                if !ok {
                        return false
                }
                node = child
        }
        return node.isEnd
}

func (t *Trie) StartsWith(prefix string) bool {
        node := t.root
        for i := 0; i < len(prefix); i++ {
                c := prefix[i]
                child, ok := node.children[c]
                if !ok {
                        return false
                }
                node = child
        }
        return true
}

func (t *Trie) Delete(word string) bool {
        found := false
        t.delete(t.root, word, 0, &found)
        return found
}

func (t *Trie) delete(node *trieNode, word string, depth int, found *bool) bool {
        if depth == len(word) {
                if !node.isEnd {
                        return false
                }
                *found = true
                node.isEnd = false
                return len(node.children) == 0
        }
        c := word[depth]
        child, ok := node.children[c]
        if !ok {
                return false
        }
        shouldDeleteChild := t.delete(child, word, depth+1, found)
        if shouldDeleteChild {
                delete(node.children, c)
                return len(node.children) == 0 && !node.isEnd
        }
        return false
}
