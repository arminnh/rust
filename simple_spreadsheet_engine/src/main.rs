mod cell;
mod cell_pos;
mod cell_range;
mod expression;
mod formula;
mod function;
mod number_or_cell_pos;
mod sheet;

use crate::sheet::Sheet;

fn run(input: &str) -> &str {
    println!("{}\n", input);
    let sheet = Sheet::parse_input(input);
    print!("{}\n\n", sheet);

    ""
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_countdown() {
        let input = "10,=A1 - 1,<,<,<,<,<,<,<,<,<";
        let output = "10,9,8,7,6,5,4,3,2,1,0";

        assert_eq!(run(input), output);
    }

    #[test]
    fn test_statistics() {
        let input = "runtime (s),average,=AVG(A2:A8)
30,median ,=MEDIAN(A2:A8)
40,stdev  ,=STDEV(A2:A8)
32,       ,
54,       ,
23,       ,
34,       ,
29,       ,";

        let output = "runtime (s),average,34.5714
30,median ,     32
40,stdev  ,9.99762
32,       ,
54,       ,
23,       ,
34,       ,
29,       ,";

        assert_eq!(run(input), output);
    }

    #[test]
    fn test_shop_items() {
        let input = "descr  ,amount, unit_price,total_price
Cookies,     4,       2.95,=B2 * C2
Coffee ,     1,=9.60 * 0.8,^
Water  ,     2,       1.20,^
Total  ,      ,           ,=SUM(D2:D4)";

        let output = "descr  ,amount,unit_price,total_price
Cookies,     4,      2.95,       11.8
Coffee ,     1,      7.68,       7.68
Water  ,     2,       1.2,        2.4
Total  ,      ,          ,      21.88";

        assert_eq!(run(input), output);
    }

    //     #[test]
    //     fn test_errors() {
    //         let input = "=OOPS(A2)  ,=AVG
    // =nope + 1  ,
    // >          ,=XYZ123,
    // =SUM(A1:A3),
    // ^          ,
    // =-1 + 1    ,'=IFERROR(A6, \"Oops!\")'
    // =1 / 0     ,^";

    //         let output = "#ERROR#: 'OOPS' is not a valid operation                        ,#ERROR#: 'AVG' must be called with (),
    // #ERROR#: Unexpected character 'n'                               ,                                     ,
    // #ERROR#: 'XYZ123' is not a known name                           ,#ERROR#: 'XYZ123' is not a known name,
    // #ERROR#: unsupported operand type(s) for +: 'int' and 'Error'   ,                                     ,
    // #ERROR#: unsupported operand type(s) for +: 'int' and 'NoneType',                                     ,
    //                                                                 0,                                    0,
    // #ERROR#: float division by zero                                 ,Oops!                                ,";

    //         assert_eq!(run(input), output);
    //     }
}
