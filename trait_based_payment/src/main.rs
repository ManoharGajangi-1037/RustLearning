trait PaymentProcesser{
    fn process_payment(&self,amount:f64)->String;
}
struct Paypal;
struct Crypto;

impl PaymentProcesser for Paypal{
    fn process_payment(&self,amount:f64)->String {
        format!("Processing payment of paypal via {}",amount)
    }
}
impl PaymentProcesser for Crypto{
    fn process_payment(&self,amount:f64)->String {
        format!("Processing payment of Crypto via {}",amount)
    }
}

fn handle_payment_process(processor:Box<dyn PaymentProcesser>,amount:f64)->String{
    processor.process_payment(amount)
}

fn main() {
    let paypal=Box::new(Paypal);
    let crypto=Box::new(Crypto);
    println!("{}",handle_payment_process(paypal, 24.0));
}
