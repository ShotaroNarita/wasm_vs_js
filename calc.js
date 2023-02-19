const matmul = (m1, m2) => {
    let ans = {}
    if (m1.cols != m2.rows) {
        return ans
    }

    ans.rows = m1.rows
    ans.cols = m2.cols

    ans.elements = new Array(m1.rows)
    for (let i = 0; i < m1.rows; i++) {
        ans.elements[i] = new Array(m2.cols)
        for (let j = 0; j < m2.cols; j++) {
            ans.elements[i][j] = 0
        }
    }

    for (let i = 0; i < m1.rows; i++) {
        for (let j = 0; j < m2.cols; j++) {
            for (let k = 0; k < m1.cols; k++) {
                ans.elements[i][j] += m1.elements[i][k] * m2.elements[k][j]
                ans.elements[i][j] %= 65536
            }
        }
    }

    return ans
}

const compute_js = (matrixstr, count) => {
    return new Promise((resolve, reject) => {
        let matrix = {}

        try {
            matrix = JSON.parse(matrixstr)
        } catch (e) {
            reject('failed to parse')
        }

        let m = Object.assign(matrix)

        for (let i = 0; i < count; i++) {
            m = matmul(m, matrix)
        }

        resolve(JSON.stringify(m))
    })
}
