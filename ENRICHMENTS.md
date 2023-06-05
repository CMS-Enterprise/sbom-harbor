# Implementing a New Enrichment Provider.

In this tutorial, we will walk through the steps required to create a new enrichment provider.
We will add support for enriching `Vulnerability` entries with an [EPSS Score](https://www.first.org/epss/model).

## Updating the CLI

Enrichments are run as CLI commands. In order to handle a new command we will need to modify the
`cli/src/commmands/enrich/mod.rs` file in the following ways.

Add new enum value to `EnrichmentProviderKind`

```rust
pub enum EnrichmentProviderKind {
    // ...
    /// Use the EPSS enrichment provider.
    Epss,
    // ...
}
```

Add a new `match` handler to the `execute` function.

```rust
pub async fn execute(args: &EnrichArgs) -> Result<(), Error> {
    match args.provider {
        // ...
        EnrichmentProviderKind::Epss => EpssProvider::execute(args).await,
        // ...
    }
}
```

Update the `ValueEnum` implementation to handle the new enum variant.

```rust
impl ValueEnum for EnrichmentProviderKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            // ...
            Self::Epss,
            // ...
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            // ...
            Self::Epss => PossibleValue::new("epss").help("Run EPSS enrichment"),
            // ...
        })
    }
}
```

Update the `FromStr` implementation to handle the new enum variant.

```rust
impl FromStr for EnrichmentProviderKind {
    type Err = ();

    fn from_str(s: &str) -> Result<EnrichmentProviderKind, Self::Err> {
        // ...
        match value {
            // ...
            "epss" => Ok(EnrichmentProviderKind::Epss),
            // ..
        }
    }
}
```

Optionally, if the new provider supports or requires args, you can define a type, and add them to
the `EnrichArgs` struct as optional fields. See the `SnykArgs` struct as an example.

## Updating Core

Add new variant to `VulnerabilityProviderKind`

```rust
pub enum VulnerabilityProviderKind {
    // ...
    /// EPSS Score provider.
    Epss,
    // ...
}
```

Update the `Display` implementation for `VulnerabilityProviderKind` to handle the new variant.

```rust
impl Display for VulnerabilityProviderKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // ...
            VulnerabilityProviderKind::Epss => write!(f, "epss"),
            // ...
        }
    }
}
```
