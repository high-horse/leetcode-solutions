function findMinArrowShots(points: number[][]): number {
    if (points.length === 0) {
        return 0;
    }

    // Sort the points by the end of each interval
    points.sort((a, b) => a[1] - b[1]);

    let minVal = Number.NEGATIVE_INFINITY;
    let end = minVal;
    let arrows = 0;

    for (let i of points) {
        if (i[0] === minVal) {
            if (i[1] > end) {
                arrows += 1;
                end = i[1];
            }
        } else if (i[0] > end) {
            arrows += 1;
            end = i[1];
        }
    }

    return arrows;
}
