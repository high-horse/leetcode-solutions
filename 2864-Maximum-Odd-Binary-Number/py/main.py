class Solution:
    def maximumOddBinaryNumber(self, s: str) -> str:
        sth_str = s
        len_s = len(sth_str)
        zeros = []
        ones = []

        for char in sth_str:
            if char == '1':
                ones.append('1')
            else:
                zeros.append('0')

        if len(ones) == 0:
            return '0' * len_s
        elif len(ones) == 1:
            return '0' * (len_s - 1) + '1'
        else:
            ones.pop()
            return ''.join(ones) + ''.join(zeros) + '1'
