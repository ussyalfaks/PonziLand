use std::{fmt::Display, ops::Deref};

use num_bigint::{BigInt, Sign};
use sqlx::{types::BigDecimal, Postgres, Type};
use torii_ingester::{conversions::FromPrimitive, prelude::U256 as RawU256};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct U256(RawU256);

impl FromPrimitive for U256 {
    fn from_primitive(
        primitive: &torii_ingester::conversions::Primitive,
    ) -> Result<Self, torii_ingester::conversions::Error> {
        Ok(U256(RawU256::from_primitive(primitive)?))
    }
}

impl Display for U256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        RawU256::fmt(&self.0, f)
    }
}

impl From<BigDecimal> for U256 {
    fn from(value: BigDecimal) -> Self {
        let data: Vec<u64> = value
            .as_bigint_and_exponent()
            .0
            .iter_u64_digits()
            .chain(std::iter::repeat(0))
            .take(4)
            .collect();

        U256(RawU256::from_u64_words([
            data[0], data[1], data[2], data[3],
        ]))
    }
}

impl From<U256> for BigDecimal {
    fn from(value: U256) -> Self {
        let data = value.0.to_u64_words();

        BigInt::from_slice(
            Sign::Plus,
            &[
                data[0] as u32,
                (data[0] >> 32) as u32,
                data[1] as u32,
                (data[1] >> 32) as u32,
                data[2] as u32,
                (data[2] >> 32) as u32,
                data[3] as u32,
                (data[3] >> 32) as u32,
            ],
        )
        .into()
    }
}

impl Deref for U256 {
    type Target = RawU256;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Type<Postgres> for U256 {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        BigDecimal::type_info()
    }

    fn compatible(ty: &<Postgres as sqlx::Database>::TypeInfo) -> bool {
        *ty == Self::type_info()
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for U256 {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let decoded = BigDecimal::decode(value)?;

        Ok(decoded.into())
    }
}

impl<'q> sqlx::Encode<'q, Postgres> for U256 {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        BigDecimal::encode((*self).into(), buf)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use sqlx::PgPool;

    async fn setup_test_db(pool: &PgPool) {
        // Create the custom type and test table
        sqlx::query(
            r#"
            DO $$
            BEGIN
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'uint_256') THEN
                    CREATE DOMAIN uint_256 AS NUMERIC NOT NULL
                    CHECK (VALUE >= 0 AND VALUE < 2::numeric ^ 256)
                    CHECK (SCALE(VALUE) = 0);
                END IF;
            END $$;
        "#,
        )
        .execute(pool)
        .await
        .expect("Failed to create test type");

        // Create the test table
        sqlx::query(
            r#"
            DROP TABLE IF EXISTS u256_test;
        "#,
        )
        .execute(pool)
        .await
        .expect("Failed to drop table if exists");

        sqlx::query(
            r#"
            CREATE TABLE u256_test (
                id SERIAL PRIMARY KEY,
                value uint_256 NOT NULL
            );
        "#,
        )
        .execute(pool)
        .await
        .expect("Failed to create test table");
    }

    #[sqlx::test(migrations = false)]
    #[ignore = "Test is using a database, only running it if wanted."]
    async fn sqlx_test_u256_db_conversion(pool: PgPool) {
        setup_test_db(&pool).await;

        // Test values
        let test_values = [U256(RawU256::from_u64_words([1, 0, 0, 0])),
            // Medium value
            U256(RawU256::from_u64_words([0xFFFFFFFF, 0, 0, 0])),
            // Larger value
            U256(RawU256::from_u64_words([0xFFFFFFFF, 0xFFFFFFFF, 0, 0])),
            // Max value
            U256(RawU256::from_u64_words([
                0xFFFFFFFFFFFFFFFF,
                0xFFFFFFFFFFFFFFFF,
                0xFFFFFFFFFFFFFFFF,
                0xFFFFFFFFFFFFFFFF,
            ]))];

        for (idx, value) in test_values.iter().enumerate() {
            // Insert value
            let (id,): (i32,) =
                sqlx::query_as(r#"INSERT INTO u256_test (value) VALUES ($1) RETURNING id"#)
                    .bind(value)
                    .fetch_one(&pool)
                    .await
                    .expect("Failed to insert test value");

            // Fetch and compare
            let (result,): (U256,) =
                sqlx::query_as(r#"SELECT value as "value: U256" FROM u256_test WHERE id = $1"#)
                    .bind(id)
                    .fetch_one(&pool)
                    .await
                    .expect("Failed to fetch test value");

            // Compare the retrieved value with the original
            assert_eq!(
                result, *value,
                "Test case {}: Conversion roundtrip failed",
                idx
            );
        }

        // Clean up
        sqlx::query!("DROP TABLE IF EXISTS u256_test;")
            .execute(&pool)
            .await
            .expect("Failed to drop test table");
    }

    #[test]
    fn test_u256_from_bigdecimal() {
        // Test case 1: Small value
        let small_decimal = BigDecimal::from_str("1234").unwrap();
        let u256_from_small = U256::from(small_decimal.clone());
        let decimal_from_u256 = BigDecimal::from(u256_from_small);
        assert_eq!(small_decimal, decimal_from_u256);

        // Test case 2: Medium value
        let medium_decimal = BigDecimal::from_str("123456789012345678901234").unwrap();
        let u256_from_medium = U256::from(medium_decimal.clone());
        let decimal_from_u256 = BigDecimal::from(u256_from_medium);
        assert_eq!(medium_decimal, decimal_from_u256);

        // Test case 3: Large value within U256 range
        let large_decimal = BigDecimal::from_str(
            "115792089237316195423570985008687907853269984665640564039457584007913129639935",
        )
        .unwrap();
        let u256_from_large = U256::from(large_decimal.clone());
        let decimal_from_u256 = BigDecimal::from(u256_from_large);
        assert_eq!(large_decimal, decimal_from_u256);
    }

    #[test]
    fn test_bigdecimal_from_u256() {
        // Create a U256 from its words
        let u256_small = U256(RawU256::from_u64_words([1234, 0, 0, 0]));
        let decimal_from_small = BigDecimal::from(u256_small);
        let u256_from_decimal = U256::from(decimal_from_small);
        assert_eq!(u256_small.0, u256_from_decimal.0);

        // Medium value with multiple words
        let u256_medium = U256(RawU256::from_u64_words([0xdeadbeef, 0xcafebabe, 0, 0]));
        let decimal_from_medium = BigDecimal::from(u256_medium);
        let u256_from_decimal = U256::from(decimal_from_medium);
        assert_eq!(u256_medium.0, u256_from_decimal.0);

        // Large value using all words
        let u256_large = U256(RawU256::from_u64_words([
            0xdeadbeef12345678,
            0xcafebabe87654321,
            0x1122334455667788,
            0x8877665544332211,
        ]));
        let decimal_from_large = BigDecimal::from(u256_large);
        let u256_from_decimal = U256::from(decimal_from_large);
        assert_eq!(u256_large.0, u256_from_decimal.0);
    }

    #[test]
    fn test_edge_cases() {
        // Test with zero
        let zero_decimal = BigDecimal::from_str("0").unwrap();
        let u256_from_zero = U256::from(zero_decimal.clone());
        let decimal_from_u256 = BigDecimal::from(u256_from_zero);
        assert_eq!(zero_decimal, decimal_from_u256);

        // Test with 1
        let one_decimal = BigDecimal::from_str("1").unwrap();
        let u256_from_one = U256::from(one_decimal.clone());
        let decimal_from_u256 = BigDecimal::from(u256_from_one);
        assert_eq!(one_decimal, decimal_from_u256);

        // Test with max U256 value
        let max_u256 = U256(RawU256::from_u64_words([
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
        ]));
        let decimal_from_max = BigDecimal::from(max_u256);
        let u256_from_decimal = U256::from(decimal_from_max.clone());
        assert_eq!(max_u256.0, u256_from_decimal.0);
    }
}
