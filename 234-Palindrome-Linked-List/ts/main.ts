class ListNode {
    val: number;
    next: ListNode | null;
    
    constructor(val: number = 0, next: ListNode | null = null) {
        this.val = val;
        this.next = next;
    }
}

function isPalindrome(head: ListNode | null): boolean {
    let head_vec: number[] = [];
    
    while (head !== null) {
        head_vec.push(head.val);
        head = head.next;
    }
    
    let first = 0;
    let length = head_vec.length;
    let last = length - 1;
    
    let i = 0;
    let loop_end = Math.floor(length / 2);
    while (i < loop_end) {
        if (head_vec[first] !== head_vec[last]) {
            return false;
        }
        first++;
        last--;
        i++;
    }
    
    return true;
}
