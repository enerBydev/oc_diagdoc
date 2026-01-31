//! Macros declarativas para oc_diagdoc.
//!
//! Proporciona macros útiles para reducir boilerplate.
//! Nota: oc_err! y bail! están en errors.rs

/// Macro para implementar Display automáticamente.
#[macro_export]
macro_rules! impl_display {
    ($type:ty, $field:ident) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.$field)
            }
        }
    };
    ($type:ty => $fmt:expr) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", ($fmt)(self))
            }
        }
    };
}

/// Macro para definir enums con Display implementado.
#[macro_export]
macro_rules! define_enum_with_display {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $display:expr
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )*
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant => write!(f, "{}", $display),)*
                }
            }
        }
    };
}

/// Macro para implementar FromStr para enums simples.
#[macro_export]
macro_rules! impl_from_str {
    ($type:ty, $($str:expr => $variant:expr),* $(,)?) => {
        impl std::str::FromStr for $type {
            type Err = String;
            
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_lowercase().as_str() {
                    $($str => Ok($variant),)*
                    other => Err(format!("Unknown value: {}", other)),
                }
            }
        }
    };
}

/// Macro para logging de fases de verificación.
#[macro_export]
macro_rules! log_phase {
    ($phase:expr, $name:expr) => {
        eprintln!("⚙️  Fase {}: {}", $phase, $name);
    };
    ($phase:expr, $name:expr, $result:expr) => {
        let icon = if $result { "✅" } else { "❌" };
        eprintln!("{} Fase {}: {}", icon, $phase, $name);
    };
}

/// Macro para benchmark simple.
#[macro_export]
macro_rules! benchmark {
    ($name:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let elapsed = start.elapsed();
        eprintln!("⏱️  {}: {:?}", $name, elapsed);
        result
    }};
    ($name:expr, $block:expr) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let elapsed = start.elapsed();
        eprintln!("⏱️  {}: {:?}", $name, elapsed);
        result
    }};
}

/// Macro para crear un hashmap literal.
#[macro_export]
macro_rules! hashmap {
    () => {
        std::collections::HashMap::new()
    };
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
}

/// Macro para crear un HashSet literal.
#[macro_export]
macro_rules! hashset {
    () => {
        std::collections::HashSet::new()
    };
    ($($value:expr),* $(,)?) => {{
        let mut set = std::collections::HashSet::new();
        $(set.insert($value);)*
        set
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_hashmap_macro_empty() {
        let map: HashMap<String, i32> = hashmap!();
        assert!(map.is_empty());
    }

    #[test]
    fn test_hashmap_macro() {
        let map = hashmap!(
            "a" => 1,
            "b" => 2
        );
        assert_eq!(map.get("a"), Some(&1));
    }

    #[test]
    fn test_hashset_macro() {
        let set = hashset!(1, 2, 3);
        assert!(set.contains(&1));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_benchmark_macro() {
        let result = benchmark!("test", { 1 + 1 });
        assert_eq!(result, 2);
    }
}
