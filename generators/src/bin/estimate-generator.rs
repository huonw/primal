/*!
Each statistic about primes can be estimated/bounded in many ways,
with varying qualities. For each bound, this program calculates the
best valid estimate at many points across the whole u64 range, to
ensure that primal-estimate is as tight as possible.
*/

/// Is this bound an upper or lower bound on the prime statistic?
enum BoundType {
    // Unconventional upper case to allow using the static's name
    // directly to construct this.
    HIGH, LOW
}

#[derive(Clone, Debug)]
pub struct Estimate {
    /// The minimum value for which this bound is correct.
    valid_from: f64,
    /// The theorem/proof of this bound.
    source: &'static str,
    /// The source code that calculates this bound.
    expression: &'static str,
    compute: fn(n_: f64, lg: f64, lglg: f64, inv_lg: f64, n_lg: f64) -> f64,
}
pub struct EstimateGroup {
    bound_type: BoundType,
    estimators: &'static [Estimate],
}


macro_rules! estimates {
    ($(
        $group: ident: {
            $($from: expr, $source: expr =>
              |$n_: pat, $lg: pat, $lglg: pat, $inv_lg: pat, $n_inv_lg: pat| { $e: expr },)*
        }
    )*) => {
        $(
            pub const $group: crate::EstimateGroup = crate::EstimateGroup {
                bound_type: crate::BoundType::$group,
                estimators: &[
                    $(
                        crate::Estimate {
                            valid_from: $from as f64,
                            source: $source,
                            expression: stringify!($e),

                            compute: |$n_: f64, $lg: f64, $lglg: f64, $inv_lg: f64, $n_inv_lg: f64| $e,
                        },
                    )*
                ]
            };
        )*
    };
}

mod prime_pi {
    // [1]: Barkley Rosser. "Explicit Bounds for Some Functions of
    // Prime Numbers". American Journal of Mathematics 63 (1):
    // 211–232. 1941. doi:[10.2307/2371291](http://dx.doi.org/10.2307/2371291).
    //
    // [2]: Dusart, Pierre. ["Autour de la fonction qui compte le
    // nombre de nombres premiers."][pdf] PhD diss., Université de
    // Limoges, 1998.
    //
    // [pdf]: http://www.unilim.fr/laco/theses/1998/T1998_01.html
    //
    // [3]: Dusart, Pierre. "Estimates of Some Functions Over Primes
    // without R.H."
    // ArXiv:[1002.0442](http://arxiv.org/abs/1002.0442). 2010.
    estimates! {
        LOW: {
            88_783, "[3] Theorem 6.9 (6.7)" => |_, _, _, inv_lg, n_inv_lg| {
                n_inv_lg * (1.0 + inv_lg * (1.0 + 2.0 * inv_lg))
            },
            32_299, "[2] Theorem 1.10 (6.)" => |_, _, _, inv_lg, n_inv_lg| {
                n_inv_lg * (1.0 + inv_lg * (1.0 + 1.8 * inv_lg))
            },
            5_393, "[2] Theorem 1.10 (5.)" => |n_, lg, _, _, _| {
                n_ / (lg - 1.0)
            },
            599, "[2] Theorem 1.10 (1.)" => |_, _, _, inv_lg, n_inv_lg| {
                n_inv_lg * (1.0 + inv_lg)
            },
            0, "[1]" => |n_, lg, _, _, _| {
                n_ / (lg + 2.0)
            },
        }
        HIGH: {
            13_220_000_000_u64, "[2] Theorem 1.10 (3.)" => |_, _, _, inv_lg, n_inv_lg| {
                n_inv_lg * (1.0 + 1.0992 * inv_lg)
            },
            2_953_652_287_u64, "[3] Theorem 6.9 (6.7)" => |_, _, _, inv_lg, n_inv_lg| {
                n_inv_lg * (1.0 + inv_lg * (1.0 + 2.334 * inv_lg))
            },
            355_991, "[2] Theorem 1.10 (3.)" => |_, _, _, inv_lg, n_inv_lg| {
                n_inv_lg * (1.0 + inv_lg * (1.0 + 2.51 * inv_lg))
            },
            60_184, "[2] Theorem 1.10 (4.)" => |n_, lg, _, _, _| {
                n_ / (lg - 1.1)
            },
            0, "[2] Theorem 1.10 (2.)" => |_, _, _, inv_lg, n_inv_lg| {
                n_inv_lg * (1.0 + 1.2762 * inv_lg)
            },
        }
    }
}
mod nth_prime {
    // [1]: Massias, Jean-Pierre; Robin, Guy. ["Bornes effectives pour
    // certaines fonctions concernant les nombres
    // premiers."](http://eudml.org/doc/247826) Journal de théorie des
    // nombres de Bordeaux 8.1 (1996): 215-242.
    //
    // [2]: Dusart, Pierre. ["Autour de la fonction qui compte le nombre
    // de nombres premiers."][pdf] PhD diss., Université de Limoges, 1998.
    //
    // [pdf]: http://www.unilim.fr/laco/theses/1998/T1998_01.html
    //
    // [3]: Dusart, Pierre. "Estimates of Some Functions Over Primes
    // without R.H."
    // ArXiv:[1002.0442](http://arxiv.org/abs/1002.0442). 2010.
    estimates! {
        LOW: {
            3, "[2] Theorem 1.6" => |n_, lg, lglg, _, _| {
                n_ * (lg + lglg - 1.0 + (lglg - 2.1) / lg)
            },
            0, "[1] Theorem A (ii)" => |n_, lg, lglg, _, _| {
                n_ * (lg + lglg - 1.0)
            },
        }
        HIGH: {
            688_383, "[3] Proposition 6.6" => |n_, lg, lglg, _, _| {
                n_ * (lg + lglg - 1.0 + (lglg - 2.0) / lg)
            },
            178_974, "[3] Lemma 6.5" => |n_, lg, lglg, _, _| {
                n_ * (lg + lglg - 1.0 + (lglg - 1.95) / lg)
            },
            39_017, "[2] Theorem 1.8" => |n_, lg, lglg, _, _| {
                n_ * (lg + lglg - 0.9484)
            },
            27_076, "[2] Theorem 1.7" => |n_, lg, lglg, _, _| {
                n_ * (lg + lglg - 1.0 + (lglg - 1.8) / lg)
            },
            15_985, "[1] Theorem A (v)" => |n_, lg, lglg, _, _| {
                n_ * (lg + lglg - 0.9427)
            },
            13, "[1] Theorem A (v)" => |n_, lg, lglg, _, _| {
                n_ * (lg + lglg - 1.0 + 1.8 * lglg / lg)
            },
            0, "[1] Theorem A (iv)" => |n_, lg, lglg, _, _| {
                n_ * (lg + lglg)
            },
        }
    }
}


fn non_nan_cmp(x: f64, y: f64) -> std::cmp::Ordering {
    x.partial_cmp(&y).unwrap()
}

const MULT: f64 = 1.001;
/// This takes a group of estimates, and prints out a series of match
/// arms covering subranges of u64, each of which is the most accurate
/// expression for the statistic in question on that range.
fn analyse(group: &EstimateGroup) {
    // Sort the estimators by their regions of validity.
    let mut estimators: Vec<_> = group.estimators.to_vec();
    estimators.sort_by(|a, b| non_nan_cmp(a.valid_from, b.valid_from));

    struct Subrange<'a> {
        estimate: &'a Estimate,
        from: f64,
    }
    let mut subranges = vec![];

    let mut current = &estimators[0];
    let mut current_first = 0.0;
    let mut upper_limit = None;

    const MAX: f64 = std::u64::MAX as f64;

    // Conceptually, this works by testing the set of valid estimates
    // on many points in the u64 range (essentially, all the powers of
    // MULT, rounded to the nearest u64), and recording the most
    // accurate estimate. There's a nested loop here to ensure that we
    // always exactly test the valid_from of each new estimator,
    // meaning the exact point where it becomes applicable, and start
    // multiplying by MULT from there.
    for (i, e) in estimators.iter().enumerate() {
        let limit = estimators.get(i + 1).map_or(MAX, |e| e.valid_from);

        let mut raw_n = e.valid_from.max(1.0);
        while raw_n < limit {
            let n_ = raw_n.floor();
            raw_n *= MULT;

            let lg = n_.ln();
            let lglg = lg.ln();
            let inv_lg = 1.0 / lg;
            let n_inv_lg = n_ * inv_lg;

            // The estimators are sorted and the MULT powers are not
            // yet up to estimators[i + 1].valid_from, so the prefix
            // up to our current estimator will be valid at
            // n_. Compute those estimators' estimates at this point.
            let estimates =
                estimators[..i + 1].iter()
                .map(|e| (e, (e.compute)(n_, lg, lglg, inv_lg, n_inv_lg)));

            // The "best" of the estimates depends whether this is
            // trying to be an upper bound, or a lower bound.
            let (best, estimate) = match group.bound_type {
                BoundType::HIGH => estimates.min_by(|a, b| non_nan_cmp(a.1, b.1)).unwrap(),
                BoundType::LOW => estimates.max_by(|a, b| non_nan_cmp(a.1, b.1)).unwrap(),
            };

            // Does the estimate overflow?
            if limit == MAX && estimate > MAX {
                upper_limit = Some(n_ / MULT);
                break
            }

            // If the best one at this point is different to the best
            // at the previous points, we've switched into a new
            // segment, so we need to record the old one.
            if best.expression != current.expression {
                subranges.push(Subrange { estimate: current, from: current_first });
                current = best;
                current_first = n_;
            }
        }
    }
    subranges.push(Subrange { estimate: current, from: current_first });

    if let Some(limit) = upper_limit {
        println!("const MAX_VALID_INPUT: u64 = {:.0};", limit);
    }
    for sr in subranges.iter().rev() {
        let guard = if sr.from == 0.0 {
            "".into()
        } else {
            format!(" if n >= {:.0}_u64", sr.from)
        };

        println!("// {}\n_{} => {},", sr.estimate.source, guard, sr.estimate.expression)
    }
}

fn main() {
    println!("prime_pi::LOW");
    analyse(&prime_pi::LOW);

    println!("prime_pi::HIGH");
    analyse(&prime_pi::HIGH);

    println!("nth_prime::LOW");
    analyse(&nth_prime::LOW);

    println!("nth_prime::HIGH");
    analyse(&nth_prime::HIGH);
}
