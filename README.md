BEncoder
========

A pure rust library for encoding and decoding BEncoded strings. More
information about this [here](https://wiki.theory.org/BitTorrentSpecification#Bencoding).

#### Usage

**Decode**

```rust
BEncoder::decode("ll5:helloei-10ee".to_string());

// Produces:
BType::List(vec![
    BType::List(vec![BType::ByteString("hello".to_string())]),
    BType::Integer(-10)
        ])
```

**Encode**

```rust
BEncoder::encode(BType::List(vec![BType::ByteString("hello".to_string())]));

// Produces:
"l5:helloe"
```

#### License

MIT
