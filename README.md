# Reading Multi-Table CSVs in Rust

There isn't a universally accepted standard for CSV files. I recently downloaded a CSV from my bank and it looked sort of like this...

```csv
Account Number,Investment Name,Symbol,Shares,Share Price,Total Value,
1234567,NVIDIA Corp,NVDA,1,130.7,130.7,



Account Number,Trade Date,Settlement Date,Transaction Type,Transaction Description,Investment Name,Symbol,Shares,Share Price,Principal Amount,Commissions and Fees,Net Amount,Accrued Interest,Account Type,
1234567,2023-03-27,2023-03-27,Buy,Buy,Nvidia Corp,NVDA,0.00000,1.0,0.30,0.0,0.30,0.0,CASH,

```

One file with two tables in it. Interesting. Let's try and convert that table data to nice Rust data structure...

The [csv crate](https://crates.io/crates/csv) let's you customize how CSVs are read using the [ReaderBuilder](https://docs.rs/csv/1.3.0/csv/struct.ReaderBuilder.html). Since we're dealing with a multi-table CSV we'll need to customize how the reader works.

1. We'll disable automatic header reading. We need to handle that for ourselves. (using [has_headers](https://docs.rs/csv/1.3.0/csv/struct.ReaderBuilder.html#method.has_headers))

2. We'll disable the automatic validation of column sizes. (using [flexible](https://docs.rs/csv/1.3.0/csv/struct.ReaderBuilder.html#method.flexible))

3. Since our CSV separates tables with 3 blank lines, we'll need to preserve those lines as we're iterating over the data. There isn't an option to preserve blank lines in the current version of [csv](https://crates.io/crates/csv), but there is a [pull request](https://github.com/BurntSushi/rust-csv/pull/308) to enable that functionality. We'll use that PR which gives us the `skip_blank_lines` option on the reader. We can use the PR branch as our csv dependency in `Cargo.toml`.

```toml
[dependencies]
# csv = "1.3.0"
csv = { git = "https://github.com/Yarn/rust-csv.git" }
```

4. Finally we'll point our csv reader to the data structure that implements the [Read trait](https://doc.rust-lang.org/nightly/std/io/trait.Read.html) (using [from_reader](https://docs.rs/csv/1.3.0/csv/struct.ReaderBuilder.html#method.from_reader)) In this case, I'll be using `std::io::BufReader` setup with `std::fs::File`.

This is how we'll setup our csv reader...

```rust
let mut rdr = csv::ReaderBuilder::new()
  .has_headers(false)
  .flexible(true)
  .skip_blank_lines(false)
  .from_reader(reader);
```

We can now iterate over the records and collect the headers and columns into a custom CsvTable struct. See `read_csv_to_tables()` in [src/main.rs](src/main.rs). This allows us to return a Vector of generic CsvTables.

Once we have the CsvTables, we can make some assumptions about the table ordering and what data type should represent those tables.

In [src/types.rs](src/types.rs) I've defined two types for the data in my `documents/example.csv`. The first table in the CSV can be represented with the `Account` struct and the second with the `Transaction` struct. Each of those types will derive instances of the `serde::Deserialize` and `Debug` traits. We'll also use serde's alias property so we define which column name goes with which struct field. This allows us to define that "Account Name" maps to account_name, "Share Price" maps to share_price, etc...

Finally, we need to create a `deserialize` function on the `CsvTable` struct (see main.rs). This function will basically loop over the rows and call [csv::StringRecord::deserialize](https://docs.rs/csv/latest/csv/struct.StringRecord.html#method.deserialize) on each row. Note that this function defines a [trait bound](https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax) `E` for the `Deserialize` trait. This ensures that the type we are *deserializing to* implements `Deserialize`. Since we intend on converting `CsvTable` into `Account` and `Transaction`, and we derived `Deserialize` for both, then we can convert the appropriate table into either of those types. See the `main` function in [src/main.rs](src/main.rs) for the final conversion of the `Vec<CsvTable>` into the `Vec<Account>` and `Vec<Transaction>` types.

The csv file is now converted into a structure we can use in our program...

```
[
    Account {
        account_number: "1234567",
        investment_name: "NVIDIA Corp",
        symbol: "NVDA",
        shares: 1.0,
        share_price: 130.7,
        total_value: 130.7,
    },
]
[
    Transation {
        account_number: "1234567",
        trade_date: "2023-03-27",
        settlement_date: "2023-03-27",
        transaction_type: "Buy",
        transaction_description: "Buy",
        investment_name: "Nvidia Corp",
        symbol: "NVDA",
        shares: 0.0,
        share_price: 1.0,
        principal_amount: 0.3,
        commissions_and_fees: 0.0,
        net_amount: 0.3,
        accrued_interest: 0.0,
        account_type: "CASH",
    },
]
```

