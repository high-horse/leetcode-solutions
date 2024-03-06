/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function hasCycle(head: ListNode | null): boolean {
    if (!head || !head.next ) {
        return false;
    }

    let seen = [];
    let current = head

    while (current != null) {
        if (seen.includes(current)) {
            return true;
        }
        seen.push(current)
        current = current.next
    }
    return false
    
};
