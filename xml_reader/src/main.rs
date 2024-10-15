use clap::Parser;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Parser)]
struct Args {
    #[arg()]
    input_file: String,
}


#[derive(Debug, Default, Clone)]
struct Product {
    id: u32,
    category: String,
    name: String,
}

#[derive(Debug, Default, Clone)]
struct Sale{
    id: String,
    product_id: u32,
    date: u32,
    quantity: f32,
    unit: String,
}


#[derive(Debug, Default)]
struct SalesAndProducts {
    products: Vec<Product>,
    sales: Vec<Sale>,
}

enum LocationItem {
    Other,
    InProduct,
    InSale,
}
enum LocationProduct {
    Other,
    InId,
    InCategory,
    InName,
}
enum LocationSale {
    Other,
    InId,
    InProductId,
    InDate,
    InQuantity,
    InUnit,
}

fn parse_xml(filename: String) -> SalesAndProducts {
    let file = std::fs::File::open(filename).unwrap();
    let file = std::io::BufReader::new(file);
    let parser = EventReader::new(file);

    let mut location_item = LocationItem::Other;
    let mut location_product = LocationProduct::Other;
    let mut location_sale = LocationSale::Other;
    
    let mut product = Product::default();
    let mut sale= Sale::default();
    let mut sales_and_products = SalesAndProducts::default();

    for event in parser{
        match &location_item {
            LocationItem::Other => match event {
                
                Ok(XmlEvent::StartElement {ref name, ..}) if name.local_name == "product" => {
                    location_item = LocationItem::InProduct;
                    location_product = LocationProduct::Other;
                }

                Ok(XmlEvent::StartElement { ref name, ..}) if name.local_name == "sale" => {
                    location_item = LocationItem::InSale;
                    location_sale = LocationSale::Other;
                    sale = Sale::default();
                }
                
                _ => {}
            },

            LocationItem::InProduct => match &location_product {
                LocationProduct::Other => match event {
                    Ok(XmlEvent::StartElement { ref name,..}) if name.local_name == "id" => {
                        location_product = LocationProduct::InId;
                    }
                
                    Ok(XmlEvent::StartElement { ref name,..}) if name.local_name == "category" => {
                        location_product = LocationProduct::InCategory;
                    }

                    Ok(XmlEvent::StartElement { ref name,..}) if name.local_name == "name" => {
                        location_product = LocationProduct::InName;
                    }

                    Ok(XmlEvent::EndElement { .. }) => {
                        location_item = LocationItem::Other;
                        sales_and_products.products.push(product.clone());
                    }

                    _ => {}
                }

                LocationProduct::InId => match event {
                    Ok(XmlEvent::Characters(chars)) => product.id = chars.parse::<u32>().unwrap(),
                    Ok(XmlEvent::EndElement { .. }) => location_product = LocationProduct::Other,
                    _ => {}
                }

                LocationProduct::InCategory => match event {
                    Ok(XmlEvent::Characters(chars)) => product.category = chars,
                    Ok(XmlEvent::EndElement { .. }) => location_product = LocationProduct::Other,
                    _ => {}
                }

                LocationProduct::InName => match event {
                    Ok(XmlEvent::Characters(chars)) => product.name = chars,
                    Ok(XmlEvent::EndElement { .. }) => location_product = LocationProduct::Other,
                    _ => {}
                }

            },

            LocationItem::InSale => match &location_sale {
                LocationSale::Other => match event {
                    Ok(XmlEvent::StartElement { ref name, ..}) if name.local_name == "id" => {
                        location_sale = LocationSale::InId;
                    }

                    Ok(XmlEvent::StartElement { ref name, ..}) if name.local_name == "product-id" => {
                        location_sale = LocationSale::InProductId;
                    }

                    Ok(XmlEvent::StartElement { ref name, ..}) if name.local_name == "date" => {
                        location_sale = LocationSale::InDate;
                    }

                    Ok(XmlEvent::StartElement { ref name, ..}) if name.local_name == "quantity" => {
                        location_sale = LocationSale::InQuantity;
                    }

                    Ok(XmlEvent::StartElement { ref name, ..}) if name.local_name == "unity" => {
                        location_sale = LocationSale::InUnit;
                    }

                    Ok(XmlEvent::EndElement { .. }) => {
                        location_item = LocationItem::Other;
                        sales_and_products.sales.push(sale.clone());
                    }

                    _ => {}

                }

                LocationSale::InId => match event {
                    Ok(XmlEvent::Characters(chars)) => sale.id = chars,
                    Ok(XmlEvent::EndElement { .. }) => location_sale = LocationSale::Other,
                    _ => {}
                }

                LocationSale::InProductId => match event {
                    Ok(XmlEvent::Characters(chars)) => sale.product_id = chars.parse::<u32>().unwrap(),
                    Ok(XmlEvent::EndElement { .. }) => location_sale = LocationSale::Other,
                    _ => {}
                }

                LocationSale::InDate => match event {
                    Ok(XmlEvent::Characters(chars)) => sale.date = chars.parse::<u32>().unwrap(),
                    Ok(XmlEvent::EndElement { .. }) => location_sale = LocationSale::Other,
                    _ => {}
                }

                LocationSale::InQuantity => match event {
                    Ok(XmlEvent::Characters(chars)) => sale.quantity = chars.parse::<f32>().unwrap(),
                    Ok(XmlEvent::EndElement { .. }) => location_sale = LocationSale::Other,
                    _ => {}
                }

                LocationSale::InUnit => match event {
                    Ok(XmlEvent::Characters(chars)) => sale.unit = chars,
                    Ok(XmlEvent::EndElement { .. }) => location_sale = LocationSale::Other,
                    _ => {}
                }
                
            }
        }
    }

    sales_and_products
}


fn main() {
    let args= Args::parse();
    let filename = args.input_file;
    let sales_and_products = parse_xml(filename);

    println!("Products:");
    for product in sales_and_products.products {
        println!("  id: {}, category: {}, name: {}", product.id, product.category, product.name);
    }
    
    println!("\nSales:");
    for sale in sales_and_products.sales {
        println!("  id: {}, product-id: {}, date: {}, quantity: {}, unit: {}", sale.id, sale.product_id, sale.date, sale.quantity, sale.unit);
    }

}
