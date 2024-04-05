fn main() {
    use positions::prelude::*;
    use rust_decimal_macros::dec;
    use maplit::hashmap;

    // Create a portfolio
    // This is really just a collection of positions
    let mut portfolio = positions::Positions::default();

    // The assets we are interested in
    let usd = Asset::USD;
    let mx = "MX".parse().unwrap();
    let cad = "CAD".parse().unwrap();

    // The instruments that we use to express our positions
    let mx_usd = Instrument::spot(&mx, &usd);
    let usd_mx = Instrument::spot(&usd, &mx);
    let mx_cad = Instrument::spot(&mx, &cad);
    let usd_cad = Instrument::spot(&usd, &cad);

    // Buy 50 MX with 1000 USD
    // "I think MX will outperform USD, and 20 USD / MX is a good price."
    let price = dec!(1000) / dec!(50);
    portfolio += mx_usd.position((price, dec!(50)));

    // Buy 100 USD with 6 MX
    // "I am going to hedge my bets a little. Might as well have some USD
    // on hand since it is the world's reserve currency."
    let price = dec!(6) / dec!(100);
    portfolio += usd_mx.position((price, dec!(6)));

    // Buy 10 MX with 3 CAD
    // "Another vote of confidence for MX."
    let price = dec!(3) / dec!(10);
    portfolio += mx_cad.position((price, dec!(10)));

    println!("\ninitial portfolio:");
    println!("{}", portfolio);

    // Presently the portfolio is expressed in terms of several abstract
    // financial instruments. (In this case currency pairs, aka "spots")
    // 
    // The actual *contents* of the portfolio are the assets that we are
    // holding. (60 MX and 6 USD)
    // 
    // The *value*, however, requires choosing a base currency.
    // (This comes down to simple dimensional analysis. Unless we explicitly
    // state the relationship between MX/USD, MX/CAD, and USD/CAD, we cannot
    // say any more than what was said above.)

    // Let's figure out the value of our portfolio in CAD.

    // The `as_expr` method converts the portfolio into an semi-mathematical
    // expression in terms of all our instruments (spot currency pairs).
    let expr = portfolio.as_expr();
    println!("portfolio as an expression:");
    println!("{expr}");

    // The `instruments` method returns all the instruments that the
    // expression depends on.
    println!("\ndependent instruments for CAD:");
    for inst in expr.instruments(&cad) {
        println!("{inst}");
    }

    // To evaluate the equity of our positions, we must provide the prices
    // of the instruments above.
    let mx_usd_price = dec!(1500) / dec!(50);           // MX has appreciated relative to USD
    let mx_cad_price = dec!(2) / dec!(10);              // MX has depreciated relative to CAD
    let usd_cad_price = mx_usd_price / mx_cad_price;    // computed from the above

    let prices = hashmap! {
        mx_usd.as_symbol().clone() => mx_usd_price,
        usd_mx.as_symbol().clone() => dec!(1.0) / mx_usd_price,

        mx_cad.as_symbol().clone() => mx_cad_price,
        // cad_mx.as_symbol().clone() => dec!(1.0) / mx_cad_price,

        usd_cad.as_symbol().clone() => usd_cad_price,
    };

    // Now we are ready to evaluate the equity of our positions.
    let equity = expr.eval(&cad, &prices).unwrap();
    println!("\nequity of the portfolio:");
    println!("{equity} CAD");

    println!("\ngiven prices:");
    println!("\tMX-USD: {mx_usd_price}");
    println!("\tMX-CAD: {mx_cad_price}");
    println!("\tUSD-CAD: {usd_cad_price}");

}
