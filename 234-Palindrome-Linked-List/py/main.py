class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

def is_palindrome(head: ListNode) -> bool:
    head_list = []

    while head:
        head_list.append(head.val)
        head = head.next

    first = 0
    length = len(head_list)
    last = length - 1

    i = 0
    loop_end = length // 2
    while i < loop_end:
        if head_list[first] != head_list[last]:
            return False
        first += 1
        last -= 1
        i += 1

    return True
