trait Investment<T> {
    fn get_amount(&self) -> T;

    fn double_amount(&mut self);
}

trait Taxable: Investment<f64> {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.get_amount() * Self::TAX_RATE
    }
}

struct Income {
    amount: f64,
}

impl Investment<f64> for Income {
    fn get_amount(&self) -> f64 {
        self.amount
    }

    fn double_amount(&mut self) {
        self.amount *= 2.0;
    }
}

impl Taxable for Income {}

struct Bonus {
    value: f64,
}

impl Investment<f64> for Bonus {
    fn get_amount(&self) -> f64 {
        self.value
    }

    fn double_amount(&mut self) {
        self.value *= 2.0;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.5;
}

struct QualityTime {
    minutes: u32,
}

impl Investment<u32> for QualityTime {
    fn get_amount(&self) -> u32 {
        self.minutes
    }
    
    fn double_amount(&mut self) {
        self.minutes *= 2;
    }
}

fn main() {
    let mut income = Income { amount: 50_000.50 };
    println!("Income tax owned: ${:.2}", income.tax_bill());
    income.double_amount();
    println!("Income double tax owned: ${:.2}", income.tax_bill());

    let mut bonus = Bonus { value: 10_000.25 };
    println!("Bonus tax owned: ${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Bonus double tax owned: ${:.2}", bonus.tax_bill());

    let weekend = QualityTime { minutes: 58 * 60 };
    println!("Relation time: {:.2} minutes", weekend.get_amount());
}
