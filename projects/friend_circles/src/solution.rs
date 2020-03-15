pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
    let mut m = m;
    let mut circles = 0;
    let mut stack = Vec::new();

    for i in 0..m.len() {
        // Visited
        if m[i][i] == 0 {
            continue;
        }

        // Mark visited
        m[i][i] = 0;
        stack.push(i);

        while let Some(next) = stack.pop() {
            for j in (i + 1)..(m.len()) {
                // Adjacent and not visited
                if m[next][j] == 1 && m[j][j] == 1 {
                    // Mark visited
                    m[j][j] = 0;
                    stack.push(j);
                }
            }
        }

        circles += 1;
    }

    return circles;
}
