// global constants
const day: u32 = 86_400;
const seconds_in_year: u32 = 31_557_600;

// enums

enum AccidentStatus {
    YesAccident(u8),
    NoAccident(u8),
}

enum PaymentCategory {
    VehiclePurchase(u8),
    Subscription(u8),
    VehicleSale(u8),
}

enum PersonStatus {
    Active(u8),
    Inactive(u8),
}

enum RoundStatus {
    First(u8),
    Continuation(u8),
    Final(u8),
    FirstAndFinal(u8),
    Removed(u8),
}

enum SubscriptionPaymentMethod {
    Linear(u8),
    Square(u8),
    Logarithmic(u8),
    Stepwise(u8),
}

enum VehicleStatus {
    InSubscription(u8),
    CanBeSwapped(u8),
    Inactive(u8),
    ReadyForSale(u8),
    Sold(u8),
}

enum YearlyDepreciation {
    DiscreteYears(u8),
    Daily(u8),
}

// structs

struct Purchase {
    original: u32,
    e_reg: u32,
    e_pre: u32,
    total: u32,
}

struct Vehicle {
    id: u32,
    make: String,
    model: String,
    submodel: String,
    trim: String,
    name: String,
    price: u32,
    km: u32,
    year: u32,
    days_in_subscription: u32,
    category: u8,
    status: u8,
    sales_date: u32,
    final_sales_date: u32,
    person_id: u32,
    subscription_id: u32,
    bought_at: u32,
}

// helper functions
fn e_reg(price: u32, tax: u32) -> f32 {
    (price as f32) * ((tax as f32) / 100.0)
}
fn e_pre(price: u32, lease_initial: u32) -> f32 {
    (price as f32) * ((lease_initial as f32) / 100.0)
}
fn e_int(price: u32, lease_initial: u32, interest: u32) -> f32 {
    (price as f32) - e_pre(price, lease_initial) * (interest as f32) / 100.0 / 12.0
}
fn purchase_price(price: u32, tax: u32, lease_initial: u32) -> Purchase {
    let e_reg_value: f32 = e_reg(price, tax);
    let e_pre_value: f32 = e_pre(price, lease_initial);
    Purchase {
        original: price,
        e_reg: e_reg_value as u32,
        e_pre: e_pre_value as u32,
        total: -(e_reg_value + e_pre_value) as u32,
    }
}
fn price_linear(price: u32, subscription_price_factor: u32) -> f32 {
    (price as f32) / (subscription_price_factor as f32)
}
fn price_log(price: f32, factor_a: f32, factor_b: f32, factor_c: f32, factor_d: f32) -> f32 {
    factor_a * (price + factor_b).log(factor_d) + factor_c
}

fn main() {
    const updated_at: u32 = 1534649378;
    const initial_person_count: u32 = 15;
    const extension_percentage: u32 = 65;
    const monthly_growth_rate: u32 = 25;
    const same_vehicle: u32 = 50;
    const same_category: u32 = 30;
    const alternate_category_1: u32 = 15;
    const alternate_category_2: u32 = 5;
    const same_price: u32 = 50;
    const alternate_price_1: u32 = 30;
    const alternate_price_2: u32 = 20;
    const popularity_0: u32 = 10;
    const popularity_1: u32 = 10;
    const popularity_2: u32 = 10;
    const popularity_3: u32 = 10;
    const popularity_4: u32 = 10;
    const popularity_5: u32 = 10;
    const popularity_6: u32 = 10;
    const popularity_7: u32 = 10;
    const popularity_8: u32 = 10;
    const popularity_9: u32 = 10;
    const subscription_price_factor: u32 = 50;
    const subscription_length: u32 = 180;
    const market_start: u32 = 7;
    const market_limit_min: u32 = 10;
    const market_limit_max: u32 = 60;
    const lease_initial_percentage: u32 = 90;
    const annual_accident: u32 = 25;
    const vehicle_tax: u32 = 10;
    const vehicle_interest: u32 = 5;
    const insurance_min: u32 = 0;
    const insurance_max: u32 = 10;
    const resell_min: u32 = 0;
    const resell_max: u32 = 10;
    const absent_same_category: u32 = 33;
    const absent_alternate_1: u32 = 33;
    const absent_leave_service: u32 = 34;
    const absent_next_option: u32 = 50;
    const absent_next_leave_service: u32 = 50;
    const vehicle_limit: u32 = 0;
    const vehicle_limit_increase_value: u32 = 0;
    const vehicle_limit_increase_lower: u32 = 0;
    const vehicle_limit_increase_upper: u32 = 0;
    const simulation_start: u32 = 1534649371;
    const simulation_end: u32 = 1629343771;
    const subscription_payment_method: u32 = 1;
    const price_factor_log_a: u32 = 1;
    const price_factor_log_b: u32 = 2;
    const price_factor_log_c: u32 = 3;
    const price_factor_log_d: u32 = 4;
    const price_factor_square_a: u32 = 1;
    const price_factor_square_b: u32 = 2;
    const price_factor_square_c: u32 = 3;
    const price_factor_square_d: u32 = 4;
    const yearly_depreciation_method: u32 = 1;
    const stepwise_lower_0: u32 = 0;
    const stepwise_upper_0: u32 = 0;
    const stepwise_value_0: u32 = 0;
    const stepwise_lower_1: u32 = 0;
    const stepwise_upper_1: u32 = 0;
    const stepwise_value_1: u32 = 0;
    const stepwise_lower_2: u32 = 0;
    const stepwise_upper_2: u32 = 0;
    const stepwise_value_2: u32 = 0;
    const stepwise_lower_3: u32 = 0;
    const stepwise_upper_3: u32 = 0;
    const stepwise_value_3: u32 = 0;
    const stepwise_lower_4: u32 = 0;
    const stepwise_upper_4: u32 = 0;
    const stepwise_value_4: u32 = 0;
    const stepwise_lower_5: u32 = 0;
    const stepwise_upper_5: u32 = 0;
    const stepwise_value_5: u32 = 0;
    const stepwise_lower_6: u32 = 0;
    const stepwise_upper_6: u32 = 0;
    const stepwise_value_6: u32 = 0;
    const stepwise_lower_7: u32 = 0;
    const stepwise_upper_7: u32 = 0;
    const stepwise_value_7: u32 = 0;
    const stepwise_lower_8: u32 = 0;
    const stepwise_upper_8: u32 = 0;
    const stepwise_value_8: u32 = 0;
    const stepwise_lower_9: u32 = 0;
    const stepwise_upper_9: u32 = 0;
    const stepwise_value_9: u32 = 0;
    const stepwise_default: u32 = 0;

    // helper variables
    const subscription_in_seconds: u32 = day * subscription_length;
    const months_in_subscription: u32 = subscription_length / 30;
    const daily_accident: f32 = (annual_accident as f32) / 365.0;
}
