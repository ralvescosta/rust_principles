use std::collections::HashMap;
fn main() {
    simple_macros();
    composite_exp_macro();
    compare_macro();
    loop_macro();
    ddd_macro();
}

fn simple_macros() {
    macro_rules! a_macro {
        () => {
            println!("This is a macro")
        };
    }

    a_macro!();

    macro_rules! x_and_y {
        (x => $e:expr) => {
            println!("X: {}", $e)
        };
        (y => $e:expr) => {
            println!("Y: {}", $e)
        };
    }

    x_and_y!(x => 10);
    x_and_y!(y => 10 + 30);

    macro_rules! build_fn {
        ($func_name:ident) => {
            fn $func_name() {
                println!("You called {:?}()", stringify!($func_name));
            }
        };
    }

    build_fn!(say_hi);
    say_hi();

    macro_rules! print_ex {
        ($e:expr) => {
            println!("{:?} = {:?}", stringify!($e), $e)
        };
    }

    print_ex!({
        let y = 20;
        let z = 30;
        z + y + 10 * 3 * 100
    });
    println!();
}

fn composite_exp_macro() {
    macro_rules! exame {
        ($l:expr; and $r:expr) => {
            println!(
                "{:?} and {:?} is {:?}",
                stringify!($l),
                stringify!($r),
                $l && $r
            )
        };

        ($l:expr; or $r:expr) => {
            println!(
                "{:?} or {:?} is {:?}",
                stringify!($l),
                stringify!($r),
                $l || $r
            )
        };
    }

    exame!(1 == 1; and 2 == 1+1);
    exame!(true; or false);
    println!();
}

fn compare_macro() {
    macro_rules! compr {
        ($id1: ident | $id2: ident <- [$start: expr; $end: expr] , $cond: expr) => {{
            let mut vec = Vec::new();

            for num in $start..$end + 1 {
                if $cond(num) {
                    vec.push(num);
                }
            }
            vec
        }};
    }

    fn even(x: i32) -> bool {
        x % 2 == 0
    }

    fn odd(x: i32) -> bool {
        x % 2 != 0
    }

    let evens = compr!(x | x <- [1;10], even);
    println!("{:?}", evens);

    let odds = compr!(y | y <- [1;10], odd);
    println!("{:?}", odds);
    println!();
}

fn loop_macro() {
    macro_rules! new_map {
        ($($key: expr => $val: expr),*) => {
            {
                let mut map = HashMap::new();

                $(
                    map.insert($key, $val);
                )*

                map
            }
        }
    }

    let map = new_map! {
        "one" => 1,
        "two" => 2
    };
    println!("{:?}", map);
    println!();
}

fn ddd_macro() {
    macro_rules! calc {
        (eval $e:expr) => {
            {
                let val: usize = $e;
                println!("{} = {}", stringify!{$e}, val);
            }
        };

        (eval $e:expr, $(eval $es: expr),+) => {
            {
                calc! { eval $e }
                calc! { $(eval $es),+ }
            }
        }
    }

    calc! {
        eval 4 * 5,
        eval 4 + 10,
        eval (10 * 3) - 20
    }
}
