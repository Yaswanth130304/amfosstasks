use scraper::{Html, Selector};
fn main() {
    let url = "https://crypto.com/price";
    let response = reqwest::blocking::get(url).expect("Could not load url.");
    let body = response.text().expect("No response body found.");
    let document = Html::parse_document(&body);
    let stock = Selector::parse("tbody.css-0>tr").expect("Could not create selector.");
    let stock_name_selector = Selector::parse(".css-ttxvk0>p").expect("Could not create selector.");
    //println!("{}",body);
    // let stock_code_selector = Selector::parse("div.css-ttxvk0>span").expect("Could not create selector.");
    let stock_price_selector = Selector::parse("div.css-0").expect("Could not create selector.");
    let stock_perce_selector=Selector::parse("div.css-16q9pr7>p").expect("Could not create selector.");  
    let stock_24v_selector = Selector::parse("td.css-1nh9lk8").expect("Could not create selector.");
    let stock_marketcap_selector = Selector::parse("tr.css-1cxc880 :nth-child(7)").expect("Could not create selector.");
    let mut wtr = csv::Writer::from_path("Stocks.csv").expect("Could not create file.");
    wtr.write_record(&["Stock Name", "Stock Code", "Price","24volume","Market Cap"]).expect("Could not write header.");
    for element in document.select(&stock)
    {
        let stock_name_element = element.select(&stock_name_selector).next().expect("could not select the selector");
        let stock_name=stock_name_element.text().collect::<String>();
        let stock_price_element=element.select(&stock_price_selector).next().expect("could not find");
        let stock_price=stock_price_element.text().collect::<String>();
        let stock_perce_element=element.select(&stock_perce_selector).next().expect("Could not find it");
        let stock_perce=stock_perce_element.text().collect::<String>();
        let stock_24v_element=element.select(&stock_24v_selector).next().expect("Could not find");
        let stock_24v = stock_24v_element.text().collect::<String>();
        let stock_marketcap_element=element.select(&stock_marketcap_selector).next().expect("Could not find");
        let stock_marketcap = stock_marketcap_element.text().collect::<String>();
        println!("{} {} {} {} {} ",stock_name,stock_price,stock_perce,stock_24v,stock_marketcap);
        wtr.write_record([&stock_name, &stock_price, &stock_perce, &stock_24v, &stock_marketcap]).expect("Could not create selector.");
    }  
    wtr.flush().expect("Could not close file");
    println!("Done");
}