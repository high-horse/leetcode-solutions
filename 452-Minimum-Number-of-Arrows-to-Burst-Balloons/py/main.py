class Solution:
    def findMinArrowShots(self, points: List[List[int]]) -> int:
        if not points:
            return 0

        # Sort the points by the end of each interval
        points.sort(key=lambda x: x[1])

        min_val = float('-inf')
        end = min_val
        arrows = 0

        for i in points:
            if i[0] == min_val:
                if i[1] > end:
                    arrows += 1
                    end = i[1]
            elif i[0] > end:
                arrows += 1
                end = i[1]

        return arrows
