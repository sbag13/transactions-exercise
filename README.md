# Rust transactions exercise

This is a simple program that reads a list of transactions from a csv file and prints the balance of each account to the stdout.

## Implementation

### Type system

The types created in the implementation are intended to minimize incorrect usage regarding the whole process. E.g. `TransactionStore`, which essentially is just a hash map, exposes only 2 necessary methods (immutable get and insert) so the transaction can be either retrieved or inserted/overwritten. Lack of mutable getter means there is no need for reverting any changes to an existing transaction in case of failure during a process - transaction is committed at the end of a process.

### Error handling

A nested enum is used to represent processing errors, allowing for differentiation between various failure reasons and enabling scenario-specific reactions. However, since the only action here is printing to stderr, using `anyhow::Result` would be sufficient too.

### Efficiency

Reading input data is handled using iterators, allowing the input file to be processed in chunks without needing to load the entire dataset into memory. Similarly, generating the list of clients for output is also implemented with an iterator, enabling each record to be written directly to a generic output sink as it is processed.

### Float output precision

A 64-bit floating point type is used internally for processing; however, the output precision is restricted to 4 decimal places. This is accomplished by implementing a custom serializer for the float fields.

### Assumptions

- The client id should be the same for referred and referrer
- Transaction type string is case insensitive (custom deserializer implemented)
- input csv file has header with column names

### Tests

Main tests are placed in two locations:

- [engine unit tests](./src/engine/tests.rs) - checks the correctness of the balance calculation
- [integration tests](./src/tests.rs) - reads the input from a file ([input files](./test_files/)) and compare the output with the expected results.