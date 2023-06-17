use nekolib::math::LinearSieve;

fn main() {
    let ls = LinearSieve::new(10);

    // $\varphi(\prod_p p^{e_p}) = \prod_p (p-1)\cdot p^{e_p-1}$
    let phi = ls.dp(0, 1, |&x, p| x * p, |&x, p| x * (p - 1));
    assert_eq!(phi, [0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4]);
}
