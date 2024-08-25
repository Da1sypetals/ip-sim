extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}

pub fn w_grad_orth_1(a11: f64, a12: f64, a21: f64, a22: f64) -> f64 {
    unsafe {
        grad_orth_1(
            a11 as libc::c_double,
            a12 as libc::c_double,
            a21 as libc::c_double,
            a22 as libc::c_double,
        ) as f64
    }
}

#[no_mangle]
pub unsafe extern "C" fn grad_orth_1(
    mut a11: libc::c_double,
    mut a12: libc::c_double,
    mut a21: libc::c_double,
    mut a22: libc::c_double,
) -> libc::c_double {
    let mut grad_orth_1_result: libc::c_double = 0.;
    grad_orth_1_result = (pow(a11, 2 as libc::c_int as libc::c_double) * a12
        + a11 * a21 * a22
        + pow(a12, 3 as libc::c_int as libc::c_double)
        + a12 * pow(a22, 2 as libc::c_int as libc::c_double)
        - 1.0f64 / 2.0f64
            * a12
            * sqrt(
                (pow(a11, 2 as libc::c_int as libc::c_double)
                    - 2 as libc::c_int as libc::c_double * a11 * a22
                    + pow(a12, 2 as libc::c_int as libc::c_double)
                    + 2 as libc::c_int as libc::c_double * a12 * a21
                    + pow(a21, 2 as libc::c_int as libc::c_double)
                    + pow(a22, 2 as libc::c_int as libc::c_double))
                    * (pow(a11, 2 as libc::c_int as libc::c_double)
                        + 2 as libc::c_int as libc::c_double * a11 * a22
                        + pow(a12, 2 as libc::c_int as libc::c_double)
                        - 2 as libc::c_int as libc::c_double * a12 * a21
                        + pow(a21, 2 as libc::c_int as libc::c_double)
                        + pow(a22, 2 as libc::c_int as libc::c_double)),
            )
        - a12
        - 1.0f64 / 4.0f64
            * sqrt(
                (pow(a11, 2 as libc::c_int as libc::c_double)
                    - 2 as libc::c_int as libc::c_double * a11 * a22
                    + pow(a12, 2 as libc::c_int as libc::c_double)
                    + 2 as libc::c_int as libc::c_double * a12 * a21
                    + pow(a21, 2 as libc::c_int as libc::c_double)
                    + pow(a22, 2 as libc::c_int as libc::c_double))
                    * (pow(a11, 2 as libc::c_int as libc::c_double)
                        + 2 as libc::c_int as libc::c_double * a11 * a22
                        + pow(a12, 2 as libc::c_int as libc::c_double)
                        - 2 as libc::c_int as libc::c_double * a12 * a21
                        + pow(a21, 2 as libc::c_int as libc::c_double)
                        + pow(a22, 2 as libc::c_int as libc::c_double)),
            )
            * (1.0f64 / 2.0f64
                * (2 as libc::c_int as libc::c_double * a12
                    - 2 as libc::c_int as libc::c_double * a21)
                * (pow(a11, 2 as libc::c_int as libc::c_double)
                    - 2 as libc::c_int as libc::c_double * a11 * a22
                    + pow(a12, 2 as libc::c_int as libc::c_double)
                    + 2 as libc::c_int as libc::c_double * a12 * a21
                    + pow(a21, 2 as libc::c_int as libc::c_double)
                    + pow(a22, 2 as libc::c_int as libc::c_double))
                + 1.0f64 / 2.0f64
                    * (2 as libc::c_int as libc::c_double * a12
                        + 2 as libc::c_int as libc::c_double * a21)
                    * (pow(a11, 2 as libc::c_int as libc::c_double)
                        + 2 as libc::c_int as libc::c_double * a11 * a22
                        + pow(a12, 2 as libc::c_int as libc::c_double)
                        - 2 as libc::c_int as libc::c_double * a12 * a21
                        + pow(a21, 2 as libc::c_int as libc::c_double)
                        + pow(a22, 2 as libc::c_int as libc::c_double)))
            * (pow(a11, 2 as libc::c_int as libc::c_double)
                + pow(a12, 2 as libc::c_int as libc::c_double)
                + pow(a21, 2 as libc::c_int as libc::c_double)
                + pow(a22, 2 as libc::c_int as libc::c_double)
                - 2 as libc::c_int as libc::c_double)
            / ((pow(a11, 2 as libc::c_int as libc::c_double)
                - 2 as libc::c_int as libc::c_double * a11 * a22
                + pow(a12, 2 as libc::c_int as libc::c_double)
                + 2 as libc::c_int as libc::c_double * a12 * a21
                + pow(a21, 2 as libc::c_int as libc::c_double)
                + pow(a22, 2 as libc::c_int as libc::c_double))
                * (pow(a11, 2 as libc::c_int as libc::c_double)
                    + 2 as libc::c_int as libc::c_double * a11 * a22
                    + pow(a12, 2 as libc::c_int as libc::c_double)
                    - 2 as libc::c_int as libc::c_double * a12 * a21
                    + pow(a21, 2 as libc::c_int as libc::c_double)
                    + pow(a22, 2 as libc::c_int as libc::c_double))))
        * (if sqrt(
            1.0f64 / 2.0f64 * pow(a11, 4 as libc::c_int as libc::c_double)
                + pow(a11, 2 as libc::c_int as libc::c_double)
                    * pow(a12, 2 as libc::c_int as libc::c_double)
                + pow(a11, 2 as libc::c_int as libc::c_double)
                    * pow(a21, 2 as libc::c_int as libc::c_double)
                - pow(a11, 2 as libc::c_int as libc::c_double)
                + 2 as libc::c_int as libc::c_double * a11 * a12 * a21 * a22
                + 1.0f64 / 2.0f64 * pow(a12, 4 as libc::c_int as libc::c_double)
                + pow(a12, 2 as libc::c_int as libc::c_double)
                    * pow(a22, 2 as libc::c_int as libc::c_double)
                - pow(a12, 2 as libc::c_int as libc::c_double)
                + 1.0f64 / 2.0f64 * pow(a21, 4 as libc::c_int as libc::c_double)
                + pow(a21, 2 as libc::c_int as libc::c_double)
                    * pow(a22, 2 as libc::c_int as libc::c_double)
                - pow(a21, 2 as libc::c_int as libc::c_double)
                + 1.0f64 / 2.0f64 * pow(a22, 4 as libc::c_int as libc::c_double)
                - pow(a22, 2 as libc::c_int as libc::c_double)
                - 1.0f64 / 2.0f64
                    * sqrt(
                        (pow(a11, 2 as libc::c_int as libc::c_double)
                            - 2 as libc::c_int as libc::c_double * a11 * a22
                            + pow(a12, 2 as libc::c_int as libc::c_double)
                            + 2 as libc::c_int as libc::c_double * a12 * a21
                            + pow(a21, 2 as libc::c_int as libc::c_double)
                            + pow(a22, 2 as libc::c_int as libc::c_double))
                            * (pow(a11, 2 as libc::c_int as libc::c_double)
                                + 2 as libc::c_int as libc::c_double * a11 * a22
                                + pow(a12, 2 as libc::c_int as libc::c_double)
                                - 2 as libc::c_int as libc::c_double * a12 * a21
                                + pow(a21, 2 as libc::c_int as libc::c_double)
                                + pow(a22, 2 as libc::c_int as libc::c_double)),
                    )
                    * (pow(a11, 2 as libc::c_int as libc::c_double)
                        + pow(a12, 2 as libc::c_int as libc::c_double)
                        + pow(a21, 2 as libc::c_int as libc::c_double)
                        + pow(a22, 2 as libc::c_int as libc::c_double)
                        - 2 as libc::c_int as libc::c_double)
                + 1 as libc::c_int as libc::c_double,
        ) - sqrt(
            1.0f64 / 2.0f64 * pow(a11, 4 as libc::c_int as libc::c_double)
                + pow(a11, 2 as libc::c_int as libc::c_double)
                    * pow(a12, 2 as libc::c_int as libc::c_double)
                + pow(a11, 2 as libc::c_int as libc::c_double)
                    * pow(a21, 2 as libc::c_int as libc::c_double)
                - pow(a11, 2 as libc::c_int as libc::c_double)
                + 2 as libc::c_int as libc::c_double * a11 * a12 * a21 * a22
                + 1.0f64 / 2.0f64 * pow(a12, 4 as libc::c_int as libc::c_double)
                + pow(a12, 2 as libc::c_int as libc::c_double)
                    * pow(a22, 2 as libc::c_int as libc::c_double)
                - pow(a12, 2 as libc::c_int as libc::c_double)
                + 1.0f64 / 2.0f64 * pow(a21, 4 as libc::c_int as libc::c_double)
                + pow(a21, 2 as libc::c_int as libc::c_double)
                    * pow(a22, 2 as libc::c_int as libc::c_double)
                - pow(a21, 2 as libc::c_int as libc::c_double)
                + 1.0f64 / 2.0f64 * pow(a22, 4 as libc::c_int as libc::c_double)
                - pow(a22, 2 as libc::c_int as libc::c_double)
                + 1.0f64 / 2.0f64
                    * sqrt(
                        (pow(a11, 2 as libc::c_int as libc::c_double)
                            - 2 as libc::c_int as libc::c_double * a11 * a22
                            + pow(a12, 2 as libc::c_int as libc::c_double)
                            + 2 as libc::c_int as libc::c_double * a12 * a21
                            + pow(a21, 2 as libc::c_int as libc::c_double)
                            + pow(a22, 2 as libc::c_int as libc::c_double))
                            * (pow(a11, 2 as libc::c_int as libc::c_double)
                                + 2 as libc::c_int as libc::c_double * a11 * a22
                                + pow(a12, 2 as libc::c_int as libc::c_double)
                                - 2 as libc::c_int as libc::c_double * a12 * a21
                                + pow(a21, 2 as libc::c_int as libc::c_double)
                                + pow(a22, 2 as libc::c_int as libc::c_double)),
                    )
                    * (pow(a11, 2 as libc::c_int as libc::c_double)
                        + pow(a12, 2 as libc::c_int as libc::c_double)
                        + pow(a21, 2 as libc::c_int as libc::c_double)
                        + pow(a22, 2 as libc::c_int as libc::c_double)
                        - 2 as libc::c_int as libc::c_double)
                + 1 as libc::c_int as libc::c_double,
        ) < 0 as libc::c_int as libc::c_double
        {
            0 as libc::c_int as libc::c_double
        } else {
            (if sqrt(
                1.0f64 / 2.0f64 * pow(a11, 4 as libc::c_int as libc::c_double)
                    + pow(a11, 2 as libc::c_int as libc::c_double)
                        * pow(a12, 2 as libc::c_int as libc::c_double)
                    + pow(a11, 2 as libc::c_int as libc::c_double)
                        * pow(a21, 2 as libc::c_int as libc::c_double)
                    - pow(a11, 2 as libc::c_int as libc::c_double)
                    + 2 as libc::c_int as libc::c_double * a11 * a12 * a21 * a22
                    + 1.0f64 / 2.0f64 * pow(a12, 4 as libc::c_int as libc::c_double)
                    + pow(a12, 2 as libc::c_int as libc::c_double)
                        * pow(a22, 2 as libc::c_int as libc::c_double)
                    - pow(a12, 2 as libc::c_int as libc::c_double)
                    + 1.0f64 / 2.0f64 * pow(a21, 4 as libc::c_int as libc::c_double)
                    + pow(a21, 2 as libc::c_int as libc::c_double)
                        * pow(a22, 2 as libc::c_int as libc::c_double)
                    - pow(a21, 2 as libc::c_int as libc::c_double)
                    + 1.0f64 / 2.0f64 * pow(a22, 4 as libc::c_int as libc::c_double)
                    - pow(a22, 2 as libc::c_int as libc::c_double)
                    - 1.0f64 / 2.0f64
                        * sqrt(
                            (pow(a11, 2 as libc::c_int as libc::c_double)
                                - 2 as libc::c_int as libc::c_double * a11 * a22
                                + pow(a12, 2 as libc::c_int as libc::c_double)
                                + 2 as libc::c_int as libc::c_double * a12 * a21
                                + pow(a21, 2 as libc::c_int as libc::c_double)
                                + pow(a22, 2 as libc::c_int as libc::c_double))
                                * (pow(a11, 2 as libc::c_int as libc::c_double)
                                    + 2 as libc::c_int as libc::c_double * a11 * a22
                                    + pow(a12, 2 as libc::c_int as libc::c_double)
                                    - 2 as libc::c_int as libc::c_double * a12 * a21
                                    + pow(a21, 2 as libc::c_int as libc::c_double)
                                    + pow(a22, 2 as libc::c_int as libc::c_double)),
                        )
                        * (pow(a11, 2 as libc::c_int as libc::c_double)
                            + pow(a12, 2 as libc::c_int as libc::c_double)
                            + pow(a21, 2 as libc::c_int as libc::c_double)
                            + pow(a22, 2 as libc::c_int as libc::c_double)
                            - 2 as libc::c_int as libc::c_double)
                    + 1 as libc::c_int as libc::c_double,
            ) - sqrt(
                1.0f64 / 2.0f64 * pow(a11, 4 as libc::c_int as libc::c_double)
                    + pow(a11, 2 as libc::c_int as libc::c_double)
                        * pow(a12, 2 as libc::c_int as libc::c_double)
                    + pow(a11, 2 as libc::c_int as libc::c_double)
                        * pow(a21, 2 as libc::c_int as libc::c_double)
                    - pow(a11, 2 as libc::c_int as libc::c_double)
                    + 2 as libc::c_int as libc::c_double * a11 * a12 * a21 * a22
                    + 1.0f64 / 2.0f64 * pow(a12, 4 as libc::c_int as libc::c_double)
                    + pow(a12, 2 as libc::c_int as libc::c_double)
                        * pow(a22, 2 as libc::c_int as libc::c_double)
                    - pow(a12, 2 as libc::c_int as libc::c_double)
                    + 1.0f64 / 2.0f64 * pow(a21, 4 as libc::c_int as libc::c_double)
                    + pow(a21, 2 as libc::c_int as libc::c_double)
                        * pow(a22, 2 as libc::c_int as libc::c_double)
                    - pow(a21, 2 as libc::c_int as libc::c_double)
                    + 1.0f64 / 2.0f64 * pow(a22, 4 as libc::c_int as libc::c_double)
                    - pow(a22, 2 as libc::c_int as libc::c_double)
                    + 1.0f64 / 2.0f64
                        * sqrt(
                            (pow(a11, 2 as libc::c_int as libc::c_double)
                                - 2 as libc::c_int as libc::c_double * a11 * a22
                                + pow(a12, 2 as libc::c_int as libc::c_double)
                                + 2 as libc::c_int as libc::c_double * a12 * a21
                                + pow(a21, 2 as libc::c_int as libc::c_double)
                                + pow(a22, 2 as libc::c_int as libc::c_double))
                                * (pow(a11, 2 as libc::c_int as libc::c_double)
                                    + 2 as libc::c_int as libc::c_double * a11 * a22
                                    + pow(a12, 2 as libc::c_int as libc::c_double)
                                    - 2 as libc::c_int as libc::c_double * a12 * a21
                                    + pow(a21, 2 as libc::c_int as libc::c_double)
                                    + pow(a22, 2 as libc::c_int as libc::c_double)),
                        )
                        * (pow(a11, 2 as libc::c_int as libc::c_double)
                            + pow(a12, 2 as libc::c_int as libc::c_double)
                            + pow(a21, 2 as libc::c_int as libc::c_double)
                            + pow(a22, 2 as libc::c_int as libc::c_double)
                            - 2 as libc::c_int as libc::c_double)
                    + 1 as libc::c_int as libc::c_double,
            ) == 0 as libc::c_int as libc::c_double
            {
                1.0f64 / 2.0f64
            } else {
                1 as libc::c_int as libc::c_double
            })
        })
        / sqrt(
            1.0f64 / 2.0f64 * pow(a11, 4 as libc::c_int as libc::c_double)
                + pow(a11, 2 as libc::c_int as libc::c_double)
                    * pow(a12, 2 as libc::c_int as libc::c_double)
                + pow(a11, 2 as libc::c_int as libc::c_double)
                    * pow(a21, 2 as libc::c_int as libc::c_double)
                - pow(a11, 2 as libc::c_int as libc::c_double)
                + 2 as libc::c_int as libc::c_double * a11 * a12 * a21 * a22
                + 1.0f64 / 2.0f64 * pow(a12, 4 as libc::c_int as libc::c_double)
                + pow(a12, 2 as libc::c_int as libc::c_double)
                    * pow(a22, 2 as libc::c_int as libc::c_double)
                - pow(a12, 2 as libc::c_int as libc::c_double)
                + 1.0f64 / 2.0f64 * pow(a21, 4 as libc::c_int as libc::c_double)
                + pow(a21, 2 as libc::c_int as libc::c_double)
                    * pow(a22, 2 as libc::c_int as libc::c_double)
                - pow(a21, 2 as libc::c_int as libc::c_double)
                + 1.0f64 / 2.0f64 * pow(a22, 4 as libc::c_int as libc::c_double)
                - pow(a22, 2 as libc::c_int as libc::c_double)
                - 1.0f64 / 2.0f64
                    * sqrt(
                        (pow(a11, 2 as libc::c_int as libc::c_double)
                            - 2 as libc::c_int as libc::c_double * a11 * a22
                            + pow(a12, 2 as libc::c_int as libc::c_double)
                            + 2 as libc::c_int as libc::c_double * a12 * a21
                            + pow(a21, 2 as libc::c_int as libc::c_double)
                            + pow(a22, 2 as libc::c_int as libc::c_double))
                            * (pow(a11, 2 as libc::c_int as libc::c_double)
                                + 2 as libc::c_int as libc::c_double * a11 * a22
                                + pow(a12, 2 as libc::c_int as libc::c_double)
                                - 2 as libc::c_int as libc::c_double * a12 * a21
                                + pow(a21, 2 as libc::c_int as libc::c_double)
                                + pow(a22, 2 as libc::c_int as libc::c_double)),
                    )
                    * (pow(a11, 2 as libc::c_int as libc::c_double)
                        + pow(a12, 2 as libc::c_int as libc::c_double)
                        + pow(a21, 2 as libc::c_int as libc::c_double)
                        + pow(a22, 2 as libc::c_int as libc::c_double)
                        - 2 as libc::c_int as libc::c_double)
                + 1 as libc::c_int as libc::c_double,
        )
        + (pow(a11, 2 as libc::c_int as libc::c_double) * a12
            + a11 * a21 * a22
            + pow(a12, 3 as libc::c_int as libc::c_double)
            + a12 * pow(a22, 2 as libc::c_int as libc::c_double)
            + 1.0f64 / 2.0f64
                * a12
                * sqrt(
                    (pow(a11, 2 as libc::c_int as libc::c_double)
                        - 2 as libc::c_int as libc::c_double * a11 * a22
                        + pow(a12, 2 as libc::c_int as libc::c_double)
                        + 2 as libc::c_int as libc::c_double * a12 * a21
                        + pow(a21, 2 as libc::c_int as libc::c_double)
                        + pow(a22, 2 as libc::c_int as libc::c_double))
                        * (pow(a11, 2 as libc::c_int as libc::c_double)
                            + 2 as libc::c_int as libc::c_double * a11 * a22
                            + pow(a12, 2 as libc::c_int as libc::c_double)
                            - 2 as libc::c_int as libc::c_double * a12 * a21
                            + pow(a21, 2 as libc::c_int as libc::c_double)
                            + pow(a22, 2 as libc::c_int as libc::c_double)),
                )
            - a12
            + 1.0f64 / 4.0f64
                * sqrt(
                    (pow(a11, 2 as libc::c_int as libc::c_double)
                        - 2 as libc::c_int as libc::c_double * a11 * a22
                        + pow(a12, 2 as libc::c_int as libc::c_double)
                        + 2 as libc::c_int as libc::c_double * a12 * a21
                        + pow(a21, 2 as libc::c_int as libc::c_double)
                        + pow(a22, 2 as libc::c_int as libc::c_double))
                        * (pow(a11, 2 as libc::c_int as libc::c_double)
                            + 2 as libc::c_int as libc::c_double * a11 * a22
                            + pow(a12, 2 as libc::c_int as libc::c_double)
                            - 2 as libc::c_int as libc::c_double * a12 * a21
                            + pow(a21, 2 as libc::c_int as libc::c_double)
                            + pow(a22, 2 as libc::c_int as libc::c_double)),
                )
                * (1.0f64 / 2.0f64
                    * (2 as libc::c_int as libc::c_double * a12
                        - 2 as libc::c_int as libc::c_double * a21)
                    * (pow(a11, 2 as libc::c_int as libc::c_double)
                        - 2 as libc::c_int as libc::c_double * a11 * a22
                        + pow(a12, 2 as libc::c_int as libc::c_double)
                        + 2 as libc::c_int as libc::c_double * a12 * a21
                        + pow(a21, 2 as libc::c_int as libc::c_double)
                        + pow(a22, 2 as libc::c_int as libc::c_double))
                    + 1.0f64 / 2.0f64
                        * (2 as libc::c_int as libc::c_double * a12
                            + 2 as libc::c_int as libc::c_double * a21)
                        * (pow(a11, 2 as libc::c_int as libc::c_double)
                            + 2 as libc::c_int as libc::c_double * a11 * a22
                            + pow(a12, 2 as libc::c_int as libc::c_double)
                            - 2 as libc::c_int as libc::c_double * a12 * a21
                            + pow(a21, 2 as libc::c_int as libc::c_double)
                            + pow(a22, 2 as libc::c_int as libc::c_double)))
                * (pow(a11, 2 as libc::c_int as libc::c_double)
                    + pow(a12, 2 as libc::c_int as libc::c_double)
                    + pow(a21, 2 as libc::c_int as libc::c_double)
                    + pow(a22, 2 as libc::c_int as libc::c_double)
                    - 2 as libc::c_int as libc::c_double)
                / ((pow(a11, 2 as libc::c_int as libc::c_double)
                    - 2 as libc::c_int as libc::c_double * a11 * a22
                    + pow(a12, 2 as libc::c_int as libc::c_double)
                    + 2 as libc::c_int as libc::c_double * a12 * a21
                    + pow(a21, 2 as libc::c_int as libc::c_double)
                    + pow(a22, 2 as libc::c_int as libc::c_double))
                    * (pow(a11, 2 as libc::c_int as libc::c_double)
                        + 2 as libc::c_int as libc::c_double * a11 * a22
                        + pow(a12, 2 as libc::c_int as libc::c_double)
                        - 2 as libc::c_int as libc::c_double * a12 * a21
                        + pow(a21, 2 as libc::c_int as libc::c_double)
                        + pow(a22, 2 as libc::c_int as libc::c_double))))
            * (if sqrt(
                1.0f64 / 2.0f64 * pow(a11, 4 as libc::c_int as libc::c_double)
                    + pow(a11, 2 as libc::c_int as libc::c_double)
                        * pow(a12, 2 as libc::c_int as libc::c_double)
                    + pow(a11, 2 as libc::c_int as libc::c_double)
                        * pow(a21, 2 as libc::c_int as libc::c_double)
                    - pow(a11, 2 as libc::c_int as libc::c_double)
                    + 2 as libc::c_int as libc::c_double * a11 * a12 * a21 * a22
                    + 1.0f64 / 2.0f64 * pow(a12, 4 as libc::c_int as libc::c_double)
                    + pow(a12, 2 as libc::c_int as libc::c_double)
                        * pow(a22, 2 as libc::c_int as libc::c_double)
                    - pow(a12, 2 as libc::c_int as libc::c_double)
                    + 1.0f64 / 2.0f64 * pow(a21, 4 as libc::c_int as libc::c_double)
                    + pow(a21, 2 as libc::c_int as libc::c_double)
                        * pow(a22, 2 as libc::c_int as libc::c_double)
                    - pow(a21, 2 as libc::c_int as libc::c_double)
                    + 1.0f64 / 2.0f64 * pow(a22, 4 as libc::c_int as libc::c_double)
                    - pow(a22, 2 as libc::c_int as libc::c_double)
                    - 1.0f64 / 2.0f64
                        * sqrt(
                            (pow(a11, 2 as libc::c_int as libc::c_double)
                                - 2 as libc::c_int as libc::c_double * a11 * a22
                                + pow(a12, 2 as libc::c_int as libc::c_double)
                                + 2 as libc::c_int as libc::c_double * a12 * a21
                                + pow(a21, 2 as libc::c_int as libc::c_double)
                                + pow(a22, 2 as libc::c_int as libc::c_double))
                                * (pow(a11, 2 as libc::c_int as libc::c_double)
                                    + 2 as libc::c_int as libc::c_double * a11 * a22
                                    + pow(a12, 2 as libc::c_int as libc::c_double)
                                    - 2 as libc::c_int as libc::c_double * a12 * a21
                                    + pow(a21, 2 as libc::c_int as libc::c_double)
                                    + pow(a22, 2 as libc::c_int as libc::c_double)),
                        )
                        * (pow(a11, 2 as libc::c_int as libc::c_double)
                            + pow(a12, 2 as libc::c_int as libc::c_double)
                            + pow(a21, 2 as libc::c_int as libc::c_double)
                            + pow(a22, 2 as libc::c_int as libc::c_double)
                            - 2 as libc::c_int as libc::c_double)
                    + 1 as libc::c_int as libc::c_double,
            ) - sqrt(
                1.0f64 / 2.0f64 * pow(a11, 4 as libc::c_int as libc::c_double)
                    + pow(a11, 2 as libc::c_int as libc::c_double)
                        * pow(a12, 2 as libc::c_int as libc::c_double)
                    + pow(a11, 2 as libc::c_int as libc::c_double)
                        * pow(a21, 2 as libc::c_int as libc::c_double)
                    - pow(a11, 2 as libc::c_int as libc::c_double)
                    + 2 as libc::c_int as libc::c_double * a11 * a12 * a21 * a22
                    + 1.0f64 / 2.0f64 * pow(a12, 4 as libc::c_int as libc::c_double)
                    + pow(a12, 2 as libc::c_int as libc::c_double)
                        * pow(a22, 2 as libc::c_int as libc::c_double)
                    - pow(a12, 2 as libc::c_int as libc::c_double)
                    + 1.0f64 / 2.0f64 * pow(a21, 4 as libc::c_int as libc::c_double)
                    + pow(a21, 2 as libc::c_int as libc::c_double)
                        * pow(a22, 2 as libc::c_int as libc::c_double)
                    - pow(a21, 2 as libc::c_int as libc::c_double)
                    + 1.0f64 / 2.0f64 * pow(a22, 4 as libc::c_int as libc::c_double)
                    - pow(a22, 2 as libc::c_int as libc::c_double)
                    + 1.0f64 / 2.0f64
                        * sqrt(
                            (pow(a11, 2 as libc::c_int as libc::c_double)
                                - 2 as libc::c_int as libc::c_double * a11 * a22
                                + pow(a12, 2 as libc::c_int as libc::c_double)
                                + 2 as libc::c_int as libc::c_double * a12 * a21
                                + pow(a21, 2 as libc::c_int as libc::c_double)
                                + pow(a22, 2 as libc::c_int as libc::c_double))
                                * (pow(a11, 2 as libc::c_int as libc::c_double)
                                    + 2 as libc::c_int as libc::c_double * a11 * a22
                                    + pow(a12, 2 as libc::c_int as libc::c_double)
                                    - 2 as libc::c_int as libc::c_double * a12 * a21
                                    + pow(a21, 2 as libc::c_int as libc::c_double)
                                    + pow(a22, 2 as libc::c_int as libc::c_double)),
                        )
                        * (pow(a11, 2 as libc::c_int as libc::c_double)
                            + pow(a12, 2 as libc::c_int as libc::c_double)
                            + pow(a21, 2 as libc::c_int as libc::c_double)
                            + pow(a22, 2 as libc::c_int as libc::c_double)
                            - 2 as libc::c_int as libc::c_double)
                    + 1 as libc::c_int as libc::c_double,
            ) > 0 as libc::c_int as libc::c_double
            {
                0 as libc::c_int as libc::c_double
            } else {
                (if sqrt(
                    1.0f64 / 2.0f64 * pow(a11, 4 as libc::c_int as libc::c_double)
                        + pow(a11, 2 as libc::c_int as libc::c_double)
                            * pow(a12, 2 as libc::c_int as libc::c_double)
                        + pow(a11, 2 as libc::c_int as libc::c_double)
                            * pow(a21, 2 as libc::c_int as libc::c_double)
                        - pow(a11, 2 as libc::c_int as libc::c_double)
                        + 2 as libc::c_int as libc::c_double * a11 * a12 * a21 * a22
                        + 1.0f64 / 2.0f64 * pow(a12, 4 as libc::c_int as libc::c_double)
                        + pow(a12, 2 as libc::c_int as libc::c_double)
                            * pow(a22, 2 as libc::c_int as libc::c_double)
                        - pow(a12, 2 as libc::c_int as libc::c_double)
                        + 1.0f64 / 2.0f64 * pow(a21, 4 as libc::c_int as libc::c_double)
                        + pow(a21, 2 as libc::c_int as libc::c_double)
                            * pow(a22, 2 as libc::c_int as libc::c_double)
                        - pow(a21, 2 as libc::c_int as libc::c_double)
                        + 1.0f64 / 2.0f64 * pow(a22, 4 as libc::c_int as libc::c_double)
                        - pow(a22, 2 as libc::c_int as libc::c_double)
                        - 1.0f64 / 2.0f64
                            * sqrt(
                                (pow(a11, 2 as libc::c_int as libc::c_double)
                                    - 2 as libc::c_int as libc::c_double * a11 * a22
                                    + pow(a12, 2 as libc::c_int as libc::c_double)
                                    + 2 as libc::c_int as libc::c_double * a12 * a21
                                    + pow(a21, 2 as libc::c_int as libc::c_double)
                                    + pow(a22, 2 as libc::c_int as libc::c_double))
                                    * (pow(a11, 2 as libc::c_int as libc::c_double)
                                        + 2 as libc::c_int as libc::c_double * a11 * a22
                                        + pow(a12, 2 as libc::c_int as libc::c_double)
                                        - 2 as libc::c_int as libc::c_double * a12 * a21
                                        + pow(a21, 2 as libc::c_int as libc::c_double)
                                        + pow(a22, 2 as libc::c_int as libc::c_double)),
                            )
                            * (pow(a11, 2 as libc::c_int as libc::c_double)
                                + pow(a12, 2 as libc::c_int as libc::c_double)
                                + pow(a21, 2 as libc::c_int as libc::c_double)
                                + pow(a22, 2 as libc::c_int as libc::c_double)
                                - 2 as libc::c_int as libc::c_double)
                        + 1 as libc::c_int as libc::c_double,
                ) - sqrt(
                    1.0f64 / 2.0f64 * pow(a11, 4 as libc::c_int as libc::c_double)
                        + pow(a11, 2 as libc::c_int as libc::c_double)
                            * pow(a12, 2 as libc::c_int as libc::c_double)
                        + pow(a11, 2 as libc::c_int as libc::c_double)
                            * pow(a21, 2 as libc::c_int as libc::c_double)
                        - pow(a11, 2 as libc::c_int as libc::c_double)
                        + 2 as libc::c_int as libc::c_double * a11 * a12 * a21 * a22
                        + 1.0f64 / 2.0f64 * pow(a12, 4 as libc::c_int as libc::c_double)
                        + pow(a12, 2 as libc::c_int as libc::c_double)
                            * pow(a22, 2 as libc::c_int as libc::c_double)
                        - pow(a12, 2 as libc::c_int as libc::c_double)
                        + 1.0f64 / 2.0f64 * pow(a21, 4 as libc::c_int as libc::c_double)
                        + pow(a21, 2 as libc::c_int as libc::c_double)
                            * pow(a22, 2 as libc::c_int as libc::c_double)
                        - pow(a21, 2 as libc::c_int as libc::c_double)
                        + 1.0f64 / 2.0f64 * pow(a22, 4 as libc::c_int as libc::c_double)
                        - pow(a22, 2 as libc::c_int as libc::c_double)
                        + 1.0f64 / 2.0f64
                            * sqrt(
                                (pow(a11, 2 as libc::c_int as libc::c_double)
                                    - 2 as libc::c_int as libc::c_double * a11 * a22
                                    + pow(a12, 2 as libc::c_int as libc::c_double)
                                    + 2 as libc::c_int as libc::c_double * a12 * a21
                                    + pow(a21, 2 as libc::c_int as libc::c_double)
                                    + pow(a22, 2 as libc::c_int as libc::c_double))
                                    * (pow(a11, 2 as libc::c_int as libc::c_double)
                                        + 2 as libc::c_int as libc::c_double * a11 * a22
                                        + pow(a12, 2 as libc::c_int as libc::c_double)
                                        - 2 as libc::c_int as libc::c_double * a12 * a21
                                        + pow(a21, 2 as libc::c_int as libc::c_double)
                                        + pow(a22, 2 as libc::c_int as libc::c_double)),
                            )
                            * (pow(a11, 2 as libc::c_int as libc::c_double)
                                + pow(a12, 2 as libc::c_int as libc::c_double)
                                + pow(a21, 2 as libc::c_int as libc::c_double)
                                + pow(a22, 2 as libc::c_int as libc::c_double)
                                - 2 as libc::c_int as libc::c_double)
                        + 1 as libc::c_int as libc::c_double,
                ) == 0 as libc::c_int as libc::c_double
                {
                    1.0f64 / 2.0f64
                } else {
                    1 as libc::c_int as libc::c_double
                })
            })
            / sqrt(
                1.0f64 / 2.0f64 * pow(a11, 4 as libc::c_int as libc::c_double)
                    + pow(a11, 2 as libc::c_int as libc::c_double)
                        * pow(a12, 2 as libc::c_int as libc::c_double)
                    + pow(a11, 2 as libc::c_int as libc::c_double)
                        * pow(a21, 2 as libc::c_int as libc::c_double)
                    - pow(a11, 2 as libc::c_int as libc::c_double)
                    + 2 as libc::c_int as libc::c_double * a11 * a12 * a21 * a22
                    + 1.0f64 / 2.0f64 * pow(a12, 4 as libc::c_int as libc::c_double)
                    + pow(a12, 2 as libc::c_int as libc::c_double)
                        * pow(a22, 2 as libc::c_int as libc::c_double)
                    - pow(a12, 2 as libc::c_int as libc::c_double)
                    + 1.0f64 / 2.0f64 * pow(a21, 4 as libc::c_int as libc::c_double)
                    + pow(a21, 2 as libc::c_int as libc::c_double)
                        * pow(a22, 2 as libc::c_int as libc::c_double)
                    - pow(a21, 2 as libc::c_int as libc::c_double)
                    + 1.0f64 / 2.0f64 * pow(a22, 4 as libc::c_int as libc::c_double)
                    - pow(a22, 2 as libc::c_int as libc::c_double)
                    + 1.0f64 / 2.0f64
                        * sqrt(
                            (pow(a11, 2 as libc::c_int as libc::c_double)
                                - 2 as libc::c_int as libc::c_double * a11 * a22
                                + pow(a12, 2 as libc::c_int as libc::c_double)
                                + 2 as libc::c_int as libc::c_double * a12 * a21
                                + pow(a21, 2 as libc::c_int as libc::c_double)
                                + pow(a22, 2 as libc::c_int as libc::c_double))
                                * (pow(a11, 2 as libc::c_int as libc::c_double)
                                    + 2 as libc::c_int as libc::c_double * a11 * a22
                                    + pow(a12, 2 as libc::c_int as libc::c_double)
                                    - 2 as libc::c_int as libc::c_double * a12 * a21
                                    + pow(a21, 2 as libc::c_int as libc::c_double)
                                    + pow(a22, 2 as libc::c_int as libc::c_double)),
                        )
                        * (pow(a11, 2 as libc::c_int as libc::c_double)
                            + pow(a12, 2 as libc::c_int as libc::c_double)
                            + pow(a21, 2 as libc::c_int as libc::c_double)
                            + pow(a22, 2 as libc::c_int as libc::c_double)
                            - 2 as libc::c_int as libc::c_double)
                    + 1 as libc::c_int as libc::c_double,
            );
    return grad_orth_1_result;
}
