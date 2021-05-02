use crate::*;

test_case!(timestamp, async move {
    run!(
        r#"
CREATE TABLE TimestampLog (
    id INTEGER,
    t1 TIMESTAMP,
    t2 TIMESTAMP,
)"#
    );

    run!(
        r#"
INSERT INTO TimestampLog VALUES
    (1, "2020-06-11 11:23:11Z",           "2021-03-01"),
    (2, "2020-09-30 12:00:00 -07:00",     "1989-01-01T00:01:00+09:00"),
    (3, "2021-04-30T07:00:00.1234-17:00", "2021-05-01T09:00:00.1234+09:00");
"#
    );

    macro_rules! t {
        ($timestamp: expr) => {
            $timestamp.parse().unwrap()
        };
    }

    use Value::*;

    test!(
        Ok(select!(
            id  | t1                             | t2
            I64 | Timestamp                      | Timestamp;
            1     t!("2020-06-11T11:23:11")        t!("2021-03-01T00:00:00");
            2     t!("2020-09-30T19:00:00")        t!("1988-12-31T15:01:00");
            3     t!("2021-05-01T00:00:00.1234")   t!("2021-05-01T00:00:00.1234")
        )),
        "SELECT id, t1, t2 FROM TimestampLog"
    );

    test!(
        Ok(select!(
            id  | t1                        | t2
            I64 | Timestamp                 | Timestamp;
            2     t!("2020-09-30T19:00:00")   t!("1988-12-31T15:01:00")
        )),
        "SELECT * FROM TimestampLog WHERE t1 > t2"
    );

    test!(
        Ok(select!(
            id  | t1                             | t2
            I64 | Timestamp                      | Timestamp;
            3     t!("2021-05-01T00:00:00.1234")   t!("2021-05-01T00:00:00.1234")
        )),
        "SELECT * FROM TimestampLog WHERE t1 = t2"
    );

    test!(
        Ok(select!(
            id  | t1                        | t2
            I64 | Timestamp                 | Timestamp;
            1     t!("2020-06-11T11:23:11")   t!("2021-03-01T00:00:00")

        )),
        r#"SELECT * FROM TimestampLog WHERE t1 = "2020-06-11T14:23:11+0300";"#
    );

    test!(
        Ok(select!(
            id  | t1                        | t2
            I64 | Timestamp                 | Timestamp;
            2     t!("2020-09-30T19:00:00")   t!("1988-12-31T15:01:00")
        )),
        r#"SELECT * FROM TimestampLog WHERE t2 < TIMESTAMP "2000-01-01";"#
    );

    test!(
        Ok(select!(
            id  | t1                             | t2
            I64 | Timestamp                      | Timestamp;
            1     t!("2020-06-11T11:23:11")        t!("2021-03-01T00:00:00");
            2     t!("2020-09-30T19:00:00")        t!("1988-12-31T15:01:00");
            3     t!("2021-05-01T00:00:00.1234")   t!("2021-05-01T00:00:00.1234")
        )),
        r#"SELECT * FROM TimestampLog WHERE TIMESTAMP "1999-01-03" < "2000-01-01";"#
    );

    test!(
        Err(ValueError::FailedToParseTimestamp("12345-678".to_owned()).into()),
        r#"INSERT INTO TimestampLog VALUES (1, "12345-678", "2021-05-01")"#
    );
});